// Plugin Store - Manages plugin state and operations
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { PluginInfo } from '@/types/plugin'

export const usePluginStore = defineStore('plugins', () => {
  const plugins = ref<PluginInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * Load all installed plugins from backend
   */
  const loadPlugins = async () => {
    loading.value = true
    error.value = null

    try {
      const installed = await invoke<any[]>('get_installed_plugins')

      plugins.value = installed.map(p => ({
        name: p.name,
        version: p.version,
        author: p.author || 'Unknown',
        description: p.description || '',
        enabled: true, // TODO: Load from saved preferences
        hasBackend: !!p.adapter_type,
        hasFrontend: p.frontend?.enabled || false,
        panelTypes: p.frontend?.panelTypes || [],
      }))

      console.log(`✅ Loaded ${plugins.value.length} plugin manifests`)
    } catch (err) {
      console.error('Failed to load plugins:', err)
      error.value = err instanceof Error ? err.message : 'Unknown error'
    } finally {
      loading.value = false
    }
  }

  /**
   * Toggle plugin enabled state
   */
  const togglePlugin = async (name: string) => {
    const plugin = plugins.value.find(p => p.name === name)
    if (plugin) {
      plugin.enabled = !plugin.enabled
      // TODO: Save preference to backend
      console.log(`Plugin ${name} ${plugin.enabled ? 'enabled' : 'disabled'}`)
    }
  }

  /**
   * Reload all plugins (backend and frontend)
   */
  const reloadPlugins = async () => {
    loading.value = true
    error.value = null

    try {
      // Reload backend plugins
      await invoke('reload_plugins')
      console.log('✅ Backend plugins reloaded')

      // Reload plugin list (this will fetch updated list from backend)
      await loadPlugins()
    } catch (err) {
      console.error('Failed to reload plugins:', err)
      error.value = err instanceof Error ? err.message : 'Failed to reload'
    } finally {
      loading.value = false
    }
  }

  /**
   * Get plugin by name
   */
  const getPlugin = (name: string): PluginInfo | undefined => {
    return plugins.value.find(p => p.name === name)
  }

  /**
   * Check if a plugin is enabled
   */
  const isPluginEnabled = (name: string): boolean => {
    const plugin = plugins.value.find(p => p.name === name)
    return plugin?.enabled || false
  }

  // Computed properties
  const enabledPlugins = computed(() => plugins.value.filter(p => p.enabled))

  const backendPlugins = computed(() => plugins.value.filter(p => p.hasBackend))

  const frontendPlugins = computed(() => plugins.value.filter(p => p.hasFrontend))

  const pluginCount = computed(() => plugins.value.length)

  return {
    // State
    plugins,
    loading,
    error,

    // Actions
    loadPlugins,
    togglePlugin,
    reloadPlugins,
    getPlugin,
    isPluginEnabled,

    // Computed
    enabledPlugins,
    backendPlugins,
    frontendPlugins,
    pluginCount,
  }
})
