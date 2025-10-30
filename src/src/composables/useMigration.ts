import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Page } from '../stores/pageStore'
import type { Dashboard } from '../stores/dashboardStore'

/**
 * M9: Migration Composable
 * Helps migrate from localStorage-based dashboards to database-backed pages
 */

// Helper to check if Tauri is available
const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

export interface MigrationStatus {
  needed: boolean
  dashboardCount: number
  pageCount: number
  migrated: boolean
  error: string | null
}

export function useMigration() {
  const status = ref<MigrationStatus>({
    needed: false,
    dashboardCount: 0,
    pageCount: 0,
    migrated: false,
    error: null,
  })
  const migrating = ref(false)

  /**
   * Check if migration is needed
   * Returns true if there are dashboards in localStorage but no pages in database
   */
  async function checkMigrationStatus(): Promise<MigrationStatus> {
    try {
      // Check for dashboards in localStorage
      const storedDashboards = localStorage.getItem('dashboards')
      const dashboards: Dashboard[] = storedDashboards ? JSON.parse(storedDashboards) : []

      status.value.dashboardCount = dashboards.length

      if (!isTauri()) {
        // In browser mode, check localStorage pages
        const storedPages = localStorage.getItem('pages')
        const pages: Page[] = storedPages ? JSON.parse(storedPages) : []
        status.value.pageCount = pages.length
        status.value.needed = dashboards.length > 0 && pages.length === 0
        return status.value
      }

      // In Tauri mode, check database
      const pages = (await invoke('get_all_pages')) as Page[]
      status.value.pageCount = pages.length

      // Migration needed if we have dashboards but no pages
      status.value.needed = dashboards.length > 0 && pages.length === 0

      return status.value
    } catch (err: any) {
      status.value.error = `Failed to check migration status: ${err.message}`
      console.error(status.value.error)
      return status.value
    }
  }

  /**
   * Convert a Dashboard to a Page
   */
  function dashboardToPage(dashboard: Dashboard): Page {
    const now = new Date().toISOString()

    return {
      id: dashboard.id,
      name: dashboard.name,
      type: 'dashboard',
      route: dashboard.name.toLowerCase().replace(/\s+/g, '-'),
      icon: '📊',
      config: {
        panels: dashboard.panels, // Store dashboard panels in config
      },
      order: 99,
      visible: true,
      created_at: dashboard.createdAt ? new Date(dashboard.createdAt).toISOString() : now,
      updated_at: dashboard.updatedAt ? new Date(dashboard.updatedAt).toISOString() : now,
    }
  }

  /**
   * Migrate all dashboards to pages
   */
  async function migrateDashboards(): Promise<{
    success: boolean
    migratedCount: number
    error?: string
  }> {
    try {
      migrating.value = true
      status.value.error = null

      // Load dashboards from localStorage
      const storedDashboards = localStorage.getItem('dashboards')
      if (!storedDashboards) {
        return { success: true, migratedCount: 0 }
      }

      const dashboards: Dashboard[] = JSON.parse(storedDashboards)
      let migratedCount = 0

      console.log(`📦 Migrating ${dashboards.length} dashboards to pages...`)

      // Convert and save each dashboard as a page
      for (const dashboard of dashboards) {
        const page = dashboardToPage(dashboard)

        if (!isTauri()) {
          // Browser mode: save to localStorage
          const storedPages = localStorage.getItem('pages')
          const pages: Page[] = storedPages ? JSON.parse(storedPages) : []

          // Check if page already exists
          const existingIndex = pages.findIndex(p => p.id === page.id)
          if (existingIndex >= 0) {
            pages[existingIndex] = page
          } else {
            pages.push(page)
          }

          localStorage.setItem('pages', JSON.stringify(pages))
          localStorage.setItem(`page_${page.id}`, JSON.stringify(page))
        } else {
          // Tauri mode: save to database
          await invoke('save_page', { page })
        }

        migratedCount++
        console.log(`✅ Migrated: ${dashboard.name}`)
      }

      // Mark migration as complete
      status.value.migrated = true
      status.value.needed = false
      status.value.pageCount = migratedCount

      // Optionally backup old dashboards
      localStorage.setItem('dashboards_backup', storedDashboards)

      console.log(`✨ Migration complete! ${migratedCount} dashboards migrated to pages`)

      return { success: true, migratedCount }
    } catch (err: any) {
      const errorMsg = `Failed to migrate dashboards: ${err.message}`
      status.value.error = errorMsg
      console.error(errorMsg, err)
      return { success: false, migratedCount: 0, error: errorMsg }
    } finally {
      migrating.value = false
    }
  }

  /**
   * Run migration command on backend (if available)
   * This uses the migrate_dashboards_to_pages Tauri command
   */
  async function runBackendMigration(): Promise<{
    success: boolean
    migratedCount: number
    error?: string
  }> {
    if (!isTauri()) {
      // Not in Tauri mode, use frontend migration
      return await migrateDashboards()
    }

    try {
      migrating.value = true
      status.value.error = null

      // Call backend migration command
      const result = (await invoke('migrate_dashboards_to_pages')) as { migrated: number }

      status.value.migrated = true
      status.value.needed = false
      status.value.pageCount = result.migrated

      console.log(`✨ Backend migration complete! ${result.migrated} dashboards migrated`)

      return { success: true, migratedCount: result.migrated }
    } catch (err: any) {
      // If backend migration fails, try frontend migration as fallback
      console.warn('Backend migration failed, trying frontend migration:', err)
      return await migrateDashboards()
    } finally {
      migrating.value = false
    }
  }

  /**
   * Clear old dashboard data after successful migration
   */
  function clearOldDashboards() {
    if (!status.value.migrated) {
      console.warn('Cannot clear dashboards: migration not completed')
      return
    }

    // Keep backup, remove active data
    localStorage.removeItem('dashboards')

    // Remove individual dashboard entries
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (key?.startsWith('dashboard_')) {
        localStorage.removeItem(key)
      }
    }

    console.log('🧹 Old dashboard data cleared')
  }

  return {
    status,
    migrating,
    checkMigrationStatus,
    migrateDashboards,
    runBackendMigration,
    clearOldDashboards,
  }
}
