# Quick Reference

**One-page guide to the Modulaur project**

## Project Overview

**Modulaur** is a modular desktop dashboard platform built with Tauri, Vue, and Rust.

- **Type:** Cross-platform desktop application
- **Languages:** TypeScript (Frontend), Rust (Backend)
- **Frameworks:** Vue 3, Tauri 2.x
- **Database:** SurrealDB (embedded)

## Essential Commands

### Development

```powershell
# Start frontend dev server
npm run dev

# Start Tauri app
npm run dev:tauri

# Build plugins and start Tauri
npm run dev:full
```

### Building

```powershell
# Full release build
npm run build:release

# Individual steps
npm run plugins:build      # Build all plugins
npm run plugins:deploy     # Set up plugin junction
npm run plugin:build <name> # Build single plugin
npm run build              # Build frontend
npm run build:tauri        # Build backend
```

### Troubleshooting

#### Pages show navigation but no content

**Problem**: Plugin content doesn't load even though pages appear in the menu.

**Cause**: The `src/public/plugins` folder was a real directory instead of a junction.

**Solution**:

```powershell
npm run plugins:deploy
```

This will convert the directory to a junction pointing to `src-tauri/plugins`.

See [docs/PLUGIN-DEPLOYMENT.md](docs/PLUGIN-DEPLOYMENT.md) for details.

### Cleaning

```powershell
# Clean everything
npm run clean

# Clean without reinstalling
npm run clean:skip-install
```

### See All Commands

```powershell
npm run
```

## Project Structure

```
modulaur/
├── src/                    # Vue frontend
├── src-tauri/              # Rust backend
├── plugins/                # Plugin source code
├── docs/                   # Current documentation
├── old-docs/              # Archived docs
└── scripts/               # Build scripts
```

## Documentation Quick Links

| What You Need       | Where to Look                                                      |
|---------------------|--------------------------------------------------------------------|
| **Getting Started** | [docs/guides/getting-started.md](./docs/guides/getting-started.md) |
| **Architecture**    | [docs/architecture/overview.md](./docs/architecture/overview.md)   |
| **Current Status**  | [docs/STATUS.md](./docs/STATUS.md)                                 |
| **Contributing**    | [CONTRIBUTING.md](./CONTRIBUTING.md)                               |

## Key Concepts

### Stages

- **Dev** - Development environment (separate database)
- **Prod** - Production environment (separate database)
- Switch in app bottom-left corner

### Plugins

- **Frontend:** Vue components (UMD modules)
- **Backend:** WASM modules (for data processing)
- **Location:** `src-tauri/plugins/`
- **Build:** Each plugin has `build.bat`

### Pages

- **Dashboard** - Grid layout with panels
- **Layout** - Slot-based custom layouts
- **Plugin-provided** - Custom page types from plugins

### Panels

- **Markdown Notepad** - Notes and docs
- **Ticket Kanban** - Task boards
- **Time Tracker** - Time logging
- **Text Converter** - Text tools
- **RSS Reader** - Feed aggregator
- **Prompt Generator** - AI prompt authoring

## Common Tasks

### Create a Dashboard

1. Click "+" in navigation bar
2. Choose "Dashboard" type
3. Select layout template
4. Add panels

### Export Data

1. Open AppMenu (☰)
2. Select "Database Export"
3. Save JSON file

### Build a Plugin

```powershell
npm run plugin:build <plugin-name>

# Examples:
npm run plugin:build time-tracker
npm run plugin:build prompt-generator
```

### Switch Themes

Click 🌓 icon in top-right corner

## Troubleshooting

| Problem                | Solution                                |
|------------------------|-----------------------------------------|
| **White screen**       | Rebuild plugins, check console          |
| **Plugin not loading** | Check `src-tauri/plugins/` has manifest |
| **Settings undefined** | Known issue, refresh app                |
| **Build fails**        | Run `cargo clean` or `npm install`      |

## Important Files

| File              | Purpose               |
|-------------------|-----------------------|
| `README.md`       | Project overview      |
| `docs/STATUS.md`  | Current project state |
| `CONTRIBUTING.md` | How to contribute     |

## Git Workflow

```powershell
# Create feature branch
git checkout -b feature/my-feature

# Make changes, commit
git add .
git commit -m "feat: Add feature"

# Fast-forward merge to main
git checkout main
git merge feature/my-feature  # Fast-forward!
git push origin main
```

## Next Steps

1. **Read:** [Getting Started Guide](./docs/guides/getting-started.md)
2. **Understand:** [Architecture Overview](./docs/architecture/overview.md)
3. **Check:** [Current Status](./docs/STATUS.md)
5. **Build:** Try creating a plugin!

## Support

- **Documentation:** [docs/](./docs/)
- **Known Issues:** [docs/STATUS.md](./docs/STATUS.md)

---

**Last Updated:** 2025-01-18
**Version:** 0.9.0 (Pre-release)

