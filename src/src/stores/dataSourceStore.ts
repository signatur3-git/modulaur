import { defineStore } from 'pinia'
import { ref } from 'vue'

/**
 * Data Source Store - Manages data source configurations
 *
 * STORAGE STRATEGY:
 * - Tauri mode (dev OR prod): Always uses SurrealDB database
 * - Pure browser mode (no Tauri): Falls back to localStorage for frontend-only development
 *
 * ENVIRONMENT FIELD:
 * - Used to distinguish dev vs production DATA SOURCES (not storage mechanism)
 * - 'dev': Development data sources (test APIs, dev credentials)
 * - 'production': Production data sources (live APIs, prod credentials)
 * - 'both': Shared between environments
 */

// Helper to check if Tauri is available (browser-only vs Tauri app, NOT dev vs prod)
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

export interface AuthConfig {
  type:
    | 'none'
    | 'bearer'
    | 'basic'
    | 'oauth2clientcredentials'
    | 'oauth2authorizationcode'
    | 'apikey'
    | 'gitlabtoken'
  token?: string
  username?: string
  password?: string
  client_id?: string
  client_secret?: string
  auth_url?: string
  token_url?: string
  api_key?: string
  api_key_header?: string
}

export interface DataSourceConfig {
  id: string
  name: string
  adapter_type: 'rest_api' | 'gitlab'
  source: string
  endpoint: string
  auth: AuthConfig | null
  parameters?: {
    method?: string
    headers?: Record<string, string>
    data_path?: string
    default_tags?: string[]
    project_id?: number | string
    per_page?: number
    max_pages?: number // M5: For historical fetch - how many pages to fetch
    status?: string
    ref?: string
    fetch_pipelines?: boolean
    fetch_jobs?: boolean
  }
  environment?: 'dev' | 'production' | 'both' // M9: Environment validation
  enabled: boolean
  auto_refresh?: boolean
  refresh_interval?: number // minutes
  data_ttl_days?: number // Days to keep data (default: 30)
  last_fetch?: string
  last_fetch_count?: number // Records fetched in last operation
  total_records?: number // Total records in database for this source
  created_at: string
  updated_at: string
}

export interface FetchHistory {
  id: string
  data_source_id: string
  timestamp: string
  record_count: number
  success: boolean
  error?: string
}

export const useDataSourceStore = defineStore('dataSource', () => {
  const dataSources = ref<DataSourceConfig[]>([])
  const fetchHistory = ref<FetchHistory[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * M9: Get current data environment (dev or production)
   * This determines which DATA SOURCES are available, not which storage is used.
   *
   * In a real deployment, this could check:
   * - Environment variable (import.meta.env.MODE)
   * - Backend configuration
   * - User settings
   *
   * For now, we use Tauri availability as a proxy:
   * - Browser-only mode = likely development
   * - Tauri mode = could be dev or prod (would need additional config)
   */
  function getCurrentEnvironment(): 'dev' | 'production' {
    // TODO: In production, this should check actual environment config
    // For now, assume Tauri = production, browser = dev
    return isTauri() ? 'production' : 'dev'
  }

  /**
   * M9: Filter data sources by current environment
   * Returns only data sources that are compatible with the current environment
   */
  function getEnvironmentCompatibleDataSources(): DataSourceConfig[] {
    const currentEnv = getCurrentEnvironment()
    return dataSources.value.filter(ds => {
      if (!ds.environment) return true // Backward compatibility: no environment set = both
      return ds.environment === 'both' || ds.environment === currentEnv
    })
  }

  // Load data sources from storage
  async function loadDataSources() {
    try {
      // In browser mode, use localStorage
      if (!isTauri()) {
        const stored = localStorage.getItem('data-sources')
        if (stored) {
          dataSources.value = JSON.parse(stored)
        }
        return dataSources.value
      }

      // In Tauri mode, could load from file (future enhancement)
      // For now, still use localStorage
      const stored = localStorage.getItem('data-sources')
      if (stored) {
        dataSources.value = JSON.parse(stored)
      }
      return dataSources.value
    } catch (err: any) {
      error.value = `Failed to load data sources: ${err.message}`
      console.error(error.value)
      return []
    }
  }

  // Create a new data source
  async function createDataSource(
    config: Omit<DataSourceConfig, 'id' | 'created_at' | 'updated_at'>
  ): Promise<DataSourceConfig> {
    try {
      loading.value = true
      error.value = null

      const now = new Date().toISOString()
      const newSource: DataSourceConfig = {
        ...config,
        id: `ds_${Date.now()}`,
        created_at: now,
        updated_at: now,
      }

      if (!isTauri()) {
        // Browser mode: save to localStorage
        dataSources.value.push(newSource)
        localStorage.setItem('data-sources', JSON.stringify(dataSources.value))
        return newSource
      }

      // M9: Tauri mode - save to database
      await safeInvoke('save_data_source', { dataSource: newSource })
      dataSources.value.push(newSource)
      return newSource
    } catch (err: any) {
      error.value = `Failed to create data source: ${err.message}`
      console.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // Update an existing data source
  async function updateDataSource(id: string, updates: Partial<DataSourceConfig>) {
    try {
      loading.value = true
      error.value = null

      const index = dataSources.value.findIndex(ds => ds.id === id)
      if (index === -1) {
        throw new Error(`Data source ${id} not found`)
      }

      dataSources.value[index] = {
        ...dataSources.value[index],
        ...updates,
        updated_at: new Date().toISOString(),
      }

      if (!isTauri()) {
        // Browser mode: save to localStorage
        localStorage.setItem('data-sources', JSON.stringify(dataSources.value))
        return dataSources.value[index]
      }

      // M9: Tauri mode - update in database
      await safeInvoke('update_data_source', { id, updates })
      return dataSources.value[index]
    } catch (err: any) {
      error.value = `Failed to update data source: ${err.message}`
      console.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // Delete a data source
  async function deleteDataSource(id: string) {
    try {
      loading.value = true
      error.value = null

      const index = dataSources.value.findIndex(ds => ds.id === id)
      if (index === -1) {
        throw new Error(`Data source ${id} not found`)
      }

      dataSources.value.splice(index, 1)

      // Also remove from fetch history
      fetchHistory.value = fetchHistory.value.filter(h => h.data_source_id !== id)
      saveFetchHistory()

      if (!isTauri()) {
        // Browser mode: update localStorage
        localStorage.setItem('data-sources', JSON.stringify(dataSources.value))
        return
      }

      // M9: Tauri mode - delete from database
      await safeInvoke('delete_data_source', { id })
    } catch (err: any) {
      error.value = `Failed to delete data source: ${err.message}`
      console.error(error.value)
      throw err
    } finally {
      loading.value = false
    }
  }

  // Test connection to a data source
  async function testConnection(config: DataSourceConfig): Promise<boolean> {
    if (!isTauri()) {
      // In browser mode, simulate success
      await new Promise(resolve => setTimeout(resolve, 1000))
      return true
    }

    try {
      loading.value = true
      error.value = null

      // M5.1: Restore encrypted credentials before testing
      let testConfig = { ...config }
      if (testConfig.auth && (testConfig.auth as any).token === '***SECURED***') {
        const { restoreAuthConfig } = await import('../composables/useCredentialMigration')
        testConfig.auth = await restoreAuthConfig(config.id, testConfig.auth)
      }

      const result = await safeInvoke('test_adapter_connection', { config: testConfig })
      return result as boolean
    } catch (err: any) {
      error.value = `Connection failed: ${err.message || err}`
      console.error('Connection test error:', err)
      return false
    } finally {
      loading.value = false
    }
  }

  // Fetch data from a data source
  // deepFetch: if true, uses configured max_pages; if false, always uses 1 page
  async function fetchData(dataSourceId: string, deepFetch: boolean = false): Promise<number> {
    console.log('🚀 fetchData() called!')
    console.log('🚀 dataSourceId:', dataSourceId)
    console.log('🚀 deepFetch:', deepFetch)
    console.log('🚀 Total dataSources:', dataSources.value.length)
    console.log(
      '🚀 DataSource IDs:',
      dataSources.value.map(ds => ds.id)
    )

    const dataSource = dataSources.value.find(ds => ds.id === dataSourceId)
    console.log('🚀 Found dataSource:', dataSource ? 'YES' : 'NO')

    if (!dataSource) {
      console.error('❌ Data source not found!', dataSourceId)
      throw new Error(`Data source ${dataSourceId} not found`)
    }

    console.log('🚀 DataSource config:', JSON.parse(JSON.stringify(dataSource)))

    if (!isTauri()) {
      // In browser mode, simulate fetch
      await new Promise(resolve => setTimeout(resolve, 1500))
      const mockCount = Math.floor(Math.random() * 50) + 10

      // Add to history
      addFetchHistory(dataSourceId, mockCount, true)

      // Update last fetch info
      updateDataSource(dataSourceId, {
        last_fetch: new Date().toISOString(),
        last_fetch_count: mockCount,
      })

      return mockCount
    }

    try {
      loading.value = true
      error.value = null

      // M5.1: Restore encrypted credentials before sending to backend
      // If credentials are secured (***SECURED***), decrypt them first
      let authConfig = dataSource.auth
      if (authConfig && (authConfig as any).token === '***SECURED***') {
        // Import dynamically to avoid circular dependency
        const { restoreAuthConfig } = await import('../composables/useCredentialMigration')
        authConfig = await restoreAuthConfig(dataSource.id, authConfig)
      }

      // Transform data source config for Rust backend
      const configForRust: any = {
        ...dataSource,
        auth: authConfig, // Use decrypted auth config
      }

      // Convert project_id to string and map field names for GitLab adapter
      if (dataSource.adapter_type === 'gitlab' && dataSource.parameters?.project_id) {
        // Normal fetch: always 1 page
        // Deep fetch: use configured max_pages (default 10 if not set)
        const maxPages = deepFetch ? dataSource.parameters.max_pages || 10 : 1

        configForRust.parameters = {
          project_id: String(dataSource.parameters.project_id),
          per_page: dataSource.parameters.per_page || 20,
          max_pages: maxPages,
          fetch_pipelines: dataSource.parameters.fetch_pipelines ?? true,
          include_jobs: dataSource.parameters.fetch_jobs ?? false,
        }

        // Add optional filters if present
        if (dataSource.parameters.status) {
          configForRust.parameters.status = dataSource.parameters.status
        }
        if (dataSource.parameters.ref) {
          configForRust.parameters.ref = dataSource.parameters.ref
        }

        console.log(
          '🚀 Built parameters for GitLab:',
          JSON.parse(JSON.stringify(configForRust.parameters))
        )
      }

      console.log(
        '🚀 Final config being sent to backend:',
        JSON.parse(JSON.stringify(configForRust))
      )
      console.log('🚀 Calling safeInvoke with fetch_adapter_data...')

      const count = (await safeInvoke('fetch_adapter_data', { config: configForRust })) as number

      console.log('🚀 Backend returned count:', count)

      // Add to history
      addFetchHistory(dataSourceId, count, true)

      // Get total records in database for this source
      let totalRecords = 0
      try {
        const recordsBySource = (await safeInvoke('get_staged_records', {
          limit: 100000,
          offset: 0,
        })) as any[]
        totalRecords = recordsBySource.filter(r => r.source === dataSource.source).length
      } catch (e) {
        console.warn('Could not get total record count:', e)
      }

      // Update last fetch info AND total records
      updateDataSource(dataSourceId, {
        last_fetch: new Date().toISOString(),
        last_fetch_count: count, // Records fetched in this operation
        total_records: totalRecords, // Total records in database for this source
      })

      return count
    } catch (err: any) {
      error.value = `Fetch failed: ${err.message || err}`
      console.error('Fetch error:', err)

      // Add failed fetch to history
      addFetchHistory(dataSourceId, 0, false, err.message || err.toString())

      throw err
    } finally {
      loading.value = false
    }
  }

  // Add fetch history entry
  function addFetchHistory(
    dataSourceId: string,
    recordCount: number,
    success: boolean,
    errorMsg?: string
  ) {
    const entry: FetchHistory = {
      id: `fh_${Date.now()}`,
      data_source_id: dataSourceId,
      timestamp: new Date().toISOString(),
      record_count: recordCount,
      success,
      error: errorMsg,
    }

    fetchHistory.value.unshift(entry) // Add to beginning

    // Keep only last 100 entries
    if (fetchHistory.value.length > 100) {
      fetchHistory.value = fetchHistory.value.slice(0, 100)
    }

    saveFetchHistory()
  }

  // Save fetch history
  function saveFetchHistory() {
    try {
      localStorage.setItem('fetch-history', JSON.stringify(fetchHistory.value))
    } catch (err: any) {
      console.error('Failed to save fetch history:', err)
    }
  }

  // Load fetch history
  function loadFetchHistory() {
    try {
      const stored = localStorage.getItem('fetch-history')
      if (stored) {
        fetchHistory.value = JSON.parse(stored)
      }
    } catch (err: any) {
      console.error('Failed to load fetch history:', err)
    }
  }

  // Get history for a specific data source
  function getDataSourceHistory(dataSourceId: string): FetchHistory[] {
    return fetchHistory.value.filter(h => h.data_source_id === dataSourceId)
  }

  // Get available adapter types
  async function getAvailableAdapters(): Promise<string[]> {
    if (!isTauri()) {
      // In browser mode, return known adapters
      return ['rest_api', 'gitlab']
    }

    try {
      const adapters = (await safeInvoke('list_adapters')) as string[]
      return adapters
    } catch (err: any) {
      console.error('Failed to list adapters:', err)
      return ['rest_api', 'gitlab'] // Fallback
    }
  }

  // Get default config for an adapter type
  async function getAdapterDefaultConfig(adapterType: string): Promise<any> {
    if (!isTauri()) {
      // Return mock config based on type
      if (adapterType === 'rest_api') {
        return {
          adapter_type: 'rest_api',
          source: 'rest-source',
          endpoint: 'https://api.example.com/data',
          auth: null,
          parameters: {
            method: 'GET',
            headers: {},
            data_path: '',
            default_tags: [],
          },
        }
      } else if (adapterType === 'gitlab') {
        return {
          adapter_type: 'gitlab',
          source: 'gitlab-source',
          endpoint: 'https://gitlab.com',
          auth: {
            type: 'bearer',
            token: '',
          },
          parameters: {
            project_id: 0,
            fetch_pipelines: true,
            fetch_jobs: true,
          },
        }
      }
      return {}
    }

    try {
      const config = await safeInvoke('get_adapter_default_config', { adapterType })
      return config
    } catch (err: any) {
      console.error('Failed to get default config:', err)
      return {}
    }
  }

  // NEW: Get all enabled data sources for multi-source selection
  function getEnabledDataSources(): DataSourceConfig[] {
    return dataSources.value.filter(ds => ds.enabled)
  }

  // NEW: Get records by source name/id (for multi-source aggregation)
  async function getRecordsBySource(sourceIds: string[]): Promise<any[]> {
    if (!isTauri()) {
      // In browser mode, generate mock data for each source
      return sourceIds.flatMap(sourceId => {
        const statuses = ['success', 'failed', 'running', 'pending', 'canceled']
        return Array.from({ length: 20 }, (_, i) => ({
          id: `mock-${sourceId}-${i}`,
          record_type: 'gitlab_pipeline',
          source: sourceId,
          timestamp: new Date(Date.now() - i * 86400000).toISOString(), // Daily data
          data: { id: i, name: `Pipeline ${i}` },
          metadata: {
            status: statuses[i % statuses.length],
            title: `Pipeline ${i}`,
            tags: ['mock', 'test'],
          },
        }))
      })
    }

    try {
      // Fetch all records and filter by source
      const allRecords = (await safeInvoke('get_staged_records', {
        limit: 10000,
        offset: 0,
      })) as any[]

      // Filter to only requested sources
      return allRecords.filter(record => sourceIds.includes(record.source))
    } catch (err: any) {
      console.error('Failed to fetch records by source:', err)
      throw err
    }
  }

  // Refresh total record counts for all data sources
  // Call this on app startup to populate total_records field
  async function refreshTotalRecords() {
    if (!isTauri()) return

    try {
      // Get all records from database
      const allRecords = (await safeInvoke('get_staged_records', {
        limit: 100000,
        offset: 0,
      })) as any[]

      // Count records per source
      const countsBySource = new Map<string, number>()
      allRecords.forEach(record => {
        const source = record.source || 'unknown'
        countsBySource.set(source, (countsBySource.get(source) || 0) + 1)
      })

      // Update each data source with its total count
      for (const ds of dataSources.value) {
        const totalRecords = countsBySource.get(ds.source) || 0
        if (ds.total_records !== totalRecords) {
          // Only update if changed (avoid unnecessary saves)
          updateDataSource(ds.id, { total_records: totalRecords })
        }
      }
    } catch (err) {
      console.warn('Failed to refresh total record counts:', err)
    }
  }

  return {
    dataSources,
    fetchHistory,
    loading,
    error,
    loadDataSources,
    loadFetchHistory,
    createDataSource,
    updateDataSource,
    deleteDataSource,
    testConnection,
    fetchData,
    getDataSourceHistory,
    getAvailableAdapters,
    getAdapterDefaultConfig,
    getEnabledDataSources,
    getRecordsBySource,
    refreshTotalRecords,
    // M9: Environment support
    getCurrentEnvironment,
    getEnvironmentCompatibleDataSources,
  }
})
