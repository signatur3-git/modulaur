import { ref, onMounted, onUnmounted } from 'vue'
import { useDataSourceStore } from '../stores/dataSourceStore'
import { useOnlineStatus } from './useOnlineStatus'

/**
 * Background Sync Composable
 * Handles automatic data refreshing for data sources with auto_refresh enabled
 * M5 Phase 2: Background Sync
 */

interface PollingState {
  sourceId: string
  intervalId: number
  lastFetch: Date
  nextFetch: Date
  isRunning: boolean
  errorCount: number
}

export function useBackgroundSync() {
  const dataSourceStore = useDataSourceStore()
  const { isOnline } = useOnlineStatus()

  const pollingStates = ref<Map<string, PollingState>>(new Map())
  const globalEnabled = ref(true)

  /**
   * Start polling for a specific data source
   */
  function startPolling(sourceId: string, intervalMinutes: number) {
    // Don't start if already polling
    if (pollingStates.value.has(sourceId)) {
      console.warn(`Already polling source: ${sourceId}`)
      return
    }

    console.log(`📡 Starting background sync for ${sourceId} (every ${intervalMinutes}m)`)

    const intervalMs = intervalMinutes * 60 * 1000

    // Initial state
    const state: PollingState = {
      sourceId,
      intervalId: 0,
      lastFetch: new Date(),
      nextFetch: new Date(Date.now() + intervalMs),
      isRunning: false,
      errorCount: 0,
    }

    // Set up interval
    const intervalId = window.setInterval(async () => {
      await executeFetch(sourceId, state)
    }, intervalMs)

    state.intervalId = intervalId
    pollingStates.value.set(sourceId, state)

    // Do initial fetch after a short delay
    setTimeout(() => executeFetch(sourceId, state), 5000)
  }

  /**
   * Stop polling for a specific data source
   */
  function stopPolling(sourceId: string) {
    const state = pollingStates.value.get(sourceId)
    if (!state) return

    console.log(`🛑 Stopping background sync for ${sourceId}`)

    window.clearInterval(state.intervalId)
    pollingStates.value.delete(sourceId)
  }

  /**
   * Execute a fetch for a data source
   */
  async function executeFetch(sourceId: string, state: PollingState) {
    // Skip if already running
    if (state.isRunning) {
      console.log(`⏭️ Skipping fetch for ${sourceId} (already running)`)
      return
    }

    // Skip if offline
    if (!isOnline.value) {
      console.log(`⏭️ Skipping fetch for ${sourceId} (offline)`)
      return
    }

    // Skip if global background sync is disabled
    if (!globalEnabled.value) {
      console.log(`⏭️ Skipping fetch for ${sourceId} (background sync disabled)`)
      return
    }

    state.isRunning = true
    state.lastFetch = new Date()

    try {
      console.log(`🔄 Background fetching data for ${sourceId}`)
      await dataSourceStore.fetchData(sourceId)

      state.errorCount = 0 // Reset error count on success
      console.log(`✅ Background fetch completed for ${sourceId}`)
    } catch (error: any) {
      state.errorCount++
      console.error(`❌ Background fetch failed for ${sourceId}:`, error.message)

      // If too many errors, stop polling
      if (state.errorCount >= 5) {
        console.error(`🛑 Too many errors for ${sourceId}, stopping background sync`)
        stopPolling(sourceId)
      }
    } finally {
      state.isRunning = false

      // Update next fetch time
      const source = dataSourceStore.dataSources.find(s => s.id === sourceId)
      if (source?.refresh_interval) {
        state.nextFetch = new Date(Date.now() + source.refresh_interval * 60 * 1000)
      }
    }
  }

  /**
   * Start all auto-refresh data sources
   */
  function startAll() {
    console.log('📡 Starting all background syncs')

    dataSourceStore.dataSources.forEach(source => {
      if (source.auto_refresh && source.enabled && source.refresh_interval) {
        startPolling(source.id, source.refresh_interval)
      }
    })
  }

  /**
   * Stop all polling
   */
  function stopAll() {
    console.log('🛑 Stopping all background syncs')

    pollingStates.value.forEach((_, sourceId) => {
      stopPolling(sourceId)
    })
  }

  /**
   * Restart a specific source (useful when config changes)
   */
  function restart(sourceId: string) {
    stopPolling(sourceId)

    const source = dataSourceStore.dataSources.find(s => s.id === sourceId)
    if (source?.auto_refresh && source.enabled && source.refresh_interval) {
      startPolling(sourceId, source.refresh_interval)
    }
  }

  /**
   * Get polling state for a source
   */
  function getState(sourceId: string): PollingState | undefined {
    return pollingStates.value.get(sourceId)
  }

  /**
   * Toggle global background sync
   */
  function toggleGlobal() {
    globalEnabled.value = !globalEnabled.value
    console.log(`📡 Background sync globally ${globalEnabled.value ? 'enabled' : 'disabled'}`)
  }

  // Auto-start on mount
  onMounted(() => {
    console.log('📡 Background sync composable mounted')
    // Small delay to let data sources load
    setTimeout(startAll, 2000)
  })

  // Clean up on unmount
  onUnmounted(() => {
    console.log('🛑 Background sync composable unmounting')
    stopAll()
  })

  return {
    pollingStates,
    globalEnabled,
    startPolling,
    stopPolling,
    startAll,
    stopAll,
    restart,
    getState,
    toggleGlobal,
  }
}
