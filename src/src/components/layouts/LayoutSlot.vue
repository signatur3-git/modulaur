<template>
  <div class="layout-slot" :class="slotClasses">
    <div v-if="showHeader" class="slot-header">
      <h3>{{ slotDefinition?.name || slotId }}</h3>
      <span class="slot-count">{{ panels.length }} panel{{ panels.length !== 1 ? 's' : '' }}</span>
    </div>

    <div class="slot-content">
      <div v-if="panels.length === 0" class="empty-slot">
        <p>{{ emptyMessage }}</p>
      </div>

      <div v-for="panel in renderedPanels" :key="panel.i" class="slot-panel">
        <PanelWrapper :panel="panel" :is-editing="false" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { LayoutSlot as LayoutSlotDef } from '@/types/pageTypes'
import PanelWrapper from '@/components/PanelWrapper.vue'

/**
 * Layout Slot Component
 *
 * Renders a named slot within a layout page.
 * Displays panels assigned to this slot in order.
 *
 * Note: Full panel rendering will be implemented in a future phase.
 * Currently shows panel placeholders.
 */

const props = withDefaults(
  defineProps<{
    slotId: string
    panels?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
    slotDefinition?: LayoutSlotDef
    showHeader?: boolean
    emptyMessage?: string
  }>(),
  {
    panels: () => [],
    showHeader: false,
    emptyMessage: 'No panels in this slot',
  }
)

// Sort panels by order
const sortedPanels = computed(() => {
  if (!props.panels || props.panels.length === 0) return []
  return [...props.panels].sort((a, b) => a.order - b.order)
})

const renderedPanels = computed(() => {
  return sortedPanels.value.map((p, idx) => ({
    i: `${props.slotId}:${p.panelId}:${idx}`,
    x: 0,
    y: 0,
    w: 0,
    h: 0,
    type: p.panelId,
    title: p.panelId,
    config: p.config || {},
  }))
})

// Slot CSS classes
const slotClasses = computed(() => {
  return {
    'has-panels': sortedPanels.value.length > 0,
    'is-empty': sortedPanels.value.length === 0,
  }
})
</script>

<style scoped>
.layout-slot {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: auto;
}

.slot-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: var(--bg-panel-header);
  border-bottom: 1px solid var(--border-color);
}

.slot-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-heading);
}

.slot-count {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.slot-content {
  flex: 1;
  overflow: auto;
  padding: var(--panel-padding);
}

.empty-slot {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 200px;
  text-align: center;
  color: var(--text-secondary);
  border: 2px dashed var(--border-color);
  border-radius: var(--panel-radius);
  background: var(--bg-empty);
}

.empty-slot p {
  margin: 0;
  font-size: 0.9rem;
}

.slot-panel {
  margin-bottom: var(--panel-gap);
}

.slot-panel:last-child {
  margin-bottom: 0;
}

.is-empty .slot-content {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
