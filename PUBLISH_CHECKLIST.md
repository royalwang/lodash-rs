# Publishing Checklist

## Pre-Release Checklist

### 1. Code Quality
- [ ] All tests pass: `cargo test --all-features`
- [ ] Clippy passes: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Formatting is correct: `cargo fmt --all -- --check`
- [ ] No unused dependencies: `cargo machete`
- [ ] Security audit passes: `cargo audit`

### 2. Documentation
- [ ] README.md is up to date
- [ ] All public APIs are documented
- [ ] Examples work correctly
- [ ] CHANGELOG.md is updated
- [ ] Documentation builds: `cargo doc --all-features --no-deps`

### 3. Version Management
- [ ] Version number is updated in Cargo.toml
- [ ] CHANGELOG.md has new version entry
- [ ] All changes are committed
- [ ] Git tag is created

### 4. Testing
- [ ] All features work correctly
- [ ] Examples compile and run
- [ ] Benchmarks pass
- [ ] Cross-platform compatibility verified

### 5. Publishing
- [ ] Dry run passes: `cargo publish --dry-run`
- [ ] All required metadata is present in Cargo.toml
- [ ] License file exists
- [ ] Repository is accessible

## Release Process

1. **Update Version**
   ```bash
   # Update version in Cargo.toml
   # Update CHANGELOG.md
   ```

2. **Run Quality Checks**
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   cargo doc --all-features --no-deps
   ```

3. **Dry Run**
   ```bash
   cargo publish --dry-run
   ```

4. **Commit and Tag**
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release version X.Y.Z"
   git tag "vX.Y.Z"
   git push origin main
   git push origin "vX.Y.Z"
   ```

5. **Publish**
   ```bash
   cargo publish
   ```

## Post-Release

- [ ] Verify package appears on crates.io
- [ ] Check documentation on docs.rs
- [ ] Update any dependent projects
- [ ] Announce release on social media/forums

## Required Metadata

Ensure these fields are present in Cargo.toml:

- `name`: Package name
- `version`: Semantic version
- `edition`: Rust edition
- `authors`: Author information
- `description`: Package description
- `license`: License identifier
- `repository`: Source repository URL
- `homepage`: Project homepage
- `documentation`: Documentation URL
- `readme`: README file path
- `keywords`: Search keywords
- `categories`: Crate categories
- `rust-version`: Minimum Rust version
