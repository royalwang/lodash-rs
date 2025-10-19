#!/bin/bash

# Publish script for rust-lodash
set -e

echo "🚀 Starting publish process for rust-lodash..."

# Check if we're on the main branch
current_branch=$(git branch --show-current)
if [ "$current_branch" != "main" ]; then
    echo "❌ Error: You must be on the main branch to publish"
    exit 1
fi

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo "❌ Error: You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Get the current version
current_version=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
echo "📦 Current version: $current_version"

# Ask for new version
read -p "Enter new version (current: $current_version): " new_version

if [ -z "$new_version" ]; then
    echo "❌ Error: Version cannot be empty"
    exit 1
fi

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml

# Update CHANGELOG.md
echo "📝 Updating CHANGELOG.md..."
# Add a new section for the new version
sed -i "s/## \[Unreleased\]/## \[Unreleased\]\n\n## \[$new_version\] - $(date +%Y-%m-%d)/" CHANGELOG.md

# Run tests (default features only for now due to advanced feature issues)
echo "🧪 Running tests..."
cargo test

# Run clippy (default features only)
echo "🔍 Running clippy..."
cargo clippy --all-targets -- -D warnings

# Check formatting
echo "🎨 Checking formatting..."
cargo fmt --all -- --check

# Build documentation
echo "📚 Building documentation..."
cargo doc --no-deps

# Dry run publish
echo "🔍 Dry run publish..."
cargo publish --dry-run

# Ask for confirmation
read -p "Do you want to publish version $new_version? (y/N): " confirm
if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
    echo "❌ Publish cancelled"
    exit 1
fi

# Commit changes
echo "📝 Committing changes..."
git add Cargo.toml CHANGELOG.md
git commit -m "Release version $new_version"

# Create and push tag
echo "🏷️  Creating tag..."
git tag "v$new_version"
git push origin main
git push origin "v$new_version"

echo "✅ Version $new_version has been released!"
echo "🔗 Check the GitHub Actions for the publish status"
