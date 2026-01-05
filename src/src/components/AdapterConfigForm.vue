<template>
  <div class="modal-overlay" @click="$emit('cancel')">
    <div class="config-form" @click.stop>
      <div class="form-header">
        <h2>
          {{ existingConfig ? 'Edit' : 'Configure' }}
          {{ getAdapterName(adapterType) }}
        </h2>
        <button @click="$emit('cancel')" class="btn-close">×</button>
      </div>

      <form @submit.prevent="handleSubmit" class="form-body">
        <!-- Common Fields -->
        <div class="form-section">
          <h3>Basic Information</h3>

          <div class="form-group">
            <label>Name <span class="required">*</span></label>
            <input v-model="config.name" type="text" placeholder="My Data Source" required />
          </div>

          <div class="form-group">
            <label>Source Identifier <span class="required">*</span></label>
            <input v-model="config.source" type="text" placeholder="my-source" required />
            <small>Used to identify records from this source</small>
          </div>
        </div>

        <!-- REST API Specific Fields -->
        <div v-if="adapterType === 'rest_api'" class="form-section">
          <h3>REST API Configuration</h3>

          <div class="form-group">
            <label>Endpoint URL <span class="required">*</span></label>
            <input
              v-model="config.endpoint"
              type="url"
              placeholder="https://api.example.com/data"
              required
            />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>HTTP Method</label>
              <select v-model="restConfig.method">
                <option value="GET">GET</option>
                <option value="POST">POST</option>
                <option value="PUT">PUT</option>
                <option value="DELETE">DELETE</option>
              </select>
            </div>

            <div class="form-group">
              <label>Data Path (JSON)</label>
              <input v-model="restConfig.data_path" type="text" placeholder="data.items" />
              <small>Path to array in response (leave empty for root)</small>
            </div>
          </div>
        </div>

        <!-- Plugin-Specific Configuration -->
        <!-- Dynamically loads configuration UI from plugins -->
        <component
          v-if="pluginConfigComponent"
          :is="pluginConfigComponent"
          :config="config"
          @update:config="updateConfig"
          class="form-section"
        />

        <!-- Fallback: Generic adapter configuration for adapters without plugins -->
        <div v-else-if="adapterType !== 'rest_api'" class="form-section">
          <h3>{{ getAdapterName(adapterType) }} Configuration</h3>
          <div class="form-group">
            <label>Endpoint URL <span class="required">*</span></label>
            <input
              v-model="config.endpoint"
              type="url"
              placeholder="https://api.example.com"
              required
            />
          </div>
          <div class="info-box">
            ℹ️ This adapter doesn't have a plugin-specific configuration UI yet. You can configure
            advanced settings via the parameters field after saving.
          </div>
        </div>

        <!-- Authentication -->
        <div class="form-section">
          <h3>Authentication</h3>

          <div class="form-group">
            <label>Authentication Type</label>
            <select v-model="config.auth.type">
              <option value="none">None</option>
              <option value="bearer">Bearer Token</option>
              <option value="basic">Basic Auth</option>
              <option value="apikey">API Key</option>
              <option value="gitlabtoken">GitLab Token</option>
            </select>
          </div>

          <!-- Bearer Token -->
          <div v-if="config.auth.type === 'bearer'" class="form-group">
            <label>Bearer Token <span class="required">*</span></label>
            <input v-model="config.auth.token" type="password" placeholder="Enter token" required />
          </div>

          <!-- GitLab Token -->
          <div v-if="config.auth.type === 'gitlabtoken'" class="form-group">
            <label>GitLab Personal Access Token <span class="required">*</span></label>
            <input
              v-model="config.auth.token"
              type="password"
              placeholder="Enter GitLab token"
              required
            />
          </div>

          <!-- Basic Auth -->
          <template v-if="config.auth.type === 'basic'">
            <div class="form-group">
              <label>Username <span class="required">*</span></label>
              <input v-model="config.auth.username" type="text" placeholder="username" required />
            </div>
            <div class="form-group">
              <label>Password <span class="required">*</span></label>
              <input
                v-model="config.auth.password"
                type="password"
                placeholder="password"
                required
              />
            </div>
          </template>

          <!-- API Key -->
          <template v-if="config.auth.type === 'apikey'">
            <div class="form-group">
              <label>API Key <span class="required">*</span></label>
              <input
                v-model="config.auth.api_key"
                type="password"
                placeholder="Enter API key"
                required
              />
            </div>
            <div class="form-group">
              <label>Header Name</label>
              <input v-model="config.auth.api_key_header" type="text" placeholder="X-API-Key" />
            </div>
          </template>
        </div>

        <!-- Advanced Options -->
        <div class="form-section">
          <h3>Advanced Options</h3>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.enabled" />
              Enable this data source
            </label>
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="config.auto_refresh" />
              Auto-refresh data
            </label>
          </div>

          <div v-if="config.auto_refresh" class="form-group">
            <label>Refresh Interval (minutes)</label>
            <input
              v-model.number="config.refresh_interval"
              type="number"
              min="1"
              placeholder="15"
            />
          </div>

          <div class="form-group">
            <label>Data Retention (days)</label>
            <input v-model.number="config.data_ttl_days" type="number" min="1" placeholder="30" />
            <small class="form-hint">
              Records older than this will be automatically deleted. Default: 30 days.
            </small>
          </div>
        </div>

        <!-- Test Connection -->
        <div
          v-if="testResult"
          class="test-result"
          :class="testResult.success ? 'success' : 'error'"
        >
          {{ testResult.message }}
        </div>

        <!-- Actions -->
        <div class="form-actions">
          <button
            type="button"
            @click="testConnection"
            class="btn btn-test"
            :disabled="testing || saving"
          >
            <span v-if="testing" class="spinner"></span>
            {{ testing ? 'Testing...' : '🔍 Test Connection' }}
          </button>
          <div class="action-spacer"></div>
          <button
            type="button"
            @click="$emit('cancel')"
            class="btn btn-cancel"
            :disabled="testing || saving"
          >
            Cancel
          </button>
          <button type="submit" class="btn btn-submit" :disabled="testing || saving">
            <span v-if="saving" class="spinner"></span>
            {{ saving ? 'Saving...' : existingConfig ? 'Update' : 'Save & Fetch' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useDataSourceStore, type DataSourceConfig } from '../stores/dataSourceStore'

const props = defineProps<{
  adapterType: string
  existingConfig?: DataSourceConfig | null
}>()

const emit = defineEmits<{
  save: [config: any]
  cancel: []
}>()

const dataSourceStore = useDataSourceStore()

// Check if there's a plugin that provides a config component for this adapter type
// TODO: Implement getPluginForAdapterType method in pluginLoader
const pluginConfigComponent = computed(() => {
  return null
})

const config = reactive<any>({
  name: '',
  adapter_type: props.adapterType,
  source: '',
  endpoint: '',
  auth: {
    type: 'none',
  },
  parameters: {},
  enabled: true,
  auto_refresh: false,
  refresh_interval: 15,
})

// Update config when plugin component emits changes
const updateConfig = (updates: any) => {
  console.log('📝 Plugin config update received:', updates)
  console.log('📝 Current config before update:', JSON.parse(JSON.stringify(config)))
  Object.assign(config, updates)
  // Also update parameters if provided
  if (updates.parameters) {
    console.log('📝 Merging parameters:', updates.parameters)
    config.parameters = { ...config.parameters, ...updates.parameters }
  }
  console.log('📝 Config after update:', JSON.parse(JSON.stringify(config)))
}

const restConfig = reactive({
  method: 'GET',
  headers: {},
  data_path: '',
  default_tags: [] as string[],
})

const testing = ref(false)
const saving = ref(false)
const testResult = ref<{ success: boolean; message: string } | null>(null)

onMounted(async () => {
  if (props.existingConfig) {
    // Editing existing source
    console.log('🔍 Loading existing config:', props.existingConfig)
    Object.assign(config, props.existingConfig)
    if (props.adapterType === 'rest_api' && config.parameters) {
      Object.assign(restConfig, config.parameters)
    }
    // For other adapters (like GitLab), parameters are handled by their plugin components
  } else {
    // New source - get defaults
    try {
      const defaults = await dataSourceStore.getAdapterDefaultConfig(props.adapterType)
      if (defaults.endpoint) config.endpoint = defaults.endpoint
      if (defaults.source) config.source = defaults.source
      if (defaults.parameters) {
        if (props.adapterType === 'rest_api') {
          Object.assign(restConfig, defaults.parameters)
        } else {
          // For plugin-based adapters, store parameters directly
          config.parameters = defaults.parameters
        }
      }
      // Set default auth type for GitLab
      if (props.adapterType === 'gitlab') {
        config.auth.type = 'gitlabtoken'
      }
    } catch (err) {
      console.error('Failed to get defaults:', err)
    }
  }
})

function getAdapterName(type: string): string {
  const names: Record<string, string> = {
    rest_api: 'REST API',
    gitlab: 'GitLab',
  }
  return names[type] || type
}

async function testConnection() {
  testing.value = true
  testResult.value = null

  try {
    const testConfig = buildConfig()
    const success = await dataSourceStore.testConnection(testConfig)

    testResult.value = {
      success,
      message: success
        ? '✅ Connection successful!'
        : `❌ ${dataSourceStore.error || 'Connection failed. Please check your configuration.'}`,
    }
  } catch (err: any) {
    console.error('Test connection exception:', err)
    testResult.value = {
      success: false,
      message: `❌ Connection error: ${err.message || err}`,
    }
  } finally {
    testing.value = false
  }
}

function buildConfig(): any {
  const finalConfig = { ...config }

  // Add adapter-specific parameters
  if (props.adapterType === 'rest_api') {
    finalConfig.parameters = { ...restConfig }
  }
  // For other adapters (like GitLab), parameters are already set by plugin components via updateConfig
  // No need to build them here - they're already in config.parameters

  // Transform auth to match Rust serde format
  if (finalConfig.auth.type === 'none' || !finalConfig.auth.type) {
    finalConfig.auth = null
  } else {
    // Build auth object based on type
    const authType = finalConfig.auth.type
    const transformedAuth: any = { type: authType }

    if (authType === 'bearer' || authType === 'gitlabtoken') {
      transformedAuth.token = finalConfig.auth.token
    } else if (authType === 'basic') {
      transformedAuth.username = finalConfig.auth.username
      transformedAuth.password = finalConfig.auth.password
    } else if (authType === 'apikey') {
      transformedAuth.header_name = finalConfig.auth.api_key_header || 'X-API-Key'
      transformedAuth.key = finalConfig.auth.api_key
    }

    finalConfig.auth = transformedAuth
  }

  return finalConfig
}

function handleSubmit() {
  console.log('💾 handleSubmit called')
  console.log('💾 Current config:', JSON.parse(JSON.stringify(config)))
  saving.value = true
  try {
    const finalConfig = buildConfig()
    console.log('💾 Built config to save:', JSON.parse(JSON.stringify(finalConfig)))
    emit('save', finalConfig)
    console.log('💾 Save event emitted')
  } finally {
    // Keep saving state for a moment to prevent double-clicks
    setTimeout(() => {
      saving.value = false
    }, 500)
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.config-form {
  background: var(--bg-modal);
  color: var(--text-primary);
  border-radius: 12px;
  width: 90%;
  max-width: 700px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.form-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
}

.form-header h2 {
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

.form-section {
  margin-bottom: 24px;
  padding-bottom: 24px;
  border-bottom: 1px solid var(--border-subtle);
}

.form-section h3 {
  margin: 0 0 16px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.form-group input[type='text'],
.form-group input[type='url'],
.form-group input[type='password'],
.form-group input[type='number'],
.form-group select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
  background: var(--bg-panel);
  color: var(--text-primary);
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-group small {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: var(--text-secondary);
}

.form-hint {
  display: block;
  margin-top: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.btn-test {
  background: var(--bg-panel-header);
  color: var(--text-primary);
  border-color: var(--border-color);
}

.btn-test:hover:not(:disabled) {
  background: var(--border-subtle);
}

.btn-cancel {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-color);
}

.btn-cancel:hover {
  background: var(--bg-panel-header);
}

.btn-submit {
  background: var(--bg-button);
  color: var(--text-on-accent);
}

.btn-submit:hover {
  background: var(--bg-button-hover);
}

.test-result.success {
  background: color-mix(in srgb, var(--accent-success) 18%, transparent);
  color: var(--accent-success);
  border: 1px solid color-mix(in srgb, var(--accent-success) 40%, transparent);
}

.test-result.error {
  background: color-mix(in srgb, var(--accent-danger) 18%, transparent);
  color: var(--accent-danger);
  border: 1px solid color-mix(in srgb, var(--accent-danger) 40%, transparent);
}

.info-box {
  padding: 12px 16px;
  background: color-mix(in srgb, var(--accent-primary) 12%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-primary) 25%, transparent);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  line-height: 1.5;
  margin-top: 12px;
}

.form-actions {
  display: flex;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}
</style>
