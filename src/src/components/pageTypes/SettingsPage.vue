<template>
  <div class="settings-page">
    <div class="settings-header">
      <h1>{{ config?.title || 'Settings' }}</h1>
      <p v-if="config?.description" class="settings-description">{{ config.description }}</p>
    </div>

    <div class="settings-content">
      <div v-if="!config?.sections || config.sections.length === 0" class="empty-state">
        <h2>⚙️ No Settings Defined</h2>
        <p>This settings page hasn't been configured yet.</p>
        <p class="hint">Add settings sections in the page configuration.</p>
      </div>

      <div v-else class="settings-sections">
        <div v-for="(section, index) in config.sections" :key="index" class="settings-section">
          <h2>{{ section.title }}</h2>
          <p v-if="section.description" class="section-description">{{ section.description }}</p>

          <div class="settings-fields">
            <SettingField
              v-for="field in section.fields"
              :key="field.id"
              :field="field"
              :value="values[field.id]"
              @update="handleFieldUpdate"
            />
          </div>
        </div>
      </div>

      <div v-if="config?.sections && config.sections.length > 0" class="settings-actions">
        <button @click="handleSave" class="btn-primary" :disabled="saving">
          {{ saving ? 'Saving...' : 'Save Settings' }}
        </button>
        <button @click="handleReset" class="btn-secondary">Reset to Defaults</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import SettingField from '@/components/settings/SettingField.vue'

// Helper to check if we're running inside the Tauri app
const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

// Safe invoke wrapper (only available in Tauri)
const safeInvoke = async (command: string, args?: any) => {
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke(command, args)
}

function storageKey(): string {
  return props.config?.storageKey || 'settings-page-values'
}

function pluginDataId(): { pluginId: string; panelId: string; key: string } {
  // Store SettingsPage values in the DB using plugin_data.
  // We treat this as a "virtual plugin" so it is namespaced and migratable.
  return {
    pluginId: 'core.settings-page',
    panelId: storageKey(),
    key: 'values',
  }
}

const props = defineProps<{
  config?: {
    title?: string
    description?: string
    sections?: Array<{
      title: string
      description?: string
      fields: Array<{
        id: string
        type: string
        label: string
        description?: string
        default?: any
        required?: boolean
        validation?: any
        options?: Array<{ value: any; label: string }>
      }>
    }>
    storageKey?: string
  }
}>()

const values = reactive<Record<string, any>>({})
const saving = ref(false)

onMounted(() => {
  loadSettings()
})

async function loadSettings() {
  // In Tauri mode: load from DB (plugin_data)
  if (isTauri()) {
    try {
      const ids = pluginDataId()
      const stored = (await safeInvoke('get_plugin_data', {
        pluginId: ids.pluginId,
        panelId: ids.panelId,
        key: ids.key,
      })) as string | null

      // One-time migration: if DB has no value yet, but localStorage does,
      // import it into the DB. This helps recover after app renames.
      if (!stored) {
        const legacy = localStorage.getItem(storageKey())
        if (legacy) {
          try {
            // Validate it is JSON before storing
            JSON.parse(legacy)
            await safeInvoke('save_plugin_data', {
              pluginId: ids.pluginId,
              panelId: ids.panelId,
              key: ids.key,
              value: legacy,
              dataType: 'json',
            })
            // Re-fetch from DB to keep a single source of truth
            const reloaded = (await safeInvoke('get_plugin_data', {
              pluginId: ids.pluginId,
              panelId: ids.panelId,
              key: ids.key,
            })) as string | null
            if (reloaded) {
              const parsed = JSON.parse(reloaded)
              Object.assign(values, parsed)
            }
          } catch (err) {
            console.warn('Found legacy localStorage settings but failed to migrate them:', err)
          }
        }
      }

      if (stored) {
        const parsed = JSON.parse(stored)
        Object.assign(values, parsed)
      }
    } catch (e) {
      // If DB read fails, fall back to localStorage for safety.
      console.error('Failed to load settings from DB. Falling back to localStorage:', e)
      const stored = localStorage.getItem(storageKey())
      if (stored) {
        try {
          const parsed = JSON.parse(stored)
          Object.assign(values, parsed)
        } catch (err) {
          console.error('Failed to load settings from localStorage fallback:', err)
        }
      }
    }
  } else {
    // Browser mode: localStorage
    const stored = localStorage.getItem(storageKey())
    if (stored) {
      try {
        const parsed = JSON.parse(stored)
        Object.assign(values, parsed)
      } catch (e) {
        console.error('Failed to load settings:', e)
      }
    }
  }

  // Apply defaults for missing values
  props.config?.sections?.forEach(section => {
    section.fields?.forEach(field => {
      if (!(field.id in values) && field.default !== undefined) {
        values[field.id] = field.default
      }
    })
  })
}

function handleFieldUpdate(fieldId: string, value: any) {
  values[fieldId] = value
}

async function handleSave() {
  saving.value = true

  try {
    if (isTauri()) {
      const ids = pluginDataId()
      await safeInvoke('save_plugin_data', {
        pluginId: ids.pluginId,
        panelId: ids.panelId,
        key: ids.key,
        value: JSON.stringify(values),
        dataType: 'json',
      })
    } else {
      // Browser mode: Save to localStorage
      localStorage.setItem(storageKey(), JSON.stringify(values))
    }

    // Show success feedback
    alert('Settings saved successfully!')
  } catch (e) {
    console.error('Failed to save settings:', e)
    alert('Failed to save settings: ' + e)
  } finally {
    saving.value = false
  }
}

function handleReset() {
  if (!confirm('Reset all settings to defaults?')) {
    return
  }

  // Reset to defaults
  props.config?.sections?.forEach(section => {
    section.fields?.forEach(field => {
      if (field.default !== undefined) {
        values[field.id] = field.default
      } else {
        delete values[field.id]
      }
    })
  })
}
</script>

<style scoped>
.settings-page {
  height: 100%;
  overflow: auto;
  background: #f8f9fa;
}

.settings-header {
  padding: 2rem;
  background: #ffffff;
  border-bottom: 1px solid #dee2e6;
}

.settings-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 1.75rem;
  color: #212529;
}

.settings-description {
  margin: 0;
  color: #6c757d;
  font-size: 0.95rem;
}

.settings-content {
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background: #ffffff;
  border-radius: 8px;
  border: 2px dashed #dee2e6;
}

.empty-state h2 {
  margin-top: 0;
  color: #495057;
}

.empty-state p {
  margin: 0.5rem 0;
  color: #6c757d;
}

.hint {
  font-size: 0.9rem;
  font-style: italic;
}

.settings-sections {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.settings-section {
  background: #ffffff;
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid #dee2e6;
}

.settings-section h2 {
  margin: 0 0 0.5rem 0;
  font-size: 1.25rem;
  color: #212529;
}

.section-description {
  margin: 0 0 1.5rem 0;
  color: #6c757d;
  font-size: 0.9rem;
}

.settings-fields {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.settings-actions {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  background: #ffffff;
  border-radius: 8px;
  border: 1px solid #dee2e6;
  margin-top: 2rem;
  position: sticky;
  bottom: 0;
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.1);
}

.btn-primary,
.btn-secondary {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background: #5a6268;
}
</style>
