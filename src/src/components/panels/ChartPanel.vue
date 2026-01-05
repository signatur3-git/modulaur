<template>
  <div class="chart-panel">
    <div class="chart-header">
      <h3>{{ panel.title || 'Chart' }}</h3>
      <div class="chart-controls">
        <label class="multi-source-toggle">
          <input type="checkbox" v-model="multiSourceMode" />
          <span>Multi-Source</span>
        </label>
        <select v-if="multiSourceMode" v-model="recordTypeFilter" class="chart-control-selector">
          <option value="">All Types</option>
          <option value="gitlab_pipeline">Pipelines</option>
          <option value="gitlab_job">Jobs</option>
          <option value="ticket">Tickets</option>
          <option value="rest_api">REST API</option>
        </select>
        <select v-model="chartType" class="chart-type-selector">
          <option value="bar">Bar</option>
          <option value="line">Line</option>
          <option value="pie">Pie</option>
          <option value="doughnut">Doughnut</option>
        </select>
        <select v-model="groupBy" class="chart-control-selector">
          <option value="status">By Status</option>
          <option value="source">By Source</option>
          <option value="type">By Type</option>
          <option value="time">Over Time</option>
        </select>
        <select v-if="groupBy === 'time'" v-model="timeBucket" class="chart-control-selector">
          <option value="hour">Hourly</option>
          <option value="day">Daily</option>
          <option value="week">Weekly</option>
          <option value="month">Monthly</option>
        </select>
        <select v-if="groupBy === 'time'" v-model="timeFieldPath" class="chart-control-selector">
          <option value="timestamp">Fetch Time</option>
          <option value="data.created_at">Created At</option>
          <option value="data.updated_at">Updated At</option>
          <option value="data.started_at">Started At</option>
          <option value="data.finished_at">Finished At</option>
        </select>
        <label v-if="groupBy === 'time'" class="stack-status-toggle">
          <input type="checkbox" v-model="stackByStatus" />
          <span>Stack by Status</span>
        </label>
        <select v-model="dataTransform" class="chart-control-selector">
          <option value="count">Count</option>
          <option value="sum">Sum</option>
          <option value="avg">Average</option>
        </select>
        <button @click="refreshData" class="btn-refresh" :disabled="loading">
          <span v-if="loading" class="spinner"></span>
          {{ loading ? 'Loading...' : 'Refresh' }}
        </button>
        <button
          v-if="multiSourceMode"
          @click="showSourceSelector = !showSourceSelector"
          class="btn-sources"
        >
          Sources ({{ selectedSources.length }})
        </button>
      </div>
    </div>

    <!-- Source Selector Modal -->
    <div v-if="multiSourceMode && showSourceSelector" class="source-selector-modal">
      <div class="source-selector-header">
        <h4>Select Data Sources</h4>
        <button @click="selectAllSources" class="btn-select-all">Select All</button>
        <button @click="clearAllSources" class="btn-clear-all">Clear All</button>
        <button @click="showSourceSelector = false" class="btn-close">×</button>
      </div>
      <div class="source-list">
        <label v-for="source in availableSources" :key="source.id" class="source-item">
          <input type="checkbox" :value="source.source" v-model="selectedSources" />
          <span class="source-name">{{ source.name }}</span>
          <span class="source-type">{{ source.adapter_type }}</span>
        </label>
        <div v-if="availableSources.length === 0" class="empty-sources">
          No data sources configured. Add data sources first.
        </div>
      </div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-if="loading && !chartData" class="loading">Loading chart data...</div>

    <div
      v-else-if="!chartData || !chartData.labels || chartData.labels.length === 0"
      class="empty-state"
    >
      <p>No data available for chart</p>
      <p class="hint">Configure a data source and fetch some data to visualize!</p>
    </div>

    <div v-else class="chart-container">
      <Line v-if="chartType === 'line'" :data="chartData" :options="chartOptions" />
      <Bar v-if="chartType === 'bar'" :data="chartData" :options="chartOptions" />
      <Pie v-if="chartType === 'pie'" :data="chartData" :options="chartOptions" />
      <Doughnut v-if="chartType === 'doughnut'" :data="chartData" :options="chartOptions" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { Line, Bar, Pie, Doughnut } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  ArcElement,
  Title,
  Tooltip,
  Legend,
  Filler,
  ChartData,
  ChartOptions,
} from 'chart.js'
import { useDataSourceStore } from '../../stores/dataSourceStore'
import { useDashboardStore } from '../../stores/dashboardStore'

// Register Chart.js components
ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  ArcElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

interface Panel {
  i: string
  id: string
  title?: string
  config?: {
    chartType?: 'line' | 'bar' | 'pie' | 'doughnut'
    recordType?: string
    dataSource?: string
    multiSourceMode?: boolean
    selectedSources?: string[]
    recordTypeFilter?: string
    timeFieldPath?: string
    groupBy?: string
    timeBucket?: string
    dataTransform?: string
    stackByStatus?: boolean
  }
}

const props = defineProps<{
  panel: Panel
}>()

const dataSourceStore = useDataSourceStore()
const dashboardStore = useDashboardStore()

// Check if we're in Tauri
const isTauri = computed(() => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
})

const chartType = ref(props.panel.config?.chartType || 'bar')
const loading = ref(false)
const error = ref('')
const records = ref<any[]>([])
const groupBy = ref(props.panel.config?.groupBy || 'status')
const timeBucket = ref(props.panel.config?.timeBucket || 'day')
const dataTransform = ref(props.panel.config?.dataTransform || 'count')
const timeFieldPath = ref(props.panel.config?.timeFieldPath || 'timestamp')
const stackByStatus = ref(props.panel.config?.stackByStatus || false)

// Multi-source state
const multiSourceMode = ref(props.panel.config?.multiSourceMode || false)
const selectedSources = ref<string[]>(props.panel.config?.selectedSources || [])
const recordTypeFilter = ref<string>(props.panel.config?.recordTypeFilter || '')
const showSourceSelector = ref(false)
const availableSources = computed(() => dataSourceStore.getEnabledDataSources())

// Save panel config whenever settings change
watch(
  [
    chartType,
    groupBy,
    timeBucket,
    dataTransform,
    timeFieldPath,
    multiSourceMode,
    selectedSources,
    recordTypeFilter,
    stackByStatus,
  ],
  () => {
    if (dashboardStore.currentDashboard) {
      dashboardStore.updatePanel(props.panel.i, {
        config: {
          ...props.panel.config,
          chartType: chartType.value,
          groupBy: groupBy.value,
          timeBucket: timeBucket.value,
          dataTransform: dataTransform.value,
          timeFieldPath: timeFieldPath.value,
          multiSourceMode: multiSourceMode.value,
          selectedSources: selectedSources.value,
          recordTypeFilter: recordTypeFilter.value,
          stackByStatus: stackByStatus.value,
        },
      })
      // Auto-save dashboard
      dashboardStore.saveDashboard()
    }
  },
  { deep: true }
)

// Source selector functions
function selectAllSources() {
  selectedSources.value = availableSources.value.map(s => s.source)
}

function clearAllSources() {
  selectedSources.value = []
}

// Watch for multiSourceMode toggle to load sources
watch(multiSourceMode, async enabled => {
  if (enabled) {
    await dataSourceStore.loadDataSources()
  }
})

// Transform records to chart data
const chartData = computed<ChartData<any>>(() => {
  if (records.value.length === 0) {
    return {
      labels: [],
      datasets: [],
    }
  }

  // Check if this is timeseries data
  const isTimeseries = groupBy.value === 'time'

  if (isTimeseries) {
    return generateTimeseriesData()
  } else {
    return generateGroupedData()
  }
})

function generateGroupedData() {
  // Group records by the specified field
  const groupField = groupBy.value
  const grouped: Record<string, any[]> = {}

  records.value.forEach(record => {
    let key: string
    if (groupField === 'status') {
      key = record.metadata?.status || 'unknown'
    } else if (groupField === 'source') {
      key = record.source || 'unknown'
    } else if (groupField === 'type') {
      key = record.record_type || 'unknown'
    } else {
      key = 'unknown'
    }

    if (!grouped[key]) {
      grouped[key] = []
    }
    grouped[key].push(record)
  })

  const labels = Object.keys(grouped).sort()

  // Apply data transformation
  const data = labels.map(label => {
    const items = grouped[label]
    switch (dataTransform.value) {
      case 'count':
        return items.length
      case 'sum':
        // Sum a numeric field if available
        return items.reduce((sum, item) => sum + (item.data?.value || 0), 0)
      case 'avg': {
        const values = items.map(item => item.data?.value || 0)
        return values.length > 0 ? values.reduce((a, b) => a + b, 0) / values.length : 0
      }
      default:
        return items.length
    }
  })

  // Color mapping
  const colors = generateColors(labels)

  return {
    labels,
    datasets: [
      {
        label: getDatasetLabel(),
        data,
        backgroundColor:
          chartType.value === 'pie' || chartType.value === 'doughnut' ? colors : colors,
        borderColor: colors.map(c => c.replace('0.8', '1')),
        borderWidth: 1,
      },
    ],
  }
}

function generateTimeseriesData() {
  // If stacking by status is enabled, create multiple datasets
  if (stackByStatus.value) {
    return generateStackedTimeseriesData()
  }

  // Group records by time buckets (original single-line behavior)
  const buckets: Record<string, any[]> = {}

  records.value.forEach(record => {
    const timestamp = getRecordTimestamp(record)
    const bucketKey = getTimeBucket(timestamp, timeBucket.value)

    if (!buckets[bucketKey]) {
      buckets[bucketKey] = []
    }
    buckets[bucketKey].push(record)
  })

  const labels = Object.keys(buckets).sort()
  const data = labels.map(label => buckets[label].length)

  return {
    labels,
    datasets: [
      {
        label: 'Records Over Time',
        data,
        backgroundColor: 'rgba(54, 162, 235, 0.5)',
        borderColor: 'rgba(54, 162, 235, 1)',
        borderWidth: 2,
        fill: chartType.value === 'line',
      },
    ],
  }
}

// Generate stacked timeseries data grouped by status
function generateStackedTimeseriesData() {
  // Group by time bucket AND status
  const buckets: Record<string, Record<string, number>> = {}

  records.value.forEach(record => {
    const timestamp = getRecordTimestamp(record)
    const bucketKey = getTimeBucket(timestamp, timeBucket.value)
    const status = record.metadata?.status || 'unknown'

    if (!buckets[bucketKey]) {
      buckets[bucketKey] = {}
    }

    buckets[bucketKey][status] = (buckets[bucketKey][status] || 0) + 1
  })

  const labels = Object.keys(buckets).sort()

  // Collect all unique statuses
  const allStatuses = new Set<string>()
  Object.values(buckets).forEach(statusCounts => {
    Object.keys(statusCounts).forEach(status => allStatuses.add(status))
  })

  // Define status order and colors (bottom to top)
  const statusOrder = [
    'success',
    'failed',
    'canceled',
    'skipped',
    'pending',
    'running',
    'manual',
    'unknown',
  ]
  const statusColors: Record<string, { bg: string; border: string }> = {
    success: { bg: 'rgba(75, 192, 192, 0.7)', border: 'rgba(75, 192, 192, 1)' },
    failed: { bg: 'rgba(255, 99, 132, 0.7)', border: 'rgba(255, 99, 132, 1)' },
    canceled: { bg: 'rgba(153, 102, 255, 0.7)', border: 'rgba(153, 102, 255, 1)' },
    skipped: { bg: 'rgba(201, 203, 207, 0.7)', border: 'rgba(201, 203, 207, 1)' },
    pending: { bg: 'rgba(255, 206, 86, 0.7)', border: 'rgba(255, 206, 86, 1)' },
    running: { bg: 'rgba(54, 162, 235, 0.7)', border: 'rgba(54, 162, 235, 1)' },
    manual: { bg: 'rgba(255, 159, 64, 0.7)', border: 'rgba(255, 159, 64, 1)' },
    unknown: { bg: 'rgba(128, 128, 128, 0.7)', border: 'rgba(128, 128, 128, 1)' },
  }

  // Sort statuses by defined order
  const sortedStatuses = Array.from(allStatuses).sort((a, b) => {
    const indexA = statusOrder.indexOf(a.toLowerCase())
    const indexB = statusOrder.indexOf(b.toLowerCase())
    const orderA = indexA === -1 ? 999 : indexA
    const orderB = indexB === -1 ? 999 : indexB
    return orderA - orderB
  })

  // Create a dataset for each status
  const datasets = sortedStatuses.map(status => {
    const data = labels.map(label => buckets[label][status] || 0)
    const colors = statusColors[status.toLowerCase()] || {
      bg: 'rgba(128, 128, 128, 0.7)',
      border: 'rgba(128, 128, 128, 1)',
    }

    return {
      label: status.charAt(0).toUpperCase() + status.slice(1),
      data,
      backgroundColor: colors.bg,
      borderColor: colors.border,
      borderWidth: 2,
      fill: true,
    }
  })

  return {
    labels,
    datasets,
  }
}

// Helper function to get timestamp from record using configurable field path
function getRecordTimestamp(record: any): Date {
  const fieldPath = timeFieldPath.value

  if (fieldPath === 'timestamp') {
    return new Date(record.timestamp)
  }

  // Navigate nested path like "data.created_at"
  const parts = fieldPath.split('.')
  let value = record

  for (const part of parts) {
    value = value?.[part]
    if (value === undefined || value === null) {
      // Fallback to timestamp if path doesn't exist
      return new Date(record.timestamp)
    }
  }

  // Try to parse the value as a date
  try {
    const date = new Date(value)
    // Check if valid date
    if (!isNaN(date.getTime())) {
      return date
    }
  } catch {
    // Ignore parsing errors
  }

  // Fallback to record timestamp
  return new Date(record.timestamp)
}

function getTimeBucket(date: Date, bucket: string): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hour = String(date.getHours()).padStart(2, '0')

  switch (bucket) {
    case 'hour':
      return `${year}-${month}-${day} ${hour}:00`
    case 'day':
      return `${year}-${month}-${day}`
    case 'week': {
      const weekStart = new Date(date)
      weekStart.setDate(date.getDate() - date.getDay())
      return `Week of ${weekStart.toISOString().split('T')[0]}`
    }
    case 'month':
      return `${year}-${month}`
    default:
      return `${year}-${month}-${day}`
  }
}

function generateColors(labels: string[]): string[] {
  // Status-specific colors
  const statusColors: Record<string, string> = {
    success: 'rgba(75, 192, 192, 0.8)',
    failed: 'rgba(255, 99, 132, 0.8)',
    running: 'rgba(54, 162, 235, 0.8)',
    pending: 'rgba(255, 206, 86, 0.8)',
    canceled: 'rgba(153, 102, 255, 0.8)',
    skipped: 'rgba(201, 203, 207, 0.8)',
    unknown: 'rgba(128, 128, 128, 0.8)',
  }

  // Generate colors for labels
  return labels.map((label, index) => {
    const lowerLabel = label.toLowerCase()
    if (statusColors[lowerLabel]) {
      return statusColors[lowerLabel]
    }
    // Generate distinct colors for other labels
    const hue = (index * 137.5) % 360
    return `hsla(${hue}, 70%, 60%, 0.8)`
  })
}

function getDatasetLabel(): string {
  const transformLabel =
    dataTransform.value === 'count'
      ? 'Count'
      : dataTransform.value === 'sum'
        ? 'Sum'
        : dataTransform.value === 'avg'
          ? 'Average'
          : 'Value'

  const groupLabel =
    groupBy.value === 'status'
      ? 'by Status'
      : groupBy.value === 'source'
        ? 'by Source'
        : groupBy.value === 'type'
          ? 'by Type'
          : ''

  return `${transformLabel} ${groupLabel}`
}

const chartOptions = computed<ChartOptions<any>>(() => ({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'top' as const,
      labels: {
        color: '#374151',
        font: {
          size: 12,
        },
      },
    },
    title: {
      display: true,
      text: props.panel.title || 'Data Chart',
      color: '#111827',
      font: {
        size: 16,
        weight: 'bold',
      },
    },
    tooltip: {
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      padding: 12,
      titleFont: {
        size: 14,
      },
      bodyFont: {
        size: 13,
      },
    },
  },
  scales:
    chartType.value === 'pie' || chartType.value === 'doughnut'
      ? undefined
      : {
          y: {
            beginAtZero: true,
            stacked: stackByStatus.value, // Enable stacking when status stacking is on
            ticks: {
              stepSize: 1,
              color: '#6b7280',
            },
            grid: {
              color: 'rgba(0, 0, 0, 0.05)',
            },
          },
          x: {
            stacked: stackByStatus.value, // Enable stacking when status stacking is on
            ticks: {
              color: '#6b7280',
            },
            grid: {
              color: 'rgba(0, 0, 0, 0.05)',
            },
          },
        },
}))

async function fetchData() {
  loading.value = true
  error.value = ''

  try {
    // Multi-source mode
    if (multiSourceMode.value) {
      if (selectedSources.value.length === 0) {
        records.value = []
        return
      }
      // Fetch from multiple sources
      let allRecords = await dataSourceStore.getRecordsBySource(selectedSources.value)

      // Filter by record type if specified
      if (recordTypeFilter.value) {
        allRecords = allRecords.filter(r => r.record_type === recordTypeFilter.value)
      }

      records.value = allRecords
      return
    }

    // Single source mode (original behavior)
    if (!isTauri.value) {
      // Browser mode - use mock data
      records.value = generateMockData()
      return
    }

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
    console.error('Chart data fetch error:', e)
  } finally {
    loading.value = false
  }
}

function generateMockData() {
  // Mock data for browser mode
  const statuses = ['success', 'failed', 'running', 'pending']
  return Array.from({ length: 20 }, (_, i) => ({
    id: `mock-${i}`,
    record_type: 'gitlab_job',
    source: 'mock-source',
    timestamp: new Date(Date.now() - i * 3600000).toISOString(),
    data: { id: i, name: `Mock Job ${i}` },
    metadata: {
      status: statuses[i % statuses.length],
      title: `Mock Job ${i}`,
      tags: ['mock', 'test'],
    },
  }))
}

function refreshData() {
  fetchData()
}

onMounted(() => {
  fetchData()
})
</script>

<style scoped>
.chart-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
  background: var(--bg-panel);
  border-radius: 8px;
  box-shadow: var(--panel-shadow);
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 2px solid var(--border-color);
}

.chart-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-heading);
}

.chart-controls {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.chart-type-selector,
.chart-control-selector {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  background: var(--bg-panel);
  color: var(--text-primary);
  cursor: pointer;
}

.chart-control-selector {
  font-size: 13px;
  padding: 5px 10px;
}

.btn-refresh {
  padding: 8px 16px;
  background: var(--bg-button);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-refresh:hover:not(:disabled) {
  background: var(--bg-button-hover);
}

.btn-refresh:disabled {
  background: var(--bg-button-secondary);
  cursor: not-allowed;
  opacity: 0.8;
}

.spinner {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: currentColor;
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
  background: color-mix(in srgb, var(--accent-danger) 12%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-danger) 30%, transparent);
  border-radius: 6px;
  color: var(--accent-danger);
  margin-bottom: 16px;
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
  font-size: 14px;
  margin-top: 8px;
  color: var(--text-muted);
}

.chart-container {
  flex: 1;
  position: relative;
  min-height: 300px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Multi-source controls */
.multi-source-toggle,
.stack-status-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  cursor: pointer;
  user-select: none;
  color: var(--text-primary);
}

.multi-source-toggle input[type='checkbox'],
.stack-status-toggle input[type='checkbox'] {
  cursor: pointer;
  width: 16px;
  height: 16px;
}

.btn-sources {
  padding: 6px 12px;
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-sources:hover {
  background: var(--bg-button-secondary-hover);
}

/* Source Selector Modal */
.source-selector-modal {
  position: absolute;
  top: 70px;
  right: 16px;
  width: 400px;
  max-height: 500px;
  background: var(--bg-modal);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  color: var(--text-primary);
}

.source-selector-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-panel-header);
}

.source-selector-header h4 {
  margin: 0;
  flex: 1;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-heading);
}

.btn-select-all,
.btn-clear-all {
  padding: 6px 12px;
  font-size: 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-panel);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.btn-select-all:hover {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border-color: var(--accent-primary);
}

.btn-clear-all:hover {
  background: var(--accent-danger);
  color: var(--text-on-accent);
  border-color: var(--accent-danger);
}

.btn-close {
  padding: 4px 12px;
  font-size: 24px;
  line-height: 1;
  border: none;
  background: none;
  cursor: pointer;
  color: var(--text-secondary);
  font-weight: bold;
}

.btn-close:hover {
  color: var(--text-heading);
}

.source-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  max-height: 400px;
}

.source-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  margin-bottom: 8px;
  cursor: pointer;
  transition: all 0.2s;
  background: var(--bg-panel);
}

.source-item:hover {
  background: var(--bg-panel-header);
  border-color: var(--accent-primary);
}

.source-item input[type='checkbox'] {
  cursor: pointer;
  width: 18px;
  height: 18px;
}

.source-name {
  flex: 1;
  font-weight: 500;
  color: var(--text-primary);
  font-size: 14px;
}

.source-type {
  font-size: 12px;
  color: var(--text-secondary);
  background: var(--bg-panel-header);
  padding: 4px 8px;
  border-radius: 4px;
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.empty-sources {
  padding: 32px;
  text-align: center;
  color: var(--text-muted);
  font-style: italic;
  font-size: 14px;
}
</style>
