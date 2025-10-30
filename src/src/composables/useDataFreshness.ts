import { computed } from 'vue'

/**
 * Composable for calculating data freshness
 * Shows how old cached data is and if it's stale
 */
export function useDataFreshness() {
  /**
   * Calculate age of a record in milliseconds
   */
  function getRecordAge(timestamp: string | Date): number {
    const recordDate = typeof timestamp === 'string' ? new Date(timestamp) : timestamp
    return Date.now() - recordDate.getTime()
  }

  /**
   * Check if data is stale (older than threshold)
   * Default threshold: 24 hours
   */
  function isStale(timestamp: string | Date, thresholdHours: number = 24): boolean {
    const ageMs = getRecordAge(timestamp)
    const thresholdMs = thresholdHours * 60 * 60 * 1000
    return ageMs > thresholdMs
  }

  /**
   * Format age as human-readable string
   * Examples: "2 minutes ago", "3 hours ago", "2 days ago"
   */
  function formatAge(timestamp: string | Date): string {
    const ageMs = getRecordAge(timestamp)

    const seconds = Math.floor(ageMs / 1000)
    const minutes = Math.floor(seconds / 60)
    const hours = Math.floor(minutes / 60)
    const days = Math.floor(hours / 24)

    if (days > 0) {
      return days === 1 ? '1 day ago' : `${days} days ago`
    } else if (hours > 0) {
      return hours === 1 ? '1 hour ago' : `${hours} hours ago`
    } else if (minutes > 0) {
      return minutes === 1 ? '1 minute ago' : `${minutes} minutes ago`
    } else {
      return seconds === 1 ? '1 second ago' : `${seconds} seconds ago`
    }
  }

  /**
   * Get freshness status with color coding
   */
  function getFreshnessStatus(timestamp: string | Date): {
    label: string
    color: string
    isStale: boolean
  } {
    const ageMs = getRecordAge(timestamp)
    const hours = ageMs / (1000 * 60 * 60)

    if (hours < 1) {
      return { label: 'Fresh', color: 'green', isStale: false }
    } else if (hours < 6) {
      return { label: 'Recent', color: 'blue', isStale: false }
    } else if (hours < 24) {
      return { label: 'Today', color: 'yellow', isStale: false }
    } else if (hours < 72) {
      return { label: 'This Week', color: 'orange', isStale: true }
    } else {
      return { label: 'Old', color: 'red', isStale: true }
    }
  }

  /**
   * Format timestamp as local date/time
   */
  function formatTimestamp(timestamp: string | Date): string {
    const date = typeof timestamp === 'string' ? new Date(timestamp) : timestamp
    return date.toLocaleString()
  }

  /**
   * Get relative time with auto-updating
   * Returns a computed value that shows relative time
   */
  function getRelativeTime(timestamp: string | Date) {
    return computed(() => formatAge(timestamp))
  }

  return {
    getRecordAge,
    isStale,
    formatAge,
    getFreshnessStatus,
    formatTimestamp,
    getRelativeTime,
  }
}
