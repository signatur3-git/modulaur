import { defineStore } from 'pinia'
import { ref } from 'vue'

// Helper to check if Tauri is available
const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

// Safe invoke wrapper
const safeInvoke = async (command: string, args?: any) => {
  if (!isTauri()) {
    throw new Error('Not in Tauri mode')
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke(command, args)
}

export interface Panel {
  i: string
  x: number
  y: number
  w: number
  h: number
  type: string // 'chart' | 'table' | 'kanban' | 'text' or plugin panel types
  title: string
  config: {
    content?: string
    chartType?: 'line' | 'bar' | 'pie' | 'doughnut'
    groupBy?: string
    timeBucket?: string
    dataTransform?: string
    timeFieldPath?: string
    stackByStatus?: boolean
    multiSourceMode?: boolean
    selectedSources?: string[]
    recordTypeFilter?: string
    dataSource?: string
    recordType?: string
    pageSize?: number
    query?: string
  }
}

export interface Dashboard {
  id: string
  name: string
  panels: Panel[]
  createdAt: number
  updatedAt: number
}

export const useDashboardStore = defineStore('dashboard', () => {
  const currentDashboard = ref<Dashboard | null>(null)
  const dashboards = ref<Dashboard[]>([])
  const isEditing = ref(false)

  async function loadDashboards() {
    try {
      const result = (await safeInvoke('get_dashboards')) as Dashboard[]
      dashboards.value = result
      return result
    } catch {
      // Fallback to localStorage in browser mode
      const stored = localStorage.getItem('dashboards')
      if (stored) {
        dashboards.value = JSON.parse(stored)
      }
      return dashboards.value
    }
  }

  async function loadDashboard(id: string) {
    try {
      const result = (await safeInvoke('get_dashboard', { id })) as Dashboard
      currentDashboard.value = result
      return result
    } catch {
      // Fallback to localStorage in browser mode
      const stored = localStorage.getItem(`dashboard_${id}`)
      if (stored) {
        currentDashboard.value = JSON.parse(stored)
      }
      return currentDashboard.value
    }
  }

  async function saveDashboard() {
    if (!currentDashboard.value) return

    currentDashboard.value.updatedAt = Date.now()

    try {
      await safeInvoke('save_dashboard', { dashboard: currentDashboard.value })
      const index = dashboards.value.findIndex(d => d.id === currentDashboard.value!.id)
      if (index >= 0) {
        dashboards.value[index] = { ...currentDashboard.value }
      } else {
        dashboards.value.push({ ...currentDashboard.value })
      }
    } catch {
      // Fallback to localStorage in browser mode
      localStorage.setItem(
        `dashboard_${currentDashboard.value.id}`,
        JSON.stringify(currentDashboard.value)
      )

      // Update dashboards list
      const index = dashboards.value.findIndex(d => d.id === currentDashboard.value!.id)
      if (index >= 0) {
        dashboards.value[index] = { ...currentDashboard.value }
      } else {
        dashboards.value.push({ ...currentDashboard.value })
      }
      localStorage.setItem('dashboards', JSON.stringify(dashboards.value))
    }
  }

  function createDashboard(name: string) {
    const newDashboard: Dashboard = {
      id: `dashboard_${Date.now()}`,
      name,
      panels: [],
      createdAt: Date.now(),
      updatedAt: Date.now(),
    }
    currentDashboard.value = newDashboard
    isEditing.value = true
    return newDashboard
  }

  function addPanel(type: string) {
    if (!currentDashboard.value) return

    const newPanel: Panel = {
      i: `panel_${Date.now()}`,
      x: 0,
      y: 0,
      w: 4,
      h: 3,
      type,
      title: `New ${type} Panel`,
      config: {},
    }

    currentDashboard.value.panels.push(newPanel)
  }

  function removePanel(panelId: string) {
    if (!currentDashboard.value) return
    currentDashboard.value.panels = currentDashboard.value.panels.filter(p => p.i !== panelId)
  }

  function updatePanel(panelId: string, updates: Partial<Panel>) {
    if (!currentDashboard.value) return
    const panel = currentDashboard.value.panels.find(p => p.i === panelId)
    if (panel) {
      Object.assign(panel, updates)
    }
  }

  function updateLayout(layout: Array<{ i: string; x: number; y: number; w: number; h: number }>) {
    if (!currentDashboard.value) return
    layout.forEach(item => {
      const panel = currentDashboard.value!.panels.find(p => p.i === item.i)
      if (panel) {
        panel.x = item.x
        panel.y = item.y
        panel.w = item.w
        panel.h = item.h
      }
    })
  }

  async function deleteDashboard(id: string) {
    try {
      await safeInvoke('delete_dashboard', { id })
      dashboards.value = dashboards.value.filter(d => d.id !== id)
      if (currentDashboard.value?.id === id) {
        currentDashboard.value = null
      }
    } catch {
      // Fallback to localStorage in browser mode
      localStorage.removeItem(`dashboard_${id}`)
      dashboards.value = dashboards.value.filter(d => d.id !== id)
      if (currentDashboard.value?.id === id) {
        currentDashboard.value = null
      }
      localStorage.setItem('dashboards', JSON.stringify(dashboards.value))
    }
  }

  async function renameDashboard(newName: string) {
    if (!currentDashboard.value || !newName.trim()) return

    currentDashboard.value.name = newName.trim()
    currentDashboard.value.updatedAt = Date.now()

    try {
      await safeInvoke('save_dashboard', { dashboard: currentDashboard.value })
      const index = dashboards.value.findIndex(d => d.id === currentDashboard.value!.id)
      if (index >= 0) {
        dashboards.value[index] = { ...currentDashboard.value }
      }
    } catch {
      // Fallback to localStorage in browser mode
      localStorage.setItem(
        `dashboard_${currentDashboard.value.id}`,
        JSON.stringify(currentDashboard.value)
      )
      const index = dashboards.value.findIndex(d => d.id === currentDashboard.value!.id)
      if (index >= 0) {
        dashboards.value[index] = { ...currentDashboard.value }
      }
      localStorage.setItem('dashboards', JSON.stringify(dashboards.value))
    }
  }

  return {
    currentDashboard,
    dashboards,
    isEditing,
    loadDashboards,
    loadDashboard,
    saveDashboard,
    createDashboard,
    addPanel,
    removePanel,
    updatePanel,
    updateLayout,
    deleteDashboard,
    renameDashboard,
  }
})
