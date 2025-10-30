# Changelog

All notable changes to Modulaur will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions workflows for CI/CD
  - Validation workflow for PR checks and merges to main
  - Release workflow with multi-platform builds (Windows, macOS, Linux)
- Comprehensive documentation
  - Plugin Development Guide
  - Theming Guide
  - Tauri Commands API Reference
  - Export/Import documentation
- LICENSE file (MIT)

### Changed
- Version bumped to 0.5.0 across all packages
- Internal planning files moved to `.internal/` directory
- Documentation cleanup and improvements
- Migration code removed (all migrations completed)

### Fixed
- Database paths in documentation (LOCALAPPDATA vs APPDATA)
- Plugin loading paths documentation
- Broken documentation links

## [0.5.0] - 2026-01-04

### Major Features
- **Plugin System**: Extensible architecture with Vue-based plugins
  - Markdown Notepad plugin
  - RSS Feed Reader plugin
  - Snippets plugin
  - Time Tracker plugin
  - Text Converter plugin
- **Pages Management**: Create custom pages with plugin panels
- **Dashboard System**: Drag-and-drop dashboard builder with grid layout
- **Database Management**: Export/import functionality for data backup
- **Tickets/Kanban**: Built-in ticket tracking system
- **Dark Mode**: Full dark/light theme support

### Technical Highlights
- Tauri 2.0 for native desktop application
- Vue 3 with Composition API
- SurrealDB embedded database (pure Rust)
- Separate dev/prod databases
- Plugin loading via junction/symlink system
- Secure credential storage

### Documentation
- Getting Started Guide
- Plugin Development Guide
- Plugin Deployment Guide
- Architecture Overview
- API Reference

## [0.2.0] - Previous Version

### Features
- Initial stable release
- Basic dashboard functionality
- Plugin system foundation
- Database integration

## [0.1.0] - Initial Release

### Features
- Basic application structure
- Proof of concept
- Tauri integration

---

## Release Notes Format

### Added
New features and capabilities

### Changed
Changes to existing functionality

### Deprecated
Features that will be removed in future versions

### Removed
Features that have been removed

### Fixed
Bug fixes and corrections

### Security
Security-related changes and fixes

---

[Unreleased]: https://github.com/yourusername/modulaur/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/yourusername/modulaur/releases/tag/v0.5.0
[0.2.0]: https://github.com/yourusername/modulaur/releases/tag/v0.2.0
[0.1.0]: https://github.com/yourusername/modulaur/releases/tag/v0.1.0

