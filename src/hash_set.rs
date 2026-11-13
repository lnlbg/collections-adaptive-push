use crate::helpers::calculate_extra_capacity;
use std::collections::HashSet;
use std::hash::Hash;

/// Extension trait for [`HashSet`] providing adaptive capacity control during insert operations.
pub trait AdaptiveHashSetExt<T> {
    /// Inserts a value using a custom capacity strategy closure.
    ///
    /// When the set is at full capacity (`len == capacity`), `strategy` is invoked with
    /// the current capacity and should return the desired target capacity.
    ///
    /// Returns `true` if the value was newly inserted, or `false` if it was already present
    /// (matching [`HashSet::insert`]).
    ///
    /// Note that `HashSet` manages its own internal load factor via [`HashSet::reserve`],
    /// so the resulting capacity may be greater than or equal to the requested target.
    /// If `strategy` returns a value `<= current_cap`, the suggestion is ignored and
    /// standard [`HashSet::insert`] reallocation behavior is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_adaptive_push::AdaptiveHashSetExt;
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::new();
    /// assert!(set.insert_with("rust", |cap| cap + 16));
    /// assert!(set.capacity() >= 16);
    /// ```
    fn insert_with<F>(&mut self, value: T, strategy: F) -> bool
    where
        F: FnOnce(usize) -> usize;

    /// Inserts a value while increasing capacity by a fixed `step` size when full.
    fn insert_by_step(&mut self, value: T, step: usize) -> bool {
        self.insert_with(value, |current_cap| current_cap.saturating_add(step))
    }

    /// Inserts a value while multiplying capacity by `factor` when full.
    ///
    /// Note: If the current capacity is `0`, `0 * factor` evaluates to `0`, so the first
    /// insertion falls back to the set's default initial allocation before applying `factor`
    /// on subsequent reallocations.
    fn insert_by_factor(&mut self, value: T, factor: usize) -> bool {
        self.insert_with(value, |current_cap| current_cap.saturating_mul(factor))
    }

    /// Inserts a value while ensuring capacity reaches at least `target_cap` when full.
    fn insert_to_target(&mut self, value: T, target_cap: usize) -> bool {
        self.insert_with(value, |current_cap| target_cap.max(current_cap))
    }
}

/// Implementation of [`AdaptiveHashSetExt`] for standard [`HashSet<T>`].
impl<T: Eq + Hash> AdaptiveHashSetExt<T> for HashSet<T> {
    fn insert_with<F>(&mut self, value: T, strategy: F) -> bool
    where
        F: FnOnce(usize) -> usize,
    {
        let len = self.len();
        let current_cap = self.capacity();

        // Check capacity without calling `contains` first to avoid hashing `value` twice.
        if len == current_cap {
            let extra_cap = calculate_extra_capacity(current_cap, strategy);
            if extra_cap > 0 {
                // `HashSet` exposes `reserve` rather than `reserve_exact` due to load-factor management.
                self.reserve(extra_cap);
            }
        }

        self.insert(value)
    }
}