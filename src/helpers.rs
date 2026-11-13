/// Calculates the extra capacity needed based on a provided strategy closure.
///
/// Compares the current capacity with the target capacity returned by `strategy`.
/// Returns the difference if the target capacity is strictly greater than `current_cap`,
/// otherwise returns `0`.
pub(crate) fn calculate_extra_capacity<F>(current_cap: usize, strategy: F) -> usize
where
    F: FnOnce(usize) -> usize,
{
    let target_cap = strategy(current_cap);
    if target_cap > current_cap {
        return target_cap - current_cap;
    }
    0
}