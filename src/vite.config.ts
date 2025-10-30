import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'
import fs from 'fs'

// Read version from package.json
const packageJson = JSON.parse(
  fs.readFileSync(path.resolve(__dirname, './package.json'), 'utf-8')
)

// Plugin to copy plugins directory for production build
// In dev mode, public/plugins is a junction to src-tauri/plugins so no middleware needed
function pluginsPlugin() {
  const pluginsSrc = path.resolve(__dirname, '../src-tauri/plugins')

  return {
    name: 'bundle-plugins',
    // Copy plugins to dist in production build
    writeBundle() {
      const pluginsDest = path.resolve(__dirname, 'dist/plugins')
      if (fs.existsSync(pluginsSrc)) {
        console.log('📦 Copying plugins to dist...')
        fs.cpSync(pluginsSrc, pluginsDest, { recursive: true })
        console.log('✅ Plugins copied to dist/plugins')
      }
    }
  }
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    pluginsPlugin()
  ],
  define: {
    __APP_VERSION__: JSON.stringify(packageJson.version),
  },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      // Allow serving files from plugins directory
      allow: ['..', '../plugins', '../src-tauri/plugins'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@plugins': path.resolve(__dirname, '../plugins'),
    },
  },
  publicDir: 'public',
})

