<template>
  <div class="navigation-bar">
    <div class="page-tabs">
      <router-link
        v-for="page in visiblePages"
        :key="getPageRoute(page)"
        :to="`/page/${getPageRoute(page)}`"
        class="page-tab"
        active-class="active"
      >
        <span v-if="page.icon" class="page-icon">{{ page.icon }}</span>
        <span class="page-name">{{ page.name }}</span>
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { usePageStore } from '@/stores/pageStore'
import type { Page } from '@/stores/pageStore'

const pageStore = usePageStore()

const visiblePages = computed(() => pageStore.visiblePages)

// Helper to extract stable route slug from page
function getPageRoute(page: Page): string {
  if (!page.route) {
    return ''
  }
  // Strip leading slash if present
  return page.route.startsWith('/') ? page.route.substring(1) : page.route
}

onMounted(async () => {
  try {
    await pageStore.loadPages()
  } catch (e) {
    console.error('Failed to load pages in NavigationBar:', e)
    // Continue anyway - app should still work
  }
})
</script>

<style scoped>
.navigation-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-sm) var(--space-lg);
  background: var(--bg-toolbar);
  border-bottom: 2px solid var(--border-color);
  box-shadow: var(--panel-shadow);
}

.page-tabs {
  display: flex;
  gap: var(--space-sm);
  flex: 1;
  overflow-x: auto;
}

.page-tab {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-lg);
  border-radius: 6px 6px 0 0;
  text-decoration: none;
  color: var(--text-primary);
  background: transparent;
  transition: all 0.2s;
  white-space: nowrap;
  border: 1px solid transparent;
  position: relative;
}

.page-tab:hover {
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.page-tab.active {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border-color: var(--accent-primary);
}

.page-icon {
  font-size: 1.2em;
}

.page-name {
  font-weight: 500;
  color: inherit;
}
</style>
