<template>
  <div class="dashboard-overview">
    <div class="overview-header">
      <h1>📊 Dashboards</h1>
      <button @click="createNewDashboard" class="btn-create">➕ Create Dashboard</button>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner">⏳</div>
      <p>Loading dashboards...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <button @click="loadDashboards" class="btn-retry">Retry</button>
    </div>

    <div v-else-if="dashboards.length === 0" class="empty-state">
      <div class="empty-icon">📋</div>
      <h2>No Dashboards Yet</h2>
      <p>Create your first dashboard to get started</p>
      <button @click="createNewDashboard" class="btn-create-large">➕ Create Dashboard</button>
    </div>

    <div v-else class="dashboard-grid">
      <div
        v-for="dashboard in dashboards"
        :key="dashboard.id"
        class="dashboard-card"
        @click="openDashboard(dashboard)"
      >
        <div class="card-header">
          <h3>{{ dashboard.name }}</h3>
          <div class="card-actions">
            <button @click.stop="renameDashboard(dashboard)" class="btn-icon" title="Rename">
              ✏️
            </button>
            <button
              @click.stop="deleteDashboard(dashboard)"
              class="btn-icon btn-danger"
              title="Delete"
            >
              🗑️
            </button>
          </div>
        </div>

        <div class="card-body">
          <div class="dashboard-info">
            <span class="panel-count">{{ dashboard.panels.length }} panels</span>
            <span class="updated-time">{{ formatDate(dashboard.updatedAt) }}</span>
          </div>

          <div class="panel-preview">
            <div
              v-for="panel in dashboard.panels.slice(0, 6)"
              :key="panel.i"
              class="panel-thumbnail"
              :title="panel.title"
            >
              {{ getPanelIcon(panel.type) }}
            </div>
            <div v-if="dashboard.panels.length > 6" class="panel-more">
              +{{ dashboard.panels.length - 6 }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Rename Modal -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ isEditing ? 'Rename Dashboard' : 'Create Dashboard' }}</h2>
          <button @click="closeModal" class="close-button">×</button>
        </div>

        <div class="modal-body">
          <label for="dashboard-name">Dashboard Name</label>
          <input
            id="dashboard-name"
            v-model="dashboardName"
            type="text"
            placeholder="My Dashboard"
            @keyup.enter="saveDashboard"
            ref="nameInput"
          />
        </div>

        <div class="modal-footer">
          <button @click="closeModal" class="btn-secondary">Cancel</button>
          <button @click="saveDashboard" class="btn-primary">
            {{ isEditing ? 'Save' : 'Create' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useDashboardStore } from '@/stores/dashboardStore'
import type { Dashboard } from '@/stores/dashboardStore'

const router = useRouter()
const dashboardStore = useDashboardStore()

const loading = ref(false)
const error = ref<string | null>(null)
const dashboards = ref<Dashboard[]>([])

const showModal = ref(false)
const isEditing = ref(false)
const dashboardName = ref('')
const editingDashboard = ref<Dashboard | null>(null)
const nameInput = ref<HTMLInputElement | null>(null)

onMounted(async () => {
  await loadDashboards()
})

async function loadDashboards() {
  loading.value = true
  error.value = null

  try {
    const result = await dashboardStore.loadDashboards()
    dashboards.value = result
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
    console.error('Failed to load dashboards:', e)
  } finally {
    loading.value = false
  }
}

function createNewDashboard() {
  isEditing.value = false
  dashboardName.value = ''
  editingDashboard.value = null
  showModal.value = true

  nextTick(() => {
    nameInput.value?.focus()
  })
}

function renameDashboard(dashboard: Dashboard) {
  isEditing.value = true
  dashboardName.value = dashboard.name
  editingDashboard.value = dashboard
  showModal.value = true

  nextTick(() => {
    nameInput.value?.focus()
    nameInput.value?.select()
  })
}

async function saveDashboard() {
  if (!dashboardName.value.trim()) {
    alert('Please enter a dashboard name')
    return
  }

  try {
    if (isEditing.value && editingDashboard.value) {
      // Rename existing dashboard
      await dashboardStore.loadDashboard(editingDashboard.value.id)
      await dashboardStore.renameDashboard(dashboardName.value.trim())
      await loadDashboards()
    } else {
      // Create new dashboard
      const newDashboard = dashboardStore.createDashboard(dashboardName.value.trim())
      await dashboardStore.saveDashboard()

      // Navigate to the new dashboard
      router.push(`/dashboard/${newDashboard.id}`)
    }

    closeModal()
  } catch (e) {
    alert(`Failed to save dashboard: ${e}`)
  }
}

async function deleteDashboard(dashboard: Dashboard) {
  if (!confirm(`Delete dashboard "${dashboard.name}"?`)) {
    return
  }

  try {
    await dashboardStore.deleteDashboard(dashboard.id)
    await loadDashboards()
  } catch (e) {
    alert(`Failed to delete dashboard: ${e}`)
  }
}

function openDashboard(dashboard: Dashboard) {
  router.push(`/dashboard/${dashboard.id}`)
}

function closeModal() {
  showModal.value = false
  dashboardName.value = ''
  editingDashboard.value = null
}

function formatDate(timestamp: number): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffDays === 0) return 'Today'
  if (diffDays === 1) return 'Yesterday'
  if (diffDays < 7) return `${diffDays} days ago`

  return date.toLocaleDateString()
}

function getPanelIcon(type: string): string {
  const icons: Record<string, string> = {
    text: '📝',
    chart: '📊',
    table: '📋',
    kanban: '📌',
    markdown: '📄',
    rss: '📰',
    time: '⏱️',
  }
  return icons[type] || '📦'
}
</script>

<style scoped>
.dashboard-overview {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.overview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.overview-header h1 {
  margin: 0;
  font-size: 2rem;
  color: #333;
}

.btn-create,
.btn-create-large {
  padding: 0.75rem 1.5rem;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.2s;
}

.btn-create:hover,
.btn-create-large:hover {
  background: #0056b3;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.spinner {
  font-size: 3rem;
  animation: spin 2s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.error-message {
  color: #dc3545;
  margin-bottom: 1rem;
}

.btn-retry {
  padding: 0.5rem 1rem;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.empty-state {
  padding: 6rem 2rem;
}

.empty-icon {
  font-size: 5rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-state h2 {
  margin: 0 0 0.5rem 0;
  color: #666;
}

.empty-state p {
  margin: 0 0 2rem 0;
  color: #999;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.dashboard-card {
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  padding: 1.5rem;
  cursor: pointer;
  transition: all 0.2s;
}

.dashboard-card:hover {
  border-color: #007bff;
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.2);
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.card-header h3 {
  margin: 0;
  font-size: 1.25rem;
  color: #333;
  flex: 1;
}

.card-actions {
  display: flex;
  gap: 0.5rem;
  opacity: 0;
  transition: opacity 0.2s;
}

.dashboard-card:hover .card-actions {
  opacity: 1;
}

.btn-icon {
  padding: 0.25rem 0.5rem;
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  border-radius: 4px;
  transition: background 0.2s;
}

.btn-icon:hover {
  background: #f0f0f0;
}

.btn-danger:hover {
  background: #fee;
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.dashboard-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.875rem;
  color: #666;
}

.panel-preview {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.panel-thumbnail {
  width: 2.5rem;
  height: 2.5rem;
  background: #f5f5f5;
  border: 1px solid #ddd;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.25rem;
}

.panel-more {
  width: 2.5rem;
  height: 2.5rem;
  background: #e0e0e0;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  font-weight: 600;
  color: #666;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.modal-content {
  background: white;
  border-radius: 8px;
  min-width: 400px;
  max-width: 500px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem 2rem;
  border-bottom: 1px solid #e0e0e0;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #333;
}

.close-button {
  background: none;
  border: none;
  font-size: 2rem;
  color: #666;
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

.close-button:hover {
  background: #f5f5f5;
  color: #333;
}

.modal-body {
  padding: 2rem;
}

.modal-body label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: #333;
}

.modal-body input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.modal-body input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.modal-footer {
  display: flex;
  gap: 1rem;
  padding: 1.5rem 2rem;
  border-top: 1px solid #e0e0e0;
}

.btn-primary,
.btn-secondary {
  flex: 1;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.2s;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover {
  background: #0056b3;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background: #5a6268;
  transform: translateY(-1px);
}
</style>
