# lodash-rs

[![Crates.io](https://img.shields.io/crates/v/lodash-rs.svg)](https://crates.io/crates/lodash-rs)
[![Documentation](https://docs.rs/lodash-rs/badge.svg)](https://docs.rs/lodash-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/royalwang/lodash-rs/workflows/CI/badge.svg)](https://github.com/royalwang/lodash-rs/actions)

A high-performance, type-safe Rust implementation of Lodash collection methods with zero-cost abstractions.

## Features

- **🚀 High Performance**: Zero-cost abstractions with optimized implementations
- **🔒 Type Safety**: Full compile-time type checking and memory safety
- **🔗 Chainable API**: Fluent interface for method chaining
- **⚡ Async Support**: Optional async/await support for modern Rust
- **🔄 Parallel Processing**: Optional parallel iteration with rayon
- **🌐 WASM Compatible**: Full WebAssembly support for browser usage
- **📦 No Std Support**: Optional `no_std` support for embedded systems

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
    .collect();
assert_eq!(result, vec![6, 12]);

// Collection operations
let numbers = vec![1, 2, 3, 4, 5];
let evens = filter(&numbers, |x| x % 2 == 0);
let sum = reduce(&numbers, |acc, x| acc + x, 0);
```

### Advanced Features

#### Async Support (Optional)

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["async"] }
```

```rust
use lodash_rs::prelude::*;

// Async operations (requires async feature)
// let async_result = map_async(&[1, 2, 3], |x| async move { x * 2 }).await;
// assert_eq!(async_result, vec![2, 4, 6]);
```

#### Parallel Processing (Optional)

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["parallel"] }
```

```rust
use lodash_rs::prelude::*;

// Parallel operations (requires parallel feature)
// let result = map_parallel(&[1, 2, 3, 4], |x| x * 2);
// assert_eq!(result, vec![2, 4, 6, 8]);
```

#### WASM Support (Optional)

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["wasm"] }
```

## API Reference

### Collection Operations

#### Iteration
- `map(collection, iteratee)` - Transform each element
- `filter(collection, predicate)` - Filter elements by condition
- `reduce(collection, iteratee, initial)` - Reduce to single value
- `forEach(collection, iteratee)` - Execute function for each element
- `forEachRight(collection, iteratee)` - Execute function from right to left

#### Query
- `find(collection, predicate)` - Find first matching element
- `findLast(collection, predicate)` - Find last matching element
- `includes(collection, value)` - Check if value exists
- `every(collection, predicate)` - Check if all elements match
- `some(collection, predicate)` - Check if any element matches
- `countBy(collection, iteratee)` - Count elements by key
- `partition(collection, predicate)` - Split into two groups

#### Transform
- `groupBy(collection, iteratee)` - Group elements by key
- `keyBy(collection, iteratee)` - Create object with keys
- `sortBy(collection, iteratee)` - Sort elements by key
- `orderBy(collection, iteratee, descending)` - Sort with direction
- `invoke(collection, method)` - Invoke method on each element

#### Operations
- `shuffle(collection)` - Randomize element order
- `sample(collection)` - Get random element
- `sampleSize(collection, n)` - Get n random elements
- `size(collection)` - Get collection length

### Chain API

```rust
use lodash_rs::chain;

let result = chain(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    .filter(|x| x % 2 == 0)    // [2, 4, 6, 8, 10]
    .map(|x| x * 3)            // [6, 12, 18, 24, 30]
    .take(3)                   // [6, 12, 18]
    .reverse()                 // [18, 12, 6]
    .collect();                // Vec<i32>
```

## Examples

See the `examples/` directory for more detailed usage examples:

- `basic_usage.rs` - Comprehensive examples of all features
- `standalone_test.rs` - Simple functionality test

## Performance

lodash-rs is designed for high performance:

- **Zero-cost abstractions**: No runtime overhead for method chaining
- **Optimized algorithms**: Efficient implementations of all operations
- **Memory efficient**: Minimal allocations and copying
- **SIMD ready**: Prepared for future SIMD optimizations

## Compatibility

- **Rust**: 1.70+
- **Platforms**: Windows, macOS, Linux
- **Architectures**: x86_64, ARM64
- **WASM**: Full browser support

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original [Lodash](https://lodash.com/) JavaScript library
- Built with modern Rust best practices
- Community feedback and contributions

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed list of changes.

## Roadmap

- [ ] Additional collection methods
- [ ] Performance benchmarks
- [ ] SIMD optimizations
- [ ] More async operations
- [ ] Enhanced WASM bindings
- [ ] Documentation improvements