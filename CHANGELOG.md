# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-07-03

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

[0.2.0]: https://github.com/eqto/passman/compare/v0.1.0...v0.2.0
