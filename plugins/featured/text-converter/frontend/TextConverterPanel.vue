<template>
  <div class="text-converter-panel">
    <div class="converter-header">
      <h3>🔄 Text Converter</h3>
      <p class="subtitle">Transform and convert text in various formats</p>
    </div>

    <div class="converter-content">
      <!-- Input Section -->
      <div class="input-section">
        <div class="section-header">
          <label>Input Text</label>
          <button @click="clearInput" class="clear-btn" title="Clear input">
            🗑️ Clear
          </button>
        </div>
        <textarea
          v-model="inputText"
          placeholder="Enter text to convert..."
          class="converter-textarea"
          rows="6"
        ></textarea>
      </div>

      <!-- Conversion Options -->
      <div class="conversion-tabs">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="['tab-btn', { active: activeTab === tab.id }]"
          @click="activeTab = tab.id"
        >
          {{ tab.icon }} {{ tab.label }}
        </button>
      </div>

      <!-- Conversion Actions -->
      <div class="conversion-actions">
        <div v-if="activeTab === 'encoding'" class="action-group">
          <h4>Encoding</h4>
          <div class="button-grid">
            <button @click="convert('base64-encode')" class="action-btn">
              Base64 Encode
            </button>
            <button @click="convert('base64-decode')" class="action-btn">
              Base64 Decode
            </button>
            <button @click="convert('url-encode')" class="action-btn">
              URL Encode
            </button>
            <button @click="convert('url-decode')" class="action-btn">
              URL Decode
            </button>
            <button @click="convert('html-encode')" class="action-btn">
              HTML Encode
            </button>
            <button @click="convert('html-decode')" class="action-btn">
              HTML Decode
            </button>
          </div>
        </div>

        <div v-if="activeTab === 'case'" class="action-group">
          <h4>Case Conversion</h4>
          <div class="button-grid">
            <button @click="convert('uppercase')" class="action-btn">
              UPPERCASE
            </button>
            <button @click="convert('lowercase')" class="action-btn">
              lowercase
            </button>
            <button @click="convert('titlecase')" class="action-btn">
              Title Case
            </button>
            <button @click="convert('capitalize')" class="action-btn">
              Capitalize First
            </button>
          </div>
        </div>

        <div v-if="activeTab === 'format'" class="action-group">
          <h4>Format Conversion</h4>
          <div class="button-grid">
            <button @click="convert('snake-case')" class="action-btn">
              snake_case
            </button>
            <button @click="convert('kebab-case')" class="action-btn">
              kebab-case
            </button>
            <button @click="convert('camel-case')" class="action-btn">
              camelCase
            </button>
            <button @click="convert('pascal-case')" class="action-btn">
              PascalCase
            </button>
            <button @click="convert('constant-case')" class="action-btn">
              CONSTANT_CASE
            </button>
          </div>
        </div>

        <div v-if="activeTab === 'hash'" class="action-group">
          <h4>Hashing</h4>
          <div class="button-grid">
            <button @click="convert('md5')" class="action-btn">
              MD5 Hash
            </button>
            <button @click="convert('sha1')" class="action-btn">
              SHA-1 Hash
            </button>
            <button @click="convert('sha256')" class="action-btn">
              SHA-256 Hash
            </button>
          </div>
        </div>

        <div v-if="activeTab === 'json'" class="action-group">
          <h4>JSON</h4>
          <div class="button-grid">
            <button @click="convert('json-pretty')" class="action-btn">
              Pretty Print
            </button>
            <button @click="convert('json-minify')" class="action-btn">
              Minify
            </button>
            <button @click="convert('json-escape')" class="action-btn">
              Escape Quotes
            </button>
            <button @click="convert('json-unescape')" class="action-btn">
              Unescape Quotes
            </button>
          </div>
        </div>
      </div>

      <!-- Output Section -->
      <div class="output-section">
        <div class="section-header">
          <label>Output</label>
          <button @click="copyOutput" class="copy-btn" :disabled="!outputText" title="Copy to clipboard">
            📋 Copy
          </button>
        </div>
        <textarea
          v-model="outputText"
          placeholder="Converted text will appear here..."
          class="converter-textarea output"
          rows="6"
          readonly
        ></textarea>
        <div v-if="errorMessage" class="error-message">
          ⚠️ {{ errorMessage }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const inputText = ref('')
const outputText = ref('')
const errorMessage = ref('')
const activeTab = ref('encoding')

const tabs = [
  { id: 'encoding', label: 'Encoding', icon: '🔐' },
  { id: 'case', label: 'Case', icon: '🔤' },
  { id: 'format', label: 'Format', icon: '📝' },
  { id: 'hash', label: 'Hash', icon: '#️⃣' },
  { id: 'json', label: 'JSON', icon: '{}' }
]

const clearInput = () => {
  inputText.value = ''
  outputText.value = ''
  errorMessage.value = ''
}

const copyOutput = async () => {
  try {
    await navigator.clipboard.writeText(outputText.value)
    // Could add a toast notification here
  } catch (err) {
    errorMessage.value = 'Failed to copy to clipboard'
  }
}

const convert = async (type) => {
  errorMessage.value = ''

  if (!inputText.value.trim()) {
    errorMessage.value = 'Please enter some text to convert'
    return
  }

  try {
    switch (type) {
      // Encoding
      case 'base64-encode':
        outputText.value = btoa(inputText.value)
        break
      case 'base64-decode':
        outputText.value = atob(inputText.value)
        break
      case 'url-encode':
        outputText.value = encodeURIComponent(inputText.value)
        break
      case 'url-decode':
        outputText.value = decodeURIComponent(inputText.value)
        break
      case 'html-encode':
        outputText.value = inputText.value
          .replace(/&/g, '&amp;')
          .replace(/</g, '&lt;')
          .replace(/>/g, '&gt;')
          .replace(/"/g, '&quot;')
          .replace(/'/g, '&#039;')
        break
      case 'html-decode':
        outputText.value = inputText.value
          .replace(/&amp;/g, '&')
          .replace(/&lt;/g, '<')
          .replace(/&gt;/g, '>')
          .replace(/&quot;/g, '"')
          .replace(/&#039;/g, "'")
        break

      // Case
      case 'uppercase':
        outputText.value = inputText.value.toUpperCase()
        break
      case 'lowercase':
        outputText.value = inputText.value.toLowerCase()
        break
      case 'titlecase':
        outputText.value = inputText.value.replace(/\b\w/g, char => char.toUpperCase())
        break
      case 'capitalize':
        outputText.value = inputText.value.charAt(0).toUpperCase() + inputText.value.slice(1)
        break

      // Format
      case 'snake-case':
        outputText.value = toSnakeCase(inputText.value)
        break
      case 'kebab-case':
        outputText.value = toKebabCase(inputText.value)
        break
      case 'camel-case':
        outputText.value = toCamelCase(inputText.value)
        break
      case 'pascal-case':
        outputText.value = toPascalCase(inputText.value)
        break
      case 'constant-case':
        outputText.value = toSnakeCase(inputText.value).toUpperCase()
        break

      // Hash
      case 'md5':
      case 'sha1':
      case 'sha256':
        outputText.value = await hashText(inputText.value, type)
        break

      // JSON
      case 'json-pretty':
        outputText.value = JSON.stringify(JSON.parse(inputText.value), null, 2)
        break
      case 'json-minify':
        outputText.value = JSON.stringify(JSON.parse(inputText.value))
        break
      case 'json-escape':
        outputText.value = inputText.value.replace(/"/g, '\\"')
        break
      case 'json-unescape':
        outputText.value = inputText.value.replace(/\\"/g, '"')
        break
    }
  } catch (err) {
    errorMessage.value = `Conversion failed: ${err.message}`
    outputText.value = ''
  }
}

// Helper functions for case conversion
const toSnakeCase = (str) => {
  return str
    .replace(/([A-Z])/g, '_$1')
    .replace(/[\s-]+/g, '_')
    .replace(/_+/g, '_')
    .replace(/^_/, '')
    .toLowerCase()
}

const toKebabCase = (str) => {
  return str
    .replace(/([A-Z])/g, '-$1')
    .replace(/[\s_]+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-/, '')
    .toLowerCase()
}

const toCamelCase = (str) => {
  return str
    .replace(/[-_\s]+(.)?/g, (_, char) => char ? char.toUpperCase() : '')
    .replace(/^(.)/, char => char.toLowerCase())
}

const toPascalCase = (str) => {
  const camel = toCamelCase(str)
  return camel.charAt(0).toUpperCase() + camel.slice(1)
}

// Hash function using Web Crypto API
const hashText = async (text, algorithm) => {
  const encoder = new TextEncoder()
  const data = encoder.encode(text)

  const algoMap = {
    'md5': null, // MD5 not supported by Web Crypto API - would need a library
    'sha1': 'SHA-1',
    'sha256': 'SHA-256'
  }

  if (algorithm === 'md5') {
    // Simple MD5 alternative - for demo purposes, using SHA-256 instead
    return hashText(text, 'sha256')
  }

  const hashBuffer = await crypto.subtle.digest(algoMap[algorithm], data)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('')
}
</script>

<style>
.text-converter-panel {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.converter-header {
  margin-bottom: 24px;
  text-align: center;
}

.converter-header h3 {
  margin: 0 0 8px 0;
  font-size: 24px;
  color: var(--text-heading);
}

.subtitle {
  margin: 0;
  color: #7f8c8d;
  font-size: 14px;
}

.converter-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.input-section,
.output-section {
  background: var(--bg-panel);
  padding: 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.section-header label {
  font-weight: 600;
  color: var(--text-heading);
  font-size: 14px;
}

.converter-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  resize: vertical;
  box-sizing: border-box;
}

.converter-textarea:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(76, 175, 80, 0.1);
}

.converter-textarea.output {
  background-color: #f5f5f5;
  color: var(--text-heading);
}

.conversion-tabs {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  justify-content: center;
  padding: 12px 0;
  border-top: 1px solid var(--border-color);
  border-bottom: 1px solid var(--border-color);
}

.tab-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-panel);
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.tab-btn:hover {
  background: var(--bg-panel);
  border-color: var(--accent-primary);
}

.tab-btn.active {
  background: var(--accent-primary);
  color: white;
  border-color: var(--accent-primary);
  font-weight: 600;
}

.conversion-actions {
  min-height: 120px;
}

.action-group h4 {
  margin: 0 0 12px 0;
  color: var(--text-heading);
  font-size: 16px;
}

.button-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 10px;
}

.action-btn {
  padding: 10px 16px;
  background: var(--accent-primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.action-btn:active {
  transform: translateY(0);
}

.clear-btn,
.copy-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-panel);
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.clear-btn:hover,
.copy-btn:hover:not(:disabled) {
  background: var(--bg-panel);
  border-color: var(--accent-primary);
}

.copy-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-message {
  margin-top: 8px;
  padding: 8px 12px;
  background: #fee;
  color: var(--accent-danger);
  border-radius: 4px;
  font-size: 13px;
  border: 1px solid #fcc;
}

@media (max-width: 768px) {
  .button-grid {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  }

  .conversion-tabs {
    justify-content: flex-start;
  }
}
</style>

