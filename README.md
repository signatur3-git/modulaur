# Modulaur

**A modular, extensible desktop dashboard platform built with Tauri, Vue, and Rust**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-green)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange)](https://www.rust-lang.org)

## 🎯 What is Modulaur?

Modulaur is a cross-platform desktop application that provides a flexible, plugin-based environment for creating custom dashboards, managing data, and integrating with external services. Think of it as a personal workspace that adapts to your needs through plugins.

### Key Features

- **🔌 Plugin System** - Extend functionality with WASM backend plugins and Vue frontend plugins
- **📊 Multi-Page Dashboards** - Create custom pages with different layouts and purposes
- **🎨 Flexible Layouts** - Drag-and-drop panels, customizable grid layouts, and reorderable navigation
- **💾 Database-First Architecture** - SurrealDB embedded storage with dev/prod separation
- **🌓 Theme Support** - Light/dark mode with CSS variable-based theming
- **🔒 Offline-First** - Works without internet, with optional data sync capabilities
- **🎭 Stage Separation** - Development and Production environments with independent data

### Built-In Plugins

- **Markdown Notepad** - Rich text editing with markdown support
- **Ticket Kanban** - Drag-and-drop task boards with project tracking
- **Time Tracker** - Project-based time logging with statistics
- **Text Converter** - Multi-format text transformation tools
- **RSS Feed Reader** - Subscribe to and read RSS/Atom feeds
- **GitLab Adapter** - Integration with GitLab APIs
- **Prompt Generator** - AI prompt authoring system with template engine

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+ with cargo
- **Windows**, macOS, or Linux

### Installation Setup

```powershell
# Clone the repository
git clone <repository-url>
cd modulaur

# Install frontend dependencies
cd src
npm install
cd ..

# Start development
# Terminal 1: Frontend dev server (Vite)
npm run dev

# Terminal 2: Tauri app (opens desktop window)
npm run dev:tauri

# Or build plugins and start Tauri in one command
npm run dev:full
```

### Building Plugins

```powershell
# Build all plugins
npm run plugins:build

# Build a specific plugin
npm run plugin:build time-tracker
npm run plugin:build markdown-notepad

# Just deploy (if already built)
npm run plugins:deploy
```

### Production Build

```powershell
# Complete release build (plugins + frontend + backend + size check)
npm run build:release

# Or step by step
npm run plugins:build
npm run build          # Frontend
npm run build:tauri    # Backend
npm run check:size     # Check binary sizes
```

### Cleaning

```powershell
# Clean all build artifacts and reinstall dependencies
npm run clean

# Clean without reinstalling
npm run clean:skip-install

# Clean specific parts
npm run clean:frontend
npm run clean:backend
```

### Available Commands

Run `npm run` to see all available commands.

## 📁 Project Structure

```
modulaur/
├── src/                       # Vue 3 frontend application
│   ├── src/
│   │   ├── components/        # Reusable Vue components
│   │   ├── views/             # Page components (Dashboard, Layout, Settings)
│   │   ├── stores/            # Pinia state management
│   │   ├── router/            # Vue Router configuration
│   │   └── assets/            # Static assets and styles
│   └── public/
│       └── plugins/           # Symlink to src-tauri/plugins (dev)
│
├── src-tauri/                 # Rust backend
│   ├── src/
│   │   ├── main.rs           # Application entry point
│   │   ├── db.rs             # SurrealDB database layer
│   │   ├── plugin_system.rs  # Plugin loading and management
│   │   ├── stage.rs          # Dev/Prod stage management
│   │   └── commands.rs       # Tauri command handlers
│   ├── plugins/              # Compiled plugin files
│   └── Cargo.toml
│
├── plugins/                   # Plugin source code
│   ├── markdown-notepad/
│   ├── ticket-kanban/
│   ├── time-tracker/
│   ├── prompt-generator/
│   └── templates/            # Plugin templates for creating new plugins
│
├── docs/                      # Current documentation
│   ├── README.md             # Documentation index
│   ├── architecture/         # System architecture docs
│   ├── guides/               # User and developer guides
│   └── api/                  # API references
│
└── old-docs/                  # Archived historical documentation
```

## 🏗️ Architecture

### Frontend (Vue 3 + TypeScript)

- **Vue 3** with Composition API
- **Pinia** for state management
- **Vue Router** for navigation
- **TypeScript** for type safety
- **Vite** for fast development

### Backend (Rust + Tauri)

- **Tauri** for native application shell
- **SurrealDB** for embedded database
- **WASM** runtime for plugin execution
- **Async Rust** for concurrent operations

### Plugin System

Plugins can provide:
- **Frontend Components** - Vue panels and page types
- **Backend Services** - WASM modules for data processing
- **Data Sources** - Adapters for external APIs
- **Page Types** - Custom page implementations

## 📖 Documentation

For detailed documentation, see the [docs](./docs) folder:

- [Architecture Overview](./docs/architecture/overview.md)
- [Plugin Development Guide](./docs/PLUGIN-DEVELOPMENT.md)
- [API Reference](./docs/api/tauri-commands.md)

## 🗺️ Roadmap

TBD

### Upcoming

- [ ] Enhanced plugin discovery and marketplace
- [ ] Data export/import improvements
- [ ] Mobile responsiveness
- [ ] Cloud sync capabilities (optional)

## 🤝 Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines.

## 📄 License

MIT License - See [LICENSE](./LICENSE) for details.

## 🙏 Acknowledgments

Built with amazing open-source technologies:
- [Tauri](https://tauri.app) - Desktop application framework
- [Vue.js](https://vuejs.org) - Frontend framework
- [SurrealDB](https://surrealdb.com) - Embedded database
- [Rust](https://www.rust-lang.org) - Backend language

