# Repository Guidelines

## Project Structure & Module Organization

This is a Rust 2024 workspace. `apps/victory` contains the CLI; `crates/` holds the runtime, GPU, shader, format, loader, and game support; `xtask/` contains repository tasks and asset-policy checks. Architecture notes live in `docs/`. `tests/fixtures/` contains standalone fixture crates and is excluded from the workspace. Keep title files, generated caches, captures, and local logs outside the checkout.

## Build, Test, and Development Commands

Use the pinned Rust 1.98.1 MSVC toolchain on Windows x64 (`rust-toolchain.toml`). Run `cargo test --workspace --locked` for the code suite; it does not need game data. CI also runs:

```powershell
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo xtask install-hooks
cargo run -p victory -- config
```

The first two commands match the Windows CI checks. The hook installs the asset-policy guard; `config` checks local path configuration.

## Coding Style & Naming Conventions

Follow Rust 2024 and `rustfmt.toml` (100-column width and field-init shorthand). Use `snake_case` for modules, functions, and tests, with test names that describe the behavior. Keep unit tests beside their module and integration tests under `crates/<crate>/tests/`. Explain safety invariants around unsafe operations.

## Testing Guidelines

Add deterministic tests for parser, translation, and host-runtime behavior. Full workspace tests may report ignored benchmarks or reference-data tests; preserve their reason and do not count them as passes. GPU or title runs require the appropriate Windows/D3D12 hardware and user-supplied game files; report the GPU, driver, flags, and exact commands. Never add game files or generated caches to tests.

## Commit & Pull Request Guidelines

Recent commits use scoped subjects such as `GPU: ...`, `Shaders: ...`, `HLE: ...`, and `Formats: ...`. Describe the symptom and behavior change in pull requests, list verification commands, and include GPU/driver details for graphics changes. Keep binaries, title assets, cache dumps, private configs, and credentials out of commits.
