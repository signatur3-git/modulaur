<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          :class="['toast', `toast-${toast.type}`]"
          @click="removeToast(toast.id)"
        >
          <span class="toast-icon">{{ getIcon(toast.type) }}</span>
          <span class="toast-message">{{ toast.message }}</span>
          <button class="toast-close" @click.stop="removeToast(toast.id)">×</button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue'

export interface Toast {
  id: number
  type: 'success' | 'error' | 'info' | 'warning'
  message: string
}

const toasts = ref<Toast[]>([])
let nextId = 1

function addToast(type: Toast['type'], message: string, duration = 5000) {
  const id = nextId++
  toasts.value.push({ id, type, message })

  if (duration > 0) {
    setTimeout(() => removeToast(id), duration)
  }
}

function removeToast(id: number) {
  const index = toasts.value.findIndex(t => t.id === id)
  if (index !== -1) {
    toasts.value.splice(index, 1)
  }
}

function getIcon(type: Toast['type']): string {
  const icons = {
    success: '✅',
    error: '❌',
    info: 'ℹ️',
    warning: '⚠️',
  }
  return icons[type]
}

// Expose methods for parent components
defineExpose({
  success: (message: string, duration?: number) => addToast('success', message, duration),
  error: (message: string, duration?: number) => addToast('error', message, duration),
  info: (message: string, duration?: number) => addToast('info', message, duration),
  warning: (message: string, duration?: number) => addToast('warning', message, duration),
})
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 10px;
  pointer-events: none;
}

.toast {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 300px;
  max-width: 500px;
  padding: 16px;
  background: var(--bg-toast);
  color: var(--text-primary);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  pointer-events: all;
  cursor: pointer;
  transition: all 0.3s ease;
}

.toast:hover {
  transform: translateX(-4px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
}

.toast-icon {
  font-size: 20px;
  flex-shrink: 0;
}

.toast-message {
  flex: 1;
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.4;
}

.toast-close {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
  flex-shrink: 0;
}

.toast-close:hover {
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.toast-success {
  border-left: 4px solid var(--accent-success);
}

.toast-error {
  border-left: 4px solid var(--accent-danger);
}

.toast-info {
  border-left: 4px solid var(--accent-primary);
}

.toast-warning {
  border-left: 4px solid var(--accent-warning);
}

/* Transition animations */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.8);
}

.toast-move {
  transition: transform 0.3s ease;
}
</style>
