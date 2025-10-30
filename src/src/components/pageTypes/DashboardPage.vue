<template>
  <div class="dashboard-page">
    <DashboardEditor />
  </div>
</template>

<script setup lang="ts">
import DashboardEditor from '@/components/DashboardEditor.vue'

/**
 * Dashboard Page Type
 *
 * Provides a free-form grid layout with drag-and-drop panels.
 * This is the existing dashboard functionality formalized as a page type.
 *
 * Features:
 * - Grid layout with configurable columns
 * - Drag-and-drop panel positioning
 * - Panel resize support
 * - Add/remove panels
 * - Panel configuration
 */

// Props for page config (if needed in the future)
defineProps<{
  config?: {
    columns?: number
    rowHeight?: number
    margin?: [number, number]
  }
}>()

// Currently uses the existing DashboardEditor without modification
// Future: Could pass config props to DashboardEditor for customization
</script>

<style scoped>
.dashboard-page {
  width: 100%;
  height: 100%;
  overflow: auto;
}
</style>
