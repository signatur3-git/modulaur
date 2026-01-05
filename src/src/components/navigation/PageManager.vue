<template>
  <div v-if="isOpen" class="modal-overlay" @click="close">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>{{ isEditing ? 'Edit Page' : 'Create New Page' }}</h2>
        <button @click="close" class="close-button">×</button>
      </div>

      <form @submit.prevent="handleSubmit" class="page-form">
        <div class="form-group">
          <label for="page-name">Page Name *</label>
          <input
            id="page-name"
            v-model="form.name"
            type="text"
            placeholder="My Page"
            required
            maxlength="50"
          />
          <span class="form-hint">The display name shown in the navigation</span>
        </div>

        <div class="form-group">
          <label for="page-route">Route *</label>
          <div class="route-input-group">
            <span class="route-prefix">/</span>
            <input
              id="page-route"
              v-model="routeInput"
              type="text"
              placeholder="my-page"
              required
              pattern="[a-z0-9-]+"
              @input="validateRoute"
            />
          </div>
          <span class="form-hint">URL path (lowercase, hyphens allowed)</span>
          <span v-if="routeError" class="form-error">{{ routeError }}</span>
        </div>

        <div class="form-group">
          <label for="page-type">Page Type *</label>
          <select id="page-type" v-model="form.type" required>
            <option v-for="pageType in availablePageTypes" :key="pageType.id" :value="pageType.id">
              {{ pageType.icon }} {{ pageType.name }}
            </option>
          </select>
          <span class="form-hint">{{ selectedPageTypeDescription }}</span>
        </div>

        <div class="form-group">
          <label for="page-icon">Icon (Emoji)</label>
          <input id="page-icon" v-model="form.icon" type="text" placeholder="📄" maxlength="2" />
          <span class="form-hint">Single emoji character</span>
        </div>

        <div class="form-group">
          <label>
            <input type="checkbox" v-model="form.visible" />
            <span>Visible in navigation</span>
          </label>
        </div>

        <div v-if="isLayoutPage" class="layout-config">
          <h3>Layout Configuration</h3>

          <div class="form-group">
            <label for="layout-template">Layout Template</label>
            <select
              id="layout-template"
              v-model="layoutTemplateId"
              @change="ensureSlotKeysForTemplate"
            >
              <option
                v-for="template in availableLayoutTemplates"
                :key="template.id"
                :value="template.id"
              >
                {{ template.name }}
              </option>
            </select>
            <span class="form-hint">Select a layout template for this page</span>
          </div>

          <div v-for="slotId in Object.keys(slotPanelSelection)" :key="slotId" class="form-group">
            <label>{{ getSlotLabel(slotId) }}</label>
            <select v-model="slotPanelSelection[slotId]" class="panel-select">
              <option value="">— None —</option>
              <option v-for="panel in availablePanels" :key="panel.id" :value="panel.id">
                {{ panel.name }}
              </option>
            </select>
          </div>
        </div>

        <div class="form-actions">
          <button type="button" @click="close" class="btn-secondary">Cancel</button>
          <button type="submit" class="btn-primary" :disabled="!isFormValid">
            {{ isEditing ? 'Update Page' : 'Create Page' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { usePageStore } from '@/stores/pageStore'
import { getAllPageTypes, getPageType } from '@/services/pageTypes'
import { getLayoutTemplateRegistry } from '@/services/layoutTemplateRegistry'
import { pluginLoader } from '@/services/pluginLoader'
import type { Page } from '@/stores/pageStore'

const props = defineProps<{
  isOpen: boolean
  page?: Page | null
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const router = useRouter()
const pageStore = usePageStore()

// Get available page types
const availablePageTypes = computed(() => getAllPageTypes())

// Get selected page type description
const selectedPageTypeDescription = computed(() => {
  const type = availablePageTypes.value.find(t => t.id === form.value.type)
  return type?.description || ''
})

// Helper to extract string ID from Thing or string
function getPageId(page: Page): string {
  if (!page.id) {
    return ''
  }

  if (typeof page.id === 'string') {
    return page.id
  }

  // It's a Thing object { tb: "pages", id: ... }
  if (typeof page.id === 'object' && 'id' in page.id) {
    const thingId = (page.id as any).id

    // Handle string IDs
    if (typeof thingId === 'string') {
      return thingId
    }

    // Handle numeric IDs
    if (typeof thingId === 'number') {
      return thingId.toString()
    }

    // Handle SurrealDB's internal ID format: { String: "..." }
    if (typeof thingId === 'object' && thingId !== null) {
      if ('String' in thingId && typeof (thingId as any).String === 'string') {
        return (thingId as any).String
      }
      if ('Number' in thingId && typeof (thingId as any).Number === 'number') {
        return (thingId as any).Number.toString()
      }
      // Fallback: stringify the complex ID
      return JSON.stringify(thingId)
    }
  }

  // Fallback: convert entire id to string
  return String(page.id)
}

const form = ref({
  name: '',
  icon: '📄',
  visible: true,
  type: 'dashboard',
})

const routeInput = ref('')
const routeError = ref('')

const layoutTemplateId = ref('')
const slotPanelSelection = ref<Record<string, string>>({})

const availableLayoutTemplates = computed(() => {
  return getLayoutTemplateRegistry().getAll()
})

const activeLayoutTemplate = computed(() => {
  if (!layoutTemplateId.value) return null
  return getLayoutTemplateRegistry().get(layoutTemplateId.value) || null
})

const availablePanels = computed(() => pluginLoader.getAvailablePanelTypes())

const isEditing = computed(() => !!props.page)
const isFormValid = computed(() => {
  return form.value.name.trim() && routeInput.value.trim() && !routeError.value
})
const isLayoutPage = computed(() => form.value.type === 'layout')

watch(
  () => props.page,
  page => {
    if (page) {
      form.value = {
        name: page.name,
        icon: page.icon || '📄',
        visible: page.visible,
        type: page.type || 'dashboard',
      }
      // Extract route without leading slash
      routeInput.value = page.route.startsWith('/') ? page.route.substring(1) : page.route

      if (isLayoutPage.value) {
        initLayoutConfigFromPage(page)
        ensureSlotKeysForTemplate()
      }
    } else {
      resetForm()
    }
  },
  { immediate: true }
)

watch(
  () => props.isOpen,
  isOpen => {
    if (!isOpen) {
      resetForm()
    }
  }
)

watch(
  () => form.value.type,
  type => {
    if (type === 'layout') {
      initLayoutConfigFromPage(props.page || null)
      ensureSlotKeysForTemplate()
    } else {
      layoutTemplateId.value = ''
      slotPanelSelection.value = {}
    }
  }
)

watch(layoutTemplateId, () => {
  ensureSlotKeysForTemplate()
})

function resetForm() {
  form.value = {
    name: '',
    icon: '📄',
    visible: true,
    type: 'dashboard',
  }
  routeInput.value = ''
  routeError.value = ''
  layoutTemplateId.value = ''
  slotPanelSelection.value = {}
}

function validateRoute() {
  const route = routeInput.value.trim()

  if (!route) {
    routeError.value = ''
    return
  }

  // Check format
  if (!/^[a-z0-9-]+$/.test(route)) {
    routeError.value = 'Only lowercase letters, numbers, and hyphens allowed'
    return
  }

  // Check for reserved routes
  const reserved = ['home', 'offline-browser', 'plugin-management', 'database-management', 'page']
  if (reserved.includes(route) && (!props.page || props.page.route !== `/${route}`)) {
    routeError.value = 'This route is reserved'
    return
  }

  // Check if route already exists (except for current page)
  const existingPage = pageStore.pages.find(p => p.route === `/${route}`)
  if (existingPage && (!props.page || existingPage.id !== props.page.id)) {
    routeError.value = 'This route already exists'
    return
  }

  routeError.value = ''
}

function initLayoutConfigFromPage(page?: Page | null) {
  const cfg = page?.config || {}
  layoutTemplateId.value =
    cfg.layout || getPageType('layout')?.defaultConfig?.layout || 'sidebar-main'

  // merge any existing slots; only track first panel per slot for now
  const next: Record<string, string> = {}
  const slots = cfg.slots || {}
  Object.keys(slots).forEach(slotId => {
    const slotItems = slots[slotId]
    if (Array.isArray(slotItems) && slotItems.length > 0 && slotItems[0]?.panelId) {
      next[slotId] = slotItems[0].panelId
    }
  })
  slotPanelSelection.value = next
}

function ensureSlotKeysForTemplate() {
  const template = activeLayoutTemplate.value
  if (!template) return

  const next = { ...slotPanelSelection.value }
  for (const slot of template.slots) {
    if (!(slot.id in next)) {
      next[slot.id] = ''
    }
  }

  // Drop selections for slots that don't exist in chosen template
  for (const key of Object.keys(next)) {
    if (!template.slots.some(s => s.id === key)) {
      delete next[key]
    }
  }

  slotPanelSelection.value = next
}

function getSlotLabel(slotId: string) {
  const template = activeLayoutTemplate.value
  const slot = template?.slots.find(s => s.id === slotId)
  return slot ? `Slot: ${slot.name}` : `Slot: ${slotId}`
}

function buildLayoutPageConfig(): any {
  const template = activeLayoutTemplate.value
  const base = getPageType('layout')?.defaultConfig
    ? { ...getPageType('layout')!.defaultConfig }
    : { layout: 'sidebar-main', slots: {} }

  const slots: Record<string, any[]> = {}
  if (template) {
    for (const slot of template.slots) {
      const panelId = (slotPanelSelection.value[slot.id] || '').trim()
      if (panelId) {
        slots[slot.id] = [{ panelId, order: 0, config: {} }]
      } else {
        slots[slot.id] = []
      }
    }
  }

  return {
    ...base,
    layout: layoutTemplateId.value || base.layout,
    slots,
  }
}

async function handleSubmit() {
  if (!isFormValid.value) return

  try {
    const route = `/${routeInput.value.trim()}`

    // Build config based on type
    const configForSave = isLayoutPage.value
      ? buildLayoutPageConfig()
      : getPageType(form.value.type)?.defaultConfig
        ? { ...getPageType(form.value.type)!.defaultConfig }
        : {}

    if (isEditing.value && props.page) {
      const pageId = getPageId(props.page)
      await pageStore.updatePage(pageId, {
        name: form.value.name.trim(),
        route,
        type: form.value.type,
        icon: form.value.icon || undefined,
        visible: form.value.visible,
        config: configForSave,
      })
    } else {
      const newPage = await pageStore.createPage({
        name: form.value.name.trim(),
        route,
        type: form.value.type,
        icon: form.value.icon || undefined,
        order: pageStore.pages.length,
        visible: form.value.visible,
        config: configForSave,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      })

      // Navigate using the page's route (stable identifier) instead of SurrealDB ID
      const routeSlug = newPage.route.startsWith('/') ? newPage.route.substring(1) : newPage.route
      router.push(`/page/${routeSlug}`)
    }

    emit('saved')
  } catch (e) {
    alert(`Failed to save page: ${e}`)
  }
}

function close() {
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.modal-content {
  background: var(--bg-modal);
  padding: 0;
  border-radius: 8px;
  min-width: 500px;
  max-width: 600px;
  max-height: 90vh;
  overflow: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.3s ease;
  color: var(--text-primary);
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem 2rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-heading);
}

.close-button {
  background: none;
  border: none;
  font-size: 2rem;
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

.close-button:hover {
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.page-form {
  padding: 2rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--text-primary);
}

.form-group input[type='text'] {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 1rem;
  transition: border-color 0.2s;
  background: var(--bg-panel);
  color: var(--text-primary);
}

.form-group input[type='text']:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.route-input-group {
  display: flex;
  align-items: center;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
  transition: border-color 0.2s;
  background: var(--bg-panel);
}

.route-input-group:focus-within {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.route-prefix {
  padding: 0.75rem;
  background: var(--bg-panel-header);
  color: var(--text-secondary);
  font-weight: 600;
  border-right: 1px solid var(--border-color);
}

.form-hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.form-error {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: var(--accent-danger);
  font-weight: 500;
}

.form-group label input[type='checkbox'] {
  margin-right: 0.5rem;
}

.layout-config {
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.layout-config h3 {
  margin: 0 0 1rem 0;
  font-size: 1.25rem;
  color: var(--text-heading);
}

select,
.panel-select {
  background: var(--bg-panel);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

select:focus,
.panel-select:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.form-actions {
  display: flex;
  gap: 1rem;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.btn-primary,
.btn-secondary {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.2s;
}

.btn-primary {
  flex: 1;
  background: var(--bg-button);
  color: var(--text-on-accent);
}

.btn-primary:hover:not(:disabled) {
  background: var(--bg-button-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
}

.btn-secondary:hover {
  background: var(--bg-button-secondary-hover);
  transform: translateY(-1px);
}
</style>
