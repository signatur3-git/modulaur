import { createApp } from 'vue'
import { createPinia } from 'pinia'
import * as Pinia from 'pinia'
import './style.css'
import App from './App.vue'
import router from './router'
import { checkTauriContext } from './tauri'
import { pluginLoader } from './services/pluginLoader'
import { useThemeStore } from './stores/themeStore'
import { registerBasePageTypes } from './services/pageTypes'
import { registerLayoutTemplates } from './services/layoutTemplates'
// Prompt generator is now a plugin - registration moved to plugin system
// import { registerPromptGenPageTypes } from './services/promptGen'
import * as Vue from 'vue'
import * as TauriCore from '@tauri-apps/api/core'

// M6: Expose Vue globally for UMD plugins
;(window as any).Vue = Vue
// Expose Pinia globally for UMD plugins that use stores
;(window as any).Pinia = Pinia
// Expose Tauri API for UMD plugins
;(window as any).TauriCore = TauriCore

// M6: Import plugin tests in dev mode
if (import.meta.env.DEV) {
  import('./test-plugins').then(() => {
    console.log('🧪 Plugin test functions loaded - check window.pluginTests')
  })
}

// Check Tauri context on startup
console.log('=== Application Starting ===')
console.log('Mode:', import.meta.env.MODE)
console.log('Dev:', import.meta.env.DEV)

// Wait for DOM to be ready, then check Tauri
setTimeout(() => {
  checkTauriContext()
}, 100)

const app = createApp(App)
app.use(createPinia())
app.use(router)

// M11: Initialize page type registry
console.log('🎨 Initializing page type system...')
registerBasePageTypes()
registerLayoutTemplates()
// Prompt generator is now a plugin - loaded via pluginLoader
console.log('✅ Page types and layout templates registered')

// M6 Phase 2: Initialize plugin loader before mounting
pluginLoader
  .initialize(app)
  .then(() => {
    console.log('✅ Plugin loader initialized')

    // Load saved theme after Pinia is available
    const themeStore = useThemeStore()
    themeStore.loadSavedTheme()

    app.mount('#app')
  })
  .catch(error => {
    console.error('Failed to initialize plugins:', error)

    // Load saved theme even if plugins fail
    try {
      const themeStore = useThemeStore()
      themeStore.loadSavedTheme()
    } catch (e) {
      console.error('Failed to load theme:', e)
    }

    // Mount app anyway - it should work without plugins
    app.mount('#app')
  })
