<template>
  <div class="time-summary-panel">
    <!-- Time Range Selector -->
    <div class="range-selector">
      <button
        v-for="range in timeRanges"
        :key="range.value"
        @click="selectedRange = range.value"
        :class="{ active: selectedRange === range.value }"
        class="range-btn"
      >
        {{ range.label }}
      </button>
    </div>

    <!-- Summary Cards -->
    <div class="summary-cards">
      <div class="summary-card">
        <div class="card-icon">⏱️</div>
        <div class="card-content">
          <div class="card-label">Total Time</div>
          <div class="card-value">{{ formatDurationShort(summary.totalDuration) }}</div>
          <div class="card-sublabel">{{ formatHours(summary.totalDuration) }} hours</div>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon">💰</div>
        <div class="card-content">
          <div class="card-label">Billable Time</div>
          <div class="card-value">{{ formatDurationShort(summary.billableDuration) }}</div>
          <div class="card-sublabel">{{ billablePercent }}% of total</div>
        </div>
      </div>

      <div class="summary-card" v-if="summary.totalAmount > 0">
        <div class="card-icon">💵</div>
        <div class="card-content">
          <div class="card-label">Total Amount</div>
          <div class="card-value">${{ summary.totalAmount.toFixed(2) }}</div>
          <div class="card-sublabel">Billable earnings</div>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon">📋</div>
        <div class="card-content">
          <div class="card-label">Entries</div>
          <div class="card-value">{{ summary.entriesCount }}</div>
          <div class="card-sublabel">Time entries logged</div>
        </div>
      </div>
    </div>

    <!-- By Project -->
    <div class="chart-section">
      <h3>Time by Project</h3>
      <div class="project-list">
        <div
          v-for="(data, project) in sortedProjects"
          :key="project"
          class="project-item"
        >
          <div class="project-header">
            <span class="project-name">{{ project }}</span>
            <span class="project-duration">{{ formatDurationShort(data.duration) }}</span>
          </div>
          <div class="project-bar">
            <div
              class="project-bar-fill"
              :style="{ width: (data.duration / summary.totalDuration * 100) + '%' }"
            ></div>
          </div>
          <div class="project-meta">
            <span>{{ data.entries }} entries</span>
            <span v-if="data.amount > 0">${{ data.amount.toFixed(2) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- By Date -->
    <div class="chart-section">
      <h3>Time by Date</h3>
      <div class="date-chart">
        <div
          v-for="(duration, date) in sortedDates"
          :key="date"
          class="date-bar"
        >
          <div class="date-label">{{ formatDateShort(date) }}</div>
          <div class="date-bar-container">
            <div
              class="date-bar-fill"
              :style="{ height: (duration / maxDayDuration * 100) + '%' }"
              :title="formatDurationShort(duration)"
            ></div>
          </div>
          <div class="date-value">{{ formatHours(duration) }}h</div>
        </div>
      </div>
    </div>

    <!-- Export Button -->
    <div class="export-section">
      <button @click="handleExport" class="btn-export-full">
        📥 Export Summary to CSV
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useTimeTracker } from '../composables/useTimeTracker';
import { formatDurationShort, formatHours } from '../utils/duration';
import { getTodayEntries, getThisWeekEntries, getEntriesInRange } from '../utils/aggregation';

const props = defineProps<{
  panel: {
    i: string;
    [key: string]: any;
  };
}>();

const {
  entries,
  loading,
  getSummary,
  exportCSV,
  loadEntries
} = useTimeTracker();

// Time range selection
const timeRanges = [
  { label: 'Today', value: 'today' },
  { label: 'This Week', value: 'week' },
  { label: 'This Month', value: 'month' },
  { label: 'All Time', value: 'all' }
];

const selectedRange = ref('week');

// Get entries for selected range
const rangeEntries = computed(() => {
  switch (selectedRange.value) {
    case 'today':
      return getTodayEntries(entries.value);
    case 'week':
      return getThisWeekEntries(entries.value);
    case 'month':
      const now = new Date();
      const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
      return getEntriesInRange(entries.value, monthStart.toISOString(), now.toISOString());
    default:
      return entries.value;
  }
});

// Summary
const summary = computed(() => getSummary(rangeEntries.value));

// Billable percentage
const billablePercent = computed(() => {
  if (summary.value.totalDuration === 0) return 0;
  return Math.round((summary.value.billableDuration / summary.value.totalDuration) * 100);
});

// Sorted projects by duration
const sortedProjects = computed(() => {
  return Object.entries(summary.value.byProject)
    .sort(([, a], [, b]) => b.duration - a.duration)
    .reduce((acc, [key, value]) => {
      acc[key] = value;
      return acc;
    }, {} as Record<string, any>);
});

// Sorted dates
const sortedDates = computed(() => {
  return Object.entries(summary.value.byDate)
    .sort(([a], [b]) => a.localeCompare(b))
    .reduce((acc, [key, value]) => {
      acc[key] = value;
      return acc;
    }, {} as Record<string, number>);
});

// Max day duration (for chart scaling)
const maxDayDuration = computed(() => {
  return Math.max(...Object.values(summary.value.byDate), 1);
});

// Format date for display
function formatDateShort(date: string): string {
  const d = new Date(date);
  return `${d.getMonth() + 1}/${d.getDate()}`;
}

// Export
function handleExport() {
  const filename = `summary-${selectedRange.value}-${new Date().toISOString().split('T')[0]}.csv`;
  exportCSV(rangeEntries.value, filename);
}

// Lifecycle
onMounted(() => {
  loadEntries();
});
</script>

<style scoped>
.time-summary-panel {
  padding: 1rem;
  height: 100%;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.range-selector {
  display: flex;
  gap: 0.5rem;
  background: var(--bg-app);
  padding: 0.5rem;
  border-radius: 8px;
}

.range-btn {
  flex: 1;
  padding: 0.5rem 1rem;
  background: var(--bg-panel);
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.range-btn:hover {
  border-color: var(--border-color);
}

.range-btn.active {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border-color: var(--accent-primary);
  font-weight: 600;
}

.summary-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.summary-card {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-panel);
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  align-items: center;
}

.card-icon {
  font-size: 2rem;
}

.card-content {
  flex: 1;
}

.card-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
}

.card-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-heading);
  margin-bottom: 0.125rem;
}

.card-sublabel {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.chart-section {
  background: var(--bg-panel);
  padding: 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
}

.chart-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: var(--text-heading);
}

.project-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.project-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.project-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.project-name {
  font-weight: 600;
  color: var(--text-heading);
  font-size: 0.875rem;
}

.project-duration {
  font-weight: 600;
  color: var(--accent-primary);
  font-size: 0.875rem;
}

.project-bar {
  height: 8px;
  background: var(--bg-app);
  border-radius: 4px;
  overflow: hidden;
}

.project-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  border-radius: 4px;
  transition: width 0.3s;
}

.project-meta {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.date-chart {
  display: flex;
  gap: 0.5rem;
  height: 150px;
  align-items: flex-end;
  overflow-x: auto;
  padding-bottom: 0.5rem;
}

.date-bar {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
  flex: 1;
  min-width: 40px;
}

.date-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  white-space: nowrap;
}

.date-bar-container {
  flex: 1;
  width: 100%;
  max-width: 40px;
  background: var(--bg-app);
  border-radius: 4px 4px 0 0;
  display: flex;
  align-items: flex-end;
  position: relative;
}

.date-bar-fill {
  width: 100%;
  background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
  border-radius: 4px 4px 0 0;
  transition: height 0.3s;
  min-height: 2px;
}

.date-value {
  font-size: 0.75rem;
  color: var(--text-primary);
  font-weight: 600;
}

.export-section {
  display: flex;
  justify-content: center;
}

.btn-export-full {
  padding: 0.75rem 2rem;
  background: var(--accent-success);
  color: var(--text-on-accent);
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-export-full:hover {
  background: var(--accent-success-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 6px rgba(72, 187, 120, 0.3);
}
</style>

