# Plugin Development Guide

**Last Updated:** 2026-01-04

This guide covers creating custom plugins for Modulaur. For deployment and loading details, see [PLUGIN-DEPLOYMENT.md](./PLUGIN-DEPLOYMENT.md).

## Overview

Modulaur plugins extend the application with custom functionality. Plugins can be:
- **Frontend-only** - UI panels (Vue components)
- **Backend-only** - Data adapters (Rust WASM)
- **Full-stack** - Combined UI + backend logic

## Quick Start

### Using Templates

The fastest way to create a plugin is to start from an existing plugin or template:

```powershell
cd plugins/featured

# Copy an existing plugin as a template
cp -r markdown-notepad my-plugin
cd my-plugin

# Update manifest.json with your plugin details
# Edit the frontend code
# Build and test
```

### Available Templates

Located in `plugins/featured/`:
- **markdown-notepad** - Simple panel with local storage
- **rss-feed-reader** - Panel with external API calls
- **time-tracker** - Panel with database integration
- **snippets** - Complex panel with CRUD operations

See `plugins/README.md` for a complete overview of available examples and templates.

## Plugin Structure

### Minimal Plugin Structure

```
my-plugin/
├── manifest.json           # Plugin metadata
├── README.md              # Documentation
├── build.bat              # Build script (Windows)
├── build.ps1              # Build script (PowerShell)
└── frontend/              # Frontend code (if UI plugin)
    ├── package.json       # Dependencies
    ├── vite.config.ts     # Build configuration
    ├── src/
    │   └── index.ts       # Entry point
    └── dist/              # Build output (generated)
        ├── my-plugin.umd.js
        └── my-plugin.css
```

### Frontend Plugin Structure (Detailed)

```
my-plugin/
├── manifest.json
└── frontend/
    ├── package.json
    ├── vite.config.ts
    └── src/
        ├── index.ts                    # Entry point
        ├── MyPluginPanel.vue           # Main component
        ├── components/                 # Sub-components
        │   ├── MyComponent.vue
        │   └── AnotherComponent.vue
        ├── composables/                # Vue composables
        │   └── useMyPlugin.ts
        └── types/                      # TypeScript types
            └── index.ts
```

## manifest.json

Every plugin requires a `manifest.json` file:

```json
{
  "id": "my-plugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "description": "A custom plugin for Modulaur",
  "author": "Your Name",
  "type": "panel",
  "entry": "frontend/dist/my-plugin.umd.js",
  "css": "frontend/dist/my-plugin.css",
  "icon": "📦",
  "tags": ["utility", "custom"]
}
```

### Manifest Fields

- **id** (required) - Unique identifier (kebab-case, no spaces)
- **name** (required) - Display name
- **version** (required) - Semantic version (e.g., "1.0.0")
- **description** - Brief description
- **author** - Your name or organization
- **type** (required) - Plugin type: `"panel"` or `"adapter"`
- **entry** (required) - Path to compiled JavaScript (relative to plugin root)
- **css** - Path to CSS file (optional)
- **icon** - Emoji or icon identifier
- **tags** - Array of tags for categorization

## Frontend Plugin Development

### 1. Setup

```powershell
cd plugins/featured
mkdir my-plugin
cd my-plugin
mkdir frontend
cd frontend

# Initialize package.json
npm init -y

# Install dependencies
npm install vue@3
npm install -D vite @vitejs/plugin-vue typescript
```

### 2. Create Entry Point

**`frontend/src/index.ts`:**
```typescript
import { App } from 'vue'
import MyPluginPanel from './MyPluginPanel.vue'

export default {
  install(app: App) {
    app.component('MyPluginPanel', MyPluginPanel)
  }
}

// Export for UMD build
export { MyPluginPanel }
```

### 3. Create Component

**`frontend/src/MyPluginPanel.vue`:**
```vue
<template>
  <div class="my-plugin">
    <h3>{{ title }}</h3>
    <p>{{ message }}</p>
    <button @click="handleClick">Click Me</button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

// Props received from the dashboard
interface Props {
  config?: {
    title?: string
    message?: string
  }
}

const props = withDefaults(defineProps<Props>(), {
  config: () => ({})
})

const title = ref(props.config?.title || 'My Plugin')
const message = ref(props.config?.message || 'Hello from plugin!')

function handleClick() {
  alert('Button clicked!')
}
</script>

<style scoped>
.my-plugin {
  padding: 20px;
}

button {
  padding: 8px 16px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  opacity: 0.8;
}
</style>
```

### 4. Configure Vite Build

**`frontend/vite.config.ts`:**
```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import { copyFileSync, existsSync, mkdirSync } from 'node:fs'

const repoRoot = process.env.MODULAUR_ROOT
  ? resolve(process.env.MODULAUR_ROOT)
  : resolve(__dirname, '../../../..')

const pluginName = 'my-plugin'
const targetDir = resolve(repoRoot, 'src-tauri/plugins', pluginName)

export default defineConfig({
  plugins: [
    vue(),
    {
      name: 'copy-manifest',
      writeBundle() {
        const manifestSrc = resolve(__dirname, '../manifest.json')
        const manifestDest = resolve(targetDir, 'manifest.json')
        if (existsSync(manifestSrc)) {
          mkdirSync(targetDir, { recursive: true })
          copyFileSync(manifestSrc, manifestDest)
          console.log('✓ Copied manifest.json to', targetDir)
        }
      }
    }
  ],
  build: {
    outDir: resolve(targetDir, 'frontend/dist'),
    emptyOutDir: true,
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'MyPlugin',
      fileName: (format) => `my-plugin.${format}.js`,
      formats: ['umd']
    },
    rollupOptions: {
      external: ['vue'],
      output: {
        globals: {
          vue: 'Vue'
        },
        assetFileNames: 'my-plugin.[ext]',
        entryFileNames: 'my-plugin.umd.js'
      }
    }
  }
})
```

### 5. Create Build Script

**`build.ps1`:**
```powershell
Write-Host "Building My Plugin..." -ForegroundColor Cyan

$ErrorActionPreference = "Stop"
$pluginDir = $PSScriptRoot

try {
    Set-Location "$pluginDir\frontend"
    
    Write-Host "[1/2] Installing dependencies..." -ForegroundColor Yellow
    npm install
    
    Write-Host "[2/2] Building UMD bundle..." -ForegroundColor Yellow
    npm run build
    
    Write-Host "✅ Build Complete!" -ForegroundColor Green
    Write-Host "Output: src-tauri/plugins/my-plugin/"
} catch {
    Write-Host "❌ Build failed: $_" -ForegroundColor Red
    exit 1
}
```

### 6. Build and Test

```powershell
# Build the plugin
.\build.ps1

# Or from project root
npm run plugin:build my-plugin

# Start the app to test
npm run dev:tauri
```

## Accessing Modulaur APIs

### Using Tauri Commands

```typescript
import { invoke } from '@tauri-apps/api/core'

// Get all records of a type
async function getSnippets() {
  const records = await invoke('get_records_by_type', {
    recordType: 'snippet'
  })
  return records
}

// Create a new record
async function createSnippet(data: any) {
  const record = await invoke('create_record', {
    record: {
      record_type: 'snippet',
      source: 'my-plugin',
      data: data
    }
  })
  return record
}
```

### Available Commands

Common Tauri commands available to plugins:

**Records:**
- `get_records_by_type(record_type: string)`
- `create_record(record: StagedRecord)`
- `update_record(id: string, record: StagedRecord)`
- `delete_record(id: string)`

**Pages:**
- `get_pages()`
- `create_page(page: Page)`
- `update_page(id: string, page: Page)`
- `delete_page(id: string)`

See [API Reference](./api/tauri-commands.md) for complete list (coming soon).

### Using LocalStorage

For simple data persistence:

```typescript
import { ref, watch } from 'vue'

// Save data automatically
const notes = ref(localStorage.getItem('my-plugin-notes') || '')

watch(notes, (newValue) => {
  localStorage.setItem('my-plugin-notes', newValue)
})
```

**Note:** LocalStorage data is browser-specific and separate from the database.

## Plugin Configuration

Plugins can receive configuration from the panel:

```vue
<script setup lang="ts">
interface Config {
  apiKey?: string
  refreshInterval?: number
  theme?: 'light' | 'dark'
}

interface Props {
  config?: Config
}

const props = withDefaults(defineProps<Props>(), {
  config: () => ({})
})

// Use config values with defaults
const apiKey = ref(props.config?.apiKey || '')
const interval = ref(props.config?.refreshInterval || 60000)
</script>
```

Users can configure panels through the dashboard's panel settings.

## Styling

### Using CSS Variables

Modulaur provides CSS variables for consistent theming:

```css
.my-component {
  background: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

button {
  background: var(--color-primary);
  color: white;
}
```

### Common Variables

- `--bg-primary` - Primary background
- `--bg-secondary` - Secondary background
- `--text-primary` - Primary text color
- `--text-secondary` - Secondary text color
- `--border-color` - Border colors
- `--color-primary` - Accent/primary color

### Responsive Design

```css
.my-component {
  padding: 20px;
}

@media (max-width: 768px) {
  .my-component {
    padding: 10px;
  }
}
```

## Testing

### Manual Testing

1. Build your plugin
2. Start the dev app: `npm run dev:tauri`
3. Create a new dashboard
4. Add your plugin as a panel
5. Test functionality

### Debugging

**Console Logging:**
```typescript
console.log('[MyPlugin] Debug message:', data)
```

**DevTools:**
- Right-click in app → "Inspect Element"
- Check Console for errors
- Check Network tab for API calls

## Publishing

### Package Your Plugin

```powershell
cd plugins/featured/my-plugin

# Create a distributable package
tar -czf my-plugin-v1.0.0.tar.gz manifest.json frontend/dist/
```

### Share Your Plugin

1. Create a GitHub repository for your plugin
2. Include comprehensive README
3. Add examples and screenshots
4. Tag releases with version numbers
5. Share on the Modulaur community (coming soon)

## Best Practices

### 1. Error Handling

```typescript
async function loadData() {
  try {
    const data = await invoke('get_data')
    return data
  } catch (error) {
    console.error('[MyPlugin] Failed to load data:', error)
    // Show user-friendly error message
    return null
  }
}
```

### 2. Loading States

```vue
<template>
  <div v-if="isLoading">Loading...</div>
  <div v-else-if="error">Error: {{ error }}</div>
  <div v-else>{{ data }}</div>
</template>

<script setup>
const isLoading = ref(true)
const error = ref(null)
const data = ref(null)

onMounted(async () => {
  try {
    data.value = await loadData()
  } catch (e) {
    error.value = e.message
  } finally {
    isLoading.value = false
  }
})
</script>
```

### 3. Unique IDs

Prefix your plugin data with the plugin ID:

```typescript
// ✅ Good - namespaced
localStorage.setItem('my-plugin-settings', data)

// ❌ Bad - might conflict
localStorage.setItem('settings', data)
```

### 4. Performance

```typescript
// Use computed for derived state
const filteredItems = computed(() => {
  return items.value.filter(item => item.active)
})

// Debounce expensive operations
import { useDebounceFn } from '@vueuse/core'

const debouncedSearch = useDebounceFn((query) => {
  performSearch(query)
}, 300)
```

## Examples

### Example 1: Simple Note-Taking Panel

See `plugins/featured/markdown-notepad/` for a complete example.

### Example 2: Data Grid with CRUD

See `plugins/featured/snippets/` for a comprehensive example.

### Example 3: External API Integration

See `plugins/featured/rss-feed-reader/` for API integration patterns.

## Troubleshooting

### Plugin Not Loading

1. Check manifest.json is valid JSON
2. Verify paths in manifest match actual files
3. Check browser console for errors
4. Rebuild: `npm run plugin:build my-plugin`

### Style Not Applied

1. Ensure CSS file is listed in manifest.json
2. Check CSS is scoped to avoid conflicts
3. Use CSS variables for theming

### Data Not Persisting

1. Check Tauri command syntax
2. Verify record has correct `source` field
3. Check database for errors: Database Management → Statistics

## Next Steps

- Review existing plugins in `plugins/featured/`
- Read [PLUGIN-DEPLOYMENT.md](./PLUGIN-DEPLOYMENT.md) for deployment details
- Check [Architecture Overview](./architecture/overview.md) for system design
- Join the community discussions (coming soon)

## Resources

- [Vue 3 Documentation](https://vuejs.org/)
- [Vite Documentation](https://vitejs.dev/)
- [Tauri Documentation](https://tauri.app/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

