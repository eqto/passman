# Passman

A cross-platform, offline-first password manager with multi-vault support.

## Features

- **Multi-vault** — Create and manage multiple vaults with independent passwords
- **Offline-first** — All data is stored locally, no cloud or network required
- **AES-256-GCM encryption** — Vault payloads encrypted with authenticated encryption
- **Argon2id key derivation** — Resistant to GPU/ASIC brute-force attacks
- **Two-layer key model** — Password → vault key → DEK → payload, enabling password changes without re-encrypting all data
- **Buttercup import** — Import `.bcup` files from [Buttercup](https://buttercup.pw)
- **KeePass import** — Import `.kdbx` databases from [KeePass](https://keepassxc.org)
- **Password generator** — Cryptographically secure random password generation
- **Auto-lock** — Vault locks automatically after 5 minutes of inactivity
- **Trash & restore** — Deleted entries go to trash with restore support
- **Cross-vault operations** — Move or copy entries between vaults
- **Security levels** — Choose Argon2id parameters at vault creation (Low / Medium / Secure / Best)
- **Dark/light theme** — Follows system theme

> **Disclaimer:** Passman is a new project and has not undergone a formal security audit. Use at your own risk.

## Download

Pre-built binaries are available for each release on the [Releases](https://github.com/Eqto/Passman/releases) page.

### Desktop App

| OS | Architecture | Download |
| --- | --- | --- |
| Linux | x86_64 | [`.rpm`](https://github.com/Eqto/Passman/releases/latest/download/passman-v0.1.0-rc.3-1.x86_64.rpm) · [`.deb`](https://github.com/Eqto/Passman/releases/latest/download/passman_v0.1.0-rc.3_amd64.deb) · [binary](https://github.com/Eqto/Passman/releases/latest/download/passman-linux-amd64) |
| macOS | Apple Silicon | [`.dmg`](https://github.com/Eqto/Passman/releases/latest/download/passman-darwin-arm64.dmg) · [binary](https://github.com/Eqto/Passman/releases/latest/download/passman-darwin-arm64) |
| macOS | Intel | [`.dmg`](https://github.com/Eqto/Passman/releases/latest/download/passman-darwin-amd64.dmg) · [binary](https://github.com/Eqto/Passman/releases/latest/download/passman-darwin-amd64) |
| Windows | x86_64 | [`.exe`](https://github.com/Eqto/Passman/releases/latest/download/passman-windows-amd64.exe) |
| Windows | ARM64 | [`.exe`](https://github.com/Eqto/Passman/releases/latest/download/passman-windows-arm64.exe) |

> Download links are updated automatically on each release.

## Project Structure

- `pkg/` — Core Go packages: `crypto` (Argon2id, AES-256-GCM), `vault` (PMV format), `buttercup` (import), `keepass` (import)
- `internal/` — Application services: `app` (vault/group/entry/password services), `state` (in-memory state), `config` (app config), `vimport` (import pipeline)
- `cmd/passman-cli/` — CLI tool for vault creation, import, export, and conversion
- `frontend/` — Svelte 5 frontend (Wails v3 desktop app)
- `main.go` — Wails v3 application entry point
- `wails.json` — Wails application configuration
- `Taskfile.yml` — Task runner for dev/build/test commands
- `docs/` — Architecture docs and [PMV file format specification](docs/format.md)

## File Format

Vaults are stored in the PMV (Passman Vault) format. See [docs/format.md](docs/format.md) for the full specification.

## Prerequisites

- **Go** 1.24+ — install from [go.dev](https://go.dev/doc/install)
- **Bun** — install from [bun.sh](https://bun.sh)
- **Wails v3 CLI** — install with `go install github.com/wailsapp/wails/v3/cmd/wails3@latest`
- **Task** (optional) — install from [taskfile.dev](https://taskfile.dev) for convenience commands

### Linux (Ubuntu 24.04)

```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev libglib2.0-dev build-essential pkg-config
```

### macOS

```bash
xcode-select --install
```

### Windows

Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/).

## Development

### Core packages & CLI

```bash
go build ./...
go test ./...
```

### Desktop app

```bash
# Using Wails CLI directly
wails3 dev

# Or using Task
task dev
```

### Production build

```bash
# Using Task
task build

# Or manually
cd frontend && bun install && bun run build && cd ..
CGO_ENABLED=1 go build -tags gtk3 -o build/bin/passman .
```

## Releases

This repository is a monorepo containing the desktop app, CLI, and shared Go packages. Keeping them together avoids the coordination overhead of publishing shared packages separately or maintaining cross-repo dependencies.

The `.github/workflows/build.yml` workflow builds the CLI and the Wails app on Ubuntu, Windows, and macOS runners and uploads the resulting artifacts.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions, code style, and PR process.

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting and security architecture details.

## License

MIT
