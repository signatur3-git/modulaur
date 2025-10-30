<template>
  <div class="time-tracker-page">
    <div class="page-header">
      <h1>⏱️ Time Tracker</h1>
      <p>Track your work hours across projects</p>
    </div>

    <!-- Tab Navigation -->
    <div class="tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        @click="activeTab = tab.id"
        :class="['tab', { active: activeTab === tab.id }]"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-label">{{ tab.label }}</span>
      </button>
    </div>

    <!-- Tab Content -->
    <div class="tab-content">
      <!-- Time Entries Tab -->
      <div v-if="activeTab === 'track'" class="tab-pane">
        <TimeLogPanel :panel="{ i: 'page-log' }" />
      </div>

      <!-- Projects Tab -->
      <div v-if="activeTab === 'projects'" class="tab-pane">
        <ProjectManagerPanel :panel="{ i: 'page-projects' }" />
      </div>

      <!-- Overview Tab -->
      <div v-if="activeTab === 'overview'" class="tab-pane">
        <div class="overview-grid">
          <div class="summary-section">
            <TimeSummaryPanel :panel="{ i: 'page-summary' }" />
          </div>
          <div class="breakdown-section">
            <TimeBreakdownPanel :panel="{ i: 'page-breakdown' }" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import TimeLogPanel from './components/TimeLogPanel.vue';
import TimeSummaryPanel from './components/TimeSummaryPanel.vue';
import ProjectManagerPanel from './components/ProjectManagerPanel.vue';
import TimeBreakdownPanel from './components/TimeBreakdownPanel.vue';

const activeTab = ref('track');

const tabs = [
  { id: 'track', label: 'Time Entries', icon: '📋' },
  { id: 'projects', label: 'Projects', icon: '📁' },
  { id: 'overview', label: 'Overview', icon: '📊' }
];
</script>

<style scoped>
.time-tracker-page {
  padding: var(--space-2xl, 2rem);
  max-width: 1400px;
  margin: 0 auto;
  background: var(--bg-app);
  min-height: 100vh;
}

.page-header {
  margin-bottom: var(--space-2xl, 2rem);
}

.page-header h1 {
  margin: 0 0 var(--space-sm, 0.5rem) 0;
  font-size: 2rem;
  color: var(--text-heading);
}

.page-header p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 1rem;
}

/* Tabs */
.tabs {
  display: flex;
  gap: var(--space-sm, 0.5rem);
  border-bottom: 2px solid var(--border-color);
  margin-bottom: var(--space-2xl, 2rem);
}

.tab {
  display: flex;
  align-items: center;
  gap: var(--space-sm, 0.5rem);
  padding: var(--space-md, 0.75rem) var(--space-xl, 1.5rem);
  background: transparent;
  border: none;
  border-bottom: 3px solid transparent;
  cursor: pointer;
  font-size: 1rem;
  color: var(--text-secondary);
  transition: all 0.2s;
  margin-bottom: -2px;
}

.tab:hover {
  color: var(--text-primary);
  background: var(--bg-panel-header);
}

.tab.active {
  color: var(--accent-primary);
  border-bottom-color: var(--accent-primary);
  font-weight: 600;
}

.tab-icon {
  font-size: 1.2rem;
}

.tab-label {
  font-size: 1rem;
}

/* Tab Content */
.tab-content {
  animation: fadeIn 0.3s;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.tab-pane {
  min-height: 400px;
  width: 100%;
}

/* Overview Grid */
.overview-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--panel-gap, 2rem);
}

.summary-section,
.breakdown-section {
  background: var(--bg-panel);
  border-radius: var(--panel-radius, 8px);
  padding: var(--panel-padding, 1rem);
  box-shadow: var(--panel-shadow);
  border: 1px solid var(--border-subtle);
}
</style>

