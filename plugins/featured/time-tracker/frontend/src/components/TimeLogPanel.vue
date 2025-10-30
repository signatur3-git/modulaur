<template>
  <div class="time-log-panel">
    <!-- Header with New Entry Button -->
    <div class="panel-header">
      <h2>⏱️ Time Entries</h2>
      <button @click="createNewEntry" class="btn-primary">
        ➕ New Entry
      </button>
    </div>

    <!-- Filters -->
    <div class="filters">
      <div class="filter-group">
        <select v-model="filters.project">
          <option value="">All Projects</option>
          <option v-for="project in projects" :key="project" :value="project">
            {{ project }}
          </option>
        </select>
      </div>

      <div class="filter-group">
        <input
          v-model="filters.startDate"
          type="date"
          placeholder="Start Date"
        />
      </div>

      <div class="filter-group">
        <input
          v-model="filters.endDate"
          type="date"
          placeholder="End Date"
        />
      </div>

      <button @click="clearFilters" class="btn-clear">Clear</button>
      <button @click="handleExport" class="btn-export">📥 Export CSV</button>
    </div>

    <!-- Summary Bar -->
    <div class="summary-bar">
      <div class="summary-item">
        <span class="label">Entries:</span>
        <strong>{{ filteredEntries.length }}</strong>
      </div>
      <div class="summary-item">
        <span class="label">Total Time:</span>
        <strong>{{ formatDurationShort(summary.totalDuration) }}</strong>
      </div>
      <div class="summary-item">
        <span class="label">Billable:</span>
        <strong>{{ formatDurationShort(summary.billableDuration) }}</strong>
      </div>
      <div class="summary-item" v-if="summary.totalAmount > 0">
        <span class="label">Amount:</span>
        <strong>${{ summary.totalAmount.toFixed(2) }}</strong>
      </div>
    </div>

    <!-- Entries List -->
    <div class="entries-list" v-if="!loading && filteredEntries.length > 0">
      <div
        v-for="entry in filteredEntries"
        :key="entry.id"
        class="entry-card"
        :class="{ 'entry-active': !entry.data.end_time }"
      >
        <div class="entry-header">
          <div class="entry-project">
            <strong>{{ entry.data.project }}</strong>
            <span v-if="entry.data.ticket_id" class="entry-ticket">
              🎫 {{ entry.data.ticket_id }}
            </span>
          </div>
          <div class="entry-duration">
            {{ entry.data.duration_seconds
              ? formatDurationShort(entry.data.duration_seconds)
              : 'RUNNING'
            }}
          </div>
        </div>

        <div class="entry-description">
          {{ entry.data.description }}
        </div>

        <div class="entry-meta">
          <span class="entry-date">
            📅 {{ formatDate(entry.timestamp) }}
          </span>
          <span class="entry-time">
            {{ formatTime(entry.timestamp) }}
            {{ entry.data.end_time ? ' - ' + formatTime(entry.data.end_time) : '' }}
          </span>
          <span v-if="entry.data.billable" class="entry-billable">💰 Billable</span>
          <span v-if="entry.data.tags && entry.data.tags.length > 0" class="entry-tags">
            🏷️ {{ entry.data.tags.join(', ') }}
          </span>
        </div>

        <div class="entry-actions">
          <button @click="editEntry(entry)" class="btn-icon" title="Edit">
            ✏️
          </button>
          <button @click="deleteEntryConfirm(entry)" class="btn-icon btn-delete" title="Delete">
            🗑️
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else-if="!loading && filteredEntries.length === 0" class="empty-state">
      <div class="empty-icon">⏱️</div>
      <p>No time entries found</p>
      <p class="empty-hint">Start a timer or add a manual entry to begin tracking time</p>
    </div>

    <!-- Loading State -->
    <div v-else-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>Loading entries...</p>
    </div>

    <!-- Error State -->
    <div v-if="error" class="error-message">
      ❌ {{ error }}
    </div>

    <!-- Edit Dialog -->
    <div v-if="editingEntry" class="modal-overlay" @click="cancelEdit">
      <div class="modal-dialog" @click.stop>
        <div class="modal-header">
          <h3>{{ editingEntry.isNew ? 'New Time Entry' : 'Edit Time Entry' }}</h3>
          <button @click="cancelEdit" class="btn-close">✕</button>
        </div>

        <form @submit.prevent="saveEdit" class="modal-body">
          <div class="form-group">
            <label>Project *</label>
            <input
              v-model="editForm.project"
              type="text"
              required
              placeholder="Project name"
              list="project-list-edit"
            />
            <datalist id="project-list-edit">
              <option v-for="project in availableProjects" :key="project" :value="project" />
            </datalist>
            <small class="form-hint">Start typing to see existing projects, or enter a new one</small>
          </div>

          <div class="form-group">
            <label>Description *</label>
            <div class="description-with-history">
              <select
                v-if="recentDescriptionsForProject.length > 0"
                v-model="selectedRecentDescription"
                @change="populateDescription"
                class="recent-descriptions-select"
              >
                <option value="">-- Recent descriptions for this project --</option>
                <option
                  v-for="(desc, index) in recentDescriptionsForProject"
                  :key="index"
                  :value="desc"
                >
                  {{ desc.length > 60 ? desc.substring(0, 60) + '...' : desc }}
                </option>
              </select>
              <textarea
                v-model="editForm.description"
                required
                rows="3"
                placeholder="What did you work on?"
              ></textarea>
            </div>
          </div>

          <div class="form-group">
            <label>Ticket ID</label>
            <input
              v-model="editForm.ticket_id"
              type="text"
              placeholder="e.g., TICKET-123"
            />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Start Time *</label>
              <input
                v-model="editForm.start_time"
                type="datetime-local"
                required
              />
            </div>

            <div class="form-group">
              <label>End Time</label>
              <input
                v-model="editForm.end_time"
                type="datetime-local"
              />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>
                <input
                  v-model="editForm.billable"
                  type="checkbox"
                />
                Billable
              </label>
            </div>

            <div class="form-group">
              <label>Hourly Rate ($)</label>
              <input
                v-model.number="editForm.hourly_rate"
                type="number"
                min="0"
                step="0.01"
                placeholder="0.00"
              />
            </div>
          </div>

          <div class="form-group">
            <label>Tags (comma-separated)</label>
            <input
              v-model="editForm.tagsString"
              type="text"
              placeholder="development, client-work"
            />
          </div>

          <div class="form-group">
            <label>Notes</label>
            <textarea
              v-model="editForm.notes"
              rows="2"
              placeholder="Additional notes..."
            ></textarea>
          </div>

          <div class="modal-footer">
            <button type="button" @click="cancelEdit" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary" :disabled="loading">
              {{ loading ? 'Saving...' : 'Save Changes' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useTimeTracker } from '../composables/useTimeTracker';
import { useProjectManager } from '../composables/useProjectManager';
import { formatDurationShort } from '../utils/duration';

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
  projects,
  loadEntries,
  filterEntries,
  getSummary,
  exportCSV,
  deleteEntry,
  updateEntry,
  createEntry
} = useTimeTracker();

const {
  projects: projectList,
  loadProjects
} = useProjectManager();

// Filters
const filters = ref({
  project: '',
  startDate: '',
  endDate: ''
});

// Edit dialog state
const editingEntry = ref<any>(null);
const editForm = ref({
  project: '',
  description: '',
  ticket_id: '',
  start_time: '',
  end_time: '',
  billable: true,
  hourly_rate: null as number | null,
  tagsString: '',
  notes: ''
});

// Recent descriptions tracking
const selectedRecentDescription = ref('');
const recentDescriptions = ref<Record<string, string[]>>({});

// Load recent descriptions from localStorage
function loadRecentDescriptions() {
  try {
    const stored = localStorage.getItem('time-tracker-recent-descriptions');
    if (stored) {
      recentDescriptions.value = JSON.parse(stored);
    }
  } catch (err) {
    console.error('Failed to load recent descriptions:', err);
  }
}

// Save recent descriptions to localStorage
function saveRecentDescriptions() {
  try {
    localStorage.setItem('time-tracker-recent-descriptions', JSON.stringify(recentDescriptions.value));
  } catch (err) {
    console.error('Failed to save recent descriptions:', err);
  }
}

// Add a description to recent list for a project
function addRecentDescription(project: string, description: string) {
  if (!project || !description || description.trim() === '') return;

  const trimmedDescription = description.trim();

  // Get or create the project's description list
  if (!recentDescriptions.value[project]) {
    recentDescriptions.value[project] = [];
  }

  const projectDescriptions = recentDescriptions.value[project];

  // Remove if already exists (to move it to the front)
  const existingIndex = projectDescriptions.indexOf(trimmedDescription);
  if (existingIndex > -1) {
    projectDescriptions.splice(existingIndex, 1);
  }

  // Add to the front
  projectDescriptions.unshift(trimmedDescription);

  // Keep only the 10 most recent
  if (projectDescriptions.length > 10) {
    projectDescriptions.splice(10);
  }

  saveRecentDescriptions();
}

// Get recent descriptions for the currently selected project in the edit form
const recentDescriptionsForProject = computed(() => {
  const project = editForm.value.project;
  if (!project || !recentDescriptions.value[project]) {
    return [];
  }
  return recentDescriptions.value[project];
});

// Populate description textarea from selected recent description
function populateDescription() {
  if (selectedRecentDescription.value) {
    editForm.value.description = selectedRecentDescription.value;
    selectedRecentDescription.value = ''; // Reset selection
  }
}

// Watch for project changes to reset selected recent description
watch(() => editForm.value.project, () => {
  selectedRecentDescription.value = '';
});

// Available projects - combine projects from both sources
const availableProjects = computed(() => {
  const projectNames = new Set<string>();

  // Add from time entries
  projects.value.forEach(p => projectNames.add(p));

  // Add from project manager
  projectList.value.forEach(p => projectNames.add(p.name));

  return Array.from(projectNames).sort();
});

// Filtered entries
const filteredEntries = computed(() => {
  const filterObj: any = {};

  if (filters.value.project) {
    filterObj.project = filters.value.project;
  }
  if (filters.value.startDate) {
    filterObj.startDate = new Date(filters.value.startDate).toISOString();
  }
  if (filters.value.endDate) {
    const endDate = new Date(filters.value.endDate);
    endDate.setHours(23, 59, 59, 999);
    filterObj.endDate = endDate.toISOString();
  }

  return filterEntries(filterObj);
});

// Summary
const summary = computed(() => getSummary(filteredEntries.value));

// Clear filters
function clearFilters() {
  filters.value = {
    project: '',
    startDate: '',
    endDate: ''
  };
}

// Export
function handleExport() {
  const filename = filters.value.project
    ? `timesheet-${filters.value.project}-${new Date().toISOString().split('T')[0]}.csv`
    : `timesheet-${new Date().toISOString().split('T')[0]}.csv`;

  exportCSV(filteredEntries.value, filename);
}

// Create new entry
function createNewEntry() {
  editingEntry.value = { isNew: true }; // Flag to indicate new entry

  const now = new Date();
  const startOfHour = new Date(now);
  startOfHour.setMinutes(0, 0, 0);

  editForm.value = {
    project: '',
    description: '',
    ticket_id: '',
    start_time: formatDateTimeLocal(startOfHour.toISOString()),
    end_time: formatDateTimeLocal(now.toISOString()),
    billable: true,
    hourly_rate: null,
    tagsString: '',
    notes: ''
  };
}

// Edit entry
function editEntry(entry: any) {
  editingEntry.value = entry;

  // Populate form with entry data
  editForm.value = {
    project: entry.data.project || '',
    description: entry.data.description || '',
    ticket_id: entry.data.ticket_id || '',
    start_time: formatDateTimeLocal(entry.timestamp),
    end_time: entry.data.end_time ? formatDateTimeLocal(entry.data.end_time) : '',
    billable: entry.data.billable ?? true,
    hourly_rate: entry.data.hourly_rate || null,
    tagsString: (entry.data.tags || []).join(', '),
    notes: entry.data.notes || ''
  };
}

// Save edit
async function saveEdit() {
  if (!editingEntry.value) return;

  try {
    if (editingEntry.value.isNew) {
      // Create new entry
      const newEntryData = {
        project: editForm.value.project,
        description: editForm.value.description,
        startTime: new Date(editForm.value.start_time).toISOString(),
        endTime: editForm.value.end_time ? new Date(editForm.value.end_time).toISOString() : undefined,
        ticketId: editForm.value.ticket_id || undefined,
        tags: editForm.value.tagsString
          ? editForm.value.tagsString.split(',').map(t => t.trim()).filter(t => t)
          : undefined,
        billable: editForm.value.billable,
        hourlyRate: editForm.value.hourly_rate || undefined,
        notes: editForm.value.notes || undefined
      };

      await createEntry(newEntryData);

      // Track recent description
      addRecentDescription(editForm.value.project, editForm.value.description);
    } else {
      // Update existing entry
      const updates: any = {
        project: editForm.value.project,
        description: editForm.value.description,
        ticket_id: editForm.value.ticket_id || null,
        billable: editForm.value.billable,
        hourly_rate: editForm.value.hourly_rate || null,
        tags: editForm.value.tagsString
          ? editForm.value.tagsString.split(',').map(t => t.trim()).filter(t => t)
          : [],
        notes: editForm.value.notes || null
      };

      // Update end time if changed
      if (editForm.value.end_time) {
        updates.end_time = new Date(editForm.value.end_time).toISOString();
      }

      await updateEntry(editingEntry.value.id, updates);

      // Track recent description
      addRecentDescription(editForm.value.project, editForm.value.description);
    }

    editingEntry.value = null;
  } catch (err) {
    console.error('Failed to save entry:', err);
  }
}

// Cancel edit
function cancelEdit() {
  editingEntry.value = null;
}

// Helper to format date for datetime-local input
function formatDateTimeLocal(timestamp: string): string {
  const date = new Date(timestamp);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  return `${year}-${month}-${day}T${hours}:${minutes}`;
}

// Delete entry
async function deleteEntryConfirm(entry: any) {
  const confirmMessage = `Delete this time entry?\n\n${entry.data.project}: ${entry.data.description}\n\nFrom: ${formatDate(entry.timestamp)} ${formatTime(entry.timestamp)}`;

  if (confirm(confirmMessage)) {
    try {
      console.log('Deleting entry with ID:', entry.id);
      await deleteEntry(entry.id);
      console.log('Entry deleted successfully');
    } catch (err) {
      console.error('Failed to delete entry:', err);
      const errorMsg = err instanceof Error ? err.message : String(err);
      alert(`Failed to delete entry: ${errorMsg}`);
    }
  }
}

// Format helpers
function formatDate(timestamp: string): string {
  return new Date(timestamp).toLocaleDateString();
}

function formatTime(timestamp: string): string {
  return new Date(timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

// Lifecycle
onMounted(() => {
  loadEntries();
  loadProjects();
  loadRecentDescriptions();
});
</script>

<style scoped>
.time-log-panel {
  padding: 1rem;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-heading);
}

.btn-primary {
  padding: 0.625rem 1.25rem;
  background: var(--bg-button);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  font-size: 0.9375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--bg-button-hover);
}

.filters {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.filter-group {
  flex: 1;
  min-width: 150px;
}

.filter-group select,
.filter-group input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #cbd5e0;
  border-radius: 6px;
  font-size: 0.875rem;
}

.btn-clear,
.btn-export {
  padding: 0.5rem 1rem;
  border: 1px solid #cbd5e0;
  border-radius: 6px;
  background: var(--bg-panel);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.btn-clear:hover {
  background: var(--bg-app);
}

.btn-export {
  background: var(--accent-success);
  color: var(--text-on-accent);
  border-color: var(--accent-success);
}

.btn-export:hover {
  background: var(--accent-success-hover);
}

.summary-bar {
  display: flex;
  gap: 1rem;
  padding: 0.75rem 1rem;
  background: var(--bg-app);
  border-radius: 8px;
  border: 1px solid #e2e8f0;
  flex-wrap: wrap;
}

.summary-item {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.summary-item .label {
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.summary-item strong {
  color: var(--text-heading);
  font-size: 1rem;
}

.entries-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.entry-card {
  padding: 1rem;
  background: var(--bg-panel);
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  transition: all 0.2s;
  position: relative;
}

.entry-card:hover {
  border-color: var(--border-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.entry-card.entry-active {
  border-color: var(--accent-primary);
  background: var(--bg-panel-header);
}

.entry-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 0.5rem;
}

.entry-project {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex: 1;
}

.entry-project strong {
  color: var(--text-heading);
  font-size: 1rem;
}

.entry-ticket {
  font-size: 0.75rem;
  color: var(--text-secondary);
  background: var(--bg-panel-header);
  padding: 0.125rem 0.5rem;
  border-radius: 4px;
}

.entry-duration {
  font-weight: 600;
  color: var(--accent-primary);
  font-size: 1rem;
}

.entry-description {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
}

.entry-meta {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.entry-billable {
  color: var(--accent-success);
  font-weight: 500;
}

.entry-tags {
  color: var(--accent-primary);
}

.entry-actions {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  display: flex;
  gap: 0.25rem;
  opacity: 0;
  transition: opacity 0.2s;
}

.entry-card:hover .entry-actions {
  opacity: 1;
}

.btn-icon {
  padding: 0.25rem 0.5rem;
  background: var(--bg-panel);
  border: 1px solid #e2e8f0;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--bg-app);
}

.btn-delete:hover {
  background: var(--accent-danger); opacity: 0.2;
  border-color: var(--accent-danger);
}

.empty-state,
.loading-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-hint {
  font-size: 0.875rem;
  color: var(--text-muted);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #e2e8f0;
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-message {
  padding: 0.75rem;
  background: var(--accent-danger); opacity: 0.2;
  color: var(--accent-danger);
  border-radius: 6px;
  font-size: 0.875rem;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.modal-dialog {
  background: var(--bg-panel);
  border-radius: 12px;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  max-width: 600px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e2e8f0;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-heading);
  font-weight: 600;
}

.btn-close {
  background: none;
  border: none;
  font-size: 1.5rem;
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

.btn-close:hover {
  background: var(--bg-app);
  color: var(--text-heading);
}

.modal-body {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.875rem;
}

.form-group input[type="text"],
.form-group input[type="number"],
.form-group input[type="datetime-local"],
.form-group textarea {
  width: 100%;
  padding: 0.625rem;
  border: 1px solid #cbd5e0;
  border-radius: 6px;
  font-size: 0.875rem;
  transition: all 0.2s;
  font-family: inherit;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-group textarea {
  resize: vertical;
}

.form-group input[type="checkbox"] {
  width: auto;
  margin-right: 0.5rem;
  cursor: pointer;
}

.form-hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-style: italic;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 1rem;
  margin-top: 1rem;
  border-top: 1px solid #e2e8f0;
}

.btn-primary,
.btn-secondary {
  padding: 0.625rem 1.25rem;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-primary);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-panel);
  color: var(--text-primary);
  border: 1px solid #cbd5e0;
}

.btn-secondary:hover {
  background: var(--bg-app);
}

/* Description with history styles */
.description-with-history {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.recent-descriptions-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #cbd5e0;
  border-radius: 6px;
  font-size: 0.875rem;
  background: var(--bg-app);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.recent-descriptions-select:hover {
  border-color: var(--accent-primary);
}

.recent-descriptions-select:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.recent-descriptions-select option {
  padding: 0.5rem;
}
</style>

