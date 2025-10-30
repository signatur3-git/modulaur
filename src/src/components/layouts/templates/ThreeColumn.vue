<template>
  <div class="three-column-layout">
    <div class="column column-left">
      <LayoutSlot
        slot-id="left"
        :panels="slots?.left"
        :show-header="true"
        :empty-message="'No panels in left column'"
      />
    </div>

    <div class="column column-center">
      <LayoutSlot
        slot-id="center"
        :panels="slots?.center"
        :show-header="true"
        :empty-message="'No panels in center column'"
      />
    </div>

    <div class="column column-right">
      <LayoutSlot
        slot-id="right"
        :panels="slots?.right"
        :show-header="true"
        :empty-message="'No panels in right column'"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import LayoutSlot from '@/components/layouts/LayoutSlot.vue'

/**
 * Three-Column Layout Template
 *
 * Three equal-width columns for balanced content display.
 * Perfect for dashboards with multiple data views or comparisons.
 *
 * Slots:
 * - left: Left column
 * - center: Center column
 * - right: Right column
 */

defineProps<{
  slots?: {
    left?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
    center?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
    right?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
  }
  config?: {
    columnWidths?: [string, string, string]
    gap?: string
  }
}>()
</script>

<style scoped>
.three-column-layout {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 1rem;
  height: 100%;
  padding: 1rem;
  overflow: auto;
}

.column {
  min-width: 0;
  overflow: auto;
  background: #ffffff;
  border: 1px solid #dee2e6;
  border-radius: 8px;
}
</style>
