<template>
  <div class="kanban-panel">
    <div class="kanban-header">
      <h3>{{ panel.title || 'Kanban Board' }}</h3>
      <div class="kanban-controls">
        <button @click="refreshData" class="btn-refresh" :disabled="loading">
          <span v-if="loading" class="spinner"></span>
          {{ loading ? 'Loading...' : 'Refresh' }}
        </button>
      </div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-if="loading && columns.length === 0" class="loading">Loading kanban data...</div>

    <div v-else-if="columns.length === 0" class="empty-state">
      <p>No columns configured</p>
      <p class="hint">Configure columns in panel settings!</p>
    </div>

    <div v-else class="kanban-board">
      <div v-for="column in columns" :key="column.id" class="kanban-column">
        <div class="column-header" :style="{ backgroundColor: column.color }">
          <h4>{{ column.title }}</h4>
          <span class="column-count">{{ column.cards.length }}</span>
        </div>

        <VueDraggable
          v-model="column.cards"
          :group="{ name: 'kanban', pull: true, put: true }"
          class="column-cards"
          :animation="200"
          ghostClass="card-ghost"
          @change="handleCardMove($event, column.id)"
        >
          <div
            v-for="card in column.cards"
            :key="card.id"
            class="kanban-card"
            @click="showCardDetails(card)"
          >
            <div class="card-header">
              <span class="card-title">{{ card.title }}</span>
              <span v-if="card.type" class="card-type">{{ card.type }}</span>
            </div>

            <div v-if="card.description" class="card-description">
              {{ card.description }}
            </div>

            <div class="card-meta">
              <span v-if="card.timestamp" class="card-time">
                {{ formatTime(card.timestamp) }}
              </span>
              <div class="card-tags">
                <span v-for="tag in card.tags" :key="tag" class="card-tag">
                  {{ tag }}
                </span>
              </div>
            </div>
          </div>
        </VueDraggable>
      </div>
    </div>

    <!-- Card Details Modal -->
    <div v-if="selectedCard" class="modal-overlay" @click="selectedCard = null">
      <div class="modal-content card-details" @click.stop>
        <h3>{{ selectedCard.title }}</h3>
        <div class="detail-section">
          <label>Type:</label>
          <span>{{ selectedCard.type || 'N/A' }}</span>
        </div>
        <div class="detail-section">
          <label>Status:</label>
          <span>{{ selectedCard.status || 'N/A' }}</span>
        </div>
        <div v-if="selectedCard.description" class="detail-section">
          <label>Description:</label>
          <p>{{ selectedCard.description }}</p>
        </div>
        <div class="detail-section">
          <label>Created:</label>
          <span>{{ formatTime(selectedCard.timestamp) }}</span>
        </div>
        <div v-if="selectedCard.tags && selectedCard.tags.length" class="detail-section">
          <label>Tags:</label>
          <div class="card-tags">
            <span v-for="tag in selectedCard.tags" :key="tag" class="card-tag">
              {{ tag }}
            </span>
          </div>
        </div>
        <div class="detail-section">
          <label>Raw Data:</label>
          <pre class="raw-data">{{ JSON.stringify(selectedCard.data, null, 2) }}</pre>
        </div>
        <button @click="selectedCard = null" class="btn-close-modal">Close</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { VueDraggable } from 'vue-draggable-plus'

interface Panel {
  id: string
  title?: string
  config?: {
    statusField?: string
    columns?: Array<{
      id: string
      title: string
      statusValue: string
      color: string
    }>
    recordType?: string
    dataSource?: string
  }
}

interface KanbanCard {
  id: string
  title: string
  description?: string
  status: string
  type?: string
  timestamp: string
  tags?: string[]
  data: any
}

interface KanbanColumn {
  id: string
  title: string
  statusValue: string
  color: string
  cards: KanbanCard[]
}

const props = defineProps<{
  panel: Panel
}>()

const isTauri = computed(() => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
})

const loading = ref(false)
const error = ref('')
const records = ref<any[]>([])
const selectedCard = ref<KanbanCard | null>(null)

// Default columns if not configured
const defaultColumns: KanbanColumn[] = [
  { id: 'pending', title: 'Pending', statusValue: 'pending', color: '#FCD34D', cards: [] },
  { id: 'running', title: 'Running', statusValue: 'running', color: '#60A5FA', cards: [] },
  { id: 'success', title: 'Success', statusValue: 'success', color: '#4ADE80', cards: [] },
  { id: 'failed', title: 'Failed', statusValue: 'failed', color: '#F87171', cards: [] },
]

const columns = computed(() => {
  const configuredColumns = props.panel.config?.columns || defaultColumns

  // Transform records into cards and organize by status
  const columnData: KanbanColumn[] = configuredColumns.map(col => ({
    ...col,
    cards: [],
  }))

  records.value.forEach(record => {
    const status = record.metadata?.status || 'unknown'
    const column = columnData.find(col => col.statusValue === status)

    if (column) {
      column.cards.push({
        id: record.id,
        title: record.metadata?.title || record.data?.name || `Record ${record.id.slice(0, 8)}`,
        description: record.metadata?.description || '',
        status: status,
        type: record.record_type,
        timestamp: record.timestamp,
        tags: record.metadata?.tags || [],
        data: record.data,
      })
    }
  })

  return columnData
})

async function fetchData() {
  if (!isTauri.value) {
    // Browser mode - use mock data
    records.value = generateMockData()
    return
  }

  loading.value = true
  error.value = ''

  try {
    const { invoke } = (window as any).__TAURI_INTERNALS__

    // Fetch records based on config
    if (props.panel.config?.recordType) {
      records.value = await invoke('get_records_by_type', {
        recordType: props.panel.config.recordType,
      })
    } else {
      records.value = await invoke('get_staged_records', {
        limit: 1000,
        offset: 0,
      })
    }
  } catch (e: any) {
    error.value = `Failed to fetch data: ${e.message || e}`
    console.error('Kanban data fetch error:', e)
  } finally {
    loading.value = false
  }
}

function generateMockData() {
  // Mock data for browser mode
  const statuses = ['pending', 'running', 'success', 'failed']
  return Array.from({ length: 20 }, (_, i) => ({
    id: `mock-${i}`,
    record_type: 'gitlab_job',
    source: 'mock-source',
    timestamp: new Date(Date.now() - i * 3600000).toISOString(),
    data: { id: i, name: `Task ${i}`, value: i * 10 },
    metadata: {
      status: statuses[i % statuses.length],
      title: `Task ${i + 1}`,
      description: `This is task number ${i + 1}`,
      tags: ['mock', 'test', `priority-${(i % 3) + 1}`],
    },
  }))
}

function handleCardMove(event: any, columnId: string) {
  console.log('Card moved:', event, 'to column:', columnId)
  // In a real app, this would update the backend
  // For now, just log the change
}

function showCardDetails(card: KanbanCard) {
  selectedCard.value = card
}

function formatTime(timestamp: string): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()

  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 60) return `${minutes}m ago`
  if (hours < 24) return `${hours}h ago`
  if (days < 30) return `${days}d ago`

  return date.toLocaleDateString()
}

function refreshData() {
  fetchData()
}

onMounted(() => {
  fetchData()
})
</script>

<style>
.kanban-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
  background: var(--bg-panel);
  border-radius: 8px;
  overflow: hidden;
}

.kanban-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 2px solid #e5e7eb;
}

.kanban-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.kanban-controls {
  display: flex;
  gap: 12px;
}

.btn-refresh {
  padding: 8px 16px;
  background: #3b82f6;
  color: var(--bg-panel);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-refresh:hover:not(:disabled) {
  background: #2563eb;
}

.btn-refresh:disabled {
  background: #9ca3af;
  cursor: not-allowed;
}

.spinner {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: var(--bg-panel);
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
  margin-right: 6px;
  vertical-align: middle;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-message {
  padding: 12px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 6px;
  color: #dc2626;
  margin-bottom: 16px;
}

.loading,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #6b7280;
}

.empty-state .hint {
  font-size: 14px;
  margin-top: 8px;
  color: #9ca3af;
}

.kanban-board {
  display: flex;
  gap: 16px;
  flex: 1;
  overflow-x: auto;
  overflow-y: hidden;
  padding-bottom: 16px;
}

.kanban-column {
  flex: 0 0 280px;
  display: flex;
  flex-direction: column;
  background: var(--bg-panel);
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  max-height: 100%;
}

.column-header {
  padding: 12px 16px;
  border-radius: 8px 8px 0 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.column-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.column-count {
  background: rgba(0, 0, 0, 0.1);
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.column-cards {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  min-height: 100px;
}

.kanban-card {
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 12px;
  margin-bottom: 8px;
  cursor: move;
  transition: all 0.2s;
}

.kanban-card:hover {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.card-ghost {
  opacity: 0.5;
  background: #f3f4f6;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 8px;
}

.card-title {
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary);
  flex: 1;
}

.card-type {
  font-size: 11px;
  padding: 2px 6px;
  background: var(--bg-panel);
  border-radius: 4px;
  color: var(--text-primary);
}

.card-description {
  font-size: 13px;
  color: var(--text-primary);
  margin-bottom: 8px;
  line-height: 1.4;
}

.card-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid #f3f4f6;
}

.card-time {
  font-size: 12px;
  color: #9ca3af;
}

.card-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.card-tag {
  font-size: 10px;
  padding: 2px 6px;
  background: #dbeafe;
  color: #1e40af;
  border-radius: 3px;
}

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
  z-index: 3000;
}

.modal-content.card-details {
  background: var(--bg-panel);
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  overflow-y: auto;
  padding: 24px;
}

.modal-content h3 {
  margin: 0 0 20px 0;
  font-size: 20px;
}

.detail-section {
  margin-bottom: 16px;
}

.detail-section label {
  display: block;
  font-weight: 600;
  font-size: 14px;
  color: #374151;
  margin-bottom: 4px;
}

.detail-section span,
.detail-section p {
  color: #6b7280;
  font-size: 14px;
}

.raw-data {
  background: #f3f4f6;
  padding: 12px;
  border-radius: 6px;
  font-size: 12px;
  overflow-x: auto;
  max-height: 200px;
}

.btn-close-modal {
  width: 100%;
  padding: 10px;
  background: #3b82f6;
  color: var(--bg-panel);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  margin-top: 16px;
}

.btn-close-modal:hover {
  background: #2563eb;
}
</style>
