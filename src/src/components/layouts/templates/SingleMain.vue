<template>
  <div class="single-main-layout">
    <LayoutSlot
      slot-id="main"
      :panels="slots?.main"
      :show-header="false"
      :empty-message="'No panels assigned. Add a panel to fill this page.'"
    />
  </div>
</template>

<script setup lang="ts">
import LayoutSlot from '@/components/layouts/LayoutSlot.vue'

/**
 * Single-Main Layout Template
 *
 * A full-page single slot layout.
 * Use this when you want a panel to fill the entire page.
 *
 * Slot:
 * - main: Full page content
 */

defineProps<{
  slots?: {
    main?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
  }
}>()
</script>

<style scoped>
.single-main-layout {
  height: 100%;
  width: 100%;
  overflow: auto;
}
</style>
