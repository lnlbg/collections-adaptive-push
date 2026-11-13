# collections-adaptive-push

A lightweight, zero-dependency Rust library providing adaptive capacity growth strategies and custom push/insert extensions for standard library collections.

## Why `collections-adaptive-push`?

Standard Rust collections use a fixed internal growth strategy (typically doubling capacity). While great for general use, certain workloads—such as memory-constrained environments, streaming parsers, or predictable batching—benefit from finer control over reallocations without rewriting your own collection wrappers.

`collections-adaptive-push` extends standard collections via ergonomic extension traits offering:

- **3 ready-to-use growth strategies:**
  1. **Step (`*_by_step`)** — increases capacity linearly by a fixed increment (`current_cap + step`).
  2. **Factor (`*_by_factor`)** — scales capacity multiplicatively (`current_cap * factor`).
  3. **Target (`*_to_target`)** — ensures capacity reaches at least a specific minimum (`target_cap`).
- **Unlimited custom strategies (`*_with`):** Pass any closure (`FnOnce(usize) -> usize`) to define your own dynamic allocation logic (e.g., Fibonacci growth, page-aligned allocation, capped exponential growth, etc.).

## Installation

Add this to your `Cargo.toml`:

~~~toml
[dependencies]
collections-adaptive-push = "0.1.0"
~~~

Or run:

~~~bash
cargo add collections-adaptive-push
~~~

> **Requires Rust 1.85+** (uses Rust `2024` edition).

## Quick Start

Import the extension traits you need and call the adaptive methods directly on standard collections:

~~~rust
use collections_adaptive_push::{
    AdaptiveHashMapExt, AdaptiveStringExt, AdaptiveVecExt,
};
use std::collections::HashMap;

fn main() {
    // 1. Built-in Step strategy on Vec
    let mut numbers = Vec::new();
    numbers.push_by_step(42, 16); // Allocates 16 slots on first push

    // 2. Built-in Factor strategy on Vec
    numbers.push_by_factor(99, 3); // Multiplies capacity by 3 when full

    // 3. Built-in Target strategy on String (UTF-8 aware)
    let mut text = String::new();
    text.push_to_target('🦀', 64); // Ensures at least 64 bytes of capacity

    // 4. Custom lambda / closure strategy on HashMap
    let mut map = HashMap::new();
    map.insert_with("key", "value", |current_cap| {
        if current_cap < 128 {
            current_cap.max(8) * 2
        } else {
            current_cap + 32 // Switch from exponential to linear growth above 128
        }
    });
}
~~~

## Supported Collections & Traits

| Collection | Extension Trait | Adaptive Methods |
| :--- | :--- | :--- |
| `Vec<T>` | `AdaptiveVecExt` | `push_with`, `push_by_step`, `push_by_factor`, `push_to_target` |
| `VecDeque<T>` | `AdaptiveVecDequeExt` | `push_back_with`, `push_back_by_step`, `push_back_by_factor`, `push_back_to_target`, `push_front_with`, `push_front_by_step`, `push_front_by_factor`, `push_front_to_target` |
| `String` | `AdaptiveStringExt` | `push_with`, `push_by_step`, `push_by_factor`, `push_to_target` |
| `HashMap<K, V>` | `AdaptiveHashMapExt` | `insert_with`, `insert_by_step`, `insert_by_factor`, `insert_to_target` |
| `HashSet<T>` | `AdaptiveHashSetExt` | `insert_with`, `insert_by_step`, `insert_by_factor`, `insert_to_target` |
| `BinaryHeap<T>` | `AdaptiveBinaryHeapExt` | `push_with`, `push_by_step`, `push_by_factor`, `push_to_target` |

## Built-in & Custom Strategies

Every `*_by_step`, `*_by_factor`, and `*_to_target` method is built directly on top of the corresponding `*_with` method. You can combine built-in methods or plug in any custom closure:

~~~rust
use collections_adaptive_push::AdaptiveVecExt;

let mut buf = Vec::with_capacity(4);

// Custom: align capacity to multiples of 64 elements
buf.push_with(1, |cap| ((cap + 1) + 63) & !63);

// Custom: fallback safely if a strategy declines to grow
buf.push_with(2, |cap| cap); // Delegates to standard Vec::push behavior
~~~

> **Note:** If the custom strategy closure returns an insufficient capacity
> (i.e., `<= current_cap`, or `< len + ch.len_utf8()` for `String`), the operation
> safely ignores the suggestion and falls back to the collection's built-in
> growth behavior (such as standard `push` / `insert` reallocation).

### Good to Know

- **Zero initial capacity with `*_by_factor`:** If a collection starts with `capacity() == 0`, `0 * factor` evaluates to `0`. Per the fallback rule above, the first insertion uses the collection's standard initial allocation, and `factor` takes effect on subsequent reallocations.
- **UTF-8 awareness for `String`:** `AdaptiveStringExt` checks `ch.len_utf8()` before deciding whether reallocation is needed and ensures multi-byte characters fit safely.
- **Exact vs. minimum reservations:** `Vec`, `VecDeque`, `String`, and `BinaryHeap` use `reserve_exact`, while `HashMap` and `HashSet` use `reserve` (respecting the underlying hash table's load-factor requirements).

## Examples

Runnable examples for each collection are available in the `examples/` directory:

~~~bash
cargo run --example vec
cargo run --example vec_deque
cargo run --example string
cargo run --example hash_map
cargo run --example hash_set
cargo run --example binary_heap
~~~

## License

This project is dual-licensed under `MIT OR Apache-2.0` (either the MIT License or the Apache License, Version 2.0, at your option).