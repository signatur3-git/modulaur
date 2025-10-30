// Frontend Plugin Types
// Defines interfaces for the plugin system

export interface PluginManifest {
  name: string
  version: string
  author: string
  description: string
  adapter_type?: string
  frontend?: {
    entry: string
    components: Array<{
      type: string
      name: string
      display_name: string
    }>
    styles: string[]
  }
  capabilities?: string[]
  permissions?: string[]
}

export interface FrontendPlugin {
  name: string
  version: string
  // eslint-disable-next-line no-unused-vars
  register: (_app: any) => void | Promise<void>
  panelTypes?: Map<string, any> // panelType -> Component
  pageTypes?: Map<string, any> // pageType -> Component (for page type plugins)
  unregister?: () => void | Promise<void>
  umdPlugin?: any // Reference to dynamically loaded UMD plugin (Phase 3)
}

export interface PluginInfo {
  name: string
  version: string
  author: string
  description: string
  enabled: boolean
  hasBackend: boolean
  hasFrontend: boolean
  panelTypes: string[]
}

export interface PluginPanelProps {
  panel: any
  config: any
  data?: any
}

export interface ComponentInfo {
  type: 'panel' | 'page' | 'theme' | 'view' // Component type
  name: string
  display_name: string
  description?: string
  icon?: string
  category?: string
  config_schema?: ConfigSchema
  // Page-specific options (when type === 'page')
  supports_config?: boolean // Whether page type supports configuration
}

export interface ConfigSchema {
  fields: ConfigField[]
}

export interface ConfigField {
  key: string
  label: string
  type: string // "text", "textarea", "select", "number", "checkbox"
  options?: ConfigOption[]
  placeholder?: string
  required?: boolean
  min?: number
  max?: number
  rows?: number
  help?: string
}

export interface ConfigOption {
  value: string | number | boolean
  label: string
}
