import { defineStore } from 'pinia'
import { ref } from 'vue'

/**
 * Settings Store - Manages app-wide settings
 * M5: Includes manual offline mode toggle
 * M9: Updated to use database backend in Tauri mode
 *
 * STORAGE STRATEGY:
 * - Tauri mode (dev OR prod): Always uses SurrealDB database
 * - Pure browser mode (no Tauri): Falls back to localStorage for frontend-only development
 */

// Helper to check if Tauri is available (browser-only vs Tauri app, NOT dev vs prod)
const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

// Safe invoke wrapper
const safeInvoke = async (command: string, args?: any) => {
  if (!isTauri()) {
    throw new Error('Not in Tauri mode')
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke(command, args)
}

export const useSettingsStore = defineStore('settings', () => {
  // Manual offline mode override
  const manualOfflineMode = ref(false)

  // Other settings
  const showDebugInfo = ref(false)
  const theme = ref<'light' | 'dark'>('light')

  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * Toggle manual offline mode
   * When enabled, app behaves as if network is disconnected
   */
  async function toggleManualOffline() {
    manualOfflineMode.value = !manualOfflineMode.value
    console.log(`📡 Manual offline mode: ${manualOfflineMode.value ? 'ON' : 'OFF'}`)
    await saveSettings()
  }

  /**
   * Set manual offline mode
   */
  async function setManualOffline(value: boolean) {
    manualOfflineMode.value = value
    await saveSettings()
  }

  /**
   * Load settings from storage
   * M9: Uses database backend in Tauri mode
   */
  async function loadSettings() {
    try {
      loading.value = true
      error.value = null

      if (!isTauri()) {
        // Browser mode: use localStorage
        const stored = localStorage.getItem('app_settings')
        if (stored) {
          const parsed = JSON.parse(stored)
          manualOfflineMode.value = parsed.manualOfflineMode || false
          showDebugInfo.value = parsed.showDebugInfo || false
          theme.value = parsed.theme || 'light'
        }
        return
      }

      // M9: Tauri mode - load from database
      const settings = (await safeInvoke('get_all_settings')) as Array<{
        key: string
        value: string
      }>

      settings.forEach(setting => {
        try {
          const value = JSON.parse(setting.value)
          switch (setting.key) {
            case 'manualOfflineMode':
              manualOfflineMode.value = value
              break
            case 'showDebugInfo':
              showDebugInfo.value = value
              break
            case 'theme':
              theme.value = value
              break
          }
        } catch (e) {
          console.warn(`Failed to parse setting ${setting.key}:`, e)
        }
      })
    } catch (err: any) {
      const errorMsg = err instanceof Error ? err.message : String(err)
      error.value = `Failed to load settings: ${errorMsg}`
      console.error('Failed to load settings:', err)

      // Fallback to localStorage
      const stored = localStorage.getItem('app_settings')
      if (stored) {
        const parsed = JSON.parse(stored)
        manualOfflineMode.value = parsed.manualOfflineMode || false
        showDebugInfo.value = parsed.showDebugInfo || false
        theme.value = parsed.theme || 'light'
      }
    } finally {
      loading.value = false
    }
  }

  /**
   * Save settings to storage
   * M9: Uses database backend in Tauri mode
   */
  async function saveSettings() {
    try {
      loading.value = true
      error.value = null

      const settings = {
        manualOfflineMode: manualOfflineMode.value,
        showDebugInfo: showDebugInfo.value,
        theme: theme.value,
      }

      if (!isTauri()) {
        // Browser mode: use localStorage
        localStorage.setItem('app_settings', JSON.stringify(settings))
        return
      }

      // M9: Tauri mode - save to database
      await safeInvoke('set_setting', {
        key: 'manualOfflineMode',
        value: JSON.stringify(manualOfflineMode.value),
      })
      await safeInvoke('set_setting', {
        key: 'showDebugInfo',
        value: JSON.stringify(showDebugInfo.value),
      })
      await safeInvoke('set_setting', {
        key: 'theme',
        value: JSON.stringify(theme.value),
      })
    } catch (err: any) {
      error.value = `Failed to save settings: ${err.message}`
      console.error(error.value)

      // Fallback to localStorage
      const settings = {
        manualOfflineMode: manualOfflineMode.value,
        showDebugInfo: showDebugInfo.value,
        theme: theme.value,
      }
      localStorage.setItem('app_settings', JSON.stringify(settings))
    } finally {
      loading.value = false
    }
  }

  // Auto-save when settings change
  async function autoSave() {
    await saveSettings()
  }

  return {
    manualOfflineMode,
    showDebugInfo,
    theme,
    loading,
    error,
    toggleManualOffline,
    setManualOffline,
    loadSettings,
    saveSettings,
    autoSave,
  }
})
