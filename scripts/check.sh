#!/bin/sh
# Run every quality gate in order, stopping at the first failure.
set -eu

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
