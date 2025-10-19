# Lodash-RS

[![Crates.io](https://img.shields.io/crates/v/lodash-rs.svg)](https://crates.io/crates/lodash-rs)
[![Documentation](https://docs.rs/lodash-rs/badge.svg)](https://docs.rs/lodash-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, type-safe Rust implementation of Lodash collection methods with zero-cost abstractions.

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

Add this to your `Cargo.toml`:

```toml
[dependencies]
lodash-rs = "0.1.0"
```

### Basic Usage

```rust
use lodash_rs::prelude::*;

// Basic operations
let doubled = map(&[1, 2, 3, 4], |x| x * 2);
assert_eq!(doubled, vec![2, 4, 6, 8]);

// Chainable operations
let result = chain(&[1, 2, 3, 4, 5])
    .filter(|x| x % 2 == 0)
    .map(|x| x * 3)
    .collect::<Vec<_>>();
assert_eq!(result, vec![6, 12]);

// Query operations
let first_even = find(&[1, 2, 3, 4, 5], |x| x % 2 == 0);
assert_eq!(first_even, Some(&2));

// Transform operations
let users = vec![("john", 30), ("jane", 25), ("bob", 35)];
let sorted = sort_by(&users, |(_, age)| *age);
assert_eq!(sorted[0], ("jane", 25));
```

### Async Usage

```rust
use lodash_rs::prelude::*;

#[tokio::main]
async fn main() {
    // Async operations
    let doubled = map_async(&[1, 2, 3], |x| async move { x * 2 }).await;
    assert_eq!(doubled, vec![2, 4, 6]);
    
    // Async chain
    let result = chain_async(&[1, 2, 3, 4, 5])
        .filter_async(|x| async move { x % 2 == 0 })
        .map_async(|x| async move { x * 3 })
        .await;
    assert_eq!(result, vec![6, 12]);
}
```

### Parallel Usage

```rust
use lodash_rs::prelude::*;

// Parallel operations (requires parallel feature)
let numbers: Vec<i32> = (1..=10000).collect();
let doubled = map_parallel(&numbers, |x| x * 2);
let evens = filter_parallel(&numbers, |x| x % 2 == 0);
```

## API Reference

### Iteration Methods

- `each(collection, iteratee)` - Iterate over elements
- `map(collection, iteratee)` - Transform each element
- `filter(collection, predicate)` - Filter elements
- `reduce(collection, iteratee, initial)` - Reduce to a single value
- `reduce_right(collection, iteratee, initial)` - Reduce from right to left
- `for_each_right(collection, iteratee)` - Iterate from right to left

### Query Methods

- `find(collection, predicate)` - Find first matching element
- `find_last(collection, predicate)` - Find last matching element
- `includes(collection, value)` - Check if value exists
- `every(collection, predicate)` - Check if all elements match
- `some(collection, predicate)` - Check if any element matches
- `count_by(collection, iteratee)` - Count elements by key
- `partition(collection, predicate)` - Split into two groups

### Transform Methods

- `group_by(collection, iteratee)` - Group elements by key
- `key_by(collection, iteratee)` - Create object with keys
- `invoke(collection, method)` - Call method on each element
- `sort_by(collection, iteratee)` - Sort by key
- `order_by(collection, iteratee, ascending)` - Sort with order control

### Collection Operations

- `size(collection)` - Get collection size
- `shuffle(collection)` - Randomize order
- `sample(collection)` - Get random element
- `sample_size(collection, n)` - Get n random elements

### Chain Operations

- `chain(collection)` - Start method chaining
- `chain_async(collection)` - Start async method chaining

## Features

### Async Support

Enable async support with the `async` feature:

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["async"] }
```

### Parallel Processing

Enable parallel processing with the `parallel` feature:

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["parallel"] }
```

### WASM Support

Enable WebAssembly support with the `wasm` feature:

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["wasm"] }
```

### Serialization

Enable serialization support with the `serialize` feature:

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["serialize"] }
```

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

## Examples

See the `examples/` directory for more detailed usage examples:

- `basic_usage.rs` - Basic functionality
- `async_usage.rs` - Async operations
- `parallel_usage.rs` - Parallel processing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original [Lodash](https://lodash.com/) library
- Built with Rust's powerful type system and zero-cost abstractions
- Leverages the excellent [rayon](https://github.com/rayon-rs/rayon) crate for parallel processing
