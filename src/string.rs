/// Extension trait for [`String`] providing adaptive capacity control during character push
/// operations.
pub trait AdaptiveStringExt {
    /// Pushes a character using a custom capacity strategy closure.
    ///
    /// When the remaining capacity cannot fit the UTF-8 byte length of `ch`
    /// (`capacity - len < ch.len_utf8()`), `strategy` is invoked with the current byte capacity
    /// and should return the desired target byte capacity.
    ///
    /// If `strategy` returns a value `< len + ch.len_utf8()`, the suggestion is ignored and
    /// standard [`String::push`] reallocation behavior is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collections_adaptive_push::AdaptiveStringExt;
    ///
    /// let mut text = String::new();
    /// text.push_with('🦀', |cap| cap + 16);
    /// assert!(text.capacity() >= 16);
    /// ```
    fn push_with<F>(&mut self, ch: char, strategy: F)
    where
        F: FnOnce(usize) -> usize;

    /// Pushes a character while increasing byte capacity by a fixed `step` size when needed.
    fn push_by_step(&mut self, ch: char, step: usize) {
        self.push_with(ch, |current_cap| current_cap.saturating_add(step));
    }

    /// Pushes a character while multiplying byte capacity by `factor` when needed.
    ///
    /// Note: If the current capacity is `0`, `0 * factor` evaluates to `0`, so the first
    /// push falls back to the string's default initial allocation before applying `factor`
    /// on subsequent reallocations.
    fn push_by_factor(&mut self, ch: char, factor: usize) {
        self.push_with(ch, |current_cap| current_cap.saturating_mul(factor));
    }

    /// Pushes a character while ensuring byte capacity reaches at least `target_cap` when needed.
    fn push_to_target(&mut self, ch: char, target_cap: usize) {
        self.push_with(ch, |current_cap| target_cap.max(current_cap));
    }
}

/// Implementation of [`AdaptiveStringExt`] for standard [`String`].
impl AdaptiveStringExt for String {
    fn push_with<F>(&mut self, ch: char, strategy: F)
    where
        F: FnOnce(usize) -> usize,
    {
        let len = self.len();
        let current_cap = self.capacity();
        let needed_bytes = ch.len_utf8();

        // Check spare byte capacity against the character's UTF-8 encoded length.
        if current_cap.saturating_sub(len) < needed_bytes {
            let target_cap = strategy(current_cap);
            let min_required_cap = len.saturating_add(needed_bytes);
            if target_cap >= min_required_cap {
                self.reserve_exact(target_cap - len);
            }
        }

        self.push(ch);
    }
}