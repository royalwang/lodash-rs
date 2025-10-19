# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Performance benchmarks
- Additional collection methods
- SIMD optimizations
- Enhanced WASM bindings

## [0.1.0] - 2024-12-19

### Added
- Initial release of rust-lodash
- Core Collection type with all Lodash collection methods
- Iteration methods: each, map, filter, reduce, forEachRight
- Query methods: find, findLast, includes, every, some, countBy, partition
- Transform methods: groupBy, keyBy, invoke, sortBy, orderBy
- Operation methods: shuffle, sample, sampleSize, size
- Chainable API with fluent interface
- Comprehensive error handling with custom error types
- Type conversion utilities and async support
- Parallel processing and WASM support (optional features)
- Extensive documentation and examples
- Complete test coverage (144 tests)
- CI/CD workflows and publishing scripts

### Features
- Zero-cost abstractions
- Type-safe operations
- Memory safety guarantees
- Async/await support (optional)
- Parallel processing with Rayon (optional)
- WebAssembly compatibility (optional)
- No-std support (optional)
- Comprehensive error handling
- Method chaining with fluent API
- Full compatibility with Lodash collection methods

### Technical Details
- Rust 1.70+ compatibility
- Cross-platform support (Windows, macOS, Linux)
- x86_64 and ARM64 architecture support
- MIT License
- Comprehensive documentation with examples
- 100% test coverage including doctests
