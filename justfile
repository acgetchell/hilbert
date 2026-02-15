# Build everything
build:
    cargo build --all

# Run all checks
check: fmt lint md test typos

# Default recipe
default:
    dev

# Run the API server
dev:
    cargo run -p api

# Check formatting
fmt:
    cargo fmt --all -- --check

# Format in place
fmt-fix:
    cargo fmt --all

# Fix issues
fix:
    fmt-fix typos-fix

# Run clippy strictly
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Markdown formatting
md:
    dprint fmt

md-check:
    dprint check

# Run tests
test:
    cargo test --all

typos:
    typos

typos-fix:
    typos --write-changes