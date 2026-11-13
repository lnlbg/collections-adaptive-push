use crate::helpers::calculate_extra_capacity;
use std::collections::HashMap;
use std::hash::Hash;

/// Extension trait for [`HashMap`] providing adaptive capacity control during insert operations.
pub trait AdaptiveHashMapExt<K, V> {
    /// Inserts a key-value pair using a custom capacity strategy closure.
    ///
    /// When the map is at full capacity (`len == capacity`), `strategy` is invoked with
    /// the current capacity and should return the desired target capacity.
    ///
    /// Returns the previous value if the key was already present (matching [`HashMap::insert`]).
    ///
    /// Note that `HashMap` manages its own internal load factor via [`HashMap::reserve`],
    /// so the resulting capacity may be greater than or equal to the requested target.
    /// If `strategy` returns a value `<= current_cap`, the suggestion is ignored and
    /// standard [`HashMap::insert`] reallocation behavior is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_adaptive_push::AdaptiveHashMapExt;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert_with("rust", 2024, |cap| cap + 16), None);
    /// assert!(map.capacity() >= 16);
    /// ```
    fn insert_with<F>(&mut self, key: K, value: V, strategy: F) -> Option<V>
    where
        F: FnOnce(usize) -> usize;

    /// Inserts a key-value pair while increasing capacity by a fixed `step` size when full.
    fn insert_by_step(&mut self, key: K, value: V, step: usize) -> Option<V> {
        self.insert_with(key, value, |current_cap| current_cap.saturating_add(step))
    }

    /// Inserts a key-value pair while multiplying capacity by `factor` when full.
    ///
    /// Note: If the current capacity is `0`, `0 * factor` evaluates to `0`, so the first
    /// insertion falls back to the map's default initial allocation before applying `factor`
    /// on subsequent reallocations.
    fn insert_by_factor(&mut self, key: K, value: V, factor: usize) -> Option<V> {
        self.insert_with(key, value, |current_cap| current_cap.saturating_mul(factor))
    }

    /// Inserts a key-value pair while ensuring capacity reaches at least `target_cap` when full.
    fn insert_to_target(&mut self, key: K, value: V, target_cap: usize) -> Option<V> {
        self.insert_with(key, value, |current_cap| target_cap.max(current_cap))
    }
}

/// Implementation of [`AdaptiveHashMapExt`] for standard [`HashMap<K, V>`].
impl<K: Eq + Hash, V> AdaptiveHashMapExt<K, V> for HashMap<K, V> {
    fn insert_with<F>(&mut self, key: K, value: V, strategy: F) -> Option<V>
    where
        F: FnOnce(usize) -> usize,
    {
        let len = self.len();
        let current_cap = self.capacity();

        // Check capacity without calling `contains_key` first to avoid hashing `key` twice.
        if len == current_cap {
            let extra_cap = calculate_extra_capacity(current_cap, strategy);
            if extra_cap > 0 {
                // `HashMap` exposes `reserve` rather than `reserve_exact` due to load-factor management.
                self.reserve(extra_cap);
            }
        }

        self.insert(key, value)
    }
}