import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useSettingsStore } from '../stores/settingsStore'

/**
 * Composable to track online/offline status
 * Uses browser's navigator.onLine API and online/offline events
 * Also respects manual offline mode from settings
 */
export function useOnlineStatus() {
  const settingsStore = useSettingsStore()
  const browserOnline = ref(navigator.onLine)
  const lastOnlineChange = ref<Date>(new Date())

  // Computed: true only if browser is online AND manual offline mode is OFF
  const isOnline = computed(() => {
    return browserOnline.value && !settingsStore.manualOfflineMode
  })

  function handleOnline() {
    browserOnline.value = true
    lastOnlineChange.value = new Date()
    console.log('📡 Connection restored:', lastOnlineChange.value)
  }

  function handleOffline() {
    browserOnline.value = false
    lastOnlineChange.value = new Date()
    console.warn('⚠️ Connection lost:', lastOnlineChange.value)
  }

  onMounted(() => {
    window.addEventListener('online', handleOnline)
    window.addEventListener('offline', handleOffline)
  })

  onUnmounted(() => {
    window.removeEventListener('online', handleOnline)
    window.removeEventListener('offline', handleOffline)
  })

  return {
    isOnline,
    lastOnlineChange,
    browserOnline, // Expose actual browser status separately
  }
}
