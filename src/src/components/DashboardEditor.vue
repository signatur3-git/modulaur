<template>
  <div class="dashboard-editor">
    <div class="editor-toolbar">
      <div class="toolbar-left">
        <div class="dashboard-name-container">
          <h2
            v-if="!isRenamingDashboard"
            @click="startRename"
            class="dashboard-name"
            :title="'Click to rename'"
          >
            {{ dashboard?.name || 'Untitled Dashboard' }}
            <span class="rename-icon">✏️</span>
          </h2>
          <input
            v-else
            ref="renameInput"
            v-model="newDashboardName"
            @blur="finishRename"
            @keyup.enter="finishRename"
            @keyup.esc="cancelRename"
            class="dashboard-name-input"
            type="text"
            placeholder="Dashboard name"
          />
        </div>
      </div>
      <div class="toolbar-right">
        <button
          v-if="!dashboardStore.isEditing"
          @click="dashboardStore.isEditing = true"
          class="btn btn-primary"
        >
          ✏️ Edit
        </button>
        <template v-else>
          <button @click="showAddPanel = true" class="btn btn-secondary">➕ Add Panel</button>
          <button @click="saveDashboard" class="btn btn-success">💾 Save</button>
          <button @click="cancelEdit" class="btn btn-outline">Cancel</button>
        </template>
      </div>
    </div>

    <div class="editor-content">
      <grid-layout
        v-if="layout.length"
        :layout="layoutItems"
        :col-num="12"
        :row-height="gridRowHeight"
        :is-draggable="dashboardStore.isEditing"
        :is-resizable="dashboardStore.isEditing"
        :vertical-compact="true"
        :use-css-transforms="true"
        @layout-updated="onLayoutUpdated"
      >
        <grid-item
          v-for="panel in layoutItems"
          :key="panel.i"
          :x="panel.x"
          :y="panel.y"
          :w="panel.w"
          :h="panel.h"
          :i="panel.i"
        >
          <PanelWrapper
            :panel="getPanelData(panel.i)"
            :is-editing="dashboardStore.isEditing"
            @edit="editPanel"
            @remove="removePanel"
          />
        </grid-item>
      </grid-layout>

      <div v-if="!layout.length" class="empty-state">
        <div class="empty-content">
          <p class="empty-icon">📊</p>
          <h3>No panels yet</h3>
          <p>Click "Add Panel" to get started</p>
        </div>
      </div>
    </div>

    <!-- Add Panel Modal -->
    <div v-if="showAddPanel" class="modal-overlay" @click="showAddPanel = false">
      <div class="modal-content" @click.stop>
        <h3>Add Panel</h3>
        <div class="panel-types">
          <!-- All panels are now loaded dynamically from plugins -->
          <button
            v-for="pluginPanel in pluginPanels"
            :key="pluginPanel.id"
            @click="addPanelOfType(pluginPanel.id)"
            class="panel-type-btn"
            :class="{ 'plugin-panel': pluginPanel.pluginName !== 'system' }"
          >
            <span class="icon">{{ pluginPanel.icon }}</span>
            <span>{{ pluginPanel.name }}</span>
            <span v-if="pluginPanel.pluginName !== 'system'" class="plugin-badge">Plugin</span>
          </button>
        </div>
        <button @click="showAddPanel = false" class="btn btn-outline">Cancel</button>
      </div>
    </div>

    <!-- Edit Panel Modal -->
    <div v-if="editingPanel" class="modal-overlay" @click="editingPanel = null">
      <div class="modal-content" @click.stop>
        <h3>Edit Panel</h3>
        <form @submit.prevent="savePanel">
          <div class="form-group">
            <label>Title</label>
            <input v-model="editingPanel.title" type="text" class="form-control" />
          </div>

          <!-- Dynamic configuration form based on panel type -->
          <DynamicConfigForm
            v-if="configSchema"
            :schema="configSchema"
            :config="editingPanel.config"
          />

          <div class="form-actions">
            <button type="submit" class="btn btn-primary">Save</button>
            <button type="button" @click="editingPanel = null" class="btn btn-outline">
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { GridLayout, GridItem } from 'grid-layout-plus'
import { useDashboardStore, type Panel } from '../stores/dashboardStore'
import { useThemeStore } from '../stores/themeStore'
import PanelWrapper from './PanelWrapper.vue'
import DynamicConfigForm from './DynamicConfigForm.vue'
import { pluginLoader } from '../services/pluginLoader'

const dashboardStore = useDashboardStore()
const themeStore = useThemeStore()

const dashboard = computed(() => dashboardStore.currentDashboard)
const showAddPanel = ref(false)
const editingPanel = ref<Panel | null>(null)

// Dashboard rename support
const isRenamingDashboard = ref(false)
const newDashboardName = ref('')
const renameInput = ref<HTMLInputElement | null>(null)

async function startRename() {
  isRenamingDashboard.value = true
  newDashboardName.value = dashboard.value?.name || ''
  await nextTick()
  renameInput.value?.focus()
  renameInput.value?.select()
}

function finishRename() {
  if (newDashboardName.value.trim() && newDashboardName.value !== dashboard.value?.name) {
    dashboardStore.renameDashboard(newDashboardName.value)
  }
  isRenamingDashboard.value = false
}

function cancelRename() {
  isRenamingDashboard.value = false
  newDashboardName.value = ''
}

// Theme support
const gridRowHeight = computed(() => themeStore.theme.rowHeight)

// Get available panel pluginPanels
const pluginPanels = computed(() => pluginLoader.getAvailablePanelTypes())

// Get configuration schema for the currently editing panel
const configSchema = computed(() => {
  if (editingPanel.value) {
    return pluginLoader.getPanelConfigSchema(editingPanel.value.type)
  }
  return undefined
})

const layout = computed(() => dashboard.value?.panels || [])

const layoutItems = computed(() =>
  layout.value.map(p => ({
    i: p.i,
    x: p.x,
    y: p.y,
    w: p.w,
    h: p.h,
  }))
)

function getPanelData(panelId: string): Panel {
  return (
    layout.value.find(p => p.i === panelId) || {
      i: panelId,
      x: 0,
      y: 0,
      w: 4,
      h: 3,
      type: 'text',
      title: 'Unknown Panel',
      config: {},
    }
  )
}

function onLayoutUpdated(
  newLayout: Array<{ i: string; x: number; y: number; w: number; h: number }>
) {
  dashboardStore.updateLayout(newLayout)
}

function addPanelOfType(type: string) {
  if (!dashboard.value) {
    console.error('❌ No dashboard loaded!')
    alert('No dashboard loaded. Please create a dashboard first.')
    return
  }

  if (!dashboardStore.isEditing) {
    console.error('❌ Not in edit mode!')
    alert('Please click Edit button first.')
    return
  }

  dashboardStore.addPanel(type)
  showAddPanel.value = false
}

function editPanel(panelId: string) {
  const panel = layout.value.find(p => p.i === panelId)
  if (panel) {
    editingPanel.value = { ...panel, config: { ...panel.config } }
  }
}

function savePanel() {
  if (editingPanel.value) {
    dashboardStore.updatePanel(editingPanel.value.i, editingPanel.value)
    editingPanel.value = null
  }
}

function removePanel(panelId: string) {
  if (confirm('Remove this panel?')) {
    dashboardStore.removePanel(panelId)
  }
}

function saveDashboard() {
  dashboardStore.saveDashboard()
  dashboardStore.isEditing = false
}

function cancelEdit() {
  if (dashboard.value) {
    dashboardStore.loadDashboard(dashboard.value.id)
  }
  dashboardStore.isEditing = false
}
</script>

<style scoped>
.dashboard-editor {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-app);
}

.editor-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--toolbar-padding);
  background: var(--bg-toolbar);
  border-bottom: 1px solid var(--border-color);
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.dashboard-name-container {
  position: relative;
}

.dashboard-name {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-heading);
  cursor: pointer;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.dashboard-name:hover {
  background: var(--bg-panel-header);
}

.rename-icon {
  font-size: 0.875rem;
  opacity: 0;
  transition: opacity 0.2s;
}

.dashboard-name:hover .rename-icon {
  opacity: 0.6;
}

.dashboard-name-input {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-heading);
  background: var(--bg-panel);
  border: 2px solid var(--accent-primary);
  border-radius: 4px;
  padding: 0.25rem 0.5rem;
  outline: none;
  min-width: 250px;
}

.dashboard-name-input:focus {
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

.toolbar-right {
  display: flex;
  gap: 0.75rem;
}

.editor-content {
  flex: 1;
  overflow: auto;
  padding: var(--panel-gap);
  position: relative;
}

.empty-state {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-content {
  text-align: center;
  color: var(--text-muted);
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.empty-content h3 {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
  color: var(--text-secondary);
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
  background: var(--bg-button);
  color: var(--text-on-accent);
}

.btn-primary:hover {
  background: var(--bg-button-hover);
}

.btn-secondary {
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
}

.btn-secondary:hover {
  background: var(--bg-button-secondary-hover);
}

.btn-success {
  background: var(--accent-success);
  color: var(--text-on-accent);
}

.btn-success:hover {
  background: var(--accent-success-hover);
}

.btn-outline {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-primary);
}

.btn-outline:hover {
  background: var(--bg-panel-header);
}

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
  background: var(--bg-modal);
  border-radius: 8px;
  padding: 2rem;
  min-width: 400px;
  max-width: 600px;
  border: 1px solid var(--border-color);
}

.modal-content h3 {
  margin: 0 0 1.5rem 0;
  color: var(--text-heading);
}

.panel-types {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.panel-type-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1.5rem;
  background: #f8f9fa;
  border: 2px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
}

.panel-type-btn:hover {
  background: #e9ecef;
  border-color: #3498db;
}

.panel-type-btn.plugin-panel {
  background: linear-gradient(135deg, #667eea15 0%, #764ba215 100%);
}

.panel-type-btn.plugin-panel:hover {
  border-color: #667eea;
}

.panel-type-btn .icon {
  font-size: 2rem;
}

.plugin-badge {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  background: #667eea;
  color: white;
  font-size: 0.65rem;
  padding: 0.15rem 0.4rem;
  border-radius: 8px;
  font-weight: 600;
  text-transform: uppercase;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #2c3e50;
}

.form-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: #6c757d;
}

.form-control {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 0.875rem;
}

.form-control:focus {
  outline: none;
  border-color: #3498db;
}

.form-actions {
  display: flex;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.form-actions .btn {
  flex: 1;
}
</style>
