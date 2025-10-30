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
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 20px;
  transition: all 0.2s;
}

.data-source-card:hover {
  border-color: #d1d5db;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 16px;
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
  color: #111827;
}

.source-type {
  margin: 0 0 4px 0;
  font-size: 13px;
  color: #6b7280;
  font-weight: 500;
}

.source-endpoint {
  margin: 0;
  font-size: 12px;
  color: #9ca3af;
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
  background: #d1fae5;
  color: #065f46;
}

.status-badge.disabled {
  background: #fee2e2;
  color: #991b1b;
}

.card-stats {
  display: flex;
  gap: 24px;
  padding: 12px 0;
  border-top: 1px solid #f3f4f6;
  border-bottom: 1px solid #f3f4f6;
  margin-bottom: 16px;
}

.stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  font-size: 11px;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: 500;
}

.stat-value {
  font-size: 14px;
  color: #111827;
  font-weight: 600;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.btn-action {
  flex: 1;
  padding: 8px 16px;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  background: white;
}

.btn-fetch {
  color: #3b82f6;
  border-color: #3b82f6;
}

.btn-fetch:hover:not(:disabled) {
  background: #3b82f6;
  color: white;
}

.btn-fetch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-deep-fetch {
  color: #8b5cf6;
  border-color: #8b5cf6;
  flex: 0 0 auto;
  min-width: 120px;
}

.btn-deep-fetch:hover:not(:disabled) {
  background: #8b5cf6;
  color: white;
}

.btn-deep-fetch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-edit {
  color: #6b7280;
}

.btn-edit:hover {
  background: #f3f4f6;
  border-color: #9ca3af;
}

.btn-delete {
  color: #dc2626;
  border-color: #fecaca;
}

.btn-delete:hover {
  background: #fee2e2;
  border-color: #dc2626;
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
