# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Migrated backend from Rust/Tauri v2 to Go/Wails v3.
- Core library restructured from Rust workspace (`passman-core`, `passman-cli`, `passman-app`) to Go module (`pkg/`, `internal/`, `cmd/passman-cli/`).
- CLI rewritten from `clap` (Rust) to `cobra` (Go).
- Frontend IPC changed from Tauri `invoke()` to auto-generated Wails bindings.
- Build system changed from `cargo` to `go build` with `Taskfile.yml` for convenience commands.
- CI workflow updated to use Go instead of Rust.
- Faster Argon2id key derivation: Go's `golang.org/x/crypto/argon2` parallelizes lanes across goroutines more efficiently than the Rust `argon2` crate, resulting in significantly faster vault creation and unlock times at all security levels while preserving identical KDF parameters.

### Added

- KeePass `.kdbx` database import (CLI: `import-keepass`, desktop app: Import dialog).
- Security level selection at vault creation (Low / Medium / Secure / Best Argon2id presets).
- `ChangeSecurityLevel` service method to upgrade Argon2id parameters on existing vaults.
- `ImportDialog` component for importing Buttercup/KeePass vaults from the desktop app.
- `SecurityLevelSlider` component for security level selection.
- `OpenVaultMenu` component for opening existing vault files.
- `PasswordGenerator` component with configurable charset and length.
- `TagManager` component for entry tag management.
- `Tree`/`TreeItem` reusable components for group and trash lists.
- `Tab` system components for vault tab management.
- Per-vault view state isolation via `stores/selection.js`.
- SCSS design system with Material Design 3 color tokens.
- `Taskfile.yml` for dev/build/test/generate commands.

### Removed

- Rust workspace (`passman-core/`, `passman-cli/`, `passman-app/`).
- Tauri v2 configuration and dependencies.
- `Cargo.toml` workspace definition.
- `Zeroizing<Vec<u8>>` key handling (replaced with Go `[]byte`).

## [0.1.0-rc.2] - 2026-07-24

### Added

- Full Material Design 3 theme integration with color tokens from Material Theme Builder.
- Chip-specific theme tokens and improved chip styling.
- Reusable Tree component for group and trash lists with collapse/expand support.
- Group drag-and-drop into subgroups with visual feedback.
- Move to trash option in group context menu.
- Per-vault view state isolation.
- Confirmation dialog before locking a vault.
- Autofocus title input on new entry creation.
- Vault UUID, entry history, and deleted_at support in core.
- Intel macOS app build target in CI.

### Changed

- Migrated all Svelte components to Svelte 5 runes mode.
- Migrated app.css to SCSS with theme and design token files.
- Reorganized frontend into feature-based directory structure.
- Split backend and frontend modules for modularity.
- Simplified architecture: extracted Label/Input/Chip components, consolidated icons.
- Removed url and notes from VaultEntry, converted to custom fields.
- Tags now derive from selected group only; removed add-tag button.
- Entry title edit on single click instead of double click.
- Declarative Tabs API with split vault UI into Vaults/Topbar components.
- Upgraded vite-plugin-svelte to v7, svelte to 5.56, vite to 8.
- Material Design 3 compliance fixes for dialogs, buttons, and scrim opacity.

### Fixed

- Fixed context menu not appearing on right-click (Svelte 5 $state proxy reassignment issue).
- Fixed entry move/copy to other vault passing wrong args and property name.
- Fixed button clipping on resize, auto-select first group.
- Fixed entry input layout, copy button clipping, and label alignment.
- Fixed tree item icon and text vertical alignment, highlight on right-click.
- Fixed error handling in CreateVaultDialog.
- Fixed vault unlock and drag indicator issues.
- Tags now sorted by name in display and on save.
- Compact Save button height in tag input row.
- Added spacing above New Entry button.

## [0.1.0-rc.1] - 2026-07-05

### Added

- Buttercup vault import functionality.
- Custom fields with OTP support on entries.
- Toast notifications for clipboard copy operations and double-click to copy passwords.
- Theme toggle supporting light, dark, and auto modes.
- Reusable dialog layout components and a consolidated dialog index.
- Unlock dialog now shows the vault name in the title and the file path in the body.
- Buttercup import dropdown with improved button styling.
- Reusable Chip component for tags and UI chips.
- Tag context menu for entry management.
- Clickable tags in the left column to filter entries.
- Double-click entry title in details to edit.

### Changed

- App icons updated and build targets simplified.
- New Entry and Edit buttons now use a neutral color style.
- Consolidated duplicate CSS into global utility classes.
- Standardized button vertical alignment and shared base styles.
- Unified popup menu classes to use global `.menu` and `.menu-item` classes.
- Improved UI styling for toast shadows/colors, selected states, and tab buttons.
- Updated GitHub Actions to Node 24-compatible versions.
- Reset vault view on unlock and clear vault data on lock.
- Dismiss entry details when the selected entry is filtered out.
- Hide New Entry button when no group is selected.
- Improved button outlines and styling.

### Fixed

- Fixed 32x32.png RGBA format for Tauri compatibility.
- Fixed button styling issues.
- Fixed UI alignment and refactored group components.
- Fixed context menu positioning and added global menu dismissal.
- Fixed theme persistence when switching modes.
- Fixed copy button clipping in entry details.
- Fixed unselecting entry when a newly selected tag is not present on it.
