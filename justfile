default:
    @just --list

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints (lib and bin only)
lint:
    cargo clippy --all-features -- -D warnings

# Alias for lint
clippy: lint

# Build the project
build:
    cargo build

# Build for release
build-release:
    cargo build --release

# Run tests
test:
    cargo test --all-features

# Run all checks (fmt, lint, test)
check: fmt-check lint test

# Publish to crates.io
publish:
    cargo publish

# Publish dry run
publish-dry:
    cargo publish --dry-run

# Refresh the Linear GraphQL schema
refresh-schema:
    ./scripts/refresh-schema.sh
