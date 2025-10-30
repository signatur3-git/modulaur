<template>
  <div class="page-view">
    <div v-if="loading" class="loading">Loading page...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="page" class="page-content">
      <!-- Render page type component dynamically -->
      <component
        v-if="pageComponent"
        :is="pageComponent"
        :pageId="currentPageId"
        :pageRoute="currentPageRoute"
        :pageName="page.name"
        :config="pageConfig"
        @update:config="updatePageConfig"
      />

      <!-- Fallback if page type not found -->
      <div v-else class="page-type-error">
        <h2>⚠️ Unknown Page Type</h2>
        <p>Page type "{{ page.type }}" is not registered.</p>
        <p>Available types: {{ availableTypes.join(', ') || 'none' }}</p>
        <div class="page-meta">
          <p><strong>Page:</strong> {{ page.name }}</p>
          <p><strong>Type:</strong> {{ page.type }}</p>
          <p><strong>Route:</strong> {{ page.route }}</p>
        </div>
      </div>
    </div>
    <div v-else class="not-found">Page not found</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import type { Component } from 'vue'
import { useRoute } from 'vue-router'
import { usePageStore } from '@/stores/pageStore'
import { getPageType, getAllPageTypes } from '@/services/pageTypes'
import type { Page } from '@/stores/pageStore'

const props = defineProps<{
  id?: string
}>()

const route = useRoute()
const pageStore = usePageStore()

const page = ref<Page | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

// Helper to extract string ID from Thing or string
function getPageId(page: Page): string {
  if (!page.id) return ''
  if (typeof page.id === 'string') return page.id

  if (typeof page.id === 'object' && 'id' in page.id) {
    const thingId = (page.id as any).id
    if (typeof thingId === 'string') return thingId
    if (typeof thingId === 'number') return thingId.toString()

    // Handle SurrealDB's internal ID format: { String: "..." }
    if (typeof thingId === 'object' && thingId !== null) {
      if ('String' in thingId && typeof (thingId as any).String === 'string') {
        return (thingId as any).String
      }
      if ('Number' in thingId && typeof (thingId as any).Number === 'number') {
        return (thingId as any).Number.toString()
      }
    }
  }

  return String(page.id)
}

// Get the page type component dynamically
const pageComponent = computed<Component | null>(() => {
  if (!page.value) return null

  const pageType = getPageType(page.value.type)
  if (!pageType) {
    console.warn(`Page type "${page.value.type}" not found in registry`)
    return null
  }

  return pageType.component
})

// Get page config
const pageConfig = computed(() => {
  return page.value?.config || {}
})

// Get current page ID (SurrealDB-generated, may change on migration)
const currentPageId = computed(() => {
  return page.value ? getPageId(page.value) : ''
})

// Get current page route (stable, user-defined identifier)
const currentPageRoute = computed(() => {
  if (!page.value?.route) return ''
  // Strip leading slash if present
  return page.value.route.startsWith('/') ? page.value.route.substring(1) : page.value.route
})

// Get available page types for error message
const availableTypes = computed(() => {
  return getAllPageTypes().map(t => t.id)
})

// Update page config (for dashboard collections)
async function updatePageConfig(newConfig: any) {
  if (!page.value) return

  try {
    const pageId = getPageId(page.value)
    await pageStore.updatePage(pageId, { config: newConfig })

    // Reload the page to reflect changes
    await loadPage()
  } catch (e) {
    console.error('Failed to update page config:', e)
    alert(`Failed to update page config: ${e}`)
  }
}

async function loadPage() {
  loading.value = true
  error.value = null

  try {
    const routeParam = props.id || (route.params.id as string)
    await pageStore.loadPages()

    // Find page by route (stable identifier) instead of by SurrealDB ID
    // The route param from URL is the page's route field (e.g., "snippets", "notes")
    // Try matching with and without leading slash
    const routeWithSlash = routeParam.startsWith('/') ? routeParam : `/${routeParam}`
    const routeWithoutSlash = routeParam.startsWith('/') ? routeParam.substring(1) : routeParam

    page.value =
      pageStore.pages.find(
        p => p.route === routeWithSlash || p.route === routeWithoutSlash || p.route === routeParam
      ) || null

    // Fallback: try finding by ID for backward compatibility
    if (!page.value) {
      page.value = pageStore.pages.find(p => getPageId(p) === routeParam) || null
    }

    if (!page.value) {
      error.value = `Page with route "${routeParam}" not found`
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadPage()
})

watch(
  () => props.id || route.params.id,
  () => {
    loadPage()
  }
)
</script>

<style scoped>
.page-view {
  padding: 2rem;
  height: 100%;
  overflow: auto;
}

.page-content {
  height: 100%;
}

.loading,
.error,
.not-found {
  text-align: center;
  padding: 3rem;
  font-size: 1.2rem;
}

.error {
  color: #dc3545;
}

.page-type-error {
  max-width: 600px;
  margin: 2rem auto;
  padding: 2rem;
  border: 2px solid #ffc107;
  border-radius: 8px;
  background: #fff3cd;
  text-align: center;
}

.page-type-error h2 {
  margin-top: 0;
  color: #856404;
}

.page-type-error p {
  margin: 0.5rem 0;
  color: #856404;
}

.page-meta {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid #ffc107;
  text-align: left;
}

.page-meta p {
  margin: 0.5rem 0;
  font-family: monospace;
  font-size: 0.9rem;
}
</style>
