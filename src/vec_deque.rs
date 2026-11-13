use crate::helpers::calculate_extra_capacity;
use std::collections::VecDeque;

/// Extension trait for [`VecDeque`] providing adaptive capacity control for both front and back
/// push operations.
pub trait AdaptiveVecDequeExt<T> {
    /// Pushes a value to the back using a custom capacity strategy closure.
    ///
    /// When the deque is at full capacity (`len == capacity`), `strategy` is invoked with
    /// the current capacity and should return the desired target capacity.
    ///
    /// If `strategy` returns a value `<= current_cap`, the suggestion is ignored and
    /// standard [`VecDeque::push_back`] reallocation behavior is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_adaptive_push::AdaptiveVecDequeExt;
    /// use std::collections::VecDeque;
    ///
    /// let mut deque = VecDeque::new();
    /// deque.push_back_with(1, |cap| cap + 8);
    /// deque.push_front_by_step(0, 4);
    /// assert!(deque.capacity() >= 8);
    /// ```
    fn push_back_with<F>(&mut self, value: T, strategy: F)
    where
        F: FnOnce(usize) -> usize;

    /// Pushes a value to the back while increasing capacity by a fixed `step` size when full.
    fn push_back_by_step(&mut self, value: T, step: usize) {
        self.push_back_with(value, |current_cap| current_cap.saturating_add(step));
    }

    /// Pushes a value to the back while multiplying capacity by `factor` when full.
    ///
    /// Note: If the current capacity is `0`, `0 * factor` evaluates to `0`, so the first
    /// push falls back to the deque's default initial allocation before applying `factor`
    /// on subsequent reallocations.
    fn push_back_by_factor(&mut self, value: T, factor: usize) {
        self.push_back_with(value, |current_cap| current_cap.saturating_mul(factor));
    }

    /// Pushes a value to the back while ensuring capacity reaches at least `target_cap` when full.
    fn push_back_to_target(&mut self, value: T, target_cap: usize) {
        self.push_back_with(value, |current_cap| target_cap.max(current_cap));
    }

    /// Pushes a value to the front using a custom capacity strategy closure.
    ///
    /// When the deque is at full capacity (`len == capacity`), `strategy` is invoked with
    /// the current capacity and should return the desired target capacity.
    ///
    /// If `strategy` returns a value `<= current_cap`, the suggestion is ignored and
    /// standard [`VecDeque::push_front`] reallocation behavior is used.
    fn push_front_with<F>(&mut self, value: T, strategy: F)
    where
        F: FnOnce(usize) -> usize;

    /// Pushes a value to the front while increasing capacity by a fixed `step` size when full.
    fn push_front_by_step(&mut self, value: T, step: usize) {
        self.push_front_with(value, |current_cap| current_cap.saturating_add(step));
    }

    /// Pushes a value to the front while multiplying capacity by `factor` when full.
    ///
    /// Note: If the current capacity is `0`, `0 * factor` evaluates to `0`, so the first
    /// push falls back to the deque's default initial allocation before applying `factor`
    /// on subsequent reallocations.
    fn push_front_by_factor(&mut self, value: T, factor: usize) {
        self.push_front_with(value, |current_cap| current_cap.saturating_mul(factor));
    }

    /// Pushes a value to the front while ensuring capacity reaches at least `target_cap` when full.
    fn push_front_to_target(&mut self, value: T, target_cap: usize) {
        self.push_front_with(value, |current_cap| target_cap.max(current_cap));
    }
}

/// Implementation of [`AdaptiveVecDequeExt`] for standard [`VecDeque<T>`].
impl<T> AdaptiveVecDequeExt<T> for VecDeque<T> {
    fn push_back_with<F>(&mut self, value: T, strategy: F)
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

        self.push_back(value);
    }

    fn push_front_with<F>(&mut self, value: T, strategy: F)
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

        self.push_front(value);
    }
}