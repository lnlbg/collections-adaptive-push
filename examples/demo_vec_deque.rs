//! Adaptive capacity strategies example for `VecDeque`.
//!
//! Run with:
//! `cargo run --example vec_deque`

use collections_adaptive_push::AdaptiveVecDequeExt;
use std::collections::VecDeque;

fn main() {
    // 1. Step strategy (back and front)
    let mut step_deque = VecDeque::new();
    step_deque.push_back_by_step("back-1", 10);
    step_deque.push_front_by_step("front-1", 10);
    println!(
        "Step: len={}, cap={}",
        step_deque.len(),
        step_deque.capacity()
    );

    // 2. Factor strategy: multiply capacity when full
    let mut factor_deque = VecDeque::with_capacity(4);
    let initial_cap = factor_deque.capacity();
    for i in 0..=initial_cap {
        factor_deque.push_back_by_factor(i, 2);
    }
    println!(
        "Factor (x2 from {initial_cap}): len={}, cap={}",
        factor_deque.len(),
        factor_deque.capacity()
    );

    // 3. Target strategy: front push targeting a minimum capacity
    let mut target_deque = VecDeque::new();
    target_deque.push_front_to_target(42, 24);
    println!(
        "Target (min 24): len={}, cap={}",
        target_deque.len(),
        target_deque.capacity()
    );

    // 4. Custom closure strategy for both ends
    let mut custom_deque = VecDeque::new();
    for i in 0..20 {
        if i % 2 == 0 {
            custom_deque.push_back_with(i, |current_cap| current_cap + 8);
        } else {
            custom_deque.push_front_with(i, |current_cap| current_cap + 8);
        }
    }
    println!(
        "Custom: len={}, cap={}",
        custom_deque.len(),
        custom_deque.capacity()
    );

    // 5. Fallback behavior: returning `<= current_cap` delegates to standard reallocation
    let mut fallback_deque = VecDeque::with_capacity(2);
    let cap_before = fallback_deque.capacity();
    for i in 0..=cap_before {
        fallback_deque.push_front_with(i, |current_cap| current_cap);
    }
    println!(
        "Fallback: len={}, cap={}",
        fallback_deque.len(),
        fallback_deque.capacity()
    );
}