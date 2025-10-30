<template>
  <div class="table-panel">
    <div class="table-header">
      <h3>{{ panel.title || 'Data Table' }}</h3>
      <div class="controls">
        <select v-model="selectedSource" @change="filterBySource" class="filter-select">
          <option value="">All Sources</option>
          <option v-for="source in dataSources" :key="source" :value="source">
            {{ source }}
          </option>
        </select>
        <select v-model="selectedType" @change="filterByType" class="filter-select">
          <option value="">All Types</option>
          <option v-for="type in recordTypes" :key="type" :value="type">
            {{ type }}
          </option>
        </select>
        <button @click="refreshData" class="btn-refresh" :disabled="loading">
          <span v-if="loading" class="spinner"></span>
          {{ loading ? 'Loading...' : 'Refresh' }}
        </button>
      </div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-if="loading && records.length === 0" class="loading">Loading data...</div>

    <div v-else-if="records.length === 0" class="empty-state">
      <p>No data available</p>
      <p class="hint">Configure an adapter and fetch some data to see it here!</p>
    </div>

    <div v-else class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th @click="sortBy('record_type')" class="sortable">
              Type
              <span class="sort-indicator" v-if="sortColumn === 'record_type'">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th @click="sortBy('source')" class="sortable">
              Source
              <span class="sort-indicator" v-if="sortColumn === 'source'">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th>Title</th>
            <th>Status</th>
            <th>Tags</th>
            <th @click="sortBy('timestamp')" class="sortable">
              Timestamp
              <span class="sort-indicator" v-if="sortColumn === 'timestamp'">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="record in paginatedRecords" :key="record.id">
            <td>
              <span class="badge" :class="'badge-' + record.record_type">
                {{ record.record_type }}
              </span>
            </td>
            <td>{{ record.source }}</td>
            <td>{{ record.metadata?.title || '-' }}</td>
            <td>
              <span
                v-if="record.metadata?.status"
                class="status-badge"
                :class="'status-' + record.metadata.status"
              >
                {{ record.metadata.status }}
              </span>
              <span v-else>-</span>
            </td>
            <td>
              <span v-for="tag in record.metadata?.tags || []" :key="tag" class="tag">
                {{ tag }}
              </span>
              <span v-if="!record.metadata?.tags || record.metadata.tags.length === 0">-</span>
            </td>
            <td>{{ formatTimestamp(record.timestamp) }}</td>
            <td>
              <button @click="viewDetails(record)" class="btn-small">View</button>
            </td>
          </tr>
        </tbody>
      </table>

      <div class="pagination">
        <div class="pagination-info">
          Showing {{ startIndex + 1 }}-{{ endIndex }} of {{ sortedRecords.length }} records
        </div>
        <div class="pagination-controls">
          <button @click="previousPage" :disabled="currentPage === 1" class="btn-page">
            Previous
          </button>
          <span class="page-info">Page {{ currentPage }} of {{ totalPages }}</span>
          <button @click="nextPage" :disabled="currentPage === totalPages" class="btn-page">
            Next
          </button>
        </div>
      </div>
    </div>

    <!-- Detail Modal -->
    <div v-if="selectedRecord" class="modal-overlay" @click="closeDetails">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Record Details</h3>
          <button @click="closeDetails" class="btn-close">×</button>
        </div>
        <div class="modal-body">
          <div class="detail-section">
            <h4>Metadata</h4>
            <dl>
              <dt>Type:</dt>
              <dd>{{ selectedRecord.record_type }}</dd>
              <dt>Source:</dt>
              <dd>{{ selectedRecord.source }}</dd>
              <dt>Timestamp:</dt>
              <dd>{{ formatTimestamp(selectedRecord.timestamp) }}</dd>
              <dt>Status:</dt>
              <dd>{{ selectedRecord.metadata?.status || 'N/A' }}</dd>
              <dt>Tags:</dt>
              <dd>{{ (selectedRecord.metadata?.tags || []).join(', ') || 'N/A' }}</dd>
            </dl>
          </div>
          <div class="detail-section">
            <h4>Raw Data</h4>
            <pre class="json-data">{{ JSON.stringify(selectedRecord.data, null, 2) }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import type { Panel } from '../../stores/dashboardStore'

// Props - accept panel like other components
const props = defineProps<{
  panel: Panel
}>()

const emit = defineEmits<{
  'update:panel': [panel: Panel]
}>()

// Extract config from panel
const config = computed(() => props.panel.config || {})

// State
const records = ref<any[]>([])
const loading = ref(false)
const error = ref('')
const selectedRecord = ref<any>(null)
const selectedSource = ref(config.value.dataSource || '')
const selectedType = ref(config.value.recordType || '')
const sortColumn = ref('timestamp')
const sortDirection = ref<'asc' | 'desc'>('desc')
const currentPage = ref(1)
const pageSize = ref(config.value.pageSize || 20)

// Check if we're in Tauri
const isTauri = computed(() => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
})

// Computed
const dataSources = computed(() => {
  const sources = new Set(records.value.map(r => r.source))
  return Array.from(sources).sort()
})

const recordTypes = computed(() => {
  const types = new Set(records.value.map(r => r.record_type))
  return Array.from(types).sort()
})

const sortedRecords = computed(() => {
  let filtered = records.value

  // Filter by source if selected
  if (selectedSource.value) {
    filtered = filtered.filter(r => r.source === selectedSource.value)
  }

  // Filter by type if selected
  if (selectedType.value) {
    filtered = filtered.filter(r => r.record_type === selectedType.value)
  }

  // Sort
  return filtered.sort((a, b) => {
    let aVal = a[sortColumn.value]
    let bVal = b[sortColumn.value]

    // Handle nested properties
    if (sortColumn.value === 'title') {
      aVal = a.metadata?.title || ''
      bVal = b.metadata?.title || ''
    }

    // Compare
    if (aVal < bVal) return sortDirection.value === 'asc' ? -1 : 1
    if (aVal > bVal) return sortDirection.value === 'asc' ? 1 : -1
    return 0
  })
})

const totalPages = computed(() => {
  return Math.ceil(sortedRecords.value.length / pageSize.value)
})

const startIndex = computed(() => {
  return (currentPage.value - 1) * pageSize.value
})

const endIndex = computed(() => {
  return Math.min(startIndex.value + pageSize.value, sortedRecords.value.length)
})

const paginatedRecords = computed(() => {
  return sortedRecords.value.slice(startIndex.value, endIndex.value)
})

// Methods
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

    if (selectedType.value) {
      records.value = await invoke('get_records_by_type', {
        recordType: selectedType.value,
      })
    } else {
      records.value = await invoke('get_staged_records', {
        limit: 1000,
        offset: 0,
      })
    }
  } catch (e: any) {
    error.value = `Failed to fetch data: ${e.message || e}`
    console.error('Fetch error:', e)
  } finally {
    loading.value = false
  }
}

function generateMockData() {
  // Mock data for browser mode
  return Array.from({ length: 10 }, (_, i) => ({
    id: `mock-${i}`,
    record_type: i % 3 === 0 ? 'rest_api' : i % 3 === 1 ? 'gitlab_pipeline' : 'gitlab_job',
    source: `mock-source-${Math.floor(i / 3)}`,
    timestamp: new Date(Date.now() - i * 3600000).toISOString(),
    data: { id: i, name: `Mock Item ${i}`, value: Math.random() * 100 },
    metadata: {
      title: `Mock Record ${i}`,
      status: i % 2 === 0 ? 'success' : 'pending',
      tags: [`tag${i % 3}`, `category${i % 2}`],
      description: `This is mock record number ${i}`,
    },
  }))
}

function refreshData() {
  fetchData()
}

function filterBySource() {
  currentPage.value = 1
  // Save selection to panel config for persistence
  emit('update:panel', {
    ...props.panel,
    config: { ...props.panel.config, dataSource: selectedSource.value },
  })
}

function filterByType() {
  currentPage.value = 1
  // Save selection to panel config for persistence
  emit('update:panel', {
    ...props.panel,
    config: { ...props.panel.config, recordType: selectedType.value },
  })
}

function sortBy(column: string) {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortColumn.value = column
    sortDirection.value = 'asc'
  }
}

function viewDetails(record: any) {
  selectedRecord.value = record
}

function closeDetails() {
  selectedRecord.value = null
}

function formatTimestamp(timestamp: string) {
  try {
    const date = new Date(timestamp)
    return date.toLocaleString()
  } catch {
    return timestamp
  }
}

function previousPage() {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

function nextPage() {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
  }
}

// Lifecycle
onMounted(() => {
  fetchData()
})

// Watch for config changes
watch(
  () => props.panel.config?.recordType,
  newType => {
    selectedType.value = newType || ''
    filterByType()
  }
)
</script>

<style scoped>
.table-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: var(--panel-padding);
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  box-shadow: var(--panel-shadow);
}

.table-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-lg);
  padding-bottom: var(--space-md);
  border-bottom: 2px solid var(--border-color);
}

.table-header h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text-heading);
}

.controls {
  display: flex;
  gap: var(--space-md);
  align-items: center;
}

.btn-refresh {
  padding: calc(var(--space-xs) * 2) var(--space-lg);
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border: none;
  border-radius: calc(var(--panel-radius) / 2);
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-refresh:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-refresh:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
}

.spinner {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: var(--text-on-accent);
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
  margin-right: calc(var(--space-xs) * 1.5);
  vertical-align: middle;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.filter-select {
  padding: calc(var(--space-xs) * 2) var(--space-md);
  border: 1px solid var(--border-subtle);
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.875rem;
  background: var(--bg-panel);
  color: var(--text-primary);
  cursor: pointer;
}

.error-message {
  padding: var(--space-md);
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: calc(var(--panel-radius) / 2);
  color: var(--accent-danger);
  margin-bottom: var(--space-lg);
}

.loading,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
}

.empty-state .hint {
  font-size: 0.875rem;
  margin-top: var(--space-sm);
  color: var(--text-muted);
}

.table-container {
  flex: 1;
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
}

.data-table thead {
  position: sticky;
  top: 0;
  background: var(--bg-panel-header);
  z-index: 10;
}

.data-table th {
  text-align: left;
  padding: var(--space-md);
  font-weight: 600;
  color: var(--text-heading);
  border-bottom: 2px solid var(--border-color);
  white-space: nowrap;
}

.data-table th.sortable {
  cursor: pointer;
  user-select: none;
}

.data-table th.sortable:hover {
  background: var(--bg-panel);
}

.sort-indicator {
  margin-left: calc(var(--space-xs));
  color: var(--accent-primary);
}

.data-table td {
  padding: var(--space-md);
  border-bottom: 1px solid var(--border-subtle);
}

.data-table tbody tr:hover {
  background: var(--bg-panel-header);
}

.badge {
  display: inline-block;
  padding: calc(var(--space-xs)) var(--space-sm);
  border-radius: calc(var(--panel-radius) / 4);
  font-size: 0.75rem;
  font-weight: 500;
  white-space: nowrap;
}

.badge-rest_api {
  background: #dbeafe;
  color: #1e40af;
}

.badge-gitlab_pipeline {
  background: #fef3c7;
  color: #92400e;
}

.badge-gitlab_job {
  background: #e0e7ff;
  color: #3730a3;
}

.status-badge {
  display: inline-block;
  padding: calc(var(--space-xs)) var(--space-sm);
  border-radius: calc(var(--panel-radius) / 4);
  font-size: 0.75rem;
  font-weight: 500;
}

.status-success {
  background: #d1fae5;
  color: #065f46;
}

.status-failed,
.status-failure {
  background: #fee2e2;
  color: #991b1b;
}

.status-pending,
.status-running {
  background: #fef3c7;
  color: #92400e;
}

.tag {
  display: inline-block;
  padding: calc(var(--space-xs) * 0.5) calc(var(--space-sm) * 1.5);
  margin-right: calc(var(--space-xs));
  background: var(--bg-panel-header);
  border-radius: 3px;
  font-size: 0.6875rem;
  color: var(--text-secondary);
}

.btn-small {
  padding: calc(var(--space-xs)) var(--space-md);
  background: var(--bg-panel-header);
  border: 1px solid var(--border-subtle);
  border-radius: calc(var(--panel-radius) / 4);
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s;
  color: var(--text-primary);
}

.btn-small:hover {
  background: var(--bg-panel);
  border-color: var(--border-color);
}

.pagination {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--space-lg);
  padding-top: var(--space-lg);
  border-top: 1px solid var(--border-color);
}

.pagination-info {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.page-info {
  font-size: 0.875rem;
  color: var(--text-primary);
}

.btn-page {
  padding: calc(var(--space-xs) * 1.5) var(--space-md);
  background: var(--bg-panel);
  border: 1px solid var(--border-subtle);
  border-radius: calc(var(--panel-radius) / 2);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
  color: var(--text-primary);
}

.btn-page:hover:not(:disabled) {
  background: var(--bg-panel-header);
  border-color: var(--border-color);
}

.btn-page:disabled {
  color: var(--text-muted);
  cursor: not-allowed;
  opacity: 0.5;
}

/* Modal */
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

.modal-content {
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  width: 90%;
  max-width: 800px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: var(--panel-shadow);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-xl);
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text-heading);
}

.btn-close {
  background: none;
  border: none;
  font-size: 28px;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--panel-radius) / 4);
}

.btn-close:hover {
  background: var(--bg-panel-header);
}

.modal-body {
  overflow-y: auto;
  padding: var(--space-xl);
}

.detail-section {
  margin-bottom: var(--space-xl);
}

.detail-section h4 {
  margin: 0 0 var(--space-md) 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-heading);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.detail-section dl {
  display: grid;
  grid-template-columns: 120px 1fr;
  gap: var(--space-sm);
  margin: 0;
}

.detail-section dt {
  font-weight: 500;
  color: var(--text-secondary);
}

.detail-section dd {
  margin: 0;
  color: var(--text-primary);
}

.json-data {
  background: var(--bg-panel-header);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  padding: var(--space-lg);
  overflow-x: auto;
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  line-height: 1.5;
  margin: 0;
  color: var(--text-primary);
}
</style>
