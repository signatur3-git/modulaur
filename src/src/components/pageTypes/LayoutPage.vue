<template>
  <div class="layout-page">
    <div v-if="!layoutTemplate" class="error-state">
      <h2>⚠️ Layout Template Not Found</h2>
      <p>Layout template "{{ config?.layout || 'unknown' }}" is not registered.</p>
      <p>Available templates: {{ availableLayouts.join(', ') || 'none' }}</p>
    </div>

    <component
      v-else
      :is="layoutTemplate.component"
      :slots="config?.slots || {}"
      :config="config || {}"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { getLayoutTemplateRegistry } from '@/services/layoutTemplateRegistry'

/**
 * Layout Page Type
 *
 * Provides fixed layouts with named slots for panel placement.
 * Unlike dashboard pages with free-form grids, layout pages have
 * predefined regions (slots) where panels can be assigned.
 *
 * Use cases:
 * - Sidebar + Main content layout
 * - Three-column layouts
 * - Header + Main + Footer layouts
 * - Custom plugin layouts
 */

const props = defineProps<{
  config?: {
    layout?: string // Layout template ID (e.g., 'sidebar-main')
    slots?: Record<string, any[]> // Slot assignments
    [key: string]: any // Additional config
  }
}>()

// Get layout template from registry
const layoutTemplate = computed(() => {
  const layoutId = props.config?.layout
  if (!layoutId) {
    console.warn('No layout specified in config')
    return null
  }

  const registry = getLayoutTemplateRegistry()
  const template = registry.get(layoutId)

  if (!template) {
    console.warn(`Layout template "${layoutId}" not found in registry`)
  }

  return template
})

// Get available layouts for error message
const availableLayouts = computed(() => {
  return getLayoutTemplateRegistry()
    .getAll()
    .map(t => t.id)
})
</script>

<style scoped>
.layout-page {
  width: 100%;
  height: 100%;
  overflow: auto;
}

.error-state {
  max-width: 600px;
  margin: 2rem auto;
  padding: 2rem;
  border: 2px solid #ffc107;
  border-radius: 8px;
  background: #fff3cd;
  text-align: center;
}

.error-state h2 {
  margin-top: 0;
  color: #856404;
}

.error-state p {
  margin: 0.5rem 0;
  color: #856404;
}
</style>
