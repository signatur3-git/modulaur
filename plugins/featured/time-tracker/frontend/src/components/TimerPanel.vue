<template>
  <div class="timer-panel">
    <!-- Active Timer Display -->
    <div v-if="activeTimer" class="active-timer">
      <div class="timer-display">
        <span class="timer-time">{{ formatDuration(elapsedSeconds) }}</span>
        <span class="timer-label">RUNNING</span>
      </div>

      <div class="timer-info">
        <div class="timer-project">
          <strong>{{ activeTimer.data.project }}</strong>
        </div>
        <div class="timer-description">
          {{ activeTimer.data.description }}
        </div>
        <div v-if="activeTimer.data.ticket_id" class="timer-ticket">
          🎫 {{ activeTimer.data.ticket_id }}
        </div>
      </div>

      <button @click="handleStopTimer" class="btn-stop" :disabled="loading">
        ⏹️ Stop Timer
      </button>
    </div>

    <!-- Start Timer Form -->
    <div v-else class="start-timer">
      <h3>Start Timer</h3>

      <div class="form-group">
        <label>Project *</label>
        <input
          v-model="newEntry.project"
          type="text"
          list="projects-list"
          placeholder="Enter project name"
          required
        />
        <datalist id="projects-list">
          <option v-for="project in projects" :key="project" :value="project" />
        </datalist>
      </div>

      <div class="form-group">
        <label>Description *</label>
        <input
          v-model="newEntry.description"
          type="text"
          placeholder="What are you working on?"
          required
        />
      </div>

      <div class="form-group">
        <label>Ticket (optional)</label>
        <input
          v-model="newEntry.ticketId"
          type="text"
          placeholder="ticket:abc123"
        />
      </div>

      <button
        @click="handleStartTimer"
        class="btn-start"
        :disabled="!canStart || loading"
      >
        ▶️ Start Timer
      </button>
    </div>

    <!-- Error Display -->
    <div v-if="error" class="error-message">
      ❌ {{ error }}
    </div>

    <!-- Quick Stats -->
    <div class="quick-stats">
      <div class="stat">
        <span class="stat-label">Today:</span>
        <strong>{{ formatDurationShort(todaySummary.totalDuration) }}</strong>
      </div>
      <div class="stat">
        <span class="stat-label">This Week:</span>
        <strong>{{ formatDurationShort(weekSummary.totalDuration) }}</strong>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useTimeTracker } from '../composables/useTimeTracker';
import { formatDuration, formatDurationShort, getElapsedTime } from '../utils/duration';

const props = defineProps<{
  panel: {
    i: string;
    [key: string]: any;
  };
}>();

const {
  activeTimer,
  loading,
  error,
  todayEntries,
  weekEntries,
  projects,
  loadEntries,
  startTimer,
  stopTimer,
  getSummary
} = useTimeTracker();

// Form data
const newEntry = ref({
  project: '',
  description: '',
  ticketId: ''
});

// Elapsed time for active timer (updates every second)
const elapsedSeconds = ref(0);
let timerInterval: number | null = null;

// Can start timer
const canStart = computed(() => {
  return newEntry.value.project.trim() && newEntry.value.description.trim();
});

// Summaries
const todaySummary = computed(() => getSummary(todayEntries.value));
const weekSummary = computed(() => getSummary(weekEntries.value));

// Update elapsed time
function updateElapsed() {
  if (activeTimer.value) {
    elapsedSeconds.value = getElapsedTime(activeTimer.value.timestamp);
  }
}

// Start timer
async function handleStartTimer() {
  try {
    await startTimer(
      newEntry.value.project,
      newEntry.value.description,
      newEntry.value.ticketId || undefined
    );

    // Clear form
    newEntry.value.description = '';
    newEntry.value.ticketId = '';
    // Keep project for next entry
  } catch (err) {
    console.error('Failed to start timer:', err);
  }
}

// Stop timer
async function handleStopTimer() {
  try {
    await stopTimer();
  } catch (err) {
    console.error('Failed to stop timer:', err);
  }
}

// Lifecycle
onMounted(async () => {
  await loadEntries();

  // Start interval for elapsed time
  timerInterval = window.setInterval(() => {
    updateElapsed();
  }, 1000);
});

onUnmounted(() => {
  if (timerInterval) {
    clearInterval(timerInterval);
  }
});
</script>

<style scoped>
.timer-panel {
  padding: 1rem;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.active-timer {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: var(--text-on-accent);
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.timer-display {
  text-align: center;
  margin-bottom: 1rem;
}

.timer-time {
  display: block;
  font-size: 3rem;
  font-weight: bold;
  font-family: 'Courier New', monospace;
  letter-spacing: 0.05em;
}

.timer-label {
  display: block;
  font-size: 0.75rem;
  opacity: 0.9;
  margin-top: 0.25rem;
  letter-spacing: 0.2em;
}

.timer-info {
  margin-bottom: 1rem;
}

.timer-project {
  font-size: 1.1rem;
  margin-bottom: 0.25rem;
}

.timer-description {
  opacity: 0.9;
  margin-bottom: 0.25rem;
}

.timer-ticket {
  font-size: 0.85rem;
  opacity: 0.8;
}

.btn-stop {
  width: 100%;
  padding: 0.75rem;
  font-size: 1rem;
  font-weight: 600;
  background: var(--bg-panel-header); opacity: 0.8;
  color: var(--text-on-accent);
  border: 2px solid white;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-stop:hover:not(:disabled) {
  background: var(--bg-panel);
  color: var(--accent-primary);
}

.btn-stop:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.start-timer {
  background: var(--bg-panel);
  padding: 1.5rem;
  border-radius: 12px;
  border: 2px solid #e2e8f0;
}

.start-timer h3 {
  margin: 0 0 1rem 0;
  color: var(--text-heading);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.25rem;
  font-weight: 500;
  color: var(--text-primary);
  font-size: 0.875rem;
}

.form-group input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #cbd5e0;
  border-radius: 6px;
  font-size: 0.875rem;
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.btn-start {
  width: 100%;
  padding: 0.75rem;
  font-size: 1rem;
  font-weight: 600;
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-start:hover:not(:disabled) {
  background: var(--accent-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 6px rgba(102, 126, 234, 0.3);
}

.btn-start:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-message {
  padding: 0.75rem;
  background: var(--accent-danger); opacity: 0.2;
  color: var(--accent-danger);
  border-radius: 6px;
  font-size: 0.875rem;
}

.quick-stats {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-app);
  border-radius: 8px;
  border: 1px solid #e2e8f0;
}

.stat {
  flex: 1;
  text-align: center;
}

.stat-label {
  display: block;
  font-size: 0.75rem;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
}

.stat strong {
  display: block;
  font-size: 1.25rem;
  color: var(--text-heading);
}
</style>

