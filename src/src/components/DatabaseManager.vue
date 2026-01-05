<template>
  <div class="database-manager">
    <h2>Database Management</h2>

    <!-- Current Environment Info -->
    <div class="env-info">
      <div class="info-card">
        <h3>Current Environment</h3>
        <div class="env-badge" :class="currentEnv">
          {{ currentEnv === 'dev' ? 'Development Database' : 'Production Database' }}
        </div>
        <p class="env-description">
          {{
            currentEnv === 'dev'
              ? 'Using data/dev/db - Safe for testing and development'
              : 'Using data/prod/db - Production data'
          }}
        </p>
      </div>
    </div>

    <!-- Database Statistics -->
    <div class="stats-section">
      <h3>Database Statistics</h3>
      <button @click="loadStats" :disabled="isLoading">
        {{ isLoading ? 'Loading...' : 'Refresh Stats' }}
      </button>

      <div v-if="stats" class="stats-grid">
        <div class="stat-card">
          <div class="stat-value">{{ stats.total_records }}</div>
          <div class="stat-label">Total Records</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{{ formatBytes(stats.size_bytes) }}</div>
          <div class="stat-label">Estimated Size</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{{ Object.keys(stats.by_type).length }}</div>
          <div class="stat-label">Record Types</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{{ Object.keys(stats.by_source).length }}</div>
          <div class="stat-label">Data Sources</div>
        </div>
      </div>

      <!-- Breakdown by Type -->
      <div v-if="stats && Object.keys(stats.by_type).length > 0" class="breakdown">
        <h4>Records by Type</h4>
        <div class="breakdown-list">
          <div v-for="(count, type) in stats.by_type" :key="type" class="breakdown-item">
            <span class="breakdown-label">{{ type }}</span>
            <span class="breakdown-value">{{ count }}</span>
          </div>
        </div>
      </div>

      <!-- Breakdown by Source -->
      <div v-if="stats && Object.keys(stats.by_source).length > 0" class="breakdown">
        <h4>Records by Source</h4>
        <div class="breakdown-list">
          <div v-for="(count, source) in stats.by_source" :key="source" class="breakdown-item">
            <span class="breakdown-label">{{ source }}</span>
            <span class="breakdown-value">{{ count }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Export/Import Section -->
    <div class="export-import-section">
      <h3>Data Backup & Restore</h3>
      <p class="section-description">
        Export all your data (pages, records, dashboards, settings) to a JSON file for backup or
        transfer between environments.
      </p>

      <div class="action-buttons">
        <!-- Export -->
        <div class="action-card">
          <h4>Export Database</h4>
          <p>Download all data as JSON file</p>
          <button @click="handleExport" :disabled="isLoading" class="btn-primary">
            📥 Export to File
          </button>
        </div>

        <!-- Import -->
        <div class="action-card">
          <h4>Import Database</h4>
          <p>Load data from JSON export file</p>
          <div class="import-controls">
            <select v-model="mergeStrategy" class="merge-strategy">
              <option value="merge">Merge (keep existing)</option>
              <option value="replace">Replace (clear first)</option>
              <option value="skip">Skip conflicts</option>
            </select>
            <input
              ref="fileInput"
              type="file"
              accept=".json"
              @change="handleFileSelect"
              style="display: none"
            />
            <button @click="triggerFileSelect" :disabled="isLoading" class="btn-primary">
              📤 Import from File
            </button>
          </div>
        </div>
      </div>

      <!-- Import Results -->
      <div v-if="importStats" class="import-results">
        <h4>Import Results</h4>
        <div class="results-grid">
          <div class="result-item">
            <span class="result-label">Records:</span>
            <span class="result-value">{{ importStats.records_imported }}</span>
          </div>
          <div class="result-item">
            <span class="result-label">Pages:</span>
            <span class="result-value">{{ importStats.pages_imported }}</span>
          </div>
          <div class="result-item">
            <span class="result-label">Data Sources:</span>
            <span class="result-value">{{ importStats.data_sources_imported }}</span>
          </div>
          <div class="result-item">
            <span class="result-label">Settings:</span>
            <span class="result-value">{{ importStats.settings_imported }}</span>
          </div>
          <div class="result-item">
            <span class="result-label">Plugin Data:</span>
            <span class="result-value">{{ importStats.plugin_data_imported }}</span>
          </div>
          <div class="result-item">
            <span class="result-label">Tickets:</span>
            <span class="result-value">{{ importStats.tickets_imported }}</span>
          </div>
          <div class="result-item">
            <span class="result-label">Dashboards:</span>
            <span class="result-value">{{ importStats.dashboards_imported }}</span>
          </div>
        </div>
        <div v-if="importStats.errors.length > 0" class="errors">
          <h5>Errors ({{ importStats.errors.length }})</h5>
          <ul>
            <li v-for="(error, idx) in importStats.errors.slice(0, 5)" :key="idx">
              {{ error }}
            </li>
            <li v-if="importStats.errors.length > 5">
              ... and {{ importStats.errors.length - 5 }} more errors
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- LocalStorage Panel Data Section -->
    <div class="export-import-section">
      <h3>Plugin Panel Data (LocalStorage)</h3>
      <p class="section-description">
        Some plugins store their content in browser LocalStorage (e.g., Markdown notes, RSS feeds).
        Use these tools to backup and restore this data separately from the database.
      </p>

      <div class="action-buttons">
        <!-- Export LocalStorage -->
        <div class="action-card">
          <h4>Export Panel Data</h4>
          <p>Download plugin panel content (notes, feeds, etc.)</p>
          <button @click="handleLocalStorageExport" class="btn-primary">
            📥 Export Panel Data
          </button>
        </div>

        <!-- Import LocalStorage -->
        <div class="action-card">
          <h4>Import Panel Data</h4>
          <p>Restore plugin panel content from backup</p>
          <div class="import-controls">
            <select v-model="localStorageMergeStrategy" class="merge-strategy">
              <option value="merge">Keep existing</option>
              <option value="replace">Overwrite all</option>
            </select>
            <input
              ref="localStorageFileInput"
              type="file"
              accept=".json"
              @change="handleLocalStorageFileSelect"
              style="display: none"
            />
            <button @click="triggerLocalStorageFileSelect" class="btn-primary">
              📤 Import Panel Data
            </button>
          </div>
        </div>
      </div>

      <!-- LocalStorage Import Results -->
      <div v-if="localStorageImportCount !== null" class="import-results">
        <h4>Panel Data Import Results</h4>
        <p>Imported {{ localStorageImportCount }} panel data items successfully!</p>
      </div>
    </div>

    <!-- Maintenance Section -->
    <div class="maintenance-section">
      <h3>Maintenance</h3>

      <div class="action-buttons">
        <!-- Cleanup Old Records -->
        <div class="action-card">
          <h4>Cleanup Old Records</h4>
          <p>Delete records older than specified days</p>
          <div class="cleanup-controls">
            <input
              v-model.number="ttlDays"
              type="number"
              min="1"
              placeholder="Days"
              class="ttl-input"
            />
            <button @click="handleCleanup" :disabled="isLoading || !ttlDays" class="btn-warning">
              🧹 Cleanup
            </button>
          </div>
        </div>

        <!-- Clear All -->
        <div class="action-card danger">
          <h4>Clear All Records</h4>
          <p>⚠️ Delete ALL records (irreversible!)</p>
          <button @click="handleClearAll" :disabled="isLoading" class="btn-danger">
            🗑️ Clear All
          </button>
        </div>
      </div>
    </div>

    <!-- Error Display -->
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useDatabaseStore } from '../stores/databaseStore'
import type { ImportStats } from '../stores/databaseStore'

const databaseStore = useDatabaseStore()

const fileInput = ref<HTMLInputElement | null>(null)
const localStorageFileInput = ref<HTMLInputElement | null>(null)
const mergeStrategy = ref<'merge' | 'replace' | 'skip'>('merge')
const localStorageMergeStrategy = ref<'merge' | 'replace'>('merge')
const ttlDays = ref<number>(30)
const importStats = ref<ImportStats | null>(null)
const localStorageImportCount = ref<number | null>(null)

const stats = computed(() => databaseStore.stats)
const isLoading = computed(() => databaseStore.isLoading)
const error = computed(() => databaseStore.error)

// Determine current environment
const currentEnv = computed(() => {
  return import.meta.env.MODE === 'production' ? 'prod' : 'dev'
})

onMounted(() => {
  loadStats()
})

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
}

async function loadStats() {
  try {
    await databaseStore.getStats()
  } catch (e) {
    console.error('Failed to load stats:', e)
  }
}

async function handleExport() {
  if (!confirm('Export all database data to a JSON file?')) return

  try {
    await databaseStore.downloadExport()
    alert('Database exported successfully!')
  } catch (e) {
    alert(`Export failed: ${e}`)
  }
}

function triggerFileSelect() {
  fileInput.value?.click()
}

async function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  const confirmMsg =
    mergeStrategy.value === 'replace'
      ? '⚠️ WARNING: This will DELETE all existing data first!\n\nAre you absolutely sure?'
      : 'Import data with merge strategy? Existing data will be kept.'

  if (!confirm(confirmMsg)) {
    target.value = '' // Reset file input
    return
  }

  try {
    const text = await file.text()
    const data = JSON.parse(text)

    const result = await databaseStore.importDatabase(data, mergeStrategy.value)
    importStats.value = result

    const summary = [
      `${result.records_imported} records`,
      `${result.pages_imported} pages`,
      `${result.data_sources_imported} data sources`,
      `${result.dashboards_imported} dashboards`,
    ].join(', ')

    alert(`Import complete!\n${summary}`)
    await loadStats()
  } catch (e) {
    alert(`Import failed: ${e}`)
  } finally {
    target.value = '' // Reset file input
  }
}

async function handleCleanup() {
  if (!confirm(`Delete records older than ${ttlDays.value} days?`)) return

  try {
    const deleted = await databaseStore.cleanupOldRecords(ttlDays.value)
    alert(`Cleaned up ${deleted} old records`)
    await loadStats()
  } catch (e) {
    alert(`Cleanup failed: ${e}`)
  }
}

async function handleClearAll() {
  const confirm1 = confirm('⚠️ DELETE ALL RECORDS?\n\nThis cannot be undone!')
  if (!confirm1) return

  const confirm2 = confirm('Are you REALLY sure? Type YES in your mind and click OK')
  if (!confirm2) return

  try {
    const count = await databaseStore.clearAllRecords()
    alert(`Cleared ${count} records`)
    importStats.value = null
    await loadStats()
  } catch (e) {
    alert(`Clear failed: ${e}`)
  }
}

function handleLocalStorageExport() {
  if (!confirm('Export plugin panel data from LocalStorage?')) return

  try {
    databaseStore.downloadLocalStorageExport()
    alert('Panel data exported successfully!')
  } catch (e) {
    alert(`Export failed: ${e}`)
  }
}

function triggerLocalStorageFileSelect() {
  localStorageFileInput.value?.click()
}

async function handleLocalStorageFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  const confirmMsg =
    localStorageMergeStrategy.value === 'replace'
      ? '⚠️ This will OVERWRITE existing plugin panel data!\n\nContinue?'
      : 'Import panel data? Existing data will be kept.'

  if (!confirm(confirmMsg)) {
    target.value = '' // Reset file input
    return
  }

  try {
    const text = await file.text()
    const data = JSON.parse(text)

    const count = databaseStore.importLocalStoragePanelData(data, localStorageMergeStrategy.value)
    localStorageImportCount.value = count

    alert(`Panel data import complete!\nImported ${count} items.`)
  } catch (e) {
    alert(`Import failed: ${e}`)
  } finally {
    target.value = '' // Reset file input
  }
}
</script>

<style scoped>
.database-manager {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
}

h2 {
  margin-bottom: 20px;
  color: var(--text-primary);
}

h3 {
  margin-top: 30px;
  margin-bottom: 15px;
  color: var(--text-secondary);
}

.env-info {
  margin-bottom: 30px;
}

.info-card {
  background: var(--bg-panel);
  border-radius: 8px;
  padding: 20px;
  border: 2px solid var(--border-color);
}

.env-badge {
  display: inline-block;
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 600;
  margin: 10px 0;
}

.env-badge.dev {
  background: #d1ecf1;
  color: #0c5460;
  border: 1px solid #bee5eb;
}

.env-badge.prod {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.env-description {
  color: var(--text-muted);
  font-size: 0.9em;
  margin-top: 10px;
}

.stats-section button {
  margin-bottom: 15px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  margin-bottom: 20px;
}

.stat-card {
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  text-align: center;
}

.stat-value {
  font-size: 2em;
  font-weight: bold;
  color: var(--accent-primary);
}

.stat-label {
  color: var(--text-muted);
  font-size: 0.9em;
  margin-top: 5px;
}

.breakdown {
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 15px;
  margin-top: 15px;
}

.breakdown h4 {
  margin-top: 0;
  margin-bottom: 10px;
  color: var(--text-secondary);
}

.breakdown-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.breakdown-item {
  display: flex;
  justify-content: space-between;
  padding: 8px;
  background: var(--bg-panel-header);
  border-radius: 4px;
}

.breakdown-label {
  color: var(--text-secondary);
}

.breakdown-value {
  font-weight: 600;
  color: var(--accent-primary);
}

.section-description {
  color: var(--text-muted);
  font-size: 0.9em;
  margin-bottom: 15px;
}

.action-buttons {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
  margin-top: 15px;
}

.action-card {
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  color: var(--text-primary);
}

.action-card.danger {
  border-color: color-mix(in srgb, var(--accent-danger) 55%, var(--border-color));
  background: color-mix(in srgb, var(--accent-danger) 10%, var(--bg-panel));
}

.action-card h4 {
  margin-top: 0;
  margin-bottom: 10px;
  color: var(--text-heading);
}

.action-card.danger h4 {
  color: var(--accent-danger);
}

.action-card p {
  color: var(--text-secondary);
  font-size: 0.9em;
  margin-bottom: 15px;
}

.import-controls,
.cleanup-controls {
  display: flex;
  gap: 10px;
  align-items: center;
}

.merge-strategy {
  flex: 1;
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-panel);
  color: var(--text-primary);
}

.ttl-input {
  width: 100px;
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-panel);
  color: var(--text-primary);
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--bg-button);
  color: var(--text-on-accent);
}

.btn-primary:hover:not(:disabled) {
  background: var(--bg-button-hover);
}

.btn-warning {
  background: #ffc107;
  color: #000;
}

.btn-warning:hover:not(:disabled) {
  background: #e0a800;
}

.btn-danger {
  background: #dc3545;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #c82333;
}

.import-results {
  margin-top: 20px;
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
}

.results-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 10px;
  margin-bottom: 15px;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: 8px;
  background: var(--bg-panel-header);
  border-radius: 4px;
}

.result-value {
  font-weight: 600;
  color: var(--accent-success);
}

.errors {
  background: #fff3cd;
  border: 1px solid #ffc107;
  border-radius: 4px;
  padding: 10px;
  margin-top: 10px;
}

.errors h5 {
  margin-top: 0;
  color: #856404;
}

.errors ul {
  margin: 10px 0 0 0;
  padding-left: 20px;
}

.errors li {
  color: #856404;
  font-size: 0.9em;
}

.error-message {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
  border-radius: 4px;
  padding: 15px;
  margin-top: 20px;
}
</style>
