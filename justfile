# ADLER ASI Justfile

set shell := ["bash", "-c"]

default:
    @just --list

# Install all dependencies
setup:
    pnpm install
    cargo check --workspace

# Start the desktop application
dev:
    pnpm --filter adler-asi-desktop dev

# Build the desktop application
build-desktop:
    pnpm --filter adler-asi-desktop build

# Run formatting checks
format:
    cargo fmt --all
    pnpm -r format

# Run linters
lint:
    cargo clippy --workspace -- -D warnings
    pnpm -r lint

# Run all unit tests
test:
    cargo test --workspace
    pnpm -r test

# Run E2E tests (requires Vite dev server)
test-e2e:
    pnpm --filter adler-asi test:e2e
