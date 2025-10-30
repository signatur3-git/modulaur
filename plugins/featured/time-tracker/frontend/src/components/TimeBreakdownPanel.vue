<template>
  <div class="time-breakdown-panel">
    <div class="panel-header">
      <h2>📊 Time Breakdown</h2>
      <div class="view-selector">
        <button
          v-for="view in views"
          :key="view.id"
          @click="currentView = view.id"
          :class="['view-btn', { active: currentView === view.id }]"
        >
          {{ view.label }}
        </button>
      </div>
    </div>

    <!-- Weekly View -->
    <div v-if="currentView === 'week'" class="breakdown-section">
      <h3>📅 This Week</h3>

      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-icon">⏱️</div>
          <div class="card-content">
            <div class="card-value">{{ formatDurationShort(weekSummary.totalDuration) }}</div>
            <div class="card-label">Total Time</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-icon">📁</div>
          <div class="card-content">
            <div class="card-value">{{ weekSummary.projectCount }}</div>
            <div class="card-label">Projects</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-icon">💰</div>
          <div class="card-content">
            <div class="card-value">${{ weekSummary.totalBillable.toFixed(2) }}</div>
            <div class="card-label">Billable</div>
          </div>
        </div>
      </div>

      <div class="project-breakdown">
        <h4>By Project</h4>
        <div class="breakdown-list">
          <div
            v-for="item in weekByProject"
            :key="item.project"
            class="breakdown-item"
          >
            <div class="item-header">
              <span class="item-name">{{ item.project }}</span>
              <span class="item-value">{{ formatDurationShort(item.duration) }}</span>
            </div>
            <div class="item-bar">
              <div
                class="item-bar-fill"
                :style="{ width: `${(item.duration / weekSummary.totalDuration) * 100}%` }"
              />
            </div>
            <div class="item-details">
              <span>{{ item.entries }} entries</span>
              <span v-if="item.billable">💰 ${{ item.billable.toFixed(2) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="daily-breakdown">
        <h4>By Day</h4>
        <div class="breakdown-list">
          <div
            v-for="item in weekByDay"
            :key="item.date"
            class="breakdown-item"
          >
            <div class="item-header">
              <span class="item-name">{{ item.label }}</span>
              <span class="item-value">{{ formatDurationShort(item.duration) }}</span>
            </div>
            <div class="item-bar">
              <div
                class="item-bar-fill"
                :style="{ width: `${(item.duration / weekSummary.totalDuration) * 100}%` }"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Monthly View -->
    <div v-if="currentView === 'month'" class="breakdown-section">
      <h3>📆 This Month</h3>

      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-icon">⏱️</div>
          <div class="card-content">
            <div class="card-value">{{ formatDurationShort(monthSummary.totalDuration) }}</div>
            <div class="card-label">Total Time</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-icon">📁</div>
          <div class="card-content">
            <div class="card-value">{{ monthSummary.projectCount }}</div>
            <div class="card-label">Projects</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-icon">💰</div>
          <div class="card-content">
            <div class="card-value">${{ monthSummary.totalBillable.toFixed(2) }}</div>
            <div class="card-label">Billable</div>
          </div>
        </div>
      </div>

      <div class="project-breakdown">
        <h4>By Project</h4>
        <div class="breakdown-list">
          <div
            v-for="item in monthByProject"
            :key="item.project"
            class="breakdown-item"
          >
            <div class="item-header">
              <span class="item-name">{{ item.project }}</span>
              <span class="item-value">{{ formatDurationShort(item.duration) }}</span>
            </div>
            <div class="item-bar">
              <div
                class="item-bar-fill"
                :style="{ width: `${(item.duration / monthSummary.totalDuration) * 100}%` }"
              />
            </div>
            <div class="item-details">
              <span>{{ item.entries }} entries</span>
              <span v-if="item.billable">💰 ${{ item.billable.toFixed(2) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="weekly-breakdown">
        <h4>By Week</h4>
        <div class="breakdown-list">
          <div
            v-for="item in monthByWeek"
            :key="item.week"
            class="breakdown-item"
          >
            <div class="item-header">
              <span class="item-name">{{ item.label }}</span>
              <span class="item-value">{{ formatDurationShort(item.duration) }}</span>
            </div>
            <div class="item-bar">
              <div
                class="item-bar-fill"
                :style="{ width: `${(item.duration / monthSummary.totalDuration) * 100}%` }"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading">Loading breakdown data...</div>
    <div v-if="error" class="error-message">❌ {{ error }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useTimeTracker } from '../composables/useTimeTracker';
import { formatDurationShort } from '../utils/duration';
import {
  getThisWeekEntries,
  getThisMonthEntries,
  getBreakdownByProject,
  getBreakdownByDay,
  getBreakdownByWeek,
  getSummary
} from '../utils/aggregation';

const props = defineProps<{
  panel: {
    i: string;
    [key: string]: any;
  };
}>();

const {
  entries,
  loading,
  error,
  loadEntries
} = useTimeTracker();

const currentView = ref('week');

const views = [
  { id: 'week', label: 'Week' },
  { id: 'month', label: 'Month' }
];

// Weekly data
const weekEntries = computed(() => getThisWeekEntries(entries.value));
const weekSummary = computed(() => getSummary(weekEntries.value));
const weekByProject = computed(() => getBreakdownByProject(weekEntries.value));
const weekByDay = computed(() => getBreakdownByDay(weekEntries.value));

// Monthly data
const monthEntries = computed(() => getThisMonthEntries(entries.value));
const monthSummary = computed(() => getSummary(monthEntries.value));
const monthByProject = computed(() => getBreakdownByProject(monthEntries.value));
const monthByWeek = computed(() => getBreakdownByWeek(monthEntries.value));

onMounted(() => {
  loadEntries();
});
</script>

<style scoped>
.time-breakdown-panel {
  padding: 1.5rem;
  background: var(--bg-panel);
  border-radius: 8px;
  color: var(--text-primary);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

/* View Selector */
.view-selector {
  display: flex;
  gap: 0.5rem;
  background: var(--bg-panel-header);
  padding: 0.25rem;
  border-radius: 6px;
}

.view-btn {
  padding: 0.5rem 1rem;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.95rem;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.view-btn:hover {
  color: var(--text-primary);
  background: var(--bg-panel-header));
}

.view-btn.active {
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

/* Summary Cards */
.summary-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.summary-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--bg-panel-header);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.card-icon {
  font-size: 2.5rem;
}

.card-content {
  flex: 1;
}

.card-value {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.card-label {
  font-size: 0.875rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* Breakdown Sections */
.breakdown-section h3 {
  margin: 0 0 1.5rem 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.project-breakdown,
.daily-breakdown,
.weekly-breakdown {
  margin-bottom: 2rem;
}

.project-breakdown h4,
.daily-breakdown h4,
.weekly-breakdown h4 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

/* Breakdown List */
.breakdown-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.breakdown-item {
  background: var(--bg-panel-header);
  padding: 1rem;
  border-radius: 6px;
  border-left: 4px solid var(--accent-primary);
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.item-name {
  font-weight: 600;
  color: var(--text-primary);
}

.item-value {
  font-weight: 700;
  color: var(--accent-primary);
}

.item-bar {
  height: 8px;
  background: var(--bg-app);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.item-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-primary), var(--accent-hover));
  transition: width 0.3s ease;
}

.item-details {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

/* Loading & Error */
.loading {
  text-align: center;
  padding: 2rem;
  color: var(--text-secondary);
}

.error-message {
  padding: 1rem;
  background: var(--accent-danger); opacity: 0.2;
  color: var(--text-on-accent);
  border-radius: 4px;
  margin-top: 1rem;
}

/* Dark Mode */
@media (prefers-color-scheme: dark) {
  .time-breakdown-panel {
    background: var(--bg-panel);
  }

  .panel-header h2,
  .breakdown-section h3,
  .project-breakdown h4,
  .daily-breakdown h4,
  .weekly-breakdown h4,
  .item-name,
  .card-value {
    color: var(--text-primary);
  }

  .summary-card,
  .breakdown-item {
    background: var(--bg-panel-header);
  }

  .view-selector {
    background: var(--bg-panel-header);
  }

  .view-btn {
    color: var(--text-secondary);
  }

  .view-btn:hover {
    color: var(--text-primary);
    background: var(--bg-panel-header));
  }

  .item-bar {
    background: var(--bg-app);
  }
}
</style>

