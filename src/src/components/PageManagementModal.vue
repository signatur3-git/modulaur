<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-dialog">
      <div class="modal-header">
        <h2>📄 Page Management</h2>
        <button @click="$emit('close')" class="close-btn">×</button>
      </div>

      <div class="modal-body">
        <!-- Create New Page Button -->
        <div class="action-bar">
          <button @click="openCreatePage" class="btn-primary">➕ Create New Page</button>
        </div>

        <!-- Pages List with Drag & Drop -->
        <div v-if="pages.length > 0" class="draggable-container" @mousedown.stop @touchstart.stop>
          <VueDraggable
            v-model="pagesList"
            class="pages-list"
            handle=".page-drag-handle"
            :animation="200"
            :force-fallback="true"
            :fallback-tolerance="3"
            ghost-class="page-ghost"
            @start="onDragStart"
            @end="handleReorder"
          >
            <div v-for="page in pagesList" :key="getPageId(page)" class="page-item">
              <div class="page-drag-handle">⋮⋮</div>
              <div class="page-info">
                <span v-if="page.icon" class="page-icon">{{ page.icon }}</span>
                <span class="page-name">{{ page.name }}</span>
                <span class="page-type">{{ page.type }}</span>
              </div>
              <div class="page-actions">
                <button @click="editPage(page)" class="btn-icon" title="Edit">✏️</button>
                <button @click="deletePage(page)" class="btn-icon btn-danger" title="Delete">
                  🗑️
                </button>
              </div>
            </div>
          </VueDraggable>
        </div>

        <div v-else class="empty-state">
          <p>No pages found. Create your first page!</p>
        </div>

        <!-- Info Section -->
        <div class="info-section">
          <p class="info-text">
            💡 <strong>Tip:</strong> Drag and drop pages to reorder them in the navigation bar.
          </p>
          <p class="info-text">📌 <strong>Coming soon:</strong> Hierarchical page organization</p>
        </div>
      </div>

      <div class="modal-footer">
        <button @click="$emit('close')" class="btn-secondary">Close</button>
      </div>
    </div>

    <!-- Page Manager Modal for Create/Edit -->
    <PageManager
      :is-open="showPageManager"
      :page="editingPage"
      @close="closePageManager"
      @saved="handlePageSaved"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { usePageStore } from '@/stores/pageStore'
import type { Page } from '@/stores/pageStore'
import PageManager from './navigation/PageManager.vue'
import { VueDraggable } from 'vue-draggable-plus'

const props = defineProps<{
  isOpen: boolean
}>()

// Using defineEmits for type safety, even though we use modelValue
defineEmits<{
  close: []
}>()

const pageStore = usePageStore()
const showPageManager = ref(false)
const editingPage = ref<Page | null>(null)

const pages = computed(() => pageStore.pages)
const pagesList = ref<Page[]>([])

// Sync pagesList with store
watch(
  pages,
  newPages => {
    pagesList.value = [...newPages]
  },
  { immediate: true }
)

// Helper to extract string ID from Thing or string
function getPageId(page: Page): string {
  if (!page.id) return ''
  if (typeof page.id === 'string') return page.id

  if (typeof page.id === 'object' && 'id' in page.id) {
    const thingId = page.id.id as any
    if (typeof thingId === 'string') return thingId
    if (typeof thingId === 'number') return thingId.toString()
    if (typeof thingId === 'object' && thingId !== null) {
      if ('String' in thingId && typeof thingId.String === 'string') {
        return thingId.String
      }
      if ('Number' in thingId && typeof thingId.Number === 'number') {
        return thingId.Number.toString()
      }
      return JSON.stringify(thingId)
    }
  }

  return String(page.id)
}

// Load pages when modal opens
watch(
  () => props.isOpen,
  async isOpen => {
    if (isOpen) {
      await pageStore.loadPages()
    }
  }
)

onMounted(async () => {
  if (props.isOpen) {
    await pageStore.loadPages()
  }
})

function openCreatePage() {
  editingPage.value = null
  showPageManager.value = true
}

function editPage(page: Page) {
  editingPage.value = page
  showPageManager.value = true
}

function closePageManager() {
  showPageManager.value = false
  editingPage.value = null
}

async function handlePageSaved() {
  closePageManager()
  await pageStore.loadPages()
}

async function deletePage(page: Page) {
  const pageId = getPageId(page)

  if (!confirm(`Delete page "${page.name}"?\n\nThis action cannot be undone.`)) {
    return
  }

  try {
    await pageStore.deletePage(pageId)
    await pageStore.loadPages()
  } catch (e) {
    alert(`Failed to delete page: ${e}`)
  }
}

// Drag and Drop handlers using VueDraggable
function onDragStart() {
  console.log('🎯 Drag started')
}

async function handleReorder() {
  console.log('🔄 Reordering pages...')

  // Extract page IDs in new order from the reordered list
  const newOrder = pagesList.value.map(p => getPageId(p))

  try {
    await pageStore.reorderPages(newOrder)
    await pageStore.loadPages()
    console.log('✅ Pages reordered successfully')
  } catch (e) {
    console.error('Failed to reorder pages:', e)
    alert(`Failed to reorder pages: ${e}`)
    // Reload to reset order on failure
    await pageStore.loadPages()
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-dialog {
  background: var(--bg-modal);
  border-radius: var(--panel-radius);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-lg);
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  color: var(--text-heading);
  font-size: 1.5rem;
}

.close-btn {
  background: none;
  border: none;
  font-size: 2rem;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.modal-body {
  padding: var(--space-lg);
  overflow-y: auto;
  flex: 1;
}

.action-bar {
  margin-bottom: var(--space-lg);
}

.btn-primary {
  padding: var(--space-sm) var(--space-lg);
  background: var(--bg-button);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--bg-button-hover);
}

.draggable-container {
  flex: 1;
  overflow-y: auto;
}

.pages-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  margin-bottom: var(--space-lg);
  min-height: 50px;
}

.page-item {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-md);
  background: var(--bg-panel-header);
  border: 2px solid transparent;
  border-radius: var(--panel-radius);
  transition: all 0.2s;
  cursor: move;
}

.page-item:hover {
  border-color: var(--border-color);
  box-shadow: var(--panel-shadow);
}

.page-ghost {
  opacity: 0.4;
  background: var(--accent-primary);
}

.page-drag-handle {
  color: var(--text-muted);
  font-size: 1.2rem;
  cursor: grab;
  user-select: none;
}

.page-drag-handle:hover {
  color: var(--text-primary);
}

.page-drag-handle:active {
  cursor: grabbing;
}

.page-info {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  flex: 1;
}

.page-icon {
  font-size: 1.2rem;
}

.page-name {
  font-weight: 500;
  color: var(--text-primary);
}

.page-type {
  font-size: 0.875rem;
  color: var(--text-secondary);
  padding: var(--space-xs) var(--space-sm);
  background: var(--bg-app);
  border-radius: 4px;
}

.page-actions {
  display: flex;
  gap: var(--space-sm);
  pointer-events: auto;
}

.btn-icon {
  background: none;
  border: 1px solid var(--border-color);
  padding: var(--space-xs) var(--space-sm);
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
  pointer-events: auto;
}

.btn-icon:hover {
  background: var(--bg-panel);
  border-color: var(--accent-primary);
}

.btn-icon.btn-danger:hover {
  background: var(--accent-danger);
  border-color: var(--accent-danger);
  color: var(--text-on-accent);
}

.empty-state {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--text-secondary);
}

.info-section {
  margin-top: var(--space-lg);
  padding: var(--space-md);
  background: var(--bg-panel-header);
  border-radius: var(--panel-radius);
  border-left: 4px solid var(--accent-primary);
}

.info-text {
  margin: var(--space-sm) 0;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.modal-footer {
  padding: var(--space-lg);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: var(--space-md);
}

.btn-secondary {
  padding: var(--space-sm) var(--space-lg);
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: var(--bg-button-secondary-hover);
}
</style>
