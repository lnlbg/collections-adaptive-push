//! Adaptive capacity strategies example for `String`.
//!
//! Run with:
//! `cargo run --example string`

use collections_adaptive_push::AdaptiveStringExt;

fn main() {
    // 1. Step strategy: grow byte capacity by a fixed step
    let mut step_str = String::new();
    step_str.push_by_step('A', 16);
    println!(
        "Step: text={:?}, bytes={}, cap={}",
        step_str,
        step_str.len(),
        step_str.capacity()
    );

    // 2. Factor strategy: multiply byte capacity when spare space runs out
    let mut factor_str = String::with_capacity(4);
    let initial_cap = factor_str.capacity();
    for _ in 0..=initial_cap {
        factor_str.push_by_factor('x', 3);
    }
    println!(
        "Factor (x3 from {initial_cap}): bytes={}, cap={}",
        factor_str.len(),
        factor_str.capacity()
    );

    // 3. Target strategy: UTF-8 aware minimum byte capacity
    let mut target_str = String::new();
    target_str.push_to_target('🦀', 32);
    println!(
        "Target (min 32): text={:?}, bytes={}, cap={}",
        target_str,
        target_str.len(),
        target_str.capacity()
    );

    // 4. Custom closure strategy: align byte capacity to multiples of 16
    let mut custom_str = String::new();
    for ch in "Adaptive Rust 🚀".chars() {
        custom_str.push_with(ch, |current_cap| {
            let needed = current_cap + ch.len_utf8();
            (needed + 15) & !15
        });
    }
    println!(
        "Custom (16-byte aligned): text={:?}, bytes={}, cap={}",
        custom_str,
        custom_str.len(),
        custom_str.capacity()
    );

    // 5. Fallback behavior: '🦀' needs 4 bytes, so `current_cap + 1` is insufficient
    //    and safely falls back to standard `String::push` reallocation.
    let mut fallback_str = String::new();
    fallback_str.push_with('🦀', |current_cap| current_cap + 1);
    println!(
        "Fallback: text={:?}, bytes={}, cap={}",
        fallback_str,
        fallback_str.len(),
        fallback_str.capacity()
    );
}