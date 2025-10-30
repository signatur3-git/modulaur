<template>
  <div class="dashboard-list">
    <div class="list-header">
      <h2>My Dashboards</h2>
      <div class="header-actions">
        <button @click="showAdapterManager = true" class="btn btn-secondary">
          📡 Data Sources
        </button>
        <button @click="createNew" class="btn btn-primary">➕ New Dashboard</button>
      </div>
    </div>

    <div v-if="loading" class="loading">Loading dashboards...</div>

    <div v-else-if="!dashboards.length" class="empty-state">
      <p class="empty-icon">📊</p>
      <h3>No dashboards yet</h3>
      <p>Create your first dashboard to get started</p>
    </div>

    <div v-else class="dashboard-grid">
      <div
        v-for="dashboard in dashboards"
        :key="dashboard.id"
        class="dashboard-card"
        @click="openDashboard(dashboard.id)"
      >
        <div class="card-header">
          <h3>{{ dashboard.name }}</h3>
          <button
            @click.stop="deleteDashboard(dashboard.id)"
            class="btn-icon btn-danger"
            title="Delete"
          >
            🗑️
          </button>
        </div>
        <div class="card-body">
          <p class="panel-count">{{ dashboard.panels.length }} panels</p>
          <p class="timestamp">Updated {{ formatDate(dashboard.updatedAt) }}</p>
        </div>
      </div>
    </div>

    <!-- New Dashboard Modal -->
    <div v-if="showNewModal" class="modal-overlay" @click="showNewModal = false">
      <div class="modal-content" @click.stop>
        <h3>Create Dashboard</h3>
        <form @submit.prevent="submitNew">
          <div class="form-group">
            <label>Dashboard Name</label>
            <input
              v-model="newDashboardName"
              type="text"
              class="form-control"
              placeholder="My Dashboard"
              autofocus
              required
            />
          </div>
          <div class="form-actions">
            <button type="submit" class="btn btn-primary">Create</button>
            <button type="button" @click="showNewModal = false" class="btn btn-outline">
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Adapter Manager Modal -->
    <AdapterManager v-if="showAdapterManager" @close="showAdapterManager = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDashboardStore } from '../stores/dashboardStore'
import { useDataSourceStore } from '../stores/dataSourceStore'
import AdapterManager from './AdapterManager.vue'

const emit = defineEmits<{
  open: [id: string]
}>()

const dashboardStore = useDashboardStore()
const dataSourceStore = useDataSourceStore()
const dashboards = ref(dashboardStore.dashboards)
const loading = ref(true)
const showNewModal = ref(false)
const showAdapterManager = ref(false)
const newDashboardName = ref('')

onMounted(async () => {
  await dashboardStore.loadDashboards()
  dashboards.value = dashboardStore.dashboards
  loading.value = false

  // Refresh total record counts for all data sources
  await dataSourceStore.refreshTotalRecords()
})

function createNew() {
  newDashboardName.value = ''
  showNewModal.value = true
}

function submitNew() {
  const dashboard = dashboardStore.createDashboard(newDashboardName.value || 'Untitled Dashboard')
  dashboardStore.saveDashboard()
  showNewModal.value = false
  emit('open', dashboard.id)
}

function openDashboard(id: string) {
  emit('open', id)
}

async function deleteDashboard(id: string) {
  if (confirm('Delete this dashboard? This cannot be undone.')) {
    await dashboardStore.deleteDashboard(id)
    dashboards.value = dashboardStore.dashboards
  }
}

function formatDate(timestamp: number): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))

  if (days === 0) return 'today'
  if (days === 1) return 'yesterday'
  if (days < 7) return `${days} days ago`
  return date.toLocaleDateString()
}
</script>

<style scoped>
.dashboard-list {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  background: var(--bg-app);
  min-height: 100vh;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.list-header h2 {
  margin: 0;
  font-size: 2rem;
  color: var(--text-heading);
}

.header-actions {
  display: flex;
  gap: 0.75rem;
}

.loading {
  text-align: center;
  padding: 3rem;
  color: var(--text-muted);
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  color: var(--text-muted);
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.empty-state h3 {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
  color: var(--text-secondary);
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.dashboard-card {
  background: var(--bg-panel);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid var(--border-subtle);
}

.dashboard-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
  border-color: var(--border-color);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.card-header h3 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-heading);
}

.card-body {
  padding: 1.5rem;
}

.panel-count {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.timestamp {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-outline {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-primary);
}

.btn-outline:hover {
  background: var(--bg-panel-header);
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-panel);
  border-radius: 8px;
  padding: 2rem;
  min-width: 400px;
  border: 1px solid var(--border-color);
}

.modal-content h3 {
  margin: 0 0 1.5rem 0;
  color: var(--text-heading);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.form-control {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.875rem;
  background: var(--bg-panel);
  color: var(--text-primary);
}

.form-control:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-actions {
  display: flex;
  gap: 0.75rem;
  margin-top: 1.5rem;
}
</style>
