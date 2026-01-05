<template>
  <div class="hello-panel">
    <div class="hello-header">
      <h2>👋 Hello from Plugin!</h2>
      <span class="badge">Plugin Panel</span>
    </div>

    <p class="hello-description">
      This is a custom panel provided by the <strong>hello-panel</strong> plugin. It demonstrates
      how plugins can extend the dashboard with custom components!
    </p>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon">📦</div>
        <div class="stat-content">
          <span class="stat-label">Panel ID</span>
          <span class="stat-value">{{ panel?.id || 'N/A' }}</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">⏰</div>
        <div class="stat-content">
          <span class="stat-label">Loaded At</span>
          <span class="stat-value">{{ loadTime }}</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon">🔢</div>
        <div class="stat-content">
          <span class="stat-label">Version</span>
          <span class="stat-value">v1.0.0</span>
        </div>
      </div>
    </div>

    <div class="config-section">
      <h3>📋 Configuration</h3>
      <div class="config-content">
        <pre>{{ configDisplay }}</pre>
      </div>
    </div>

    <div class="action-section">
      <button @click="incrementCounter" class="action-btn primary">
        🎯 Clicked {{ counter }} times
      </button>

      <button @click="showAlert" class="action-btn secondary">💬 Show Alert</button>

      <button @click="logToConsole" class="action-btn secondary">📝 Log to Console</button>
    </div>

    <div class="info-box">
      <strong>💡 Plugin Features:</strong>
      <ul>
        <li>✅ Dynamic component loading</li>
        <li>✅ State management (counter: {{ counter }})</li>
        <li>✅ Event handling (button clicks)</li>
        <li>✅ Config access</li>
        <li>✅ Full Vue 3 capabilities</li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'

const props = defineProps({
  panel: {
    type: Object,
    default: () => ({}),
  },
  config: {
    type: Object,
    default: () => ({}),
  },
})

const counter = ref(0)
const loadTime = ref('')

const configDisplay = computed(() => {
  if (!props.config || Object.keys(props.config).length === 0) {
    return '{ /* No configuration */ }'
  }
  return JSON.stringify(props.config, null, 2)
})

onMounted(() => {
  loadTime.value = new Date().toLocaleTimeString()
  console.log('✅ HelloPanel mounted:', {
    panel: props.panel,
    config: props.config,
  })
})

const incrementCounter = () => {
  counter.value++
  console.log(`Counter incremented to ${counter.value}`)
}

const showAlert = () => {
  alert(`Hello from plugin! Counter is at ${counter.value}`)
}

const logToConsole = () => {
  console.log('📝 Hello Panel Log:', {
    counter: counter.value,
    loadTime: loadTime.value,
    panel: props.panel,
    config: props.config,
  })
}
</script>

<style scoped>
.hello-panel {
  padding: 1.5rem;
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border-radius: 8px;
  height: 100%;
  overflow-y: auto;
}

.hello-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.hello-header h2 {
  margin: 0;
  font-size: 1.5rem;
}

.badge {
  background: var(--bg-panel-header);
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: bold;
}

.hello-description {
  line-height: 1.6;
  margin-bottom: 1.5rem;
  opacity: 0.95;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  background: var(--bg-panel);
  padding: 1rem;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  backdrop-filter: blur(10px);
}

.stat-icon {
  font-size: 1.5rem;
}

.stat-content {
  display: flex;
  flex-direction: column;
}

.stat-label {
  font-size: 0.75rem;
  opacity: 0.8;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 1rem;
  font-weight: bold;
  margin-top: 0.25rem;
}

.config-section {
  margin-bottom: 1.5rem;
}

.config-section h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1rem;
}

.config-content {
  background: var(--bg-panel-header);
  padding: 1rem;
  border-radius: 6px;
  max-height: 150px;
  overflow-y: auto;
}

.config-content pre {
  margin: 0;
  color: #fff;
  font-size: 0.85rem;
  font-family: 'Monaco', 'Courier New', monospace;
}

.action-section {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.action-btn {
  padding: 0.75rem 1.25rem;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  flex: 1;
  min-width: 140px;
}

.action-btn.primary {
  background: var(--bg-panel);
  color: #667eea;
}

.action-btn.primary:hover {
  background: var(--bg-panel-header);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.action-btn.secondary {
  background: var(--bg-panel-header);
  color: var(--text-on-accent);
}

.action-btn.secondary:hover {
  background: color-mix(in srgb, var(--bg-panel-header) 70%, transparent);
  transform: translateY(-2px);
}

.info-box {
  background: var(--bg-panel-header);
  padding: 1rem;
  border-radius: 6px;
  border-left: 3px solid rgba(255, 255, 255, 0.5);
}

.info-box strong {
  display: block;
  margin-bottom: 0.5rem;
}

.info-box ul {
  margin: 0;
  padding-left: 1.5rem;
}

.info-box li {
  margin: 0.25rem 0;
  line-height: 1.6;
}
</style>
