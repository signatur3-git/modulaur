<template>
  <div class="data-source-card">
    <div class="card-header">
      <div class="source-icon">{{ getAdapterIcon(source.adapter_type) }}</div>
      <div class="source-info">
        <h3>{{ source.name }}</h3>
        <p class="source-type">{{ getAdapterName(source.adapter_type) }}</p>
        <p class="source-endpoint">{{ formatEndpoint(source.endpoint) }}</p>
      </div>
      <div class="source-status">
        <span :class="['status-badge', source.enabled ? 'enabled' : 'disabled']">
          {{ source.enabled ? '✓ Enabled' : '✕ Disabled' }}
        </span>
      </div>
    </div>

    <div class="card-stats">
      <div class="stat">
        <span class="stat-label">Last Fetch:</span>
        <span class="stat-value">{{ formatLastFetch(source.last_fetch) }}</span>
      </div>
      <div class="stat">
        <span class="stat-label">Total Records:</span>
        <span
          class="stat-value"
          :title="source.last_fetch_count ? `Last fetch: ${source.last_fetch_count} records` : ''"
        >
          {{ source.total_records || 0 }}
        </span>
      </div>
      <div v-if="source.auto_refresh" class="stat">
        <span class="stat-label">Auto-refresh:</span>
        <span class="stat-value">Every {{ source.refresh_interval }}m</span>
      </div>
    </div>

    <div class="card-actions">
      <button
        @click="handleFetch"
        class="btn-action btn-fetch"
        :disabled="fetching || !isOnline"
        :title="isOnline ? 'Fetch data from source' : 'Cannot fetch while offline'"
      >
        <span v-if="fetching" class="spinner"></span>
        <span v-else-if="!isOnline">⚠️ Offline</span>
        <span v-else>↻ Fetch Now</span>
      </button>
      <button
        v-if="source.adapter_type === 'gitlab'"
        @click="handleDeepFetch"
        class="btn-action btn-deep-fetch"
        :disabled="fetching || !isOnline"
        title="Fetch historical data (10 pages)"
      >
        <span v-if="fetching">⏳</span>
        <span v-else>📚 Deep Fetch</span>
      </button>
      <button @click="$emit('edit', source)" class="btn-action btn-edit" :disabled="fetching">
        ⚙️ Edit
      </button>
      <button
        @click="$emit('delete', source.id)"
        class="btn-action btn-delete"
        :disabled="fetching"
      >
        🗑️ Delete
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useOnlineStatus } from '../composables/useOnlineStatus'
import type { DataSourceConfig } from '../stores/dataSourceStore'

const props = defineProps<{
  source: DataSourceConfig
}>()

const emit = defineEmits<{
  fetch: [id: string]
  deepFetch: [id: string]
  edit: [source: DataSourceConfig]
  delete: [id: string]
}>()

const fetching = ref(false)
const { isOnline } = useOnlineStatus()

async function handleFetch() {
  console.log('🔵 handleFetch() called!')
  console.log('🔵 isOnline:', isOnline.value)
  console.log('🔵 source.id:', props.source.id)
  console.log('🔵 source.name:', props.source.name)

  if (!isOnline.value) {
    console.warn('⚠️ Cannot fetch data while offline')
    return
  }

  console.log('🔵 Setting fetching to true...')
  fetching.value = true
  try {
    console.log('🔵 Emitting fetch event with ID:', props.source.id)
    emit('fetch', props.source.id)
    // Wait a bit to allow the parent to handle the fetch
    await new Promise(resolve => setTimeout(resolve, 500))
  } finally {
    // Reset after a delay to show the loading state
    setTimeout(() => {
      console.log('🔵 Resetting fetching to false')
      fetching.value = false
    }, 1000)
  }
}

async function handleDeepFetch() {
  if (!isOnline.value) {
    console.warn('Cannot fetch data while offline')
    return
  }

  // Just emit - AdapterManager will handle confirmation and actual fetch
  fetching.value = true
  try {
    emit('deepFetch', props.source.id)
    // Wait longer for deep fetch
    await new Promise(resolve => setTimeout(resolve, 1000))
  } finally {
    // Reset after a longer delay
    setTimeout(() => {
      fetching.value = false
    }, 3000)
  }
}

function getAdapterIcon(type: string): string {
  const icons: Record<string, string> = {
    rest_api: '📊',
    gitlab: '🔧',
  }
  return icons[type] || '📡'
}

function getAdapterName(type: string): string {
  const names: Record<string, string> = {
    rest_api: 'REST API',
    gitlab: 'GitLab',
  }
  return names[type] || type
}

function formatEndpoint(endpoint: string): string {
  try {
    const url = new URL(endpoint)
    return url.hostname + url.pathname
  } catch {
    return endpoint
  }
}

function formatLastFetch(timestamp?: string): string {
  if (!timestamp) return 'Never'

  try {
    const date = new Date(timestamp)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)

    if (diffMins < 1) return 'Just now'
    if (diffMins < 60) return `${diffMins}m ago`

    const diffHours = Math.floor(diffMins / 60)
    if (diffHours < 24) return `${diffHours}h ago`

    const diffDays = Math.floor(diffHours / 24)
    return `${diffDays}d ago`
  } catch {
    return 'Unknown'
  }
}
</script>

<style scoped>
.data-source-card {
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  transition: all 0.2s;
}

.data-source-card:hover {
  border-color: var(--border-color);
  box-shadow: var(--panel-shadow);
}

.source-icon {
  font-size: 40px;
  flex-shrink: 0;
}

.source-info {
  flex: 1;
}

.source-info h3 {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-heading);
}

.source-type {
  margin: 0 0 4px 0;
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
}

.source-endpoint {
  margin: 0;
  font-size: 12px;
  color: var(--text-muted);
  font-family: 'Courier New', monospace;
}

.source-status {
  flex-shrink: 0;
}

.status-badge {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.enabled {
  background: color-mix(in srgb, var(--accent-success) 18%, transparent);
  color: var(--accent-success);
}

.status-badge.disabled {
  background: color-mix(in srgb, var(--accent-danger) 18%, transparent);
  color: var(--accent-danger);
}

.card-stats {
  display: flex;
  gap: 24px;
  padding: 12px 0;
  border-top: 1px solid var(--border-subtle);
  border-bottom: 1px solid var(--border-subtle);
  margin-bottom: 16px;
}

.stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: 500;
}

.stat-value {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 600;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.btn-action {
  flex: 1;
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.btn-fetch {
  color: var(--accent-primary);
  border-color: var(--accent-primary);
}

.btn-fetch:hover:not(:disabled) {
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

.btn-fetch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-deep-fetch {
  color: var(--accent-warning);
  border-color: var(--accent-warning);
  flex: 0 0 auto;
  min-width: 120px;
}

.btn-deep-fetch:hover:not(:disabled) {
  background: var(--accent-warning);
  color: var(--text-on-accent);
}

.btn-deep-fetch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-edit {
  color: var(--text-secondary);
}

.btn-edit:hover {
  background: var(--border-subtle);
  border-color: var(--border-color);
}

.btn-delete {
  color: var(--accent-danger);
  border-color: color-mix(in srgb, var(--accent-danger) 40%, transparent);
}

.btn-delete:hover {
  background: color-mix(in srgb, var(--accent-danger) 18%, transparent);
  border-color: var(--accent-danger);
}

.spinner {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
  margin-right: 6px;
  vertical-align: middle;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
