# Passman

A cross-platform, offline-first password manager with multi-vault support.

## Features

- **Multi-vault** — Create and manage multiple vaults with independent passwords
- **Offline-first** — All data is stored locally, no cloud or network required
- **AES-256-GCM encryption** — Vault payloads encrypted with authenticated encryption
- **Argon2id key derivation** — Resistant to GPU/ASIC brute-force attacks
- **Two-layer key model** — Password → vault key → DEK → payload, enabling password changes without re-encrypting all data
- **Buttercup import** — Import `.bcup` files from [Buttercup](https://buttercup.pw)
- **Password generator** — Cryptographically secure random password generation
- **Auto-lock** — Vault locks automatically after 5 minutes of inactivity
- **Trash & restore** — Deleted entries go to trash with restore support
- **Cross-vault operations** — Move or copy entries between vaults
- **Dark/light theme** — Follows system theme

> **Disclaimer:** Passman is a new project and has not undergone a formal security audit. Use at your own risk.

## Download

Pre-built binaries are available for each release on the [Releases](https://github.com/Eqto/Passman/releases) page.

### Desktop App

| OS | Architecture | Download |
| --- | --- | --- |
| Linux | x86_64 | [`.AppImage`](https://github.com/Eqto/Passman/releases/latest/download/Passman_0.1.0-rc.2_amd64.AppImage) · [`.deb`](https://github.com/Eqto/Passman/releases/latest/download/passman_0.1.0-rc.2_amd64.deb) · [`.rpm`](https://github.com/Eqto/Passman/releases/latest/download/passman-0.1.0-rc.2-1.x86_64.rpm) |
| macOS | Apple Silicon | [`.dmg`](https://github.com/Eqto/Passman/releases/latest/download/Passman_0.1.0-rc.2_aarch64.dmg) |
| macOS | Intel | [`.dmg`](https://github.com/Eqto/Passman/releases/latest/download/Passman_0.1.0-rc.2_x64.dmg) |
| Windows | x86_64 | [`.exe` (installer)](https://github.com/Eqto/Passman/releases/latest/download/Passman_0.1.0-rc.2_x64-setup.exe) · [portable `.exe`](https://github.com/Eqto/Passman/releases/latest/download/passman-portable.exe) |

> Download links are updated automatically on each release.

### CLI

The CLI is built alongside the desktop app. Pre-built binaries are available on the [Releases](https://github.com/Eqto/Passman/releases/latest) page under the CI artifacts.

## Project Structure

- `passman-core/` — Core Rust library (crypto, vault format, config, Buttercup import)
- `passman-app/` — Tauri desktop application (Svelte 5 + Rust)
- `passman-cli/` — Rust CLI tools for vault creation, import, export, and conversion
- `docs/` — Architecture docs and [PMV file format specification](docs/format.md)

## File Format

Vaults are stored in the PMV (Passman Vault) format. See [docs/format.md](docs/format.md) for the full specification.

## Prerequisites

- **Rust** (stable) — install via [rustup](https://rustup.rs)
- **Bun** — install from [bun.sh](https://bun.sh)
- **Tauri v2 system dependencies** — see [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)

### Linux (Ubuntu 24.04)

```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev libgtk-3-dev libsoup2.4-dev build-essential curl wget libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### macOS

```bash
xcode-select --install
```

### Windows

Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/).

## Development

### Core library & CLI

```bash
cargo build --workspace
cargo test --workspace
```

### Desktop app

```bash
cd passman-app
bun install
bun run tauri dev
```

## Releases

This repository is a monorepo containing the desktop app, CLI, and shared `passman-core` library. Keeping them together avoids the coordination overhead of publishing `passman-core` separately or maintaining cross-repo git dependencies.

When releasing, you can either:

- Use a single workspace tag (e.g., `v0.1.0`) when the app, CLI, and core ship together.
- Use component-specific tags (e.g., `app-v0.1.0`, `cli-v0.1.0`, `passman-core-v0.1.0`) when components need independent releases.

The `.github/workflows/build.yml` workflow builds the CLI and the Tauri app on Ubuntu, Windows, and macOS runners and uploads the resulting artifacts.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions, code style, and PR process.

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting and security architecture details.

## License

MIT
