# Contributing to Passman

Thank you for your interest in contributing! This document covers the basics.

## Prerequisites

- **Go** 1.24+ — from [go.dev](https://go.dev/doc/install)
- **Bun** (for the frontend — https://bun.sh)
- **Wails v3 CLI** — `go install github.com/wailsapp/wails/v3/cmd/wails3@latest`
- **GTK/WebKit system dependencies** for your OS (see [README](README.md#prerequisites))
- **Task** (optional) — from [taskfile.dev](https://taskfile.dev)

## Building

```sh
# Build all Go packages
go build ./...

# Build the CLI only
go build -o passman-cli ./cmd/passman-cli

# Run the desktop app in development mode (hot reload)
wails3 dev
# or
task dev

# Production build
task build
# or manually:
cd frontend && bun install && bun run build && cd ..
CGO_ENABLED=1 go build -tags gtk3 -o build/bin/passman .
```

## Testing

```sh
# Run all Go tests
go test ./...

# Run vault package tests only
go test ./pkg/vault/...
```

## Code Style

- Run `gofmt -w .` before committing
- Run `go vet ./...` and fix any issues
- Go: follow standard `gofmt` formatting and `golangci-lint` if available
- Frontend: 2-space indent for JS/Svelte/CSS/SCSS

## Pull Request Process

1. Fork the repo and create a branch from `main`
2. Make your changes, keeping commits focused
3. Ensure `go test ./...` and `go vet ./...` pass
4. If adding a new feature, include tests
5. Open a PR with a clear description of what and why

## Reporting Issues

- Use GitHub Issues for bugs and feature requests
- For security vulnerabilities, see [SECURITY.md](SECURITY.md) — do not open a public issue

## Project Structure

```
pkg/              # Core Go packages: crypto, vault, buttercup, keepass
internal/         # App services: app, state, config, vimport
cmd/passman-cli/  # CLI tool: create, import, export, convert, extract
frontend/         # Svelte 5 frontend (Wails v3 desktop app)
main.go           # Wails v3 application entry point
docs/             # Architecture and format documentation
```
