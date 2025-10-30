<template>
  <div class="sidebar-main-layout">
    <aside class="sidebar" :style="sidebarStyle">
      <LayoutSlot
        slot-id="sidebar"
        :panels="slots?.sidebar"
        :show-header="true"
        :empty-message="'No panels in sidebar. Add panels to this slot.'"
      />
    </aside>

    <main class="main-content">
      <LayoutSlot
        slot-id="main"
        :panels="slots?.main"
        :show-header="true"
        :empty-message="'No panels in main area. Add panels to this slot.'"
      />
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import LayoutSlot from '@/components/layouts/LayoutSlot.vue'

/**
 * Sidebar-Main Layout Template
 *
 * A classic two-column layout with a sidebar on the left
 * and main content area on the right.
 *
 * Slots:
 * - sidebar: Left sidebar (navigation, filters, etc.)
 * - main: Main content area (primary content)
 */

const props = defineProps<{
  slots?: {
    sidebar?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
    main?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
  }
  config?: {
    sidebarWidth?: string
    collapsible?: boolean
  }
}>()

// Sidebar styling from config
const sidebarStyle = computed(() => {
  const width = props.config?.sidebarWidth || '300px'
  return {
    width,
    minWidth: width,
    maxWidth: width,
  }
})
</script>

<style scoped>
.sidebar-main-layout {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.sidebar {
  border-right: 1px solid #dee2e6;
  background: #ffffff;
  overflow: auto;
}

.main-content {
  flex: 1;
  overflow: auto;
  background: #ffffff;
}
</style>
