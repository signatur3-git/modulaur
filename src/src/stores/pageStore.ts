import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// SurrealDB Thing type
interface Thing {
  tb: string
  id: string | number
}

export interface Page {
  id: string | Thing // Can be either string or Thing from SurrealDB
  name: string
  route: string
  type: string
  icon?: string
  config?: any
  order: number
  visible: boolean
  created_at: string
  updated_at: string
}

// Helper to extract string ID from Thing or string
function getPageId(page: Page): string {
  if (!page.id) {
    return ''
  }

  if (typeof page.id === 'string') {
    return page.id
  }

  // It's a Thing object { tb: "pages", id: ... }
  if (typeof page.id === 'object' && 'id' in page.id) {
    const thingId = page.id.id as any

    // Handle string IDs
    if (typeof thingId === 'string') {
      return thingId
    }

    // Handle numeric IDs
    if (typeof thingId === 'number') {
      return thingId.toString()
    }

    // Handle SurrealDB's internal ID format: { String: "..." }
    if (typeof thingId === 'object' && thingId !== null) {
      if ('String' in thingId && typeof thingId.String === 'string') {
        return thingId.String
      }
      if ('Number' in thingId && typeof thingId.Number === 'number') {
        return thingId.Number.toString()
      }
      // Fallback: stringify the complex ID
      return JSON.stringify(thingId)
    }
  }

  // Fallback: convert entire id to string
  return String(page.id)
}

export const usePageStore = defineStore('pages', () => {
  // State
  const pages = ref<Page[]>([])
  const currentPageId = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const visiblePages = computed(() =>
    pages.value.filter(p => p.visible).sort((a, b) => a.order - b.order)
  )

  const currentPage = computed(() => pages.value.find(p => getPageId(p) === currentPageId.value))

  // Actions
  async function loadPages() {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<Page[]>('get_pages')
      pages.value = result
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to load pages:', e)
      // Return empty array on error for graceful degradation
      return []
    } finally {
      loading.value = false
    }
  }

  async function createPage(pageData: Partial<Page>): Promise<Page> {
    loading.value = true
    error.value = null

    try {
      const newPage = await invoke<Page>('create_page', {
        page: {
          ...pageData,
          id: null, // Let SurrealDB generate Thing ID
          created_at: pageData.created_at || new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      })
      pages.value.push(newPage)
      return newPage
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updatePage(id: string, updates: Partial<Page>): Promise<Page> {
    loading.value = true
    error.value = null

    try {
      const updated = await invoke<Page>('update_page', {
        id,
        updates: {
          ...updates,
          updated_at: new Date().toISOString(),
        },
      })

      const index = pages.value.findIndex(p => getPageId(p) === id)
      if (index !== -1) {
        pages.value[index] = updated
      }

      return updated
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deletePage(id: string): Promise<void> {
    loading.value = true
    error.value = null

    try {
      await invoke('delete_page', { id })
      pages.value = pages.value.filter(p => getPageId(p) !== id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function reorderPages(pageIds: string[]): Promise<void> {
    loading.value = true
    error.value = null

    try {
      await invoke('reorder_pages', { pageIds })

      // Update local order
      pageIds.forEach((id, index) => {
        const page = pages.value.find(p => getPageId(p) === id)
        if (page) {
          page.order = index
        }
      })
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    // State
    pages,
    currentPageId,
    loading,
    error,

    // Getters
    visiblePages,
    currentPage,

    // Actions
    loadPages,
    createPage,
    updatePage,
    deletePage,
    reorderPages,
  }
})
