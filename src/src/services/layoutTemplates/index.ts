// Register Layout Templates
// This module registers the built-in layout templates

import { getLayoutTemplateRegistry } from '@/services/layoutTemplateRegistry'
import { markRaw } from 'vue'

// Import layout template components
import SidebarMain from '@/components/layouts/templates/SidebarMain.vue'
import ThreeColumn from '@/components/layouts/templates/ThreeColumn.vue'
import HeaderMainFooter from '@/components/layouts/templates/HeaderMainFooter.vue'
import SingleMain from '@/components/layouts/templates/SingleMain.vue'

/**
 * Register all base layout templates
 */
export function registerLayoutTemplates(): void {
  const registry = getLayoutTemplateRegistry()

  // Register Single-Main Layout
  registry.register({
    id: 'single-main',
    name: 'Single Panel (Full Page)',
    description: 'Single full-page slot. Use this when you want one panel to fill the entire page.',
    icon: '⬛',
    slots: [
      {
        id: 'main',
        name: 'Main',
        description: 'Full page content',
        constraints: {
          minPanels: 0,
          maxPanels: 1,
        },
      },
    ],
    preview:
      'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjE1MCIgeG1zbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB3aWR0aD0iMjAwIiBoZWlnaHQ9IjE1MCIgZmlsbD0iI2Y1ZjVmNSIvPjwvc3ZnPg==',
    component: markRaw(SingleMain),
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'single',
    },
  })

  // Register Sidebar-Main Layout
  registry.register({
    id: 'sidebar-main',
    name: 'Sidebar + Main',
    description: 'Classic two-column layout with sidebar on left and main content on right',
    icon: '📐',
    slots: [
      {
        id: 'sidebar',
        name: 'Sidebar',
        description: 'Left sidebar for navigation, filters, or secondary content',
        defaultWidth: '300px',
        constraints: {
          minPanels: 0,
          maxPanels: 5,
          resizable: true,
          collapsible: true,
        },
      },
      {
        id: 'main',
        name: 'Main Content',
        description: 'Main content area',
        constraints: {
          minPanels: 0,
          resizable: false,
        },
      },
    ],
    preview:
      'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjE1MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB3aWR0aD0iNjAiIGhlaWdodD0iMTUwIiBmaWxsPSIjZTBmMmY3Ii8+PHJlY3QgeD0iNjUiIHdpZHRoPSIxMzUiIGhlaWdodD0iMTUwIiBmaWxsPSIjZjVmNWY1Ii8+PC9zdmc+',
    component: markRaw(SidebarMain),
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'two-column',
    },
  })

  // Register Three-Column Layout
  registry.register({
    id: 'three-column',
    name: 'Three Columns',
    description: 'Three equal-width columns for balanced content display',
    icon: '⫴',
    slots: [
      {
        id: 'left',
        name: 'Left Column',
        description: 'Left column',
        constraints: {
          minPanels: 0,
          maxPanels: 5,
        },
      },
      {
        id: 'center',
        name: 'Center Column',
        description: 'Center column',
        constraints: {
          minPanels: 0,
          maxPanels: 5,
        },
      },
      {
        id: 'right',
        name: 'Right Column',
        description: 'Right column',
        constraints: {
          minPanels: 0,
          maxPanels: 5,
        },
      },
    ],
    preview:
      'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjE1MCIgeG1zbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB3aWR0aD0iNjAiIGhlaWdodD0iMTUwIiBmaWxsPSIjZTBmMmY3Ii8+PHJlY3QgeD0iNjUiIHdpZHRoPSI2MCIgaGVpZ2h0PSIxNTAiIGZpbGw9IiNmNWY1ZjUiLz48cmVjdCB4PSIxMzAiIHdpZHRoPSI3MCIgaGVpZ2h0PSIxNTAiIGZpbGw9IiNlMGYyZjciLz48L3N2Zz4=',
    component: markRaw(ThreeColumn),
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'multi-column',
    },
  })

  // Register Header-Main-Footer Layout
  registry.register({
    id: 'header-main-footer',
    name: 'Header + Main + Footer',
    description: 'Three-section vertical layout with header, scrollable main area, and footer',
    icon: '☰',
    slots: [
      {
        id: 'header',
        name: 'Header',
        description: 'Top section for title, navigation, or summary',
        defaultHeight: 'auto',
        constraints: {
          minPanels: 0,
          maxPanels: 3,
        },
      },
      {
        id: 'main',
        name: 'Main Content',
        description: 'Scrollable main content area',
        constraints: {
          minPanels: 0,
        },
      },
      {
        id: 'footer',
        name: 'Footer',
        description: 'Bottom section for actions, status, or metadata',
        defaultHeight: 'auto',
        constraints: {
          minPanels: 0,
          maxPanels: 3,
        },
      },
    ],
    preview:
      'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjE1MCIgeG1zbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB3aWR0aD0iMjAwIiBoZWlnaHQ9IjMwIiBmaWxsPSIjZTBmMmY3Ii8+PHJlY3QgeT0iMzUiIHdpZHRoPSIyMDAiIGhlaWdodD0iODAiIGZpbGw9IiNmNWY1ZjUiLz48cmVjdCB5PSIxMjAiIHdpZHRoPSIyMDAiIGhlaWdodD0iMzAiIGZpbGw9IiNlMGYyZjciLz48L3N2Zz4=',
    component: markRaw(HeaderMainFooter),
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'vertical',
    },
  })

  console.log(
    '✅ Registered layout templates (Single-Main, Sidebar-Main, Three-Column, Header-Main-Footer)'
  )
}

/**
 * Get layout template by ID (convenience method)
 */
export function getLayoutTemplate(templateId: string) {
  return getLayoutTemplateRegistry().get(templateId)
}

/**
 * Get all available layout templates
 */
export function getAllLayoutTemplates() {
  return getLayoutTemplateRegistry().getAll()
}
