// Register Base Page Types
// This module registers the built-in page types

import { getPageTypeRegistry } from '@/services/pageTypeRegistry'
import type { PageType } from '@/types/pageTypes'
import { markRaw } from 'vue'

// Import page type components
import DashboardPage from '@/components/pageTypes/DashboardPage.vue'
import DashboardCollectionPage from '@/components/pageTypes/DashboardCollectionPage.vue'
import LayoutPage from '@/components/pageTypes/LayoutPage.vue'
import SettingsPage from '@/components/pageTypes/SettingsPage.vue'

/**
 * Register all base page types
 */
export function registerBasePageTypes(): void {
  const registry = getPageTypeRegistry()

  // Register Dashboard Collection Page Type
  registry.register({
    id: 'dashboard-collection',
    name: 'Dashboard Collection',
    icon: '📊',
    description:
      'A collection of dashboards with overview page. Each dashboard collection can have multiple dashboards that can be navigated between.',
    component: markRaw(DashboardCollectionPage),
    supportsConfig: true,
    configSchema: {
      type: 'object',
      properties: {
        dashboards: {
          type: 'array',
          title: 'Dashboards',
          description: 'List of dashboards in this collection',
          items: {
            type: 'object',
            properties: {
              id: { type: 'string' },
              name: { type: 'string' },
              panels: { type: 'array' },
              createdAt: { type: 'number' },
              updatedAt: { type: 'number' },
            },
          },
        },
      },
    },
    defaultConfig: {
      dashboards: [],
    },
    canHaveChildren: false,
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'visualization',
    },
  })

  // Register Dashboard Page Type (Legacy - single dashboard)
  registry.register({
    id: 'dashboard',
    name: 'Dashboard Page',
    icon: '📊',
    description:
      'Free-form grid layout with drag-and-drop panels. Perfect for creating custom dashboards with charts, tables, and widgets.',
    component: markRaw(DashboardPage),
    supportsConfig: true,
    configSchema: {
      type: 'object',
      properties: {
        columns: {
          type: 'number',
          title: 'Grid Columns',
          description: 'Number of columns in the grid',
          default: 12,
        },
        rowHeight: {
          type: 'number',
          title: 'Row Height',
          description: 'Height of each row in pixels',
          default: 30,
        },
        compactMode: {
          type: 'boolean',
          title: 'Compact Mode',
          description: 'Minimize vertical gaps between panels',
          default: true,
        },
      },
    },
    defaultConfig: {
      columns: 12,
      rowHeight: 30,
      compactMode: true,
    },
    canHaveChildren: false,
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'visualization',
    },
  })

  // Register Layout Page Type
  registry.register({
    id: 'layout',
    name: 'Layout Page',
    icon: '📐',
    description:
      'Fixed layout with named slots for panel placement. Choose from predefined layouts like sidebar, three-column, or header-footer.',
    component: markRaw(LayoutPage),
    supportsConfig: true,
    configSchema: {
      type: 'object',
      properties: {
        layout: {
          type: 'string',
          title: 'Layout Template',
          description: 'Choose a layout template',
          enum: ['single-main', 'sidebar-main', 'three-column', 'header-main-footer'],
          default: 'sidebar-main',
        },
        slots: {
          type: 'object',
          title: 'Slot Assignments',
          description: 'Panel assignments for each slot',
        },
      },
      required: ['layout'],
    },
    defaultConfig: {
      layout: 'sidebar-main',
      slots: {},
    },
    canHaveChildren: false,
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'layout',
    },
  })

  // Register Settings Page Type
  registry.register({
    id: 'settings',
    name: 'Settings Page',
    icon: '⚙️',
    description:
      'Form-based configuration page with various field types. Perfect for application settings, preferences, or configuration panels.',
    component: markRaw(SettingsPage),
    supportsConfig: true,
    configSchema: {
      type: 'object',
      properties: {
        title: {
          type: 'string',
          title: 'Page Title',
          default: 'Settings',
        },
        description: {
          type: 'string',
          title: 'Description',
        },
        sections: {
          type: 'array',
          title: 'Settings Sections',
          description: 'Groups of related settings',
        },
        storageKey: {
          type: 'string',
          title: 'Storage Key',
          description: 'LocalStorage key for persisting values',
          default: 'settings-page-values',
        },
      },
    },
    defaultConfig: {
      title: 'Settings',
      sections: [],
      storageKey: 'settings-page-values',
    },
    canHaveChildren: false,
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'forms',
    },
  })

  console.log('✅ Registered base page types (Dashboard, Layout, Settings)')
}

/**
 * Get page type by ID (convenience method)
 */
export function getPageType(typeId: string): PageType | undefined {
  return getPageTypeRegistry().get(typeId)
}

/**
 * Get all available page types
 */
export function getAllPageTypes(): PageType[] {
  return getPageTypeRegistry().getAll()
}

/**
 * Check if a page type exists
 */
export function hasPageType(typeId: string): boolean {
  return getPageTypeRegistry().has(typeId)
}
