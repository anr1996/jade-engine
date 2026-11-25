# AGENTS.md - rules for every agent

## Project
jade-engine is a small Rust game engine for a Morrowind-style
open-world RPG. It is a Cargo workspace with one crate per area.

## Before you start
1. Read this file and your task file: tasks/<ID>.md
2. Only change files inside the crates your task lists, plus
   any other paths your task file explicitly names.

## Rules
- Rust edition 2024, stable toolchain.
- No new dependencies unless your task allows them. Add allowed
  ones to your own crate's Cargo.toml, never the root Cargo.toml.
- New public functions get unit tests.
- Every public item has a doc comment.
- No unsafe code without a comment explaining why.
- Prefer clear code over clever code. keep functions small.
- Never change engine-core interfaces unless your task owns engine-core.
  Otherwise, request it in your report.

## Running commands
- cargo build, clippy, test and scripts/check.sh may be blocked
  by the automatic safety review. If so, stop and ask the lead to 
  approve. Never retry or work around a denial.
- Run scripts/check.sh before every commit.

## Done means
- cargo fmt --all -- --check passes
- cargo clippy --workspace --all-targets -- -D warnings passes
- cargo test --workspace passes
- You wrote tasks/<ID>.report.md: what changed, decisions, known gaps.

## Git
- Commit only on your branch. Message format: "<ID>: <summary>"
- Never push. Never rewrite history.
- Never run git config. Your identity is provided by the lead.
