<template>
  <div class="prompt-renderer-page">
    <div class="page-header">
      <h1>🎯 Prompt Renderer</h1>
      <p>Generate prompts from entry points with variables and context</p>
    </div>

    <div class="renderer-layout">
      <!-- Configuration Panel -->
      <aside class="config-panel">
        <div class="panel-section">
          <h3>🎯 Entry Point</h3>
          <select v-model="selectedEntryPointId" class="template-select">
            <option value="">-- Select Entry Point --</option>
            <optgroup v-for="ns in namespaces" :key="ns" :label="ns">
              <option
                v-for="entryPoint in entryPointsByNamespace(ns)"
                :key="getIdString(entryPoint.id)"
                :value="getIdString(entryPoint.id)"
              >
                {{ entryPoint.name }}
              </option>
            </optgroup>
          </select>

          <div v-if="selectedEntryPoint" class="template-info">
            <p class="template-description">{{ selectedEntryPoint.description }}</p>
            <div class="template-meta">
              <span class="badge">{{ selectedEntryPoint.namespace }}</span>
              <span v-for="tag in selectedEntryPoint.tags || []" :key="tag" class="tag">{{
                tag
              }}</span>
            </div>
          </div>
        </div>

        <div v-if="selectedEntryPoint" class="panel-section">
          <h3>📊 Variables</h3>
          <div class="variables-form">
            <div
              v-for="variable in selectedEntryPoint.variables || []"
              :key="variable.id"
              class="variable-field"
            >
              <label :for="variable.id">
                {{ variable.name || variable.id }}
                <span v-if="variable.required" class="required">*</span>
              </label>
              <p v-if="variable.description" class="field-description">
                {{ variable.description }}
              </p>

              <!-- String input -->
              <input
                v-if="variable.type === 'string'"
                :id="variable.id"
                v-model="variableValues[variable.id]"
                type="text"
                :required="variable.required"
                :placeholder="variable.default_value || ''"
              />

              <!-- Number input -->
              <input
                v-else-if="variable.type === 'number'"
                :id="variable.id"
                v-model.number="variableValues[variable.id]"
                type="number"
                :required="variable.required"
              />

              <!-- Boolean checkbox -->
              <label v-else-if="variable.type === 'boolean'" class="checkbox-label">
                <input :id="variable.id" v-model="variableValues[variable.id]" type="checkbox" />
                <span>{{ variable.name || variable.id }}</span>
              </label>

              <!-- Enum select -->
              <select
                v-else-if="variable.type === 'enum'"
                :id="variable.id"
                v-model="variableValues[variable.id]"
                :required="variable.required"
              >
                <option value="">-- Select --</option>
                <option v-for="val in variable.enum_values" :key="val" :value="val">
                  {{ val }}
                </option>
              </select>

              <!-- Array (comma-separated) -->
              <div v-else-if="variable.type === 'array'" class="array-input">
                <input
                  :id="variable.id"
                  :value="arrayToString(variableValues[variable.id])"
                  @change="
                    (e: Event) =>
                      handleArrayInput(variable.id, (e.target as HTMLInputElement).value)
                  "
                  @blur="
                    (e: Event) =>
                      handleArrayInput(variable.id, (e.target as HTMLInputElement).value)
                  "
                  type="text"
                  placeholder="item1, item2, item3"
                />
                <span class="hint"
                  >Comma-separated values (press Tab or click outside to apply)</span
                >
              </div>

              <!-- Textarea for long text -->
              <textarea
                v-else-if="variable.type === 'object'"
                :id="variable.id"
                :value="JSON.stringify(variableValues[variable.id], null, 2)"
                @input="
                  (e: Event) =>
                    handleObjectInput(variable.id, (e.target as HTMLTextAreaElement).value)
                "
                rows="4"
                placeholder="{}"
              />

              <!-- Default text input -->
              <input v-else :id="variable.id" v-model="variableValues[variable.id]" type="text" />
            </div>

            <div v-if="(selectedEntryPoint.variables || []).length === 0" class="no-variables">
              <p>This entry point has no variables</p>
            </div>
          </div>
        </div>

        <div class="panel-section">
          <h3>⚙️ Options</h3>
          <div class="options-form">
            <div class="option-field">
              <label for="count">Number of Outputs</label>
              <input
                id="count"
                v-model.number="renderOptions.count"
                type="number"
                min="1"
                max="10"
              />
            </div>
            <div class="option-field">
              <label for="format">Output Format</label>
              <select id="format" v-model="renderOptions.format">
                <option value="text">Plain Text</option>
                <option value="markdown">Markdown</option>
                <option value="json">JSON</option>
              </select>
            </div>
          </div>
        </div>

        <div class="panel-actions">
          <button
            @click="renderPrompts"
            class="btn-primary render-btn"
            :disabled="!canRender || rendering"
          >
            {{ rendering ? 'Rendering...' : '🚀 Render Prompts' }}
          </button>
        </div>
      </aside>

      <!-- Output Panel -->
      <main class="output-panel">
        <div class="output-header">
          <h3>📤 Output</h3>
          <div v-if="lastResult" class="output-meta">
            <span>{{ lastResult.outputs.length }} output(s)</span>
            <span>{{ lastResult.metadata.duration_ms }}ms</span>
          </div>
        </div>

        <div v-if="!lastResult" class="output-empty">
          <h2>⬅️ Configure and Render</h2>
          <p>Select a template, fill in variables, and click "Render Prompts"</p>
        </div>

        <div v-else-if="lastResult.errors && lastResult.errors.length > 0" class="output-errors">
          <h4>❌ Errors</h4>
          <div v-for="(error, i) in lastResult.errors" :key="i" class="error-item">
            <strong>{{ error.code }}:</strong> {{ error.message }}
            <span v-if="error.location" class="error-location">at {{ error.location }}</span>
          </div>
        </div>

        <div v-else class="output-results">
          <div v-for="(output, index) in lastResult.outputs" :key="index" class="output-item">
            <div class="output-item-header">
              <span class="output-number">Output {{ index + 1 }}</span>
              <button @click="copyToClipboard(output.text)" class="btn-icon" title="Copy">
                📋
              </button>
            </div>
            <pre class="output-text">{{ output.text }}</pre>
            <div v-if="output.warnings && output.warnings.length > 0" class="output-warnings">
              <span v-for="(warning, wi) in output.warnings" :key="wi" class="warning">
                ⚠️ {{ warning }}
              </span>
            </div>
          </div>
        </div>

        <div v-if="lastResult" class="output-footer">
          <div class="render-info">
            <strong>Variables Used:</strong>
            {{ lastResult.metadata.variables_used.join(', ') || 'none' }}
          </div>
          <div class="render-info">
            <strong>Sections Used:</strong>
            {{ lastResult.metadata.sections_used.join(', ') || 'none' }}
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch, onMounted } from 'vue'
import { usePromptStore } from '@/stores/promptStore'
import { renderPrompt } from '@/services/promptRenderEngine'
import type { RenderResult, RenderOptions, PromptSection } from '@/types/promptTypes'

const store = usePromptStore()

// State
const selectedEntryPointId = ref('')
const variableValues = reactive<Record<string, any>>({})
const renderOptions = reactive<RenderOptions>({
  count: 1,
  format: 'text',
})
const rendering = ref(false)
const lastResult = ref<RenderResult | null>(null)

// Computed - entry points are sections with is_entry_point=true
const entryPoints = computed(() => store.sections.filter(s => s.is_entry_point))
const namespaces = computed(() => store.namespaces)

const selectedEntryPoint = computed<PromptSection | undefined>(() => {
  if (!selectedEntryPointId.value) return undefined
  return entryPoints.value.find(s => store.getIdString(s.id) === selectedEntryPointId.value)
})

const canRender = computed(() => {
  if (!selectedEntryPoint.value) return false

  // Check required variables
  const variables = selectedEntryPoint.value.variables || []
  for (const variable of variables) {
    if (variable.required) {
      const value = variableValues[variable.id]
      if (value === undefined || value === null || value === '') {
        return false
      }
    }
  }

  return true
})

// Methods
function getIdString(id: any): string {
  return store.getIdString(id)
}

function entryPointsByNamespace(ns: string): PromptSection[] {
  return entryPoints.value.filter(s => s.namespace === ns)
}

function arrayToString(arr: any): string {
  if (!Array.isArray(arr)) return ''
  return arr.join(', ')
}

function handleArrayInput(variableId: string, value: string) {
  // Split by comma, trim whitespace, filter out empty strings
  const items = value
    .split(',')
    .map(s => s.trim())
    .filter(s => s.length > 0)
  variableValues[variableId] = items
}

function handleObjectInput(variableId: string, value: string) {
  try {
    variableValues[variableId] = JSON.parse(value)
  } catch {
    // Keep as is if invalid JSON
  }
}

async function renderPrompts() {
  if (!selectedEntryPoint.value) return

  rendering.value = true
  try {
    lastResult.value = await renderPrompt({
      template_id: selectedEntryPointId.value,
      context: {
        variables: { ...variableValues },
      },
      options: { ...renderOptions },
    })
  } catch (e) {
    lastResult.value = {
      success: false,
      outputs: [],
      errors: [
        {
          code: 'RENDER_FAILED',
          message: String(e),
        },
      ],
      metadata: {
        template_id: selectedEntryPointId.value,
        timestamp: new Date().toISOString(),
        duration_ms: 0,
        variables_used: [],
        sections_used: [],
      },
    }
  } finally {
    rendering.value = false
  }
}

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    alert('Copied to clipboard!')
  } catch {
    alert('Failed to copy')
  }
}

// Watch for entry point changes
watch(selectedEntryPoint, newEntryPoint => {
  // Reset variables
  Object.keys(variableValues).forEach(key => delete variableValues[key])

  // Apply defaults
  if (newEntryPoint) {
    const variables = newEntryPoint.variables || []
    for (const variable of variables) {
      if (variable.default_value !== undefined) {
        variableValues[variable.id] = variable.default_value
      }
    }
  }

  // Clear results
  lastResult.value = null
})

// Lifecycle
onMounted(async () => {
  await store.loadAll()
})
</script>

<style scoped>
.prompt-renderer-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f8f9fa;
}

.page-header {
  padding: 1.5rem 2rem;
  background: #ffffff;
  border-bottom: 1px solid #dee2e6;
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 1.5rem;
}

.page-header p {
  margin: 0;
  color: #6c757d;
}

.renderer-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.config-panel {
  width: 400px;
  background: #ffffff;
  border-right: 1px solid #dee2e6;
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.panel-section {
  padding: 1.25rem;
  border-bottom: 1px solid #dee2e6;
}

.panel-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: #495057;
}

.template-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  font-size: 1rem;
}

.template-info {
  margin-top: 1rem;
}

.template-description {
  margin: 0 0 0.75rem 0;
  color: #6c757d;
  font-size: 0.9rem;
}

.template-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.badge {
  padding: 0.25rem 0.5rem;
  background: #e9ecef;
  border-radius: 4px;
  font-size: 0.8rem;
}

.tag {
  padding: 0.125rem 0.5rem;
  background: #e7f1ff;
  border-radius: 4px;
  font-size: 0.8rem;
  color: #007bff;
}

.variables-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.variable-field label {
  display: block;
  font-weight: 600;
  margin-bottom: 0.25rem;
  font-size: 0.95rem;
}

.required {
  color: #dc3545;
}

.field-description {
  margin: 0 0 0.5rem 0;
  font-size: 0.85rem;
  color: #6c757d;
}

.variable-field input,
.variable-field select,
.variable-field textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  font-size: 0.95rem;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: normal;
  cursor: pointer;
}

.checkbox-label input {
  width: auto;
}

.array-input .hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.8rem;
  color: #6c757d;
}

.no-variables {
  text-align: center;
  padding: 1rem;
  color: #6c757d;
}

.options-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.option-field label {
  display: block;
  font-size: 0.95rem;
  margin-bottom: 0.25rem;
}

.option-field input,
.option-field select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #dee2e6;
  border-radius: 4px;
}

.panel-actions {
  padding: 1.25rem;
  border-top: 1px solid #dee2e6;
  margin-top: auto;
}

.render-btn {
  width: 100%;
  padding: 0.75rem 1rem;
  font-size: 1rem;
}

.btn-primary {
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.output-panel {
  flex: 1;
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.output-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background: #ffffff;
  border-bottom: 1px solid #dee2e6;
}

.output-header h3 {
  margin: 0;
}

.output-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.9rem;
  color: #6c757d;
}

.output-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  color: #6c757d;
  text-align: center;
}

.output-errors {
  padding: 1.5rem;
  background: #f8d7da;
}

.output-errors h4 {
  margin: 0 0 1rem 0;
  color: #721c24;
}

.error-item {
  padding: 0.75rem;
  background: #ffffff;
  border: 1px solid #f5c6cb;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.error-location {
  font-size: 0.9rem;
  color: #6c757d;
}

.output-results {
  flex: 1;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.output-item {
  background: #ffffff;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  overflow: hidden;
}

.output-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
}

.output-number {
  font-weight: 600;
  font-size: 0.9rem;
}

.btn-icon {
  padding: 0.25rem 0.5rem;
  background: none;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  cursor: pointer;
}

.btn-icon:hover {
  background: #e9ecef;
}

.output-text {
  margin: 0;
  padding: 1rem;
  white-space: pre-wrap;
  font-family: inherit;
  font-size: 0.95rem;
  line-height: 1.6;
  background: #ffffff;
}

.output-warnings {
  padding: 0.5rem 1rem;
  background: #fff3cd;
  border-top: 1px solid #ffc107;
}

.warning {
  display: block;
  font-size: 0.85rem;
  color: #856404;
}

.output-footer {
  padding: 1rem 1.5rem;
  background: #ffffff;
  border-top: 1px solid #dee2e6;
}

.render-info {
  font-size: 0.85rem;
  color: #6c757d;
  margin-bottom: 0.25rem;
}
</style>
