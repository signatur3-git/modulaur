<template>
  <div class="panel-wrapper" :class="`panel-${panel.type}`">
    <!-- Only the header should be used as drag handle for grid-layout -->
    <div class="panel-header vue-grid-item-drag-handle">
      <h3>{{ panel.title }}</h3>
      <div v-if="isEditing" class="panel-actions">
        <button @click="$emit('edit', panel.i)" class="btn-icon" title="Edit">⚙️</button>
        <button @click="$emit('remove', panel.i)" class="btn-icon btn-danger" title="Remove">
          ✕
        </button>
      </div>
    </div>
    <!-- Content should NOT be draggable by grid-layout, only by internal drag systems -->
    <div class="panel-content no-drag">
      <component :is="panelComponent" :panel="panel" :is-editing="isEditing" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Panel } from '../stores/dashboardStore'
import { pluginLoader } from '@/services/pluginLoader'
import TextPanel from './panels/TextPanel.vue'

interface Props {
  panel: Panel
  isEditing: boolean
}

const props = defineProps<Props>()
defineEmits<{
  edit: [id: string]
  remove: [id: string]
}>()

const panelComponent = computed(() => {
  // Get component from plugin loader (includes built-in panels now)
  const pluginComponent = pluginLoader.getPanelComponent(props.panel.type)
  if (pluginComponent) {
    console.log(`🔌 Using component for panel type: ${props.panel.type}`)
    return pluginComponent
  }

  // Fallback to a default component if nothing found
  console.warn(
    `⚠️ No component found for panel type: ${props.panel.type}, using TextPanel as fallback`
  )
  return TextPanel
})
</script>

<style scoped>
.panel-wrapper {
  height: 100%;
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  box-shadow: var(--panel-shadow);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: box-shadow 0.2s ease;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--panel-header-padding);
  background: var(--bg-panel-header);
  border-bottom: 1px solid var(--border-color);
}

/* When panel is in a draggable grid, show move cursor on header */
.vue-grid-item.is-draggable .panel-header.vue-grid-item-drag-handle {
  cursor: move;
}

.panel-header h3 {
  margin: 0;
  font-size: 1rem;
  user-select: none; /* Prevent text selection while dragging */
  font-weight: 600;
  color: var(--text-heading);
}

.panel-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-icon {
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 0.25rem;
  font-size: 1rem;
  opacity: 0.7;
  transition: opacity 0.2s;
  color: var(--text-secondary);
}

.btn-icon:hover {
  opacity: 1;
  background: var(--border-color);
  border-radius: 4px;
}

.btn-danger:hover {
  color: var(--accent-danger);
}

.panel-content {
  flex: 1;
  overflow: auto;
  padding: var(--panel-padding);
  /* Ensure internal components can handle their own events */
  position: relative;
}

/* Ensure panel content allows internal drag operations */
.panel-content.no-drag {
  cursor: default;
  /* Content should not be draggable by grid, but allow child dragging */
  pointer-events: auto;
}
</style>
