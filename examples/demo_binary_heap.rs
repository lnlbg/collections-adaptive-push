//! Adaptive capacity strategies example for `BinaryHeap`.
//!
//! Run with:
//! `cargo run --example binary_heap`

use collections_adaptive_push::AdaptiveBinaryHeapExt;
use std::collections::BinaryHeap;

fn main() {
    // 1. Step strategy: grow linearly by a fixed step size
    let mut step_heap = BinaryHeap::new();
    step_heap.push_by_step(42, 16);
    println!(
        "Step: len={}, cap={}, top={:?}",
        step_heap.len(),
        step_heap.capacity(),
        step_heap.peek()
    );

    // 2. Factor strategy: multiply capacity when full
    let mut factor_heap = BinaryHeap::with_capacity(4);
    let initial_cap = factor_heap.capacity();
    for i in 0..=initial_cap {
        factor_heap.push_by_factor(i, 3);
    }
    println!(
        "Factor (x3 from {initial_cap}): len={}, cap={}, top={:?}",
        factor_heap.len(),
        factor_heap.capacity(),
        factor_heap.peek()
    );

    // 3. Target strategy: ensure at least a specific minimum capacity
    let mut target_heap = BinaryHeap::new();
    target_heap.push_to_target(100, 32);
    println!(
        "Target (min 32): len={}, cap={}",
        target_heap.len(),
        target_heap.capacity()
    );

    // 4. Custom closure strategy: add a dynamic margin based on current capacity
    let mut custom_heap = BinaryHeap::new();
    for i in 0..30 {
        custom_heap.push_with(i, |current_cap| current_cap + 10);
    }
    println!(
        "Custom (+10): len={}, cap={}, top={:?}",
        custom_heap.len(),
        custom_heap.capacity(),
        custom_heap.peek()
    );

    // 5. Fallback behavior: returning `<= current_cap` delegates to standard `BinaryHeap::push`
    let mut fallback_heap = BinaryHeap::with_capacity(2);
    fallback_heap.push(1);
    fallback_heap.push(2);
    fallback_heap.push_with(3, |current_cap| current_cap);
    println!(
        "Fallback: len={}, cap={}, top={:?}",
        fallback_heap.len(),
        fallback_heap.capacity(),
        fallback_heap.peek()
    );
}