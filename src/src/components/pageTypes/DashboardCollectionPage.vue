<template>
  <div class="dashboard-collection-page">
    <!-- Dashboard Overview -->
    <div v-if="!selectedDashboardId" class="dashboard-overview">
      <div class="overview-header">
        <h1>📊 {{ pageName || 'Dashboards' }}</h1>
        <button @click="createNewDashboard" class="btn-create">➕ Create Dashboard</button>
      </div>

      <div v-if="loading" class="loading-state">
        <div class="spinner">⏳</div>
        <p>Loading dashboards...</p>
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
          @click="openDashboard(dashboard.id)"
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

    <!-- Dashboard Editor -->
    <div v-else class="dashboard-editor-container">
      <div class="dashboard-header">
        <button @click="backToOverview" class="btn-back">← Back to Dashboards</button>
        <h2>{{ currentDashboard?.name }}</h2>
      </div>
      <DashboardEditorWrapper
        v-if="currentDashboard"
        :dashboard="currentDashboard"
        :collection-id="collectionId"
        @save="handleDashboardSave"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import DashboardEditorWrapper from '@/components/DashboardEditorWrapper.vue'

interface Dashboard {
  id: string
  name: string
  panels: any[]
  createdAt: number
  updatedAt: number
}

const props = defineProps<{
  pageId: string
  pageRoute?: string // Stable route identifier for navigation
  pageName?: string
  config?: {
    dashboards?: Dashboard[]
  }
}>()

const emit = defineEmits<{
  'update:config': [config: any]
}>()

const route = useRoute()
const router = useRouter()

const loading = ref(false)
const dashboards = ref<Dashboard[]>([])
const selectedDashboardId = ref<string | null>(null)
const currentDashboard = ref<Dashboard | null>(null)

const showModal = ref(false)
const isEditing = ref(false)
const dashboardName = ref('')
const editingDashboard = ref<Dashboard | null>(null)
const nameInput = ref<HTMLInputElement | null>(null)

const collectionId = computed(() => props.pageId)

// Load dashboards from page config
function loadDashboards() {
  dashboards.value = props.config?.dashboards || []
}

// Watch for config changes
watch(
  () => props.config,
  () => {
    loadDashboards()
  },
  { deep: true }
)

// Watch for route changes to handle dashboard selection
watch(
  () => route.params.dashboardId,
  dashboardId => {
    if (dashboardId && typeof dashboardId === 'string') {
      selectedDashboardId.value = dashboardId
      const dashboard = dashboards.value.find(d => d.id === dashboardId)
      currentDashboard.value = dashboard || null
    } else {
      selectedDashboardId.value = null
      currentDashboard.value = null
    }
  },
  { immediate: true }
)

onMounted(() => {
  loadDashboards()
})

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

  const updatedDashboards = [...dashboards.value]

  if (isEditing.value && editingDashboard.value) {
    // Rename existing dashboard
    const index = updatedDashboards.findIndex(d => d.id === editingDashboard.value!.id)
    if (index >= 0) {
      updatedDashboards[index] = {
        ...updatedDashboards[index],
        name: dashboardName.value.trim(),
        updatedAt: Date.now(),
      }
    }
  } else {
    // Create new dashboard
    const newDashboard: Dashboard = {
      id: `dashboard_${Date.now()}`,
      name: dashboardName.value.trim(),
      panels: [],
      createdAt: Date.now(),
      updatedAt: Date.now(),
    }
    updatedDashboards.push(newDashboard)

    // Navigate to the new dashboard using stable route
    selectedDashboardId.value = newDashboard.id
    const routeSlug = props.pageRoute || props.pageId
    router.push(`/page/${routeSlug}/dashboard/${newDashboard.id}`)
  }

  // Update config
  emit('update:config', { dashboards: updatedDashboards })
  closeModal()
}

async function deleteDashboard(dashboard: Dashboard) {
  if (!confirm(`Delete dashboard "${dashboard.name}"?`)) {
    return
  }

  const updatedDashboards = dashboards.value.filter(d => d.id !== dashboard.id)
  emit('update:config', { dashboards: updatedDashboards })
}

function openDashboard(dashboardId: string) {
  const routeSlug = props.pageRoute || props.pageId
  router.push(`/page/${routeSlug}/dashboard/${dashboardId}`)
}

function backToOverview() {
  const routeSlug = props.pageRoute || props.pageId
  router.push(`/page/${routeSlug}`)
}

function handleDashboardSave(updatedDashboard: Dashboard) {
  // Update the dashboard in the collection
  const updatedDashboards = dashboards.value.map(d =>
    d.id === updatedDashboard.id ? updatedDashboard : d
  )

  // Emit config update
  emit('update:config', { dashboards: updatedDashboards })

  // Update local state
  dashboards.value = updatedDashboards
  currentDashboard.value = updatedDashboard
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
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`
  if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`
  return `${Math.floor(diffDays / 365)} years ago`
}

function getPanelIcon(type: string): string {
  const icons: Record<string, string> = {
    chart: '📊',
    table: '📋',
    text: '📝',
    markdown: '📄',
    rss: '📰',
    time: '⏱️',
    kanban: '📌',
    snippets: '💾',
    converter: '🔄',
  }
  return icons[type] || '📦'
}
</script>

<style scoped>
.dashboard-collection-page {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.dashboard-overview {
  padding: 20px;
  height: 100%;
  overflow-y: auto;
}

.overview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.overview-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: #333;
}

.btn-create {
  padding: 10px 20px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-create:hover {
  background: #0056b3;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.spinner {
  font-size: 48px;
  animation: spin 2s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 20px;
  opacity: 0.5;
}

.empty-state h2 {
  margin: 0 0 10px 0;
  color: #666;
}

.empty-state p {
  margin: 0 0 20px 0;
  color: #999;
}

.btn-create-large {
  padding: 12px 24px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-create-large:hover {
  background: #0056b3;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.dashboard-card {
  background: white;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.2s;
}

.dashboard-card:hover {
  border-color: #007bff;
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.15);
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 15px;
}

.card-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-actions {
  display: flex;
  gap: 8px;
  margin-left: 10px;
}

.btn-icon {
  padding: 4px 8px;
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 16px;
  border-radius: 4px;
  transition: background 0.2s;
}

.btn-icon:hover {
  background: #f8f9fa;
}

.btn-icon.btn-danger:hover {
  background: #fee;
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.dashboard-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  color: #666;
}

.panel-count {
  font-weight: 500;
}

.updated-time {
  color: #999;
}

.panel-preview {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.panel-thumbnail {
  width: 40px;
  height: 40px;
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}

.panel-more {
  width: 40px;
  height: 40px;
  background: #e9ecef;
  border: 1px solid #dee2e6;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
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
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 500px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #dee2e6;
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.close-button {
  background: none;
  border: none;
  font-size: 28px;
  cursor: pointer;
  color: #999;
  line-height: 1;
  padding: 0;
  width: 32px;
  height: 32px;
}

.close-button:hover {
  color: #333;
}

.modal-body {
  padding: 20px;
}

.modal-body label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #333;
}

.modal-body input {
  width: 100%;
  padding: 10px;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}

.modal-body input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 20px;
  border-top: 1px solid #dee2e6;
}

.btn-secondary {
  padding: 10px 20px;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: #5a6268;
}

.btn-primary {
  padding: 10px 20px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: #0056b3;
}

/* Dashboard Editor Container */
.dashboard-editor-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.dashboard-header {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 15px 20px;
  background: white;
  border-bottom: 1px solid #dee2e6;
}

.btn-back {
  padding: 8px 16px;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-back:hover {
  background: #5a6268;
}

.dashboard-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #333;
}
</style>
