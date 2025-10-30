// Frontend Plugin Loader
// Dynamically loads and manages Vue plugin components

import type { App, Component } from 'vue'
import { markRaw } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { PluginManifest, FrontendPlugin } from '@/types/plugin'
import { BUILT_IN_PANELS, type PanelConfigSchema } from '@/panels/builtInPanels'
import { getPageTypeRegistry } from '@/services/pageTypeRegistry'

// PHASE 2: Hardcoded plugin imports
// TODO Week 3: Replace with dynamic loading
import HelloPanel from '@/plugins/HelloPanel.vue'

class PluginLoader {
  private plugins: Map<string, FrontendPlugin> = new Map()
  private app: App | null = null
  private initialized = false
  private pluginManifests: Map<string, PluginManifest> = new Map()

  /**
   * Initialize the plugin loader and load all frontend plugins
   */
  async initialize(app: App): Promise<void> {
    if (this.initialized) {
      console.warn('PluginLoader already initialized')
      return
    }

    this.app = app
    console.log('🔌 Initializing Frontend Plugin Loader...')

    try {
      // Register built-in panels as system plugins
      this.registerBuiltInPanels()

      // Get list of installed plugins from backend
      const installedPlugins = await invoke<PluginManifest[]>('get_installed_plugins')
      console.log(`Found ${installedPlugins.length} installed plugins`)

      // Load frontend plugins
      let loadedCount = 0
      for (const manifest of installedPlugins) {
        console.log(`🔍 Checking plugin: ${manifest.name}`, {
          hasFrontend: !!manifest.frontend,
          manifest,
        })

        // If manifest has a frontend section, it's a frontend plugin
        if (manifest.frontend) {
          const success = await this.loadPlugin(manifest)
          if (success) loadedCount++
        }
      }

      this.initialized = true
      console.log(`✅ Loaded ${loadedCount} frontend plugins`)
    } catch (error) {
      console.error('Failed to initialize plugin loader:', error)
      // Continue - app should work without plugins
    }
  }

  /**
   * Load a single plugin from its manifest
   */
  private async loadPlugin(manifest: PluginManifest): Promise<boolean> {
    try {
      console.log(`🔧 Loading plugin: ${manifest.name}`)

      // Store manifest for later retrieval
      this.pluginManifests.set(manifest.name, manifest)

      // PHASE 2: Hardcoded hello-panel (backward compatibility)
      if (manifest.name === 'hello-panel') {
        const panelTypes = new Map<string, Component>()
        panelTypes.set('hello-panel', HelloPanel)

        const plugin: FrontendPlugin = {
          name: 'hello-panel',
          version: '1.0.0',
          register: (app: App) => {
            app.component('HelloPanel', HelloPanel)
            console.log('✅ Hello Panel component registered globally!')
          },
          panelTypes,
        }

        if (this.app) {
          plugin.register(this.app)
        }

        this.plugins.set(manifest.name, plugin)
        console.log(`✅ Loaded plugin: ${plugin.name} v${plugin.version}`)
        return true
      }

      // PHASE 3: Dynamic UMD plugin loading
      if (manifest.frontend?.entry) {
        console.log(`📦 Loading UMD plugin: ${manifest.name}`)
        return await this.loadUmdPlugin(manifest)
      }

      console.log(`⏭️  Skipping plugin ${manifest.name} (no frontend entry)`)
      return false
    } catch (error) {
      console.error(`❌ Failed to load plugin ${manifest.name}:`, error)
      return false
    }
  }

  /**
   * Load a UMD plugin dynamically (Phase 3)
   */
  private async loadUmdPlugin(manifest: PluginManifest): Promise<boolean> {
    try {
      if (!manifest.frontend) {
        throw new Error('No frontend configuration in manifest')
      }

      // Convert path to asset URL that Tauri can serve
      const pluginUrl = `/plugins/${manifest.name}/${manifest.frontend.entry}`
      console.log(`📥 Loading UMD from: ${pluginUrl}`)

      // Load CSS if specified
      if (manifest.frontend.styles && manifest.frontend.styles.length > 0) {
        for (const stylePath of manifest.frontend.styles) {
          const styleUrl = `/plugins/${manifest.name}/${stylePath}`
          this.loadStylesheet(styleUrl)
        }
      }

      // Dynamically import the UMD module
      const script = document.createElement('script')
      script.src = pluginUrl
      script.type = 'text/javascript'

      await new Promise<void>((resolve, reject) => {
        script.onload = () => resolve()
        script.onerror = () => reject(new Error(`Failed to load script: ${pluginUrl}`))
        document.head.appendChild(script)
      })

      // Find the UMD plugin on window
      const pascalCaseName = manifest.name
        .split('-')
        .map(s => s.charAt(0).toUpperCase() + s.slice(1))
        .join('')
      const globalNames = [
        pascalCaseName + 'Plugin',
        pascalCaseName,
        manifest.name.replace(/-/g, ''),
        manifest.name.replace(/-/g, '').toLowerCase() + 'plugin',
        'GitLabAdapterPlugin',
      ]

      let umdPlugin: any = null
      for (const name of globalNames) {
        if ((window as any)[name]) {
          umdPlugin = (window as any)[name]
          console.log(`✅ Found UMD plugin at window.${name}`)
          break
        }
      }

      if (!umdPlugin) {
        throw new Error(`UMD plugin not found on window. Tried: ${globalNames.join(', ')}`)
      }

      // Unwrap ES Module if needed
      if (umdPlugin.__esModule && umdPlugin.default) {
        umdPlugin = umdPlugin.default
      }

      // Create plugin wrapper
      const plugin: FrontendPlugin = {
        name: manifest.name,
        version: manifest.version,
        register: (app: App) => {
          if (umdPlugin.install && typeof umdPlugin.install === 'function') {
            umdPlugin.install(app)
            console.log(`✅ Installed UMD plugin: ${manifest.name}`)
          } else {
            const componentNames = manifest.frontend?.components?.map(c => c.name) || []
            if (umdPlugin.components) {
              Object.entries(umdPlugin.components).forEach(([name, component]) => {
                app.component(name, component as Component)
                console.log(`✅ Registered component: ${name}`)
              })
            } else {
              componentNames.forEach(name => {
                if (umdPlugin[name]) {
                  app.component(name, umdPlugin[name] as Component)
                  console.log(`✅ Registered component: ${name}`)
                }
              })
            }
          }
        },
        panelTypes: new Map(),
        pageTypes: new Map(),
        umdPlugin,
      }

      // Populate panelTypes and pageTypes based on component type
      const components = manifest.frontend?.components || []
      console.log(
        `🔍 Plugin ${manifest.name} has ${components.length} components:`,
        components.map(c => `${c.name} (${c.type})`)
      )

      for (const componentDef of components) {
        // Try multiple ways to find the component
        const component =
          umdPlugin.components?.[componentDef.name] ||
          umdPlugin[componentDef.name] ||
          umdPlugin.default?.components?.[componentDef.name] ||
          umdPlugin.default?.[componentDef.name]

        if (!component) {
          console.warn(`⚠️ Component ${componentDef.name} not found in plugin ${manifest.name}`)
          console.log(`   Available keys:`, Object.keys(umdPlugin))
          if (umdPlugin.components)
            console.log(`   Available components:`, Object.keys(umdPlugin.components))
          continue
        }

        console.log(`✅ Found component ${componentDef.name} (type: ${componentDef.type})`)

        if (componentDef.type === 'page') {
          // Register as page type
          plugin.pageTypes!.set(componentDef.name, component as Component)
          this.registerPageType(manifest, componentDef, component as Component)
        } else {
          // Default to panel type
          plugin.panelTypes!.set(componentDef.name, component as Component)
        }
      }

      // Register with app
      if (this.app) {
        plugin.register(this.app)
      }

      this.plugins.set(manifest.name, plugin)
      console.log(`✅ Loaded UMD plugin: ${plugin.name} v${plugin.version}`)
      return true
    } catch (error) {
      console.error(`❌ Failed to load UMD plugin ${manifest.name}:`, error)
      return false
    }
  }

  /**
   * Register a page type from a plugin component
   */
  private registerPageType(
    manifest: PluginManifest,
    componentDef: any,
    component: Component
  ): void {
    try {
      const registry = getPageTypeRegistry()
      const pageTypeId = componentDef.name.toLowerCase().replace(/page$/i, '')

      registry.register({
        id: pageTypeId,
        name: componentDef.display_name || componentDef.name,
        icon: componentDef.icon || '📄',
        description: componentDef.description || `Page from ${manifest.name} plugin`,
        component: markRaw(component),
        supportsConfig: componentDef.supports_config || false,
        configSchema: componentDef.config_schema,
        metadata: {
          version: manifest.version,
          author: manifest.author,
          category: componentDef.category || 'plugin',
          plugin: manifest.name,
        },
      })

      console.log(`📄 Registered page type "${pageTypeId}" from plugin ${manifest.name}`)
    } catch (error) {
      console.error(`❌ Failed to register page type from ${manifest.name}:`, error)
    }
  }

  /**
   * Load a CSS stylesheet dynamically
   */
  private loadStylesheet(url: string): void {
    const link = document.createElement('link')
    link.rel = 'stylesheet'
    link.href = url
    document.head.appendChild(link)
    console.log(`📦 Loaded stylesheet: ${url}`)
  }

  /**
   * Get a panel component by panel type
   */
  getPanelComponent(panelType: string): any {
    for (const plugin of this.plugins.values()) {
      if (plugin.panelTypes) {
        const component = plugin.panelTypes.get(panelType)
        if (component) {
          return component
        }
      }
    }
    return null
  }

  /**
   * Get all available panel types
   */
  getAvailablePanelTypes(): Array<{
    id: string
    name: string
    icon: string
    pluginName: string
    componentName: string
  }> {
    const panels: Array<any> = []

    // Add built-in panels first
    for (const panel of BUILT_IN_PANELS) {
      panels.push({
        id: panel.id,
        name: panel.name,
        icon: panel.icon,
        pluginName: 'system',
        componentName: panel.id,
      })
    }

    // Add plugin panels
    for (const plugin of this.plugins.values()) {
      if (BUILT_IN_PANELS.some(p => p.id === plugin.name)) {
        continue // Skip built-in panels already added
      }

      const manifest = this.pluginManifests.get(plugin.name)
      if (manifest?.frontend?.components) {
        for (const component of manifest.frontend.components) {
          if (component.type === 'panel') {
            const comp = component as any
            panels.push({
              id: component.name,
              name: component.display_name || component.name,
              icon: comp.icon || this.getIconForPlugin(plugin.name),
              pluginName: plugin.name,
              componentName: component.name,
            })
          }
        }
      }
    }

    return panels
  }

  /**
   * Get all available page types from plugins
   */
  getAvailablePageTypes(): Array<{
    id: string
    name: string
    icon: string
    description: string
    pluginName: string
    componentName: string
  }> {
    const pages: Array<any> = []

    for (const plugin of this.plugins.values()) {
      const manifest = this.pluginManifests.get(plugin.name)
      if (manifest?.frontend?.components) {
        for (const component of manifest.frontend.components) {
          if (component.type === 'page') {
            const comp = component as any
            const pageTypeId = component.name.toLowerCase().replace(/page$/i, '')
            pages.push({
              id: pageTypeId,
              name: component.display_name || component.name,
              icon: comp.icon || '📄',
              description: comp.description || '',
              pluginName: plugin.name,
              componentName: component.name,
            })
          }
        }
      }
    }

    return pages
  }

  /**
   * Get configuration schema for a panel type
   */
  getPanelConfigSchema(panelType: string): PanelConfigSchema | undefined {
    // Check built-in panels
    const builtInPanel = BUILT_IN_PANELS.find(p => p.id === panelType)
    if (builtInPanel?.configSchema) {
      return builtInPanel.configSchema
    }

    // Check plugin manifests
    for (const manifest of this.pluginManifests.values()) {
      if (manifest.frontend?.components) {
        for (const component of manifest.frontend.components) {
          const comp = component as any
          if (component.name === panelType && comp.config_schema) {
            return comp.config_schema
          }
        }
      }
    }

    return undefined
  }

  /**
   * Get icon emoji for a plugin
   */
  private getIconForPlugin(pluginName: string): string {
    const iconMap: Record<string, string> = {
      'hello-panel': '👋',
      'text-converter': '🔄',
      'gitlab-adapter': '🦊',
      'chart-panel': '📊',
      'kanban-panel': '📌',
    }
    return iconMap[pluginName] || '🔌'
  }

  /**
   * Register built-in panels as system plugins
   */
  private registerBuiltInPanels(): void {
    console.log('🔧 Registering built-in panels...')

    for (const panel of BUILT_IN_PANELS) {
      const plugin: FrontendPlugin = {
        name: panel.id,
        version: '1.0.0',
        register: (app: App) => {
          app.component(panel.id, panel.component)
          console.log(`✅ Registered built-in panel component: ${panel.id}`)
        },
        panelTypes: new Map([[panel.id, panel.component]]),
      }

      plugin.register(this.app!)
      this.plugins.set(plugin.name, plugin)
      console.log(`✅ Loaded built-in panel as plugin: ${plugin.name}`)
    }
  }

  /**
   * Get initialization status
   */
  isInitialized(): boolean {
    return this.initialized
  }
}

// Export singleton instance
export const pluginLoader = new PluginLoader()
