use crate::helpers::calculate_extra_capacity;

/// Extension trait for [`Vec`] providing adaptive capacity control during push operations.
pub trait AdaptiveVecExt<T> {
    /// Pushes a value using a custom capacity strategy closure.
    ///
    /// When the vector is at full capacity (`len == capacity`), `strategy` is invoked with
    /// the current capacity and should return the desired target capacity.
    ///
    /// If `strategy` returns a value `<= current_cap`, the suggestion is ignored and
    /// standard [`Vec::push`] reallocation behavior is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_adaptive_push::AdaptiveVecExt;
    ///
    /// let mut vec = Vec::new();
    /// vec.push_with(10, |cap| cap + 8);
    /// assert!(vec.capacity() >= 8);
    /// ```
    fn push_with<F>(&mut self, value: T, strategy: F)
    where
        F: FnOnce(usize) -> usize;

    /// Pushes a value while increasing capacity by a fixed `step` size when full.
    fn push_by_step(&mut self, value: T, step: usize) {
        self.push_with(value, |current_cap| current_cap.saturating_add(step));
    }

    /// Pushes a value while multiplying capacity by `factor` when full.
    ///
    /// Note: If the current capacity is `0`, `0 * factor` evaluates to `0`, so the first
    /// push falls back to the vector's default initial allocation before applying `factor`
    /// on subsequent reallocations.
    fn push_by_factor(&mut self, value: T, factor: usize) {
        self.push_with(value, |current_cap| current_cap.saturating_mul(factor));
    }

    /// Pushes a value while ensuring capacity reaches at least `target_cap` when full.
    fn push_to_target(&mut self, value: T, target_cap: usize) {
        self.push_with(value, |current_cap| target_cap.max(current_cap));
    }
}

/// Implementation of [`AdaptiveVecExt`] for standard [`Vec<T>`].
impl<T> AdaptiveVecExt<T> for Vec<T> {
    fn push_with<F>(&mut self, value: T, strategy: F)
    where
        F: FnOnce(usize) -> usize,
    {
        let len = self.len();
        let current_cap = self.capacity();

        // Only evaluate the strategy when a reallocation is actually needed.
        if len == current_cap {
            let extra_cap = calculate_extra_capacity(current_cap, strategy);
            if extra_cap > 0 {
                self.reserve_exact(extra_cap);
            }
        }

        self.push(value);
    }
}