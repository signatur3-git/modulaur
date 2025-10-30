# Plugins Directory Structure

This directory contains plugins for the Modulaur platform.

## 📁 Directory Organization

### `/featured` - Production-Ready Plugins
**Purpose:** Real, production-quality plugins that demonstrate best practices

**Current Plugins:**
- `gitlab-adapter/` - GitLab CI/CD pipeline and job data adapter (COMING SOON)
  - Full-stack plugin (Rust WASM backend + Vue UMD frontend)
  - Real-world complexity (HTTP, auth, pagination)
  - Template for other adapters

**Use These:**
- ✅ For production use
- ✅ As templates for new plugins
- ✅ To learn best practices
- ✅ Reference implementation

### `/templates` - Plugin Development Templates
**Purpose:** Bare-bones templates for quick plugin development

**Templates:**
- `backend-only/` - WASM adapter plugin (coming soon)
- `frontend-only/` - Vue panel plugin (coming soon)
- `full-stack/` - Complete plugin template (coming soon)

**Use These:**
- ✅ To start new plugin projects
- ✅ For quick prototyping
- ✅ Copy and customize

### `/examples` - Simple Learning Examples
**Purpose:** Minimal examples for learning plugin concepts

**Current Examples:**
- `example-adapter/` - Simple WASM backend example
- `hello-panel/` - Simple Vue frontend example
- `hello-world/` - Minimal full example

**Use These:**
- ✅ To learn plugin basics
- ✅ For understanding concepts
- ✅ Quick experiments

---

## 🚀 Quick Start

### Using a Featured Plugin
```bash
# Featured plugins are ready to use
cd featured/gitlab-adapter
./build-all.sh
# Plugin is ready! Backend + Frontend built
```

### Starting from a Template
```bash
# Copy template and customize
cp -r templates/full-stack my-new-plugin
cd my-new-plugin
# Edit manifest.json, implement your logic
```

### Learning from Examples
```bash
# Examples are simple and educational
cd examples/hello-panel
# Read the code, understand the pattern
```

---

## 📋 Plugin Structure

All plugins follow this structure:

```
plugin-name/
├── manifest.json              # Plugin metadata
├── README.md                  # Plugin documentation
│
├── backend/                   # Optional: Rust WASM backend
│   ├── Cargo.toml
│   ├── src/lib.rs
│   └── plugin_name.wasm      # Built artifact
│
└── frontend/                  # Optional: Vue UMD frontend
    ├── package.json
    ├── vite.config.js
    ├── src/
    │   ├── index.js          # Entry point
    │   └── *.vue             # Components
    └── dist/
        └── plugin-name.umd.js  # Built artifact
```

---

## 🎯 Plugin Categories

### Data Adapters (Backend + Frontend)
Fetch data from external sources

**Examples:**
- `gitlab-adapter` - GitLab CI/CD data (featured)
- `jira-adapter` - Jira tickets (future)
- `github-adapter` - GitHub data (future)

### Visualization Panels (Frontend Only)
Custom dashboard panels

**Examples:**
- `hello-panel` - Demo panel (example)
- `gantt-chart-panel` - Gantt visualization (future)
- `calendar-panel` - Calendar view (future)

### Full-Stack Plugins (Backend + Frontend)
Complete features with data + UI

**Examples:**
- `gitlab-adapter` - Data + config UI (featured)

---

## 🔧 Development Workflow

### 1. Choose Starting Point
- **Featured Plugin:** Copy and modify for similar functionality
- **Template:** Start from scratch with structure
- **Example:** Learn concepts first

### 2. Develop
```bash
# Backend (if needed)
cd backend
cargo build --target wasm32-unknown-unknown --release

# Frontend (if needed)
cd frontend
npm install
npm run build
```

### 3. Test
```bash
# Copy to main plugins directory
cp -r . ../../standalone-tauri/plugins/your-plugin

# Test in app
cargo tauri dev
```

### 4. Package
```bash
# Create distributable package
./package.sh  # Creates plugin-name.zip
```

---

## 📚 Documentation

### For Plugin Users
- See individual plugin READMEs
- Check manifest.json for capabilities
- Read installation instructions

### For Plugin Developers
- See `featured/gitlab-adapter/` for complete example
- Check `templates/` for starting points
- Read `examples/` for learning
- See main documentation: `documentation/M6-PLUGIN-*.md`

---

## 🎉 Contributing

Want to contribute a plugin?

1. **Develop** using a template or featured plugin as base
2. **Test** thoroughly in development
3. **Document** with clear README
4. **Package** as .zip with all artifacts
5. **Submit** for review

**Quality Standards:**
- ✅ Clear documentation
- ✅ Working build scripts
- ✅ Tests included
- ✅ Follows manifest spec
- ✅ Secure (no credentials hardcoded)

---

## 🔐 Security

**Plugin Safety:**
- Plugins run in sandboxed environment
- Backend: WASM sandbox (memory-safe)
- Frontend: Browser sandbox (script isolation)
- Permissions declared in manifest

**Best Practices:**
- ✅ Use secure credential storage
- ✅ Validate all inputs
- ✅ Handle errors gracefully
- ✅ Log security events
- ❌ Never hardcode secrets

---

## 📊 Plugin Registry

Coming soon: Online plugin marketplace!

**Features:**
- 🔍 Browse available plugins
- 📥 One-click install
- ⭐ Ratings and reviews
- 🔄 Automatic updates
- 🛡️ Security scanning

---

**Last Updated:** 2025-11-10  
**Plugin System Version:** M6 Phase 3  
**Questions?** See documentation/M6-PLUGIN-*.md files

