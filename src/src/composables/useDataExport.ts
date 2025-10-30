import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

/**
 * Composable for data export functionality
 * Exports cached data to various formats (CSV, JSON)
 * Generates offline reports
 */
export function useDataExport() {
  const isExporting = ref(false)
  const exportError = ref<string | null>(null)

  /**
   * Convert records to CSV format
   */
  function exportToCSV(records: any[], filename: string = 'data-export.csv') {
    isExporting.value = true
    exportError.value = null

    try {
      if (records.length === 0) {
        throw new Error('No data to export')
      }

      // Get all unique keys from all records
      const allKeys = new Set<string>()
      records.forEach(record => {
        Object.keys(record.data || {}).forEach(key => allKeys.add(key))
        allKeys.add('record_type')
        allKeys.add('source')
        allKeys.add('timestamp')
      })

      const headers = Array.from(allKeys)

      // Create CSV header
      const csvLines = [headers.join(',')]

      // Add data rows
      records.forEach(record => {
        const row = headers.map(header => {
          let value
          if (header === 'record_type') value = record.record_type
          else if (header === 'source') value = record.source
          else if (header === 'timestamp') value = record.timestamp
          else value = record.data?.[header]

          // Escape commas and quotes
          if (value === undefined || value === null) return ''
          const strValue = String(value)
          if (strValue.includes(',') || strValue.includes('"')) {
            return `"${strValue.replace(/"/g, '""')}"`
          }
          return strValue
        })
        csvLines.push(row.join(','))
      })

      const csv = csvLines.join('\n')
      downloadFile(csv, filename, 'text/csv')

      return { success: true, recordCount: records.length }
    } catch (error) {
      exportError.value = error instanceof Error ? error.message : 'Export failed'
      return { success: false, error: exportError.value }
    } finally {
      isExporting.value = false
    }
  }

  /**
   * Export records to JSON format
   */
  function exportToJSON(records: any[], filename: string = 'data-export.json') {
    isExporting.value = true
    exportError.value = null

    try {
      const json = JSON.stringify(records, null, 2)
      downloadFile(json, filename, 'application/json')

      return { success: true, recordCount: records.length }
    } catch (error) {
      exportError.value = error instanceof Error ? error.message : 'Export failed'
      return { success: false, error: exportError.value }
    } finally {
      isExporting.value = false
    }
  }

  /**
   * Generate offline report with all data
   */
  async function generateOfflineReport() {
    isExporting.value = true
    exportError.value = null

    try {
      // Get all records from database
      const records = await invoke<any[]>('get_staged_records', {
        limit: 10000,
        offset: 0,
      })

      if (records.length === 0) {
        throw new Error('No data available for report')
      }

      // Group records by source
      const bySource = records.reduce(
        (acc, record) => {
          const source = record.source || 'unknown'
          if (!acc[source]) acc[source] = []
          acc[source].push(record)
          return acc
        },
        {} as Record<string, any[]>
      )

      // Generate report
      const report = {
        generated_at: new Date().toISOString(),
        total_records: records.length,
        sources: Object.keys(bySource).map(source => ({
          name: source,
          record_count: bySource[source].length,
          record_types: [...new Set(bySource[source].map((r: any) => r.record_type))],
          oldest_record: bySource[source].reduce(
            (oldest: any, r: any) => (!oldest || r.timestamp < oldest ? r.timestamp : oldest),
            null
          ),
          newest_record: bySource[source].reduce(
            (newest: any, r: any) => (!newest || r.timestamp > newest ? r.timestamp : newest),
            null
          ),
        })),
        all_records: records,
      }

      const filename = `offline-report-${new Date().toISOString().split('T')[0]}.json`
      exportToJSON([report], filename)

      return { success: true, report }
    } catch (error) {
      exportError.value = error instanceof Error ? error.message : 'Report generation failed'
      return { success: false, error: exportError.value }
    } finally {
      isExporting.value = false
    }
  }

  /**
   * Helper function to trigger file download
   */
  function downloadFile(content: string, filename: string, mimeType: string) {
    const blob = new Blob([content], { type: mimeType })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = filename
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
  }

  return {
    isExporting,
    exportError,
    exportToCSV,
    exportToJSON,
    generateOfflineReport,
  }
}
