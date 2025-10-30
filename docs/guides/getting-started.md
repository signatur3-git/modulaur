# Getting Started

**Last Updated:** 2025-01-18

Welcome to Modulaur! This guide will help you set up your development environment and start using the application.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required
- **Node.js** 18.x or higher ([Download](https://nodejs.org/))
- **Rust** 1.70 or higher ([Install](https://www.rust-lang.org/tools/install))
- **Git** ([Download](https://git-scm.com/))

### Platform-Specific
**Windows:**
- Visual Studio Build Tools or Visual Studio with C++ workload
- WebView2 (usually pre-installed on Windows 10/11)

**macOS:**
- Xcode Command Line Tools: `xcode-select --install`

**Linux:**
- Development essentials: `sudo apt-get install build-essential`
- WebKit2GTK: `sudo apt-get install libwebkit2gtk-4.0-dev`

## Installation

### Clone the Repository

```powershell
git clone <repository-url>
cd modulaur
```

### Install Frontend Dependencies

```powershell
# Install dependencies
npm run install:all
```

Or manually:
```powershell
cd src
npm install
cd ..
```

### Build Rust Backend (Optional)

```powershell
cd src-tauri
cargo build
cd ..
```

## Development Setup

### Option 1: Two Terminal Workflow (Recommended)

**Terminal 1: Frontend Dev Server**
```powershell
npm run dev
```
This starts Vite dev server with hot reload at `http://localhost:5173`

**Terminal 2: Tauri Desktop App**
```powershell
npm run dev:tauri
```
This opens the desktop application window.

### Option 2: Build Plugins + Start Tauri

```powershell
npm run dev:full
```
This builds all plugins and starts the Tauri app in one command.
This will build and start both frontend and backend together.

## First Launch

### Initial Setup

1. **Launch the app** using one of the methods above
2. **Choose a stage** - Click "Dev" in the bottom-left corner (this is your development environment)
3. **Explore the default dashboard** - You'll see a welcome page with sample content

### Create Your First Dashboard

1. Click the **"+"** button in the navigation bar
2. Enter a name like "My Dashboard"
3. Select **"Dashboard"** as the page type
4. Choose a layout template (e.g., "Sidebar + Main")
5. Click **"Create"**

### Add Panels to Your Dashboard

1. Open your new dashboard
2. Click **"Add Panel"** in a layout slot
3. Choose a panel type:
   - **Markdown Notepad** - For notes and documentation
   - **Ticket Kanban** - For task management
   - **Time Tracker** - For time logging
   - **RSS Feed Reader** - For news feeds
4. Configure the panel and click **"Save"**

### Customize Layout

- **Drag panels** - Click and drag the header to reorder
- **Resize panels** - Drag the bottom-right corner
- **Delete panels** - Click the trash icon in the panel header

## Understanding Stages

Modulaur has two **independent environments**:

### Development (Dev)
- For testing and experimentation
- Separate database from Production
- Safe to break things!

### Production (Prod)
- Your "real" workspace
- Separate database from Dev
- For day-to-day use

**Switch stages** using the toggle in the bottom-left corner. Each stage has its own:
- Pages and dashboards
- Settings
- Data

## Building Plugins

### Using Existing Plugins

All built-in plugins are pre-compiled. To rebuild them:

```powershell
# Build all plugins at once
npm run plugins:build

# Or build individual plugins
npm run plugin:build markdown-notepad
npm run plugin:build time-tracker
npm run plugin:build prompt-generator
```

### Creating a New Plugin

See the [Plugin Development Guide](../PLUGIN-DEVELOPMENT.md) for detailed instructions.

Quick start:
```powershell
cd plugins
cp -r templates\vue-panel-plugin my-plugin
cd my-plugin
# Edit manifest.json and src files
npm install
npm run build
cd ..\..
npm run plugin:build my-plugin
```

## Configuration

### Application Settings

Access settings via:
1. Click your username/avatar (top-right)
2. Select **"Settings"**
3. Adjust preferences:
   - Theme (light/dark)
   - Default stage
   - Plugin settings

### Database Location

Databases are stored at:
- **Windows:** `%LOCALAPPDATA%\modulaur\data\`
- **macOS:** `~/Library/Application Support/modulaur/data/`
- **Linux:** `~/.local/share/modulaur/data/`

Structure:
- `dev/db` - Development database (SurrealDB)
- `prod/db` - Production database (SurrealDB)

## Common Tasks

### Export Your Data

1. Go to **Database Management** (from the main menu)
2. Click **"Export to File"** in the Database Backup & Restore section
3. Save the JSON file (includes pages, records, dashboards, settings, etc.)

**Optional: Export Plugin Panel Data**
- Some plugins store content in browser LocalStorage (e.g., Markdown notes, RSS feeds)
- Click **"Export Panel Data"** to backup this separately
- This creates a second JSON file for LocalStorage content

### Import Data

1. Go to **Database Management** (from the main menu)
2. Click **"Import from File"**
3. Select your export strategy:
   - **Merge** - Keep existing data, add new items
   - **Replace** - ⚠️ Delete all data first, then import
   - **Skip** - Skip items that already exist
4. Choose your JSON file
5. Confirm import

**Optional: Import Plugin Panel Data**
- If you exported LocalStorage panel data, import it separately
- Click **"Import Panel Data"** and select the panel data JSON file

### Switch Themes

Click the 🌓 icon in the top-right corner to toggle between light and dark mode.

### Reorder Pages

1. Open the **AppMenu**
2. Navigate to **"Page Management"**
3. Drag pages to reorder them
4. Changes save automatically

## Troubleshooting

### App Won't Start

**Check Node.js version:**
```powershell
node --version  # Should be 18+
```

**Check Rust version:**
```powershell
rustc --version  # Should be 1.70+
```

**Clear build cache:**
```powershell
npm run clean
```

### White Screen / Blank App

**Check console for errors:**
- Right-click in the app window
- Select "Inspect Element"
- Check the Console tab

**Common fixes:**
- Rebuild plugins: `npm run plugins:build`
- Clear cache and rebuild: `npm run clean`
- Restart both frontend and backend

### Plugin Not Loading

**Verify plugin is built:**
```powershell
ls src-tauri\plugins\<plugin-name>\
```
Should contain:
- `manifest.json`
- `frontend/dist/<plugin-name>.umd.js` (for UI plugins)

**Rebuild the plugin:**
```powershell
npm run plugin:build <plugin-name>
```

### Database Errors

**Reset database (⚠️ deletes all data):**
```powershell
# Close the app first
Remove-Item -Path "$env:LOCALAPPDATA\modulaur\data\dev" -Recurse -Force
Remove-Item -Path "$env:LOCALAPPDATA\modulaur\data\prod" -Recurse -Force
```

**Export before resetting** if you want to keep data!

### Build Errors

**Clean all build artifacts:**
```powershell
npm run clean
```

**Or clean specific parts:**
```powershell
npm run clean:frontend
npm run clean:backend
```

## Next Steps

Now that you're set up, explore:

- [Architecture Overview](../architecture/overview.md) - Understand how it works
- [Plugin Development Guide](../PLUGIN-DEVELOPMENT.md) - Create your own plugins
- [Theme Customization](./theming.md) - Customize the look
- [API Reference](../api/tauri-commands.md) - Backend API docs

## Getting Help

- **Check docs:** Browse the [documentation](../README.md)
- **Check issues:** Review known issues in [STATUS.md](../STATUS.md)

Happy building! 🚀
