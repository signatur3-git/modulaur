<template>
  <div id="app">
    <!-- M5: Offline Status Banner -->
    <div v-if="!isOnline" class="offline-banner">
      <span class="offline-icon">⚠️</span>
      <span class="offline-message">
        <span v-if="settingsStore.manualOfflineMode">
          Manual Offline Mode - Testing offline behavior
        </span>
        <span v-else> Offline Mode - Data fetching unavailable. Showing cached data. </span>
      </span>
      <span class="offline-time">
        <span v-if="settingsStore.manualOfflineMode"> (Manual) </span>
        <span v-else> Last online: {{ formatTime(lastOnlineChange) }} </span>
      </span>
    </div>

    <nav class="app-nav">
      <div class="nav-brand">
        <router-link to="/home" class="home-link" title="Go to Home">
          <h1>📊 Modulaur</h1>
        </router-link>
        <span class="version">v{{ appVersion }}</span>
        <!-- M5: Status Indicator -->
        <span v-if="isOnline" class="status-indicator online" title="Online">●</span>
        <span v-else class="status-indicator offline" title="Offline">●</span>
      </div>
      <div class="nav-actions">
        <!-- M5: App Menu -->
        <AppMenu
          @openOfflineBrowser="openOfflineBrowser"
          @openPluginManagement="openPluginManagement"
          @openDatabaseManagement="openDatabaseManagement"
        />
      </div>
    </nav>

    <!-- M10: Page Navigation -->
    <NavigationBar />

    <main class="app-main">
      <router-view />
    </main>

    <!-- Toast Notifications -->
    <ToastNotification ref="toastRef" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useSettingsStore } from './stores/settingsStore'
import { usePageStore } from './stores/pageStore'
import { useOnlineStatus } from './composables/useOnlineStatus'
import { useBackgroundSync } from './composables/useBackgroundSync'
import { setToastInstance } from './composables/useToast'
import ToastNotification from './components/ToastNotification.vue'
import AppMenu from './components/AppMenu.vue'
import NavigationBar from './components/navigation/NavigationBar.vue'

const router = useRouter()
const settingsStore = useSettingsStore()
const pageStore = usePageStore()
const toastRef = ref()

// App version from package.json (injected by Vite)
const appVersion = __APP_VERSION__

// M5 Phase 1: Online/Offline status tracking
const { isOnline, lastOnlineChange } = useOnlineStatus()

// M5 Phase 2: Background sync
useBackgroundSync()

onMounted(async () => {
  setToastInstance(toastRef.value)
  // M5 Phase 1: Load settings from localStorage
  settingsStore.loadSettings()

  // M10: Load pages for navigation bar
  pageStore.loadPages().catch(e => {
    console.error('Failed to load pages:', e)
    // Continue anyway - app should still work
  })
})

function openOfflineBrowser() {
  router.push('/offline-browser')
}

function openPluginManagement() {
  router.push('/plugin-management')
}

function openDatabaseManagement() {
  router.push('/database-management')
}

function formatTime(date: Date): string {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)

  if (hours > 0) return `${hours}h ago`
  if (minutes > 0) return `${minutes}m ago`
  return `${seconds}s ago`
}
</script>

<style scoped>
#app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-app);
}

/* M5: Offline Status Banner */
.offline-banner {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 24px;
  background: linear-gradient(135deg, #ffa726 0%, #ff9800 100%);
  color: white;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(255, 152, 0, 0.3);
  animation: slideDown 0.3s ease-out;
}

@keyframes slideDown {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.offline-icon {
  font-size: 18px;
}

.offline-message {
  flex: 1;
}

.offline-time {
  font-size: 12px;
  opacity: 0.9;
  font-weight: normal;
}

.app-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background: var(--bg-nav);
  border-bottom: 2px solid var(--border-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.nav-brand {
  display: flex;
  align-items: baseline;
  gap: 1rem;
}

.home-link {
  text-decoration: none;
  color: inherit;
  display: flex;
  align-items: center;
  transition: opacity 0.2s;
}

.home-link:hover {
  opacity: 0.8;
}

.home-link h1 {
  cursor: pointer;
}

.nav-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.nav-brand h1 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-heading);
}

.version {
  font-size: 0.75rem;
  color: var(--text-muted);
  font-weight: normal;
}

/* M5: Status Indicator */
.status-indicator {
  font-size: 12px;
  margin-left: 8px;
  animation: pulse 2s ease-in-out infinite;
}

.status-indicator.online {
  color: var(--accent-success);
  text-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
}

.status-indicator.offline {
  color: var(--accent-danger);
  text-shadow: 0 0 8px rgba(239, 68, 68, 0.5);
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.btn-nav {
  padding: 0.5rem 1rem;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-nav:hover {
  background: var(--bg-panel-header);
}

.app-main {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

/* Global: Prevent text selection during drag operations */
.sortable-drag,
.sortable-ghost {
  user-select: none !important;
  -webkit-user-select: none !important;
  -moz-user-select: none !important;
  -ms-user-select: none !important;
}

/* Prevent text selection in panels when dragging anything */
body.dragging,
body.dragging * {
  user-select: none !important;
  -webkit-user-select: none !important;
  -moz-user-select: none !important;
  -ms-user-select: none !important;
  cursor: grabbing !important;
}
</style>
