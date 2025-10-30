# Plugin Deployment Architecture

## Overview

The plugin deployment system uses a **junction** (symbolic link) to eliminate the need for copying plugin files during development. This provides instant updates when plugins are rebuilt.

## Directory Structure

```
modulaur/
├── plugins/
│   └── featured/
│       ├── snippets/
│       │   ├── frontend/        # Plugin source code
│       │   │   ├── src/
│       │   │   └── dist/        # Built files (gitignored)
│       │   └── manifest.json
│       └── ... (other plugins)
│
├── src-tauri/
│   └── plugins/                 # Built plugin files (gitignored)
│       ├── snippets/
│       │   ├── frontend/
│       │   │   └── dist/
│       │   └── manifest.json
│       └── ... (copied from build)
│
└── src/
    └── public/
        └── plugins/             # Junction to src-tauri/plugins (gitignored)
            └── (points to src-tauri/plugins/)
```

## How It Works

### Plugin Loading Paths

The application uses **different plugin directories** depending on the build mode:

#### Development Mode (`cargo tauri dev`)
- **Backend loads from**: `modulaur/src-tauri/plugins/`
- **Frontend serves from**: `modulaur/src/public/plugins/` (junction → `src-tauri/plugins/`)
- **Benefit**: Instant updates when rebuilding plugins

#### Production Mode (Release build)
- **Backend loads from**: `%LOCALAPPDATA%\modulaur\plugins\` (Windows)
  - Linux: `~/.local/share/modulaur/plugins/`
  - macOS: `~/Library/Application Support/modulaur/plugins/`
- **Frontend serves from**: Bundled application resources
- **Note**: Plugins must be installed to AppData for production use

### 1. Build Phase
When you run `npm run plugins:build`:
- Each plugin's frontend is built using Vite
- Build output goes to `plugins/featured/[plugin]/frontend/dist/`
- The build script copies:
  - `frontend/dist/*` → `src-tauri/plugins/[plugin]/frontend/dist/`
  - `manifest.json` → `src-tauri/plugins/[plugin]/manifest.json`

### 2. Deployment Phase
When you run `npm run plugins:deploy` (or `scripts/plugins/deploy.ps1`):
- Checks if `src/public/plugins` exists
- If it's a regular directory, **converts it to a junction**
- If it's already a junction, verifies it points to the correct location
- Creates junction: `src/public/plugins` → `src-tauri/plugins`

### 3. Development Mode
- Vite dev server serves files from `src/public/plugins`
- Since it's a junction, it actually reads from `src-tauri/plugins`
- Backend loads plugins from `src-tauri/plugins` (local project directory)
- When you rebuild a plugin, changes are **instantly available** (no copy needed!)

### 4. Production Mode
- Frontend serves plugins from the built application resources
- Backend loads plugins from `%LOCALAPPDATA%\modulaur\plugins` (Windows) or equivalent AppData location
- Plugins must be installed to the AppData directory for production use
- The `src-tauri/plugins` directory is only used during development

## Commands

### Full Plugin Workflow
```powershell
# Build all plugins and set up junction
npm run plugins:build
npm run plugins:deploy

# Or combined (includes frontend build + backend build)
npm run build:release
```

### Individual Plugin
```powershell
# Build a single plugin
npm run plugin:build snippets
```

### Deploy Only
```powershell
# Set up/verify junction (no rebuild)
npm run plugins:deploy
```

## Troubleshooting

### Problem: Pages not showing plugin content

**Symptom**: Navigation works but page content doesn't load

**Cause**: `src/public/plugins` was a real directory instead of a junction

**Solution**: Run `npm run plugins:deploy`

This will:
1. Delete the real directory
2. Create a proper junction
3. Point it to `src-tauri/plugins`

### Problem: Junction points to wrong location

**Symptom**: Plugins load but show old/wrong content

**Solution**: `npm run plugins:deploy` will automatically detect and fix this

### Problem: Permission denied when creating junction

**Cause**: Windows may require admin rights to create junctions

**Solutions**:
1. Run PowerShell as Administrator and execute `npm run plugins:deploy`
2. Or use Developer Mode in Windows 10/11:
   - Settings → Update & Security → For developers → Developer Mode: ON
   - This allows creating junctions without admin rights

### Verify Junction Status

```powershell
# Check if it's a junction and where it points
fsutil reparsepoint query "src\public\plugins"

# Or
Get-Item "src\public\plugins" -Force | Select-Object LinkType, Target
```

Expected output:
```
LinkType: Junction
Target: D:\workspaces\modulaur\src-tauri\plugins
```

## Why Junction Instead of Copying?

### With Copying (Old Method)
- ❌ Build plugin → Run copy script → Restart dev server
- ❌ Double disk space usage
- ❌ Easy to forget copy step
- ❌ Out-of-sync issues

### With Junction (Current Method)
- ✅ Build plugin → Changes instantly available
- ✅ Single source of truth
- ✅ No manual copy needed
- ✅ Always in sync

## Production Plugin Installation

In production, plugins need to be installed to the user's AppData directory:

**Windows**: `%LOCALAPPDATA%\modulaur\plugins\`
**Linux**: `~/.local/share/modulaur/plugins/`
**macOS**: `~/Library/Application Support/modulaur/plugins/`

### Plugin Directory Structure (Production)
```
%LOCALAPPDATA%\modulaur\
└── plugins/
    ├── snippets/
    │   ├── frontend/
    │   │   └── dist/
    │   │       ├── snippets.umd.js
    │   │       └── snippets.css
    │   └── manifest.json
    ├── markdown-notepad/
    └── ... (other plugins)
```

### How Plugins Get There
- During development: Manually copy from `src-tauri/plugins/` to AppData
- For releases: Bundled with the installer or downloaded separately
- Future: Plugin marketplace/installer (planned feature)

## Git Configuration

Both directories are gitignored:
```gitignore
src-tauri/plugins/        # Built plugin files
src/public/plugins/       # Junction (doesn't store actual files)
```

Only the plugin source code in `plugins/featured/` is committed to git.

## Notes

- Junctions are Windows-specific. On Linux/Mac, this would be a symlink.
- The junction is transparent to applications - Vite treats it like a normal directory
- Deleting the junction doesn't delete the target files
- The deploy script handles all edge cases automatically

