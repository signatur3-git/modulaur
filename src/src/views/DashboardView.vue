<template>
  <div class="dashboard-view">
    <DashboardEditor />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDashboardStore } from '@/stores/dashboardStore'
import DashboardEditor from '@/components/DashboardEditor.vue'

const route = useRoute()
const router = useRouter()
const dashboardStore = useDashboardStore()

onMounted(async () => {
  const dashboardId = route.params.id as string

  if (!dashboardId) {
    // No ID provided, go to overview
    router.push('/home')
    return
  }

  try {
    await dashboardStore.loadDashboard(dashboardId)
  } catch (e) {
    console.error('Failed to load dashboard:', e)
    alert(`Failed to load dashboard: ${e}`)
    router.push('/home')
  }
})
</script>

<style scoped>
.dashboard-view {
  height: 100%;
  overflow: auto;
}
</style>
