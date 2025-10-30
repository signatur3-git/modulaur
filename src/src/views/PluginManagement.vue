<template>
  <div class="plugin-management">
    <div class="page-header">
      <h1>🔌 Plugin Management</h1>
      <p>Manage installed plugins and configure their settings</p>
    </div>

    <div class="plugin-stats">
      <div class="stat-card">
        <div class="stat-icon">📦</div>
        <div class="stat-content">
          <div class="stat-value">{{ totalPlugins }}</div>
          <div class="stat-label">Total Plugins</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">✅</div>
        <div class="stat-content">
          <div class="stat-value">{{ enabledPlugins }}</div>
          <div class="stat-label">Enabled</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">⚙️</div>
        <div class="stat-content">
          <div class="stat-value">{{ backendPlugins }}</div>
          <div class="stat-label">Backend</div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">🎨</div>
        <div class="stat-content">
          <div class="stat-value">{{ frontendPlugins }}</div>
          <div class="stat-label">Frontend</div>
        </div>
      </div>
    </div>

    <div class="plugins-list">
      <div v-if="loading" class="loading">Loading plugins...</div>

      <div v-else-if="plugins.length === 0" class="empty-state">
        <div class="empty-icon">📦</div>
        <h3>No Plugins Installed</h3>
        <p>Install plugins to extend the platform with custom adapters and features.</p>
      </div>

      <div v-else class="plugin-cards">
        <div
          v-for="plugin in plugins"
          :key="plugin.name"
          class="plugin-card"
          :class="{ disabled: !isPluginEnabled(plugin.name) }"
        >
          <div class="plugin-header">
            <div class="plugin-info">
              <h3>
                {{ plugin.name }}
                <span v-if="plugin.adapter_type" class="adapter-badge">
                  {{ plugin.adapter_type }}
                </span>
              </h3>
              <div class="plugin-version">v{{ plugin.version }}</div>
            </div>

            <div class="plugin-toggle">
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  :checked="isPluginEnabled(plugin.name)"
                  @change="togglePlugin(plugin.name)"
                />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>

          <div class="plugin-description">
            {{ plugin.description }}
          </div>

          <div class="plugin-meta">
            <div class="meta-item">
              <span class="meta-icon">👤</span>
              <span>{{ plugin.author }}</span>
            </div>
            <div v-if="plugin.capabilities?.length" class="meta-item">
              <span class="meta-icon">⚡</span>
              <span>{{ plugin.capabilities.join(', ') }}</span>
            </div>
          </div>

          <div class="plugin-features">
            <span v-if="hasBackend(plugin)" class="feature-badge backend">
              <span class="badge-icon">⚙️</span>
              Backend
            </span>
            <span v-if="plugin.frontend" class="feature-badge frontend">
              <span class="badge-icon">🎨</span>
              Frontend
            </span>
            <span v-if="plugin.adapter_type" class="feature-badge adapter">
              <span class="badge-icon">🔌</span>
              Adapter
            </span>
          </div>

          <div class="plugin-actions">
            <button class="btn-secondary" @click="viewPluginDetails(plugin)">
              <span class="btn-icon">ℹ️</span>
              Details
            </button>

            <button v-if="plugin.frontend" class="btn-secondary" @click="configurePlugin(plugin)">
              <span class="btn-icon">⚙️</span>
              Configure
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Plugin Details Modal -->
    <div v-if="selectedPlugin" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ selectedPlugin.name }}</h2>
          <button class="btn-close" @click="closeModal">×</button>
        </div>

        <div class="modal-body">
          <div class="detail-section">
            <h3>Information</h3>
            <div class="detail-grid">
              <div class="detail-item">
                <div class="detail-label">Name</div>
                <div class="detail-value">{{ selectedPlugin.name }}</div>
              </div>
              <div class="detail-item">
                <div class="detail-label">Version</div>
                <div class="detail-value">{{ selectedPlugin.version }}</div>
              </div>
              <div class="detail-item">
                <div class="detail-label">Author</div>
                <div class="detail-value">{{ selectedPlugin.author }}</div>
              </div>
              <div v-if="selectedPlugin.adapter_type" class="detail-item">
                <div class="detail-label">Adapter Type</div>
                <div class="detail-value">{{ selectedPlugin.adapter_type }}</div>
              </div>
            </div>
          </div>

          <div v-if="selectedPlugin.capabilities?.length" class="detail-section">
            <h3>Capabilities</h3>
            <div class="capabilities-list">
              <span v-for="cap in selectedPlugin.capabilities" :key="cap" class="capability-tag">
                {{ cap }}
              </span>
            </div>
          </div>

          <div class="detail-section">
            <h3>Description</h3>
            <p>{{ selectedPlugin.description }}</p>
          </div>

          <div class="detail-section">
            <h3>Features</h3>
            <ul class="features-list">
              <li v-if="hasBackend(selectedPlugin)">✅ Backend plugin (WASM)</li>
              <li v-if="selectedPlugin.frontend">✅ Frontend components</li>
              <li v-if="selectedPlugin.adapter_type">
                ✅ Data adapter for {{ selectedPlugin.adapter_type }}
              </li>
            </ul>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" @click="closeModal">Close</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { PluginManifest } from '@/types/plugin'

const plugins = ref<PluginManifest[]>([])
const loading = ref(true)
const selectedPlugin = ref<PluginManifest | null>(null)
const disabledPlugins = ref<Set<string>>(new Set())

// Load disabled plugins from localStorage
const loadDisabledPlugins = () => {
  const stored = localStorage.getItem('disabledPlugins')
  if (stored) {
    try {
      const disabled = JSON.parse(stored)
      disabledPlugins.value = new Set(disabled)
    } catch (e) {
      console.error('Failed to load disabled plugins:', e)
    }
  }
}

// Save disabled plugins to localStorage
const saveDisabledPlugins = () => {
  localStorage.setItem('disabledPlugins', JSON.stringify(Array.from(disabledPlugins.value)))
}

// Computed stats
const totalPlugins = computed(() => plugins.value.length)
const enabledPlugins = computed(() => plugins.value.filter(p => isPluginEnabled(p.name)).length)
const backendPlugins = computed(() => plugins.value.filter(p => hasBackend(p)).length)
const frontendPlugins = computed(() => plugins.value.filter(p => p.frontend).length)

// Helper functions
const hasBackend = (plugin: PluginManifest) => {
  return plugin.capabilities && plugin.capabilities.length > 0
}

const isPluginEnabled = (pluginName: string) => {
  return !disabledPlugins.value.has(pluginName)
}

// Load plugins
const loadPlugins = async () => {
  try {
    loading.value = true
    plugins.value = await invoke<PluginManifest[]>('get_installed_plugins')
    console.log('✅ Loaded plugins:', plugins.value)
  } catch (error) {
    console.error('Failed to load plugins:', error)
  } finally {
    loading.value = false
  }
}

// Toggle plugin enabled state
const togglePlugin = (pluginName: string) => {
  if (disabledPlugins.value.has(pluginName)) {
    disabledPlugins.value.delete(pluginName)
    console.log(`✅ Enabled plugin: ${pluginName}`)
  } else {
    disabledPlugins.value.add(pluginName)
    console.log(`❌ Disabled plugin: ${pluginName}`)
  }
  saveDisabledPlugins()

  // Note: Full enable/disable would require app reload
  alert('Plugin state changed. Please restart the app for changes to take effect.')
}

// View plugin details
const viewPluginDetails = (plugin: PluginManifest) => {
  selectedPlugin.value = plugin
}

// Configure plugin
const configurePlugin = (plugin: PluginManifest) => {
  alert(`Configuration UI for ${plugin.name} - Coming soon!`)
  // TODO: Implement plugin-specific configuration
}

// Close modal
const closeModal = () => {
  selectedPlugin.value = null
}

// Initialize
onMounted(() => {
  loadDisabledPlugins()
  loadPlugins()
})
</script>

<style scoped>
.plugin-management {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 2rem;
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  color: #2c3e50;
}

.page-header p {
  margin: 0;
  color: #7f8c8d;
}

/* Stats Cards */
.plugin-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.stat-icon {
  font-size: 2rem;
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  color: #2c3e50;
}

.stat-label {
  font-size: 0.875rem;
  color: #7f8c8d;
}

/* Plugin Cards */
.plugins-list {
  background: white;
  border-radius: 8px;
  padding: 2rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  max-height: calc(100vh - 400px); /* Leave room for header and stats */
  overflow-y: auto;
}

.loading,
.empty-state {
  text-align: center;
  padding: 3rem;
  color: #7f8c8d;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.plugin-cards {
  display: grid;
  gap: 1.5rem;
}

.plugin-card {
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.plugin-card:hover {
  border-color: #3498db;
  box-shadow: 0 4px 8px rgba(52, 152, 219, 0.2);
}

.plugin-card.disabled {
  opacity: 0.6;
  background: #f8f9fa;
}

.plugin-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.plugin-info h3 {
  margin: 0 0 0.25rem 0;
  font-size: 1.25rem;
  color: #2c3e50;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.adapter-badge {
  display: inline-block;
  background: #3498db;
  color: white;
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-weight: normal;
}

.plugin-version {
  font-size: 0.875rem;
  color: #7f8c8d;
}

/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 26px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.4s;
  border-radius: 26px;
}

.toggle-slider:before {
  position: absolute;
  content: '';
  height: 20px;
  width: 20px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.4s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: #2ecc71;
}

input:checked + .toggle-slider:before {
  transform: translateX(24px);
}

.plugin-description {
  color: #555;
  margin-bottom: 1rem;
  line-height: 1.5;
}

.plugin-meta {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 1rem;
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: #666;
}

.meta-icon {
  font-size: 1rem;
}

.plugin-features {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
  flex-wrap: wrap;
}

.feature-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  font-size: 0.875rem;
  font-weight: 500;
}

.feature-badge.backend {
  background: #e8f5e9;
  color: #2e7d32;
}

.feature-badge.frontend {
  background: #e3f2fd;
  color: #1565c0;
}

.feature-badge.adapter {
  background: #fff3e0;
  color: #e65100;
}

.badge-icon {
  font-size: 1rem;
}

.plugin-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-secondary {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: #e0e0e0;
  border-color: #ccc;
}

.btn-icon {
  font-size: 1rem;
}

/* Modal */
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
  padding: 2rem;
}

.modal-content {
  background: white;
  border-radius: 8px;
  max-width: 600px;
  width: 100%;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e0e0e0;
}

.modal-header h2 {
  margin: 0;
  color: #2c3e50;
}

.btn-close {
  background: none;
  border: none;
  font-size: 2rem;
  cursor: pointer;
  color: #7f8c8d;
  padding: 0;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-close:hover {
  color: #2c3e50;
}

.modal-body {
  padding: 1.5rem;
}

.detail-section {
  margin-bottom: 2rem;
}

.detail-section:last-child {
  margin-bottom: 0;
}

.detail-section h3 {
  margin: 0 0 1rem 0;
  color: #2c3e50;
  font-size: 1.125rem;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.detail-item {
  padding: 0.75rem;
  background: #f8f9fa;
  border-radius: 4px;
}

.detail-label {
  font-size: 0.75rem;
  color: #7f8c8d;
  text-transform: uppercase;
  margin-bottom: 0.25rem;
}

.detail-value {
  font-size: 1rem;
  color: #2c3e50;
  font-weight: 500;
}

.capabilities-list {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.capability-tag {
  padding: 0.5rem 1rem;
  background: #e3f2fd;
  color: #1565c0;
  border-radius: 4px;
  font-size: 0.875rem;
}

.features-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.features-list li {
  padding: 0.5rem 0;
  border-bottom: 1px solid #f0f0f0;
}

.features-list li:last-child {
  border-bottom: none;
}

.modal-footer {
  padding: 1.5rem;
  border-top: 1px solid #e0e0e0;
  display: flex;
  justify-content: flex-end;
}
</style>
