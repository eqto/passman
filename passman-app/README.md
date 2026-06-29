# Passman App

Tauri + Svelte desktop application for Passman.

## Prerequisites

### Linux

Install Tauri system dependencies:

```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.0-dev libgtk-3-dev libsoup2.4-dev libjavascriptcoregtk-4.0-dev build-essential curl wget libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

For other platforms, see the [Tauri prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites).

## Development

```bash
npm install
npm run tauri:dev
```

## Build

```bash
npm run tauri:build
```

## Project Structure

- `src/` - Svelte frontend
- `src-tauri/src/` - Rust backend
  - `main.rs` - Tauri entry point
  - `commands.rs` - Tauri command handlers
- `src-tauri/icons/` - Application icons
