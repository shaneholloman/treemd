# justfile for treemd - Markdown tree viewer and navigator

# Default recipe - show available commands
default:
    @just --list

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode with optimizations
release:
    cargo build --release

# Run the project with a test file
run FILE="/tmp/test_multi_codeblock.md":
    cargo run -- {{FILE}}

# Run the release build with a test file
run-release FILE="/tmp/test_multi_codeblock.md":
    ./target/release/treemd {{FILE}}

# Run all tests
test:
    cargo test

# Run tests with output shown
test-verbose:
    cargo test -- --nocapture

# Check code without building
check:
    cargo check

# Run clippy for linting
lint:
    cargo clippy -- -D warnings

# Format code with rustfmt
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt -- --check

# Clean build artifacts
clean:
    cargo clean

# Install the binary to ~/.cargo/bin
install:
    cargo install --path .

# Uninstall the binary
uninstall:
    cargo uninstall treemd

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Full CI check: format, lint, test, build
ci: fmt-check lint test release
    @echo "✅ All CI checks passed!"

# Watch and rebuild on file changes (requires cargo-watch)
watch:
    cargo watch -x check -x test -x run

# Generate and open documentation
doc:
    cargo doc --open

# Show project statistics
stats:
    @echo "Lines of code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo "\nDependencies:"
    @cargo tree --depth 1

# Create a new release build and show binary size
release-info: release
    @echo "Release binary:"
    @ls -lh target/release/treemd | awk '{print $5, $9}'
    @echo "\nStripped binary size:"
    @strip target/release/treemd
    @ls -lh target/release/treemd | awk '{print $5, $9}'
