/**
 * Database Management Store
 *
 * Provides export/import functionality for backing up and restoring data
 * between dev and prod environments.
 *
 * Features:
 * - Database Export/Import: All data (pages, records, dashboards, settings, etc.)
 * - LocalStorage Export/Import: Plugin panel data stored in browser
 * - Database Statistics: View counts and sizes
 * - Cleanup Operations: Clear records, cleanup old data
 *
 * Environment Databases:
 * - Dev mode: Uses data/dev/db
 * - Prod mode: Uses data/prod/db
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface DatabaseStats {
  total_records: number
  size_bytes: number
  by_type: Record<string, number>
  by_source: Record<string, number>
}

export interface ImportStats {
  records_imported: number
  pages_imported: number
  data_sources_imported: number
  settings_imported: number
  plugin_data_imported: number
  tickets_imported: number
  dashboards_imported: number
  errors: string[]
}

export interface DatabaseExport {
  version: string
  exported_at: string
  data: {
    records: any[]
    pages: any[]
    data_sources: any[]
    settings: any[]
    plugin_data: any[]
    tickets: any[]
    dashboards: any[]
  }
}

export const useDatabaseStore = defineStore('database', () => {
  const stats = ref<DatabaseStats | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  /**
   * Get current database statistics
   */
  async function getStats(): Promise<DatabaseStats> {
    isLoading.value = true
    error.value = null

    try {
      const result = await invoke<DatabaseStats>('get_database_stats')
      stats.value = result
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Export all database data to JSON
   * Returns a JSON object that can be saved to file or imported elsewhere
   */
  async function exportDatabase(): Promise<DatabaseExport> {
    isLoading.value = true
    error.value = null

    try {
      const result = await invoke<DatabaseExport>('export_database')
      console.log('Database exported:', {
        records: result.data.records.length,
        pages: result.data.pages.length,
        data_sources: result.data.data_sources.length,
        settings: result.data.settings.length,
        plugin_data: result.data.plugin_data.length,
        tickets: result.data.tickets.length,
      })
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Import database data from JSON export
   */
  async function importDatabase(
    data: DatabaseExport,
    mergeStrategy: 'replace' | 'merge' | 'skip' = 'replace'
  ): Promise<ImportStats> {
    isLoading.value = true
    error.value = null

    try {
      const result = await invoke<ImportStats>('import_database', {
        importData: data,
        mergeStrategy,
      })
      console.log('Database imported:', result)
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Download database export as JSON file
   */
  async function downloadExport(): Promise<void> {
    const exportData = await exportDatabase()

    // Create blob and download
    const blob = new Blob([JSON.stringify(exportData, null, 2)], {
      type: 'application/json',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `database-export-${new Date().toISOString().split('T')[0]}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  /**
   * Clear all records from database
   */
  async function clearAllRecords(): Promise<number> {
    isLoading.value = true
    error.value = null

    try {
      const count = await invoke<number>('clear_all_records')
      console.log(`Cleared ${count} records`)

      // Refresh stats after clearing
      await getStats()

      return count
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Clean up old records based on TTL
   */
  async function cleanupOldRecords(ttlDays: number, source?: string): Promise<number> {
    isLoading.value = true
    error.value = null

    try {
      const result = await invoke<{ deleted: number }>('cleanup_old_records', {
        ttlDays,
        source,
      })
      console.log(`Cleaned up ${result.deleted} old records`)

      // Refresh stats after cleanup
      await getStats()

      return result.deleted
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Export localStorage panel data
   *
   * Plugin panels (markdown notes, RSS feeds, etc.) store their content in localStorage,
   * NOT in the database. This function exports all plugin panel data for backup/migration.
   *
   * @returns Export object with all localStorage panel data
   */
  function exportLocalStoragePanelData(): any {
    const data: Record<string, any> = {}
    let count = 0

    // Pattern-based extraction of plugin panel data
    const pluginPatterns = [
      'markdown-notes-', // Markdown Notepad: markdown-notes-{panel.i}
      'rss-reader-', // RSS Feed Reader: rss-reader-{panel.i}
      'time-tracker-', // Time Tracker: time-tracker-recent-descriptions
      // Add more patterns as plugins are added
    ]

    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (!key) continue

      // Check if this key matches any plugin pattern
      const isPluginData = pluginPatterns.some(pattern => key.startsWith(pattern))

      if (isPluginData) {
        try {
          const value = localStorage.getItem(key)
          if (value) {
            // Try to parse as JSON, store as-is if not JSON
            try {
              data[key] = JSON.parse(value)
            } catch {
              data[key] = value
            }
            count++
          }
        } catch (e) {
          console.warn(`Failed to export localStorage key: ${key}`, e)
        }
      }
    }

    console.log(`Exported ${count} localStorage panel data items`)

    return {
      version: '1.0',
      exported_at: new Date().toISOString(),
      type: 'localStorage_panel_data',
      count,
      data,
    }
  }

  /**
   * Import localStorage panel data
   *
   * Restores plugin panel content from an export created by exportLocalStoragePanelData()
   *
   * @param importData The export object from exportLocalStoragePanelData()
   * @param mergeStrategy How to handle conflicts:
   *   - "replace": Overwrite existing localStorage data
   *   - "merge": Keep existing data if key exists (don't overwrite)
   * @returns Number of items imported
   */
  function importLocalStoragePanelData(
    importData: any,
    mergeStrategy: 'replace' | 'merge' = 'replace'
  ): number {
    if (!importData.data || importData.type !== 'localStorage_panel_data') {
      throw new Error('Invalid import data format')
    }

    console.log(`[Panel Data Import] Starting import with strategy: ${mergeStrategy}`)
    console.log(
      `[Panel Data Import] Import data contains ${Object.keys(importData.data).length} keys`
    )

    let imported = 0
    let skipped = 0

    for (const [key, value] of Object.entries(importData.data)) {
      try {
        // Check if key already exists
        const exists = localStorage.getItem(key) !== null

        if (exists && mergeStrategy === 'merge') {
          console.log(`[Panel Data Import] Skipping existing key: ${key}`)
          skipped++
          continue
        }

        // Store the data
        const stringValue = typeof value === 'string' ? value : JSON.stringify(value)
        localStorage.setItem(key, stringValue)
        console.log(
          `[Panel Data Import] ✅ Imported key: ${key} (${typeof value}, ${stringValue.length} chars)`
        )
        imported++

        // Verify it was stored
        const verification = localStorage.getItem(key)
        if (!verification) {
          console.error(`[Panel Data Import] ❌ Verification failed for key: ${key}`)
        }
      } catch (e) {
        console.error(`[Panel Data Import] ❌ Failed to import localStorage key: ${key}`, e)
      }
    }

    console.log(`[Panel Data Import] Complete: ${imported} imported, ${skipped} skipped`)
    console.log(`[Panel Data Import] localStorage now has ${localStorage.length} total items`)

    // List all panel-related keys for debugging
    const panelKeys = []
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (key && (key.startsWith('markdown-notes-') || key.startsWith('rss-reader-'))) {
        panelKeys.push(key)
      }
    }
    console.log(`[Panel Data Import] Panel-related keys in localStorage:`, panelKeys)

    return imported
  }

  /**
   * Download localStorage panel data export as JSON file
   */
  function downloadLocalStorageExport(): void {
    const exportData = exportLocalStoragePanelData()

    const blob = new Blob([JSON.stringify(exportData, null, 2)], {
      type: 'application/json',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `panel-data-export-${new Date().toISOString().split('T')[0]}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  return {
    // State
    stats,
    isLoading,
    error,

    // Actions
    getStats,
    exportDatabase,
    importDatabase,
    downloadExport,
    clearAllRecords,
    cleanupOldRecords,
    // LocalStorage panel data functions
    exportLocalStoragePanelData,
    importLocalStoragePanelData,
    downloadLocalStorageExport,
  }
})
