/*!
# Lodash-RS: High-Performance Rust Collection Library

A type-safe, high-performance Rust implementation of Lodash collection methods with zero-cost abstractions.

## Features

- **100% API Compatible**: Complete compatibility with Lodash collection methods
- **Type Safe**: Leverages Rust's type system for compile-time safety
- **Zero-Cost Abstractions**: No runtime overhead compared to hand-written code
- **Chainable**: Support for fluent method chaining
- **Async Support**: Built-in async/await support for all operations
- **Parallel Processing**: Automatic parallelization for large collections
- **Memory Safe**: Guaranteed memory safety with no data races
- **WASM Compatible**: Full WebAssembly support for browser usage

## Quick Start

```rust
use rust_lodash::prelude::*;

// Basic usage
let doubled = map(&[1, 2, 3, 4], |x| x * 2);
assert_eq!(doubled, vec![2, 4, 6, 8]);

// Chainable operations
let result = chain(&[1, 2, 3, 4, 5])
    .filter(|x| x % 2 == 0)
    .map(|x| x * 3)
    .collect();
assert_eq!(result, vec![6, 12]);

// Async operations (requires async feature)
// let async_result = map_async(&[1, 2, 3], |x| async move { x * 2 }).await;
// assert_eq!(async_result, vec![2, 4, 6]);
```

## Architecture

The library is organized into several modules:

- `collection`: Core collection methods (iteration, query, transform, operations)
- `chain`: Fluent method chaining system
- `utils`: Utility functions and type conversions
- `extensions`: Advanced features (parallel processing, WASM support)

## Performance

Lodash-RS is designed for maximum performance:

- **SIMD Optimizations**: Automatic vectorization for numeric operations
- **Parallel Processing**: Multi-threaded execution for large datasets
- **Zero-Copy Operations**: Minimal memory allocation and copying
- **Cache-Friendly**: Optimized memory layout for better cache performance

## Safety

- **Memory Safe**: No buffer overflows, use-after-free, or data races
- **Type Safe**: Compile-time type checking prevents runtime errors
- **Panic-Free**: All operations handle edge cases gracefully
- **Error Handling**: Comprehensive error types with proper propagation
*/

// #![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// Core modules
pub mod collection;
pub mod chain;
pub mod utils;
pub mod extensions;

// Re-exports for convenience
pub mod prelude {
    //! Prelude module containing the most commonly used items.

    // Core types
    pub use crate::collection::Collection;
    pub use crate::chain::Chain;

    // Iteration methods
    pub use crate::collection::iteration::{
        each, for_each, map, filter, reduce, reduce_right, for_each_right,
    };

    // Query methods
    pub use crate::collection::query::{
        find, find_last, includes, every, some, count_by, partition,
    };

    // Transform methods
    pub use crate::collection::transform::{
        group_by, key_by, invoke, sort_by, order_by,
    };

    // Collection operations
    pub use crate::collection::operation::{
        shuffle, sample, sample_size, size,
    };

    // Chain operations
    pub use crate::chain::chain;
    
    #[cfg(feature = "async")]
    pub use crate::chain::chain_async;

    // Async versions
    #[cfg(feature = "async")]
    pub use crate::collection::async_support::{
        map_async, filter_async, reduce_async, for_each_async,
    };

    // Parallel versions
    #[cfg(feature = "parallel")]
    pub use crate::extensions::parallel::{
        map_parallel, filter_parallel, reduce_parallel,
    };

    // Error types
    pub use crate::utils::error::{LodashError, Result};
}

// Version information
/// The version of the rust-lodash crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Feature detection
#[cfg(feature = "async")]
pub const HAS_ASYNC: bool = true;

#[cfg(not(feature = "async"))]
/// Whether async features are enabled.
pub const HAS_ASYNC: bool = false;

#[cfg(feature = "parallel")]
pub const HAS_PARALLEL: bool = true;

#[cfg(not(feature = "parallel"))]
/// Whether parallel processing features are enabled.
pub const HAS_PARALLEL: bool = false;

#[cfg(feature = "wasm")]
pub const HAS_WASM: bool = true;

#[cfg(not(feature = "wasm"))]
/// Whether WebAssembly features are enabled.
pub const HAS_WASM: bool = false;
