# Repository Guidelines

## Project Structure & Module Organization
- Core Rust compiler lives in `framec/src/frame_c/`; visitors sit in `visitors/`, with Rust runtime support in `frame_runtime/`.
- Integration tests and fixtures belong under `framec_tests/` (`common/tests/` for shared .frm specs, `python/` and `typescript/` for language-specific assets). Never drop test artifacts in the repo root.
- Reference docs, design notes, and coordination plans reside in `docs/` (start with `docs/HOW_TO.md` and `docs/plans/ai_planning.md`).

## Build, Test, and Development Commands
- `cargo build` / `cargo build --release`: Compile the transpiler; release builds power the CLI (`./target/release/framec`).
- `./target/release/framec --help`: Inspect language targets and flags before running experiments.
- `python3 framec_tests/runner/frame_test_runner.py --languages python typescript --framec ./target/release/framec`: Canonical end-to-end validation; add `--transpile-only` for quick checks.
- `cargo fmt` and `cargo clippy --all-targets --all-features`: Enforce formatting and lint hygiene before reviews.

## Coding Style & Naming Conventions
- Follow Rust defaults: 4-space indentation, `snake_case` for modules/functions, `PascalCase` for types. Run `cargo fmt` to stay consistent.
- Keep generated code paths deterministic; avoid ad-hoc logging. Favor descriptive visitor method names (e.g., `visit_transition_stmt`) that mirror AST nodes.
- Rustdoc comments (`///`) are preferred for public APIs; inline `//` comments only when behavior is non-obvious.

## Testing Guidelines
- All new behavior requires Frame specs in `framec_tests/common/tests/` (or the relevant language-specific folder) plus automated runner coverage.
- Execute Python and TypeScript suites together unless the change is explicitly scoped; record any exclusions.
- Use the runner’s patterns/configs instead of bespoke scripts (e.g., `python3 framec_tests/runner/frame_test_runner.py -c configs/all_tests.json`).
- Do not claim validation without fully executing generated targets as documented in `CLAUDE.local.md`.

## Commit & Pull Request Guidelines
- Follow Conventional Commits: `feat(scope):`, `fix(parser):`, `docs(runtime):`, etc., matching recent history.
- When releasing, update `[workspace.package].version` in `Cargo.toml`, run `./scripts/sync-versions.sh`, and keep changelog entries aligned.
- PRs should cite impacted tests, summarize language targets exercised, and link related issues or bug IDs.
- Request review once `cargo fmt`, `cargo clippy`, and the full test runner pass; include command outputs in the PR description.

## Agent Workflow Tips
- Begin each session by rereading `docs/HOW_TO.md` and `CLAUDE.md`, checking `git status`, and noting outstanding TODOs in `docs/plans/ai_planning.md`.
- Never implement temporary workarounds; fix root causes and keep documentation synchronized when behavior changes.
