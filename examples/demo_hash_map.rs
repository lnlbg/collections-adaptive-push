//! Adaptive capacity strategies example for `HashMap`.
//!
//! Run with:
//! `cargo run --example hash_map`

use collections_adaptive_push::AdaptiveHashMapExt;
use std::collections::HashMap;

fn main() {
    // 1. Step strategy: increase capacity by a fixed step
    let mut step_map = HashMap::new();
    step_map.insert_by_step("alpha", 1, 16);
    println!("Step: len={}, cap={}", step_map.len(), step_map.capacity());

    // 2. Factor strategy: multiply capacity when full
    let mut factor_map = HashMap::with_capacity(4);
    let initial_cap = factor_map.capacity();
    for i in 0..=initial_cap {
        factor_map.insert_by_factor(i, i * 10, 2);
    }
    println!(
        "Factor (x2 from {initial_cap}): len={}, cap={}",
        factor_map.len(),
        factor_map.capacity()
    );

    // 3. Target strategy: reserve at least `target_cap` slots
    let mut target_map = HashMap::new();
    target_map.insert_to_target("rust", 2024, 32);
    println!(
        "Target (min 32): len={}, cap={}",
        target_map.len(),
        target_map.capacity()
    );

    // 4. Custom closure strategy: capped exponential growth
    let mut custom_map = HashMap::new();
    for i in 0..40 {
        custom_map.insert_with(i, format!("val-{i}"), |current_cap| {
            let doubled = current_cap.max(8) * 2;
            doubled.min(current_cap + 32)
        });
    }
    println!(
        "Custom (capped): len={}, cap={}",
        custom_map.len(),
        custom_map.capacity()
    );

    // 5. Fallback behavior: returning `<= current_cap` delegates to standard `HashMap::insert`
    let mut fallback_map = HashMap::with_capacity(2);
    let cap_before = fallback_map.capacity();
    for i in 0..=cap_before {
        fallback_map.insert_with(i, i, |current_cap| current_cap);
    }
    println!(
        "Fallback: len={}, cap={}",
        fallback_map.len(),
        fallback_map.capacity()
    );
}