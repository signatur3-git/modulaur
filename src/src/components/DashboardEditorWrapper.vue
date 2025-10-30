<template>
  <DashboardEditor @save-requested="handleSaveRequest" />
</template>

<script setup lang="ts">
import { onMounted, watch, onBeforeUnmount } from 'vue'
import { useDashboardStore } from '@/stores/dashboardStore'
import DashboardEditor from '@/components/DashboardEditor.vue'

interface Dashboard {
  id: string
  name: string
  panels: any[]
  createdAt: number
  updatedAt: number
}

const props = defineProps<{
  dashboard: Dashboard
  collectionId: string
}>()

const emit = defineEmits<{
  save: [dashboard: Dashboard]
}>()

const dashboardStore = useDashboardStore()

// Store the original saveDashboard function
const originalSaveDashboard = dashboardStore.saveDashboard

// Load the dashboard into the store when mounted or when dashboard changes
function loadDashboardIntoStore() {
  // Set the current dashboard in the store
  dashboardStore.currentDashboard = {
    id: props.dashboard.id,
    name: props.dashboard.name,
    panels: props.dashboard.panels,
    createdAt: props.dashboard.createdAt,
    updatedAt: props.dashboard.updatedAt,
  }

  // Set editing mode to false initially
  dashboardStore.isEditing = false
}

// Override the saveDashboard function to emit to parent instead
async function handleSaveRequest() {
  if (dashboardStore.currentDashboard) {
    dashboardStore.currentDashboard.updatedAt = Date.now()
    emit('save', {
      id: dashboardStore.currentDashboard.id,
      name: dashboardStore.currentDashboard.name,
      panels: dashboardStore.currentDashboard.panels,
      createdAt: dashboardStore.currentDashboard.createdAt,
      updatedAt: dashboardStore.currentDashboard.updatedAt,
    })
    dashboardStore.isEditing = false
  }
}

onMounted(() => {
  loadDashboardIntoStore()

  // Override the saveDashboard method
  dashboardStore.saveDashboard = handleSaveRequest
})

onBeforeUnmount(() => {
  // Restore the original saveDashboard method
  dashboardStore.saveDashboard = originalSaveDashboard
})

watch(
  () => props.dashboard,
  () => {
    loadDashboardIntoStore()
  },
  { deep: true }
)
</script>
