import { ref } from 'vue'

/**
 * Data Management Composable
 * Handles database cleanup, TTL, and storage tracking
 * M5 Phase 3: Data Management
 */

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

export function useDataManagement() {
  const isCleaningUp = ref(false)
  const lastCleanup = ref<Date | null>(null)
  const totalRecords = ref(0)
  const dbSizeBytes = ref(0)

  /**
   * Get database statistics
   */
  async function getStats() {
    try {
      const stats = (await safeInvoke('get_database_stats')) as {
        total_records: number
        size_bytes: number
      }

      totalRecords.value = stats.total_records
      dbSizeBytes.value = stats.size_bytes

      return stats
    } catch (error: any) {
      console.error('Failed to get database stats:', error)
      throw error
    }
  }

  /**
   * Clean up old records based on TTL
   */
  async function cleanupOldData(ttlDays: number = 30, source?: string) {
    if (isCleaningUp.value) {
      console.warn('Cleanup already in progress')
      return { deleted: 0 }
    }

    isCleaningUp.value = true
    console.log(`🧹 Starting cleanup: TTL=${ttlDays} days, source=${source || 'all'}`)

    try {
      const result = (await safeInvoke('cleanup_old_records', {
        ttlDays,
        source: source || null,
      })) as { deleted: number }

      lastCleanup.value = new Date()
      console.log(`✅ Cleanup complete: ${result.deleted} records deleted`)

      // Refresh stats
      await getStats()

      return result
    } catch (error: any) {
      console.error('❌ Cleanup failed:', error)
      throw error
    } finally {
      isCleaningUp.value = false
    }
  }

  /**
   * Clean up by specific source and its TTL setting
   */
  async function cleanupBySource(sourceId: string, ttlDays: number) {
    return await cleanupOldData(ttlDays, sourceId)
  }

  /**
   * Run automatic cleanup for all sources with TTL configured
   */
  async function runAutoCleanup(
    sources: Array<{ id: string; source: string; data_ttl_days?: number }>
  ) {
    console.log('🧹 Running automatic cleanup for all sources')

    let totalDeleted = 0

    for (const source of sources) {
      if (source.data_ttl_days && source.data_ttl_days > 0) {
        try {
          const result = await cleanupBySource(source.source, source.data_ttl_days)
          totalDeleted += result.deleted
          console.log(
            `  ✓ ${source.id}: ${result.deleted} records deleted (TTL: ${source.data_ttl_days}d)`
          )
        } catch (error) {
          console.error(`  ✗ ${source.id}: Cleanup failed`, error)
        }
      }
    }

    console.log(`✅ Auto cleanup complete: ${totalDeleted} total records deleted`)
    return { totalDeleted }
  }

  /**
   * Format bytes to human-readable size
   */
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B'

    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))

    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i]
  }

  /**
   * Get formatted database size
   */
  function getFormattedSize(): string {
    return formatBytes(dbSizeBytes.value)
  }

  /**
   * Schedule automatic cleanup (runs daily)
   */
  function scheduleAutoCleanup(
    sources: Array<{ id: string; source: string; data_ttl_days?: number }>
  ) {
    // Run cleanup daily at 3 AM
    const now = new Date()
    const next3AM = new Date()
    next3AM.setHours(3, 0, 0, 0)

    if (next3AM <= now) {
      // If 3 AM already passed today, schedule for tomorrow
      next3AM.setDate(next3AM.getDate() + 1)
    }

    const msUntil3AM = next3AM.getTime() - now.getTime()

    console.log(`⏰ Scheduled auto cleanup for ${next3AM.toLocaleString()}`)

    setTimeout(() => {
      runAutoCleanup(sources)

      // Reschedule for next day
      setInterval(
        () => {
          runAutoCleanup(sources)
        },
        24 * 60 * 60 * 1000
      ) // Every 24 hours
    }, msUntil3AM)
  }

  /**
   * M5: Delete all records of a specific type
   */
  async function deleteRecordsByType(recordType: string) {
    console.log(`🗑️ Deleting all records of type: ${recordType}`)

    try {
      const result = (await safeInvoke('delete_records_by_type', {
        recordType,
      })) as { deleted: number }

      console.log(`✅ Deleted ${result.deleted} records of type '${recordType}'`)

      // Refresh stats
      await getStats()

      return result
    } catch (error: any) {
      console.error('❌ Delete by type failed:', error)
      throw error
    }
  }

  /**
   * M5: Delete records by source and type combination
   */
  async function deleteRecordsBySourceAndType(source: string, recordType: string) {
    console.log(`🗑️ Deleting records of type '${recordType}' from source '${source}'`)

    try {
      const result = (await safeInvoke('delete_records_by_source_and_type', {
        source,
        recordType,
      })) as { deleted: number }

      console.log(`✅ Deleted ${result.deleted} records`)

      // Refresh stats
      await getStats()

      return result
    } catch (error: any) {
      console.error('❌ Delete by source+type failed:', error)
      throw error
    }
  }

  return {
    isCleaningUp,
    lastCleanup,
    totalRecords,
    dbSizeBytes,
    getStats,
    cleanupOldData,
    cleanupBySource,
    runAutoCleanup,
    formatBytes,
    getFormattedSize,
    scheduleAutoCleanup,
    deleteRecordsByType,
    deleteRecordsBySourceAndType,
  }
}
