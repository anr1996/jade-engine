# AGENTS.md - rules for every agent

## Project
jade-engine is a small Rust game engine for a Morrowind-style
open-world RPG. It is a Cargo workspace with one crate per area.

## Before you start
1. Read this file and your task file: tasks/<ID>.md
2. Only change files inside the crates your task lists.

## Rules
- Rust edition 2024, stable toolchain.
- No new dependencies unless your task allows them.
- Every public item has a doc comment.
- No unsafe code without a comment explaining why.
- Prefer clear code over clever code. keep functions small.
- Never change engine-core interfaces yourself. Request it in your report.

## Done means
- cargo fmt --all -- --check passes
- cargo clippy --workspace --all-targets -- -D warnings passes
- cargo test --workspace passes
- You wrote tasks/<ID>.report.md: what changed, decisions, known gaps.

## Git
- Commit only on your branch. Message format: "<ID>: <summary>"
- Never push. Never rewrite history.
