#![warn(missing_docs)]
//! # Collections Adaptive Push
//!
//! A lightweight Rust library providing adaptive capacity growth strategies and custom
//! push/insert extensions for standard library collections.
//!
//! Each supported collection offers:
//! - **3 built-in growth strategies**:
//!   - **Step** (`*_by_step`): increases capacity linearly by a fixed increment.
//!   - **Factor** (`*_by_factor`): scales capacity multiplicatively by a given factor.
//!   - **Target** (`*_to_target`): ensures capacity reaches at least a specific minimum.
//! - **Custom closure strategies** (`*_with`): accepts any `FnOnce(usize) -> usize` closure
//!   to compute the desired target capacity from the current capacity.
//!
//! > **Note:** If the custom strategy closure returns an insufficient capacity
//! > (i.e., `<= current_cap`, or `< len + ch.len_utf8()` for `String`), the operation
//! > safely ignores the suggestion and falls back to the collection's built-in
//! > growth behavior (such as standard `push` / `insert` reallocation).
//!
//! ## Quick Example
//!
//! ```rust
//! use collections_adaptive_push::{AdaptiveStringExt, AdaptiveVecExt};
//!
//! let mut numbers = Vec::new();
//! numbers.push_by_step(1, 16);
//! numbers.push_with(2, |cap| cap + 32);
//!
//! let mut text = String::new();
//! text.push_to_target('🦀', 64);
//! assert!(text.capacity() >= 64);
//! ```

/// Adaptive capacity extension for [`std::collections::BinaryHeap`].
pub mod binary_heap;
/// Adaptive capacity extension for [`std::collections::HashMap`].
pub mod hash_map;
/// Adaptive capacity extension for [`std::collections::HashSet`].
pub mod hash_set;
pub(crate) mod helpers;
/// Adaptive capacity extension for [`String`].
pub mod string;
/// Adaptive capacity extension for [`Vec`].
pub mod vec;
/// Adaptive capacity extension for [`std::collections::VecDeque`].
pub mod vec_deque;

pub use binary_heap::AdaptiveBinaryHeapExt;
pub use hash_map::AdaptiveHashMapExt;
pub use hash_set::AdaptiveHashSetExt;
pub use string::AdaptiveStringExt;
pub use vec::AdaptiveVecExt;
pub use vec_deque::AdaptiveVecDequeExt;