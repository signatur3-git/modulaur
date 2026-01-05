<template>
  <div class="offline-data-browser">
    <div class="browser-header">
      <h2>📦 Offline Data Browser</h2>
      <div class="header-actions">
        <span v-if="!isOnline" class="offline-badge">⚠️ Offline Mode</span>
        <button @click="refreshData" :disabled="loading" class="btn-refresh">
          {{ loading ? '⟳ Loading...' : '🔄 Refresh' }}
        </button>
      </div>
    </div>

    <!-- Stats Summary -->
    <div class="stats-summary">
      <div class="stat-card">
        <div class="stat-label">Total Records</div>
        <div class="stat-value">{{ totalRecords }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Data Sources</div>
        <div class="stat-value">{{ uniqueSources.length }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Record Types</div>
        <div class="stat-value">{{ uniqueTypes.length }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Oldest Data</div>
        <div class="stat-value">{{ oldestDataAge }}</div>
      </div>
    </div>

    <!-- Filters -->
    <div class="filters">
      <div class="filter-group">
        <label>Source:</label>
        <select v-model="selectedSource">
          <option value="">All Sources</option>
          <option v-for="source in uniqueSources" :key="source" :value="source">
            {{ source }}
          </option>
        </select>
      </div>

      <div class="filter-group">
        <label>Type:</label>
        <select v-model="selectedType">
          <option value="">All Types</option>
          <option v-for="type in uniqueTypes" :key="type" :value="type">
            {{ type }}
          </option>
        </select>
      </div>

      <div class="filter-group">
        <label>Search:</label>
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search in data..."
          class="search-input"
        />
      </div>

      <div class="filter-group">
        <label>Show:</label>
        <select v-model="pageSize">
          <option :value="25">25 records</option>
          <option :value="50">50 records</option>
          <option :value="100">100 records</option>
          <option :value="500">500 records</option>
        </select>
      </div>
    </div>

    <!-- Export Actions -->
    <div class="export-actions">
      <button
        @click="handleExportCSV"
        :disabled="filteredRecords.length === 0 || isExporting"
        class="btn-export"
      >
        📊 Export to CSV
      </button>
      <button
        @click="handleExportJSON"
        :disabled="filteredRecords.length === 0 || isExporting"
        class="btn-export"
      >
        📄 Export to JSON
      </button>
      <button
        @click="handleGenerateReport"
        :disabled="allRecords.length === 0 || isExporting"
        class="btn-export"
      >
        📋 Generate Report
      </button>
    </div>

    <!-- Data Table -->
    <div class="data-table-container">
      <table v-if="paginatedRecords.length > 0" class="data-table">
        <thead>
          <tr>
            <th @click="sortBy('source')">
              Source {{ sortColumn === 'source' ? (sortDirection === 'asc' ? '▲' : '▼') : '' }}
            </th>
            <th @click="sortBy('record_type')">
              Type {{ sortColumn === 'record_type' ? (sortDirection === 'asc' ? '▲' : '▼') : '' }}
            </th>
            <th @click="sortBy('timestamp')">
              Last Updated
              {{ sortColumn === 'timestamp' ? (sortDirection === 'asc' ? '▲' : '▼') : '' }}
            </th>
            <th>Freshness</th>
            <th>Data Preview</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="record in paginatedRecords" :key="record.id?.id || Math.random()">
            <td>{{ record.source }}</td>
            <td>{{ record.record_type }}</td>
            <td>
              <div class="timestamp-cell">
                <div>{{ formatTimestamp(record.timestamp) }}</div>
                <div class="age-text">{{ formatAge(record.timestamp) }}</div>
              </div>
            </td>
            <td>
              <span
                class="freshness-badge"
                :class="`freshness-${getFreshnessStatus(record.timestamp).color}`"
              >
                {{ getFreshnessStatus(record.timestamp).label }}
              </span>
            </td>
            <td class="data-preview">
              {{ getDataPreview(record.data) }}
            </td>
            <td>
              <button @click="viewDetails(record)" class="btn-view">👁️ View</button>
            </td>
          </tr>
        </tbody>
      </table>

      <div v-else-if="loading" class="empty-state">
        <p>⟳ Loading data...</p>
      </div>

      <div v-else class="empty-state">
        <p>📭 No data available</p>
        <p class="empty-hint">
          {{
            isOnline
              ? 'Fetch data from your configured sources to see it here.'
              : 'No cached data available. Connect to the internet and fetch data first.'
          }}
        </p>
      </div>
    </div>

    <!-- Pagination -->
    <div v-if="filteredRecords.length > pageSize" class="pagination">
      <button @click="currentPage--" :disabled="currentPage === 0" class="btn-page">
        ← Previous
      </button>
      <span class="page-info">
        Page {{ currentPage + 1 }} of {{ totalPages }} ({{ filteredRecords.length }} records)
      </span>
      <button @click="currentPage++" :disabled="currentPage >= totalPages - 1" class="btn-page">
        Next →
      </button>
    </div>

    <!-- Detail Modal -->
    <div v-if="selectedRecord" class="modal-overlay" @click="closeDetails">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Record Details</h3>
          <button @click="closeDetails" class="btn-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="detail-section">
            <h4>Metadata</h4>
            <table class="detail-table">
              <tr>
                <td><strong>Source:</strong></td>
                <td>{{ selectedRecord.source }}</td>
              </tr>
              <tr>
                <td><strong>Type:</strong></td>
                <td>{{ selectedRecord.record_type }}</td>
              </tr>
              <tr>
                <td><strong>Timestamp:</strong></td>
                <td>{{ formatTimestamp(selectedRecord.timestamp) }}</td>
              </tr>
              <tr>
                <td><strong>Age:</strong></td>
                <td>{{ formatAge(selectedRecord.timestamp) }}</td>
              </tr>
            </table>
          </div>
          <div class="detail-section">
            <h4>Data</h4>
            <pre class="json-preview">{{ JSON.stringify(selectedRecord.data, null, 2) }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useOnlineStatus } from '../composables/useOnlineStatus'
import { useDataExport } from '../composables/useDataExport'
import { useDataFreshness } from '../composables/useDataFreshness'

const { isOnline } = useOnlineStatus()
const { isExporting, exportToCSV, exportToJSON, generateOfflineReport } = useDataExport()
const { formatAge, getFreshnessStatus, formatTimestamp } = useDataFreshness()

// Data
const allRecords = ref<any[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// Filters
const selectedSource = ref('')
const selectedType = ref('')
const searchQuery = ref('')
const pageSize = ref(50)
const currentPage = ref(0)

// Sorting
const sortColumn = ref<'source' | 'record_type' | 'timestamp'>('timestamp')
const sortDirection = ref<'asc' | 'desc'>('desc')

// Detail view
const selectedRecord = ref<any | null>(null)

// Computed
const uniqueSources = computed(() => {
  const sources = new Set(allRecords.value.map(r => r.source))
  return Array.from(sources).sort()
})

const uniqueTypes = computed(() => {
  const types = new Set(allRecords.value.map(r => r.record_type))
  return Array.from(types).sort()
})

const totalRecords = computed(() => allRecords.value.length)

const oldestDataAge = computed(() => {
  if (allRecords.value.length === 0) return 'N/A'
  const oldest = allRecords.value.reduce(
    (oldest, r) => (!oldest || r.timestamp < oldest ? r.timestamp : oldest),
    null
  )
  return oldest ? formatAge(oldest) : 'N/A'
})

const filteredRecords = computed(() => {
  let filtered = allRecords.value

  // Filter by source
  if (selectedSource.value) {
    filtered = filtered.filter(r => r.source === selectedSource.value)
  }

  // Filter by type
  if (selectedType.value) {
    filtered = filtered.filter(r => r.record_type === selectedType.value)
  }

  // Filter by search query
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(r => {
      const dataStr = JSON.stringify(r.data).toLowerCase()
      return (
        dataStr.includes(query) ||
        r.source.toLowerCase().includes(query) ||
        r.record_type.toLowerCase().includes(query)
      )
    })
  }

  // Sort
  filtered.sort((a, b) => {
    let aVal = a[sortColumn.value]
    let bVal = b[sortColumn.value]

    if (sortColumn.value === 'timestamp') {
      aVal = new Date(aVal).getTime()
      bVal = new Date(bVal).getTime()
    }

    if (sortDirection.value === 'asc') {
      return aVal > bVal ? 1 : -1
    } else {
      return aVal < bVal ? 1 : -1
    }
  })

  return filtered
})

const totalPages = computed(() => Math.ceil(filteredRecords.value.length / pageSize.value))

const paginatedRecords = computed(() => {
  const start = currentPage.value * pageSize.value
  return filteredRecords.value.slice(start, start + pageSize.value)
})

// Methods
async function refreshData() {
  loading.value = true
  error.value = null

  try {
    allRecords.value = await invoke<any[]>('get_staged_records', {
      limit: 10000,
      offset: 0,
    })
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load data'
    console.error('Failed to load offline data:', e)
  } finally {
    loading.value = false
  }
}

function sortBy(column: typeof sortColumn.value) {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortColumn.value = column
    sortDirection.value = 'desc'
  }
}

function getDataPreview(data: any): string {
  if (!data) return 'N/A'
  const str = JSON.stringify(data)
  return str.length > 100 ? str.substring(0, 100) + '...' : str
}

function viewDetails(record: any) {
  selectedRecord.value = record
}

function closeDetails() {
  selectedRecord.value = null
}

async function handleExportCSV() {
  const filename = `offline-data-${new Date().toISOString().split('T')[0]}.csv`
  const result = exportToCSV(filteredRecords.value, filename)
  if (result.success) {
    alert(`Successfully exported ${result.recordCount} records to CSV`)
  } else {
    alert(`Export failed: ${result.error}`)
  }
}

async function handleExportJSON() {
  const filename = `offline-data-${new Date().toISOString().split('T')[0]}.json`
  const result = exportToJSON(filteredRecords.value, filename)
  if (result.success) {
    alert(`Successfully exported ${result.recordCount} records to JSON`)
  } else {
    alert(`Export failed: ${result.error}`)
  }
}

async function handleGenerateReport() {
  const result = await generateOfflineReport()
  if (result.success) {
    alert('Offline report generated successfully!')
  } else {
    alert(`Report generation failed: ${result.error}`)
  }
}

// Lifecycle
onMounted(() => {
  refreshData()
})
</script>

<style scoped>
.offline-data-browser {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.browser-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.browser-header h2 {
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 10px;
  align-items: center;
}

.offline-badge {
  padding: 4px 12px;
  background: #ff9800;
  color: white;
  border-radius: 4px;
  font-size: 14px;
}

.btn-refresh {
  padding: 8px 16px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-refresh:hover {
  background: #1976d2;
}

.btn-refresh:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.stats-summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  margin-bottom: 20px;
}

.stat-card {
  background: var(--bg-panel);
  padding: 15px;
  border-radius: 8px;
  box-shadow: var(--panel-shadow);
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 5px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: var(--text-heading);
}

.filters {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  margin-bottom: 20px;
  padding: 15px;
  background: var(--bg-panel);
  border-radius: 8px;
  box-shadow: var(--panel-shadow);
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.filter-group label {
  font-size: 12px;
  font-weight: bold;
  color: var(--text-secondary);
}

.filter-group select,
.search-input {
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
  background: var(--bg-panel);
  color: var(--text-primary);
}

.export-actions {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.btn-export {
  padding: 8px 16px;
  background: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-export:hover {
  background: #45a049;
}

.btn-export:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.data-table-container {
  background: var(--bg-panel);
  border-radius: 8px;
  box-shadow: var(--panel-shadow);
  overflow-x: auto;
  margin-bottom: 20px;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
}

.data-table th {
  background: var(--bg-panel-header);
  padding: 12px;
  text-align: left;
  font-weight: bold;
  cursor: pointer;
  user-select: none;
  color: var(--text-primary);
}

.data-table th:hover {
  background: var(--border-subtle);
}

.data-table td {
  padding: 12px;
  border-top: 1px solid var(--border-color);
}

.timestamp-cell {
  font-size: 12px;
}

.age-text {
  color: var(--text-secondary);
  font-size: 11px;
  margin-top: 2px;
}

.freshness-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: bold;
}

.freshness-green {
  background: #4caf50;
  color: white;
}

.freshness-blue {
  background: #2196f3;
  color: white;
}

.freshness-yellow {
  background: #ff9800;
  color: white;
}

.freshness-orange {
  background: #ff5722;
  color: white;
}

.freshness-red {
  background: #f44336;
  color: white;
}

.data-preview {
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: monospace;
  font-size: 12px;
}

.btn-view {
  padding: 4px 12px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.btn-view:hover {
  background: #1976d2;
}

.empty-state {
  padding: 40px;
  text-align: center;
  color: var(--text-secondary);
}

.empty-hint {
  font-size: 14px;
  margin-top: 10px;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 20px;
  padding: 20px;
}

.btn-page {
  padding: 8px 16px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-page:hover {
  background: #1976d2;
}

.btn-page:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.page-info {
  font-size: 14px;
  color: var(--text-secondary);
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-modal);
  border-radius: 8px;
  padding: 20px;
  max-width: 800px;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  color: var(--text-primary);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
}

.btn-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary);
}

.btn-close:hover {
  color: var(--text-primary);
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-section h4 {
  margin-top: 0;
  margin-bottom: 10px;
  color: var(--text-heading);
}

.detail-table {
  width: 100%;
  border-collapse: collapse;
}

.detail-table td {
  padding: 8px;
  border: 1px solid var(--border-color);
}

.json-preview {
  background: var(--bg-panel-header);
  padding: 15px;
  border-radius: 4px;
  overflow-x: auto;
  font-size: 12px;
  max-height: 400px;
}
</style>
