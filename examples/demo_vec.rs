//! Adaptive capacity strategies example for `Vec`.
//!
//! Run with:
//! `cargo run --example vec`

use collections_adaptive_push::AdaptiveVecExt;

fn main() {
    // 1. Step strategy: grow linearly by a fixed step size
    let mut step_vec = Vec::new();
    step_vec.push_by_step(1, 16);
    println!("Step: len={}, cap={}", step_vec.len(), step_vec.capacity());

    // 2. Factor strategy: multiply capacity when full
    let mut factor_vec = Vec::with_capacity(4);
    let initial_cap = factor_vec.capacity();
    for i in 0..=initial_cap {
        factor_vec.push_by_factor(i, 3);
    }
    println!(
        "Factor (x3 from {initial_cap}): len={}, cap={}",
        factor_vec.len(),
        factor_vec.capacity()
    );

    // 3. Target strategy: ensure at least a specific minimum capacity
    let mut target_vec = Vec::new();
    target_vec.push_to_target("hello", 32);
    println!(
        "Target (min 32): len={}, cap={}",
        target_vec.len(),
        target_vec.capacity()
    );

    // 4. Custom closure strategy: hybrid exponential-to-linear growth
    let mut custom_vec = Vec::with_capacity(4);
    for i in 0..40 {
        custom_vec.push_with(i, |current_cap| {
            if current_cap < 32 {
                current_cap.max(4) * 2
            } else {
                current_cap + 12
            }
        });
    }
    println!(
        "Custom (hybrid): len={}, cap={}",
        custom_vec.len(),
        custom_vec.capacity()
    );

    // 5. Fallback behavior: returning `<= current_cap` delegates to standard `Vec::push`
    let mut fallback_vec = Vec::with_capacity(2);
    fallback_vec.push(1);
    fallback_vec.push(2);
    fallback_vec.push_with(3, |current_cap| current_cap);
    println!(
        "Fallback: len={}, cap={}",
        fallback_vec.len(),
        fallback_vec.capacity()
    );
}