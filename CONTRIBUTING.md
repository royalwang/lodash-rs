# Contributing to lodash-rs

Thank you for your interest in contributing to lodash-rs! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A GitHub account

### Setting Up the Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/lodash-rs.git
   cd lodash-rs
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/royalwang/lodash-rs.git
   ```

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run doctests only
cargo test --doc
```

### Code Quality

```bash
# Check code formatting
cargo fmt --check

# Run clippy lints
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all -W clippy::pedantic
```

### Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Build with all features
cargo build --all-features
```

## Contributing Guidelines

### Code Style

- Follow Rust's official style guidelines
- Use `cargo fmt` to format your code
- Use meaningful variable and function names
- Add documentation for all public APIs
- Include examples in documentation

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(chain): add reverse operation to chain API
fix(query): resolve issue with find_last on empty collections
docs(api): add examples for group_by function
```

### Pull Request Process

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit them:
   ```bash
   git add .
   git commit -m "feat: add your feature"
   ```

3. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

4. Create a Pull Request on GitHub

### Pull Request Requirements

- [ ] All tests pass
- [ ] Code is properly formatted
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Examples are provided for new features
- [ ] Commit messages follow conventional format

## Adding New Features

### Collection Methods

When adding new collection methods:

1. Add the function to the appropriate module in `src/collection/`
2. Add comprehensive tests
3. Add documentation with examples
4. Update the main library exports in `src/lib.rs`
5. Add examples to `examples/basic_usage.rs`

### Chain Operations

For chain operations:

1. Add the operation to `src/chain/mod.rs`
2. Update the executor in `src/chain/executor.rs`
3. Add tests for the chain operation
4. Update documentation

### Async Operations

For async operations:

1. Add to `src/collection/async_support.rs`
2. Ensure proper feature gating with `#[cfg(feature = "async")]`
3. Add async tests
4. Update documentation

## Testing Guidelines

### Unit Tests

- Place tests in the same file as the code being tested
- Use descriptive test names
- Test edge cases and error conditions
- Use `assert_eq!` for exact matches
- Use `assert!` for boolean conditions

### Integration Tests

- Place in `tests/` directory
- Test complete workflows
- Test feature interactions

### Documentation Tests

- Include runnable examples in documentation
- Use `#` to hide setup code from examples
- Ensure examples are realistic and useful

## Performance Considerations

- Use `cargo bench` to measure performance
- Consider memory allocations
- Use iterators efficiently
- Avoid unnecessary copying
- Consider SIMD optimizations for future

## Feature Flags

When adding new features:

1. Add feature flag to `Cargo.toml`
2. Use `#[cfg(feature = "feature-name")]` for conditional compilation
3. Update documentation to mention feature requirements
4. Add feature to CI/CD testing

## Documentation

### API Documentation

- Use `///` for public API documentation
- Include examples in documentation
- Use `# Examples` section for code examples
- Document parameters and return values
- Use `# Panics` section for panic conditions
- Use `# Errors` section for error conditions

### README Updates

- Update feature list when adding new features
- Add examples for new functionality
- Update compatibility information if needed

## Issue Reporting

When reporting issues:

1. Check existing issues first
2. Use the issue template
3. Provide minimal reproduction code
4. Include system information (OS, Rust version)
5. Include error messages and stack traces

## Release Process

Releases are handled by maintainers:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release tag
4. Publish to crates.io

## Community Guidelines

- Be respectful and inclusive
- Help others learn and grow
- Provide constructive feedback
- Follow the Rust Code of Conduct

## Getting Help

- Check existing issues and discussions
- Ask questions in GitHub Discussions
- Join the Rust community Discord
- Read the Rust documentation

Thank you for contributing to lodash-rs! 🦀
