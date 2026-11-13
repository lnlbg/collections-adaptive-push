//! Adaptive capacity strategies example for `HashSet`.
//!
//! Run with:
//! `cargo run --example hash_set`

use collections_adaptive_push::AdaptiveHashSetExt;
use std::collections::HashSet;

fn main() {
    // 1. Step strategy: increase capacity by a fixed step
    let mut step_set = HashSet::new();
    let inserted = step_set.insert_by_step("alpha", 16);
    println!(
        "Step: inserted={inserted}, len={}, cap={}",
        step_set.len(),
        step_set.capacity()
    );

    // 2. Factor strategy: multiply capacity when full
    let mut factor_set = HashSet::with_capacity(4);
    let initial_cap = factor_set.capacity();
    for i in 0..=initial_cap {
        factor_set.insert_by_factor(i, 2);
    }
    println!(
        "Factor (x2 from {initial_cap}): len={}, cap={}",
        factor_set.len(),
        factor_set.capacity()
    );

    // 3. Target strategy: reserve at least `target_cap` slots
    let mut target_set = HashSet::new();
    target_set.insert_to_target("rust", 32);
    println!(
        "Target (min 32): len={}, cap={}",
        target_set.len(),
        target_set.capacity()
    );

    // 4. Custom closure strategy: linear growth in chunks of 16
    let mut custom_set = HashSet::new();
    for i in 0..35 {
        custom_set.insert_with(i, |current_cap| current_cap + 16);
    }
    println!(
        "Custom (+16): len={}, cap={}",
        custom_set.len(),
        custom_set.capacity()
    );

    // 5. Fallback behavior: returning `<= current_cap` delegates to standard `HashSet::insert`
    let mut fallback_set = HashSet::with_capacity(2);
    let cap_before = fallback_set.capacity();
    for i in 0..=cap_before {
        fallback_set.insert_with(i, |current_cap| current_cap);
    }
    println!(
        "Fallback: len={}, cap={}",
        fallback_set.len(),
        fallback_set.capacity()
    );
}