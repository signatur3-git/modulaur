<template>
  <div class="app-menu">
    <!-- Menu Button -->
    <button @click="toggleMenu" class="menu-button" title="App Menu">⚙️</button>

    <!-- Menu Dropdown -->
    <div v-if="isOpen" class="menu-dropdown">
      <div class="menu-header">
        <h3>Settings</h3>
        <button @click="closeMenu" class="close-button">×</button>
      </div>

      <div class="menu-section">
        <h4>Connection</h4>

        <!-- Manual Offline Mode Toggle -->
        <label class="menu-item toggle-item">
          <div class="item-info">
            <span class="item-label">Manual Offline Mode</span>
            <span class="item-description"> Test offline behavior without disconnecting </span>
          </div>
          <input
            type="checkbox"
            v-model="settingsStore.manualOfflineMode"
            @change="onManualOfflineChange"
            class="toggle-input"
          />
          <span class="toggle-slider"></span>
        </label>

        <!-- Connection Status Info -->
        <div class="menu-item info-item">
          <div class="status-info">
            <div class="status-row">
              <span class="status-label">Browser Status:</span>
              <span :class="['status-value', browserOnline ? 'online' : 'offline']">
                {{ browserOnline ? '🟢 Online' : '🔴 Offline' }}
              </span>
            </div>
            <div class="status-row">
              <span class="status-label">Effective Status:</span>
              <span :class="['status-value', isOnline ? 'online' : 'offline']">
                {{ isOnline ? '🟢 Online' : '🔴 Offline' }}
              </span>
            </div>
            <div v-if="settingsStore.manualOfflineMode" class="status-note">
              ⚠️ Manual offline mode active
            </div>
          </div>
        </div>
      </div>

      <div class="menu-section">
        <h4>Debug</h4>

        <label class="menu-item toggle-item">
          <div class="item-info">
            <span class="item-label">Show Debug Info</span>
            <span class="item-description"> Display additional debug information </span>
          </div>
          <input
            type="checkbox"
            v-model="settingsStore.showDebugInfo"
            @change="settingsStore.autoSave()"
            class="toggle-input"
          />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <!-- M5.1: Security Section -->
      <div class="menu-section">
        <h4>🔒 Security</h4>

        <div v-if="!credentialsMigrated" class="menu-item warning-item">
          <span class="warning-icon">⚠️</span>
          <span class="warning-text">Credentials not encrypted</span>
        </div>

        <div v-if="credentialsMigrated" class="menu-item success-item">
          <span class="success-icon">✅</span>
          <span class="success-text">Credentials encrypted</span>
          <div class="info-text">Migrated: {{ migrationDate }}</div>
        </div>

        <button
          v-if="!credentialsMigrated"
          @click="migrateCredentials"
          class="menu-button-action btn-primary"
          :disabled="isMigrating"
        >
          <span v-if="isMigrating">🔄 Encrypting...</span>
          <span v-else>🔒 Encrypt Credentials</span>
        </button>

        <button
          v-if="credentialsMigrated && showRollback"
          @click="confirmRollback"
          class="menu-button-action btn-warning"
          :disabled="isMigrating"
        >
          ↩️ Rollback Encryption
        </button>

        <p v-if="!credentialsMigrated" class="help-text">
          Encrypt API tokens using AES-256 encryption
        </p>
      </div>

      <div class="menu-section">
        <h4>Data Management</h4>

        <button @click="openOfflineBrowser" class="menu-button-action">
          📊 Browse Offline Data
        </button>

        <button @click="openDatabaseManagement" class="menu-button-action">
          💾 Database Manager
        </button>

        <div class="menu-item info-item">
          <div class="stats-info">
            <div class="stats-row">
              <span class="stats-label">Total Records:</span>
              <span class="stats-value">{{ totalRecords }}</span>
            </div>
            <div class="stats-row">
              <span class="stats-label">Database Size:</span>
              <span class="stats-value">{{ dbSize }}</span>
            </div>
            <div v-if="lastCleanup" class="stats-row">
              <span class="stats-label">Last Cleanup:</span>
              <span class="stats-value">{{ formatCleanupTime(lastCleanup) }}</span>
            </div>
          </div>
        </div>

        <button @click="runCleanup" class="menu-button-action" :disabled="isCleaningUp">
          <span v-if="isCleaningUp">🔄 Cleaning...</span>
          <span v-else>🧹 Clean Old Data (30d)</span>
        </button>

        <div class="menu-item-group">
          <button
            @click="deleteByType('gitlab_pipeline')"
            class="menu-button-action btn-warning"
            :disabled="isCleaningUp"
          >
            🗑️ Delete All Pipelines
          </button>
          <button
            @click="deleteByType('gitlab_job')"
            class="menu-button-action btn-warning"
            :disabled="isCleaningUp"
          >
            🗑️ Delete All Jobs
          </button>
        </div>
      </div>

      <div class="menu-section">
        <h4>🔌 Plugins</h4>

        <button @click="openPluginManagement" class="menu-button-action">🔌 Manage Plugins</button>
      </div>

      <div class="menu-section">
        <h4>🎨 Theme</h4>

        <div class="menu-item">
          <div class="item-info">
            <span class="item-label">Theme</span>
            <span class="item-description"> Choose your preferred theme </span>
          </div>
          <select v-model="currentTheme" class="theme-selector">
            <option v-for="theme in availableThemes" :key="theme.name" :value="theme.name">
              {{ theme.displayName }}
            </option>
          </select>
        </div>
      </div>

      <div class="menu-section">
        <h4>📄 Page Management</h4>

        <button @click="openPageManagementModal" class="menu-button-action">📄 Manage Pages</button>

        <div class="menu-item info-item">
          <div class="stats-info">
            <div class="stats-row">
              <span class="stats-label">Total Pages:</span>
              <span class="stats-value">{{ totalPages }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="menu-footer">
        <span class="version-info">v1.0.0 - M6</span>
      </div>
    </div>

    <!-- Backdrop -->
    <div v-if="isOpen" class="menu-backdrop" @click="closeMenu"></div>

    <!-- Page Management Modal -->
    <PageManagementModal :is-open="showPageManagement" @close="closePageManagement" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useSettingsStore } from '../stores/settingsStore'
import { usePageStore } from '../stores/pageStore'
import { useOnlineStatus } from '../composables/useOnlineStatus'
import { useDataManagement } from '../composables/useDataManagement'
import {
  migrateDataSourcesToSecureStorage,
  areCredentialsMigrated,
  rollbackMigration,
} from '../composables/useCredentialMigration'
import { useThemeStore } from '../stores/themeStore'
import PageManagementModal from './PageManagementModal.vue'

const emit = defineEmits<{
  openOfflineBrowser: []
  openPluginManagement: []
  openDatabaseManagement: []
}>()

const settingsStore = useSettingsStore()
const pageStore = usePageStore()
const { isOnline, browserOnline } = useOnlineStatus()
const {
  isCleaningUp,
  lastCleanup,
  totalRecords,
  getStats,
  cleanupOldData,
  deleteRecordsByType,
  getFormattedSize,
} = useDataManagement()

const themeStore = useThemeStore()

const isOpen = ref(false)
const dbSize = ref('0 B')
const showPageManagement = ref(false)
const totalPages = computed(() => pageStore.pages.length)

// M5.1: Security state
const isMigrating = ref(false)
const credentialsMigrated = ref(areCredentialsMigrated())
const showRollback = ref(false) // Debug option
const migrationDate = computed(() => {
  const date = localStorage.getItem('credentials-migrated-date')
  if (!date) return ''
  return new Date(date).toLocaleDateString()
})

onMounted(async () => {
  // Load database stats when menu opens
  try {
    await getStats()
    dbSize.value = getFormattedSize()
  } catch (error) {
    console.error('Failed to load database stats:', error)
  }
})

function toggleMenu() {
  isOpen.value = !isOpen.value

  // Refresh stats when opening menu
  if (isOpen.value) {
    refreshStats()
  }
}

function closeMenu() {
  isOpen.value = false
}

function openOfflineBrowser() {
  emit('openOfflineBrowser')
  closeMenu()
}

function openPluginManagement() {
  emit('openPluginManagement')
  closeMenu()
}

function openDatabaseManagement() {
  emit('openDatabaseManagement')
  closeMenu()
}

function openPageManagementModal() {
  showPageManagement.value = true
  // Don't close menu - user can see both
}

function closePageManagement() {
  showPageManagement.value = false
}

async function refreshStats() {
  try {
    await getStats()
    dbSize.value = getFormattedSize()
  } catch (error) {
    console.error('Failed to refresh stats:', error)
  }
}

async function runCleanup() {
  try {
    const result = await cleanupOldData(30) // 30 days TTL
    console.log(`Cleanup complete: ${result.deleted} records deleted`)

    // Refresh stats after cleanup
    await refreshStats()
  } catch (error: any) {
    console.error('Cleanup failed:', error)
    alert(`Cleanup failed: ${error.message || error}`)
  }
}

async function deleteByType(recordType: string) {
  const typeNames: Record<string, string> = {
    gitlab_pipeline: 'Pipelines',
    gitlab_job: 'Jobs',
  }

  const typeName = typeNames[recordType] || recordType

  const confirmed = confirm(
    `Delete ALL ${typeName}?\n\nThis will permanently delete all ${typeName.toLowerCase()} from ALL data sources.\n\nThis action cannot be undone!`
  )

  if (!confirmed) return

  try {
    const result = await deleteRecordsByType(recordType)
    alert(`Deleted ${result.deleted} ${typeName.toLowerCase()}`)

    // Refresh stats
    await refreshStats()
  } catch (error: any) {
    console.error(`Delete ${typeName} failed:`, error)
    alert(`Failed to delete ${typeName}: ${error.message || error}`)
  }
}

function onManualOfflineChange() {
  settingsStore.autoSave()

  if (settingsStore.manualOfflineMode) {
    console.log('📵 Entering manual offline mode')
  } else {
    console.log('📶 Exiting manual offline mode')
  }
}

function formatCleanupTime(date: Date): string {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)

  if (days > 0) return `${days}d ago`
  if (hours > 0) return `${hours}h ago`
  if (minutes > 0) return `${minutes}m ago`
  return 'Just now'
}

// M5.1: Credential Migration Functions
async function migrateCredentials() {
  const confirmed = confirm(
    '🔒 Encrypt Credentials?\n\n' +
      'This will encrypt your API tokens using AES-256 encryption.\n\n' +
      '✅ Tokens will be secure at rest\n' +
      '✅ Machine-specific protection\n' +
      '✅ Automatic backup created\n\n' +
      'Continue?'
  )

  if (!confirmed) return

  isMigrating.value = true

  try {
    console.log('🔒 Starting credential migration...')
    const result = await migrateDataSourcesToSecureStorage()

    if (result.success) {
      credentialsMigrated.value = true
      alert(
        `✅ Migration successful!\n\n` +
          `Encrypted ${result.migrated} data source${result.migrated !== 1 ? 's' : ''}.\n\n` +
          `Your credentials are now encrypted.`
      )
      console.log('✅ Credential migration complete:', result)
    } else {
      throw new Error(result.error || 'Migration failed')
    }
  } catch (error: any) {
    console.error('❌ Migration failed:', error)
    alert(
      `❌ Migration Failed\n\n` +
        `Error: ${error.message || error}\n\n` +
        `Your credentials remain unchanged.`
    )
  } finally {
    isMigrating.value = false
  }
}

async function confirmRollback() {
  const confirmed = confirm(
    '⚠️ Rollback Encryption?\n\n' +
      'This will restore credentials to plaintext.\n\n' +
      '❌ Credentials will NOT be encrypted\n' +
      '❌ Previous security will be removed\n\n' +
      'Only use this if something went wrong.\n\n' +
      'Continue with rollback?'
  )

  if (!confirmed) return

  isMigrating.value = true

  try {
    const success = rollbackMigration()

    if (success) {
      credentialsMigrated.value = false
      alert('✅ Rollback successful\n\nCredentials restored to plaintext.')
      console.log('✅ Rollback complete')
    } else {
      throw new Error('Rollback failed - no backup found')
    }
  } catch (error: any) {
    console.error('❌ Rollback failed:', error)
    alert(`❌ Rollback Failed\n\n${error.message || error}`)
  } finally {
    isMigrating.value = false
  }
}

// Theme selection
const currentTheme = computed({
  get: () => themeStore.currentTheme,
  set: value => themeStore.setTheme(value),
})

const availableThemes = computed(() => themeStore.availableThemes)
</script>

<style scoped>
.app-menu {
  position: relative;
}

.menu-button {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  background: var(--bg-panel);
  font-size: 18px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-primary);
}

.menu-button:hover {
  background: var(--bg-panel-header);
  border-color: var(--border-color);
}

.menu-dropdown {
  position: absolute;
  top: 50px;
  right: 0;
  width: 360px;
  background: var(--bg-panel);
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  animation: slideDown 0.2s ease-out;
  border: 1px solid var(--border-color);
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.menu-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.menu-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-heading);
}

.close-button {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  font-size: 24px;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.2s;
}

.close-button:hover {
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.menu-section {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.menu-section:last-child {
  border-bottom: none;
}

.menu-section h4 {
  margin: 0 0 12px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.menu-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
  cursor: pointer;
}

.toggle-item {
  position: relative;
}

.item-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.item-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.item-description {
  font-size: 12px;
  color: var(--text-muted);
}

/* Custom Toggle Switch */
.toggle-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  background-color: var(--border-color);
  border-radius: 24px;
  transition: background-color 0.3s;
  flex-shrink: 0;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: var(--bg-panel);
  border-radius: 50%;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-input:checked + .toggle-slider {
  background-color: var(--accent-success);
}

.toggle-input:checked + .toggle-slider::before {
  transform: translateX(20px);
}

/* Status Info */
.info-item {
  cursor: default;
  padding: 12px;
  background: var(--bg-panel-header);
  border-radius: 8px;
  margin-top: 8px;
  border: 1px solid var(--border-subtle);
}

.status-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.status-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
}

.status-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.status-value {
  font-weight: 600;
}

.status-value.online {
  color: var(--accent-success);
}

.status-value.offline {
  color: var(--accent-danger);
}

.status-note {
  margin-top: 4px;
  padding: 8px;
  background: #fff3cd;
  border-left: 3px solid #ff9800;
  border-radius: 4px;
  font-size: 12px;
  color: #856404;
}

/* Data Management Stats */
.stats-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stats-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
}

.stats-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.stats-value {
  font-weight: 600;
  color: var(--text-primary);
}

.menu-button-action {
  width: 100%;
  padding: 10px 16px;
  margin-top: 12px;
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.menu-button-action.btn-primary {
  background: var(--accent-primary);
  margin-bottom: 12px;
}

.menu-button-action.btn-primary:hover {
  background: var(--accent-hover);
}

.menu-button-action.btn-warning {
  background: var(--accent-danger);
}

.menu-button-action:hover:not(:disabled) {
  background: var(--accent-hover);
}

.menu-button-action:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
}

.menu-item-group {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.menu-item-group button {
  flex: 1;
}

.menu-footer {
  padding: 12px 20px;
  text-align: center;
  background: var(--bg-panel-header);
  border-radius: 0 0 12px 12px;
  border-top: 1px solid var(--border-color);
}

.version-info {
  font-size: 12px;
  color: var(--text-muted);
}

.menu-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: transparent;
  z-index: 999;
}

/* M5.1: Security Section Styles */
.warning-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: #fff3cd;
  border-left: 3px solid #ff9800;
  border-radius: 4px;
  margin-bottom: 12px;
}

.warning-icon {
  font-size: 18px;
}

.warning-text {
  font-size: 14px;
  color: #856404;
  font-weight: 500;
}

.success-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px;
  background: #d4edda;
  border-left: 3px solid #28a745;
  border-radius: 4px;
  margin-bottom: 12px;
}

.success-icon {
  font-size: 18px;
}

.success-text {
  font-size: 14px;
  color: #155724;
  font-weight: 500;
}

.info-text {
  font-size: 12px;
  color: #155724;
  opacity: 0.8;
}

.help-text {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 8px;
  line-height: 1.4;
}

/* Theme Selector */
.theme-selector {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-panel);
  font-size: 14px;
  color: var(--text-primary);
  cursor: pointer;
  transition: border-color 0.2s;
}

.theme-selector:hover {
  border-color: var(--accent-primary);
}

.theme-selector:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}
</style>
