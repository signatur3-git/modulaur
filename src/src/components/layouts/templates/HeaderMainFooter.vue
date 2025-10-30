<template>
  <div class="header-main-footer-layout">
    <header class="header-section">
      <LayoutSlot
        slot-id="header"
        :panels="slots?.header"
        :show-header="false"
        :empty-message="'No panels in header'"
      />
    </header>

    <main class="main-section">
      <LayoutSlot
        slot-id="main"
        :panels="slots?.main"
        :show-header="true"
        :empty-message="'No panels in main content area'"
      />
    </main>

    <footer class="footer-section">
      <LayoutSlot
        slot-id="footer"
        :panels="slots?.footer"
        :show-header="false"
        :empty-message="'No panels in footer'"
      />
    </footer>
  </div>
</template>

<script setup lang="ts">
import LayoutSlot from '@/components/layouts/LayoutSlot.vue'

/**
 * Header-Main-Footer Layout Template
 *
 * Classic three-section vertical layout.
 * Header and footer have fixed heights, main area fills remaining space.
 *
 * Slots:
 * - header: Top section (title, navigation, summary info)
 * - main: Main content area (primary content, scrollable)
 * - footer: Bottom section (actions, status, metadata)
 */

defineProps<{
  slots?: {
    header?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
    main?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
    footer?: Array<{
      panelId: string
      order: number
      config?: Record<string, any>
    }>
  }
  config?: {
    headerHeight?: string
    footerHeight?: string
  }
}>()
</script>

<style scoped>
.header-main-footer-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.header-section {
  flex-shrink: 0;
  border-bottom: 1px solid #dee2e6;
  background: #f8f9fa;
}

.main-section {
  flex: 1;
  overflow: auto;
  background: #ffffff;
}

.footer-section {
  flex-shrink: 0;
  border-top: 1px solid #dee2e6;
  background: #f8f9fa;
}
</style>
