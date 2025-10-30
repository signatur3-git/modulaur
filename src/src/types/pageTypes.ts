// Page Type System - Core Type Definitions

import type { Component } from 'vue'

/**
 * Page Type Definition
 * Defines a type of page that can be created (dashboard, layout, settings, etc.)
 */
export interface PageType {
  id: string // Unique identifier (e.g., 'dashboard', 'layout', 'settings')
  name: string // Display name (e.g., 'Dashboard Page')
  icon: string // Emoji icon for UI
  description: string // User-facing description
  component: Component // Vue component to render this page type
  supportsConfig: boolean // Whether this type has configurable options
  configSchema?: ConfigSchema // JSON schema for configuration validation
  defaultConfig?: Record<string, any> // Default configuration values
  canHaveChildren?: boolean // Whether this page can contain child pages
  metadata?: Record<string, any> // Additional metadata for extensions
}

/**
 * Configuration Schema
 * Defines the structure and validation for page type configuration
 */
export interface ConfigSchema {
  type: 'object'
  properties: Record<string, ConfigProperty>
  required?: string[]
}

export interface ConfigProperty {
  type: 'string' | 'number' | 'boolean' | 'array' | 'object'
  title?: string
  description?: string
  default?: any
  enum?: any[]
  items?: ConfigProperty
  properties?: Record<string, ConfigProperty>
}

/**
 * Page Configuration
 * Runtime configuration for a page instance
 */
export interface PageConfig {
  type: string // Page type ID
  config?: Record<string, any> // Type-specific configuration
  layout?: string // Layout template ID (for layout pages)
  slots?: PageSlots // Slot assignments (for layout pages)
}

/**
 * Slot Assignments for Layout Pages
 */
export interface PageSlots {
  [slotId: string]: SlotContent[]
}

export interface SlotContent {
  panelId: string // ID of panel assigned to this slot
  order: number // Order within the slot
  config?: Record<string, any> // Panel-specific config for this slot
}

/**
 * Layout Template Definition
 * Defines a layout structure with named slots
 */
export interface LayoutTemplate {
  id: string // Unique identifier (e.g., 'sidebar-main')
  name: string // Display name
  description: string // User-facing description
  icon: string // Emoji icon
  slots: LayoutSlot[] // Available slots in this layout
  preview: string // SVG or data URL preview image
  component: Component // Vue component that renders this layout
  metadata?: Record<string, any> // Additional metadata
}

/**
 * Layout Slot Definition
 */
export interface LayoutSlot {
  id: string // Unique slot identifier (e.g., 'sidebar', 'main')
  name: string // Display name
  description?: string // Optional description
  defaultWidth?: string // CSS width (e.g., '300px', '30%')
  defaultHeight?: string // CSS height
  constraints?: SlotConstraints // Slot constraints
}

export interface SlotConstraints {
  minPanels?: number // Minimum panels required
  maxPanels?: number // Maximum panels allowed
  allowedPanelTypes?: string[] // Which panel types are allowed
  resizable?: boolean // Whether slot can be resized
  collapsible?: boolean // Whether slot can be collapsed
}

/**
 * Page Type Registry Events
 */
export interface PageTypeRegistryEvents {
  // eslint-disable-next-line no-unused-vars
  'type-registered': (arg: PageType) => void
  // eslint-disable-next-line no-unused-vars
  'type-unregistered': (arg: string) => void
  // eslint-disable-next-line no-unused-vars
  'layout-registered': (arg: LayoutTemplate) => void
  // eslint-disable-next-line no-unused-vars
  'layout-unregistered': (arg: string) => void
}
