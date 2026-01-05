<template>
  <div class="adapter-manager">
    <div class="manager-header">
      <h2>📡 Data Source Manager</h2>
      <button @click="$emit('close')" class="btn-close">×</button>
    </div>

    <div class="tab-navigation">
      <button
        @click="activeTab = 'available'"
        :class="['tab-button', { active: activeTab === 'available' }]"
      >
        Available Adapters
      </button>
      <button
        @click="activeTab = 'sources'"
        :class="['tab-button', { active: activeTab === 'sources' }]"
      >
        My Data Sources ({{ dataSourceStore.dataSources.length }})
      </button>
    </div>

    <div class="tab-content">
      <!-- Available Adapters Tab -->
      <div v-if="activeTab === 'available'" class="available-adapters">
        <p class="tab-description">
          Connect to external data sources and fetch data into your dashboards.
        </p>

        <div class="adapter-list">
          <div v-for="adapter in availableAdapters" :key="adapter.type" class="adapter-card">
            <div class="adapter-icon">{{ adapter.icon }}</div>
            <div class="adapter-info">
              <h3>{{ adapter.name }}</h3>
              <p>{{ adapter.description }}</p>
              <div class="adapter-features">
                <span v-for="feature in adapter.features" :key="feature" class="feature-tag">
                  {{ feature }}
                </span>
              </div>
            </div>
            <button @click="configureAdapter(adapter.type)" class="btn-configure">Configure</button>
          </div>
        </div>
      </div>

      <!-- My Data Sources Tab -->
      <div v-if="activeTab === 'sources'" class="my-data-sources">
        <div v-if="dataSourceStore.dataSources.length === 0" class="empty-state">
          <p class="empty-icon">📭</p>
          <h3>No data sources configured</h3>
          <p>Click "Available Adapters" to configure your first data source.</p>
        </div>

        <div v-else class="source-list">
          <DataSourceCard
            v-for="source in dataSourceStore.dataSources"
            :key="source.id"
            :source="source"
            @fetch="handleFetch"
            @deepFetch="handleDeepFetch"
            @edit="handleEdit"
            @delete="handleDelete"
          />
        </div>
      </div>
    </div>

    <!-- Configuration Modal -->
    <AdapterConfigForm
      v-if="showConfigForm"
      :adapter-type="selectedAdapterType"
      :existing-config="editingSource"
      @save="handleSave"
      @cancel="closeConfigForm"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDataSourceStore, type DataSourceConfig } from '../stores/dataSourceStore'
import { useToast } from '../composables/useToast'
import DataSourceCard from './DataSourceCard.vue'
import AdapterConfigForm from './AdapterConfigForm.vue'

defineEmits<{
  close: []
}>()

const dataSourceStore = useDataSourceStore()
const toast = useToast()

const activeTab = ref<'available' | 'sources'>('available')
const showConfigForm = ref(false)
const selectedAdapterType = ref<string>('')
const editingSource = ref<DataSourceConfig | null>(null)

const availableAdapters = ref([
  {
    type: 'rest_api',
    name: 'REST API',
    icon: '📊',
    description: 'Fetch data from any REST API endpoint with flexible authentication options.',
    features: ['OAuth2', 'Bearer Token', 'API Key', 'Custom Headers'],
  },
  {
    type: 'gitlab',
    name: 'GitLab',
    icon: '🔧',
    description: 'Monitor GitLab pipelines and jobs for your projects.',
    features: ['Pipelines', 'Jobs', 'Status Tracking', 'CI/CD Metrics'],
  },
])

onMounted(async () => {
  await dataSourceStore.loadDataSources()
  dataSourceStore.loadFetchHistory()

  // Try to get available adapters from backend
  try {
    await dataSourceStore.getAvailableAdapters()
  } catch {
    // Silently fall back to default adapter list
  }
})

function configureAdapter(adapterType: string) {
  selectedAdapterType.value = adapterType
  editingSource.value = null
  showConfigForm.value = true
}

async function handleFetch(sourceId: string) {
  console.log('🟢 AdapterManager.handleFetch() called!')
  console.log('🟢 sourceId:', sourceId)
  console.log('🟢 Total sources:', dataSourceStore.dataSources.length)

  try {
    const source = dataSourceStore.dataSources.find(s => s.id === sourceId)
    console.log('🟢 Found source:', source ? source.name : 'NOT FOUND')

    if (!source) {
      console.error('❌ Source not found!', sourceId)
      toast.error(`Data source not found: ${sourceId}`)
      return
    }

    console.log('🟢 Showing info toast...')
    toast.info(`Fetching data from ${source?.name || 'source'}...`)

    console.log('🟢 Calling dataSourceStore.fetchData()...')
    const count = await dataSourceStore.fetchData(sourceId)
    console.log('🟢 Fetch complete! Count:', count)

    toast.success(`Successfully fetched ${count} records!`)
  } catch (err: any) {
    console.error('❌ Fetch error in AdapterManager:', err)
    toast.error(`Fetch failed: ${err.message || err}`)
  }
}

async function handleDeepFetch(sourceId: string) {
  try {
    const source = dataSourceStore.dataSources.find(s => s.id === sourceId)
    if (!source) return

    // Use configured max_pages or default to 10 if not set
    const maxPages = source.parameters?.max_pages || 10

    const confirmed = confirm(
      `Deep Fetch will fetch ${maxPages} pages of historical data (up to ${maxPages * 20} records). This may take a while and use API quota. Continue?`
    )

    if (!confirmed) return

    toast.info(`Starting deep fetch from ${source.name}... Fetching ${maxPages} pages.`)

    // Pass deepFetch: true to use configured max_pages instead of always 1
    const count = await dataSourceStore.fetchData(sourceId, true)

    toast.success(`Deep fetch complete! Fetched ${count} records from historical data.`)
  } catch (err: any) {
    toast.error(`Deep fetch failed: ${err.message || err}`)
  }
}

function handleEdit(source: DataSourceConfig) {
  selectedAdapterType.value = source.adapter_type
  editingSource.value = source
  showConfigForm.value = true
}

async function handleDelete(sourceId: string) {
  const source = dataSourceStore.dataSources.find(s => s.id === sourceId)
  if (!source) return

  if (confirm(`Delete data source "${source.name}"?\n\nThis cannot be undone.`)) {
    try {
      dataSourceStore.deleteDataSource(sourceId)
      toast.success('Data source deleted successfully')
    } catch (err: any) {
      toast.error(`Failed to delete: ${err.message || err}`)
    }
  }
}

async function handleSave(config: any) {
  try {
    let sourceId: string

    if (editingSource.value) {
      // Update existing
      dataSourceStore.updateDataSource(editingSource.value.id, config)
      toast.success('Data source updated successfully')
      sourceId = editingSource.value.id
    } else {
      // Create new
      const newSource = await dataSourceStore.createDataSource(config)
      toast.success('Data source created successfully')
      sourceId = newSource.id

      // Auto-fetch for new datasources (matches "Save & Fetch" button label)
      try {
        toast.info(`Fetching data from ${config.name}...`)
        const count = await dataSourceStore.fetchData(sourceId)
        toast.success(`Successfully fetched ${count} records!`)
      } catch (err: any) {
        toast.error(`Fetch failed: ${err.message || err}`)
      }
    }

    closeConfigForm()
    activeTab.value = 'sources' // Switch to sources tab
  } catch (err: any) {
    toast.error(`Failed to save: ${err.message || err}`)
  }
}

function closeConfigForm() {
  showConfigForm.value = false
  selectedAdapterType.value = ''
  editingSource.value = null
}
</script>

<style scoped>
.adapter-manager {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 90%;
  max-width: 900px;
  max-height: 90vh;
  background: var(--bg-modal);
  color: var(--text-primary);
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  z-index: 1000;
}

.manager-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
}

.manager-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-heading);
}

.btn-close {
  background: none;
  border: none;
  font-size: 32px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-close:hover {
  background: var(--bg-panel-header);
  color: var(--text-heading);
}

.tab-navigation {
  display: flex;
  border-bottom: 1px solid var(--border-color);
  padding: 0 24px;
}

.tab-button {
  padding: 12px 20px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-button:hover {
  color: var(--text-heading);
}

.tab-button.active {
  color: var(--accent-primary);
  border-bottom-color: var(--accent-primary);
}

.tab-description {
  margin: 0 0 24px 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.adapter-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.adapter-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  transition: all 0.2s;
}

.adapter-card:hover {
  background: var(--bg-panel-header);
  border-color: var(--border-color);
}

.adapter-icon {
  font-size: 48px;
  flex-shrink: 0;
}

.adapter-info {
  flex: 1;
}

.adapter-info h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-heading);
}

.adapter-info p {
  margin: 0 0 12px 0;
  color: var(--text-secondary);
  font-size: 14px;
  line-height: 1.5;
}

.adapter-features {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.feature-tag {
  padding: 4px 10px;
  background: var(--bg-panel-header);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-secondary);
}

.btn-configure {
  padding: 10px 24px;
  background: var(--bg-button);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
  flex-shrink: 0;
}

.btn-configure:hover {
  background: var(--bg-button-hover);
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-secondary);
}

.empty-state h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  color: var(--text-heading);
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.source-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
</style>
