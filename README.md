# jade-engine

An experiment in AI agent orchestraction: a small Rust game engine for an open-world RPG, built by AI coding agents that I direct,
review, and merge.

## How it works
- Each task is written as a spec in `tasks/<ID>.md`.
- An agent (codeWhale + DeepSeek) runs in an isolated Docker container
  on its own git branch, with access to this repo only.
- Agents follow the rules in `AGENTS.md` and must pass
  `scripts/check.sh` (format, lint, tests) before anything is merged.
- I review each plan before work starts, verify the result myself,
  and merge to `main`.
- The cost and outcome of every task are tracked in `docs/agent-log.md`.

## Status
- [x] L-01 Workspace skeleton: 12 engine crates, 2 apps
- [] M0: Foundation: game loop, window, GPU setup

## Run
cargo run -p sandbox

