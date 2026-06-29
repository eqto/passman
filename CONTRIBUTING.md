# Contributing to Passman

Thank you for your interest in contributing! This document covers the basics.

## Prerequisites

- **Rust** (stable, via [rustup](https://rustup.rs))
- **Bun** (for the frontend — https://bun.sh)
- **Tauri v2 prerequisites** for your OS (see https://tauri.app/start/prerequisites/)

## Building

```sh
# Build everything
cargo build --workspace

# Build the CLI only
cargo build -p passman-cli

# Build the desktop app (frontend + backend)
cd passman-app
bun install
bun run tauri dev
```

## Testing

```sh
# Run all tests
cargo test --workspace

# Run core library tests only
cargo test -p passman-core

# Run CLI integration tests
cargo test -p passman-cli
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix any issues
- Rust: follow standard `rustfmt` formatting
- Frontend: 2-space indent for JS/Svelte/CSS, 4-space for Rust

## Pull Request Process

1. Fork the repo and create a branch from `main`
2. Make your changes, keeping commits focused
3. Ensure `cargo test --workspace` and `cargo clippy` pass
4. If adding a new feature, include tests
5. Open a PR with a clear description of what and why

## Reporting Issues

- Use GitHub Issues for bugs and feature requests
- For security vulnerabilities, see [SECURITY.md](SECURITY.md) — do not open a public issue

## Project Structure

```
passman-core/     # Core library: crypto, vault format, Buttercup import
passman-cli/      # CLI tool: create, import, export, convert, extract
passman-app/      # Tauri desktop app (Svelte frontend + Rust backend)
docs/             # Architecture and format documentation
```
