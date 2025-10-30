<template>
  <div class="create-ticket-modal" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <h3>{{ isEdit ? 'Edit Ticket' : 'Create New Ticket' }}</h3>
        <button class="close-btn" @click="$emit('close')">×</button>
      </div>

      <form @submit.prevent="handleSubmit" class="ticket-form">
        <!-- Title -->
        <div class="form-group">
          <label for="title">Title <span class="required">*</span></label>
          <input
            id="title"
            v-model="form.title"
            type="text"
            placeholder="Enter ticket title..."
            required
            autofocus
          />
        </div>

        <!-- Type -->
        <div class="form-group">
          <label>Type <span class="required">*</span></label>
          <div class="type-options">
            <label
              v-for="type in ticketTypes"
              :key="type.value"
              class="type-option"
              :class="{ active: form.ticket_type === type.value }"
            >
              <input type="radio" v-model="form.ticket_type" :value="type.value" required />
              <span class="type-icon">{{ type.icon }}</span>
              <span class="type-label">{{ type.label }}</span>
            </label>
          </div>
        </div>

        <!-- Description -->
        <div class="form-group">
          <label for="description">Description</label>
          <textarea
            id="description"
            v-model="form.description"
            placeholder="Describe the ticket... (Markdown supported)"
            rows="6"
          ></textarea>
          <small class="help-text">Markdown supported</small>
        </div>

        <!-- Priority -->
        <div class="form-group">
          <label for="priority">Priority</label>
          <select id="priority" v-model="form.priority">
            <option value="low">🟢 Low</option>
            <option value="medium" selected>🟡 Medium</option>
            <option value="high">🟠 High</option>
            <option value="critical">🔴 Critical</option>
          </select>
        </div>

        <!-- Assignee -->
        <div class="form-group">
          <label for="assignee">Assignee</label>
          <input
            id="assignee"
            v-model="form.assignee"
            type="text"
            placeholder="Enter assignee email or name..."
            list="assignee-suggestions"
          />
          <datalist id="assignee-suggestions">
            <option v-for="assignee in allAssignees" :key="assignee" :value="assignee" />
          </datalist>
        </div>

        <!-- Tags -->
        <div class="form-group">
          <label for="tags">Tags</label>
          <div class="tags-input-wrapper">
            <div class="tags-display">
              <span v-for="(tag, index) in form.tags" :key="index" class="tag">
                {{ tag }}
                <button type="button" @click="removeTag(index)" class="tag-remove">×</button>
              </span>
            </div>
            <input
              id="tags"
              v-model="tagInput"
              type="text"
              placeholder="Type and press Enter to add tags..."
              @keydown.enter.prevent="addTag"
              @keydown="handleTagKeydown"
              list="tag-suggestions"
            />
          </div>
          <datalist id="tag-suggestions">
            <option v-for="tag in allTags" :key="tag" :value="tag" />
          </datalist>
          <small class="help-text">Press Enter or comma to add tag</small>
        </div>

        <!-- Estimate & Due Date (in a row) -->
        <div class="form-row">
          <div class="form-group">
            <label for="estimate">Estimate (points)</label>
            <input
              id="estimate"
              v-model.number="form.estimate"
              type="number"
              min="0"
              step="0.5"
              placeholder="Story points..."
            />
          </div>

          <div class="form-group">
            <label for="due_date">Due Date</label>
            <input id="due_date" v-model="form.due_date" type="date" :min="today" />
          </div>
        </div>

        <!-- Actions -->
        <div class="form-actions">
          <button type="button" class="btn btn-secondary" @click="$emit('close')">Cancel</button>
          <button type="submit" class="btn btn-primary" :disabled="loading">
            {{ loading ? 'Saving...' : isEdit ? 'Update Ticket' : 'Create Ticket' }}
          </button>
        </div>
      </form>

      <!-- Error message -->
      <div v-if="error" class="error-message">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import {
  useTicketStore,
  type Ticket,
  type CreateTicketData,
  type TicketType,
  type Priority,
} from '@/stores/ticketStore'

const props = defineProps<{
  ticket?: Ticket // If provided, we're editing
  boardId?: string // Board/panel this ticket belongs to
}>()

const emit = defineEmits<{
  close: []
  created: [ticket: Ticket]
  updated: [ticket: Ticket]
}>()

const ticketStore = useTicketStore()

// Form state
const form = ref<{
  title: string
  description?: string
  ticket_type: TicketType
  priority: Priority
  assignee?: string
  tags: string[]
  estimate?: number
  due_date?: string
}>({
  title: '',
  description: '',
  ticket_type: 'task',
  priority: 'medium',
  assignee: '',
  tags: [],
  estimate: undefined,
  due_date: '',
})

const tagInput = ref('')
const loading = ref(false)
const error = ref('')

// Is edit mode
const isEdit = computed(() => !!props.ticket)

// Ticket type options
const ticketTypes = [
  { value: 'task' as TicketType, label: 'Task', icon: '✓' },
  { value: 'bug' as TicketType, label: 'Bug', icon: '🐛' },
  { value: 'feature' as TicketType, label: 'Feature', icon: '✨' },
  { value: 'story' as TicketType, label: 'Story', icon: '📖' },
  { value: 'epic' as TicketType, label: 'Epic', icon: '🎯' },
]

// Today's date for min date
const today = computed(() => {
  return new Date().toISOString().split('T')[0]
})

// All assignees for autocomplete
const allAssignees = computed(() => ticketStore.allAssignees)

// All tags for autocomplete
const allTags = computed(() => ticketStore.allTags)

// Initialize form with ticket data if editing
watch(
  () => props.ticket,
  ticket => {
    if (ticket) {
      form.value = {
        title: ticket.title,
        description: ticket.description || '',
        ticket_type: ticket.ticket_type,
        priority: ticket.priority,
        assignee: ticket.assignee || '',
        tags: [...ticket.tags],
        estimate: ticket.estimate,
        due_date: ticket.due_date || '',
      }
    }
  },
  { immediate: true }
)

// Tag management
function addTag() {
  const tag = tagInput.value.trim()
  if (tag && !form.value.tags.includes(tag)) {
    form.value.tags.push(tag)
  }
  tagInput.value = ''
}

function handleTagKeydown(event: KeyboardEvent) {
  if (event.key === ',') {
    event.preventDefault()
    addTag()
  }
}

function removeTag(index: number) {
  form.value.tags.splice(index, 1)
}

// Form submission
async function handleSubmit() {
  loading.value = true
  error.value = ''

  try {
    // Clean up empty fields
    const data: any = {
      title: form.value.title,
      ticket_type: form.value.ticket_type,
      priority: form.value.priority,
    }

    if (form.value.description) data.description = form.value.description
    if (form.value.assignee) data.assignee = form.value.assignee
    if (form.value.tags.length > 0) data.tags = form.value.tags
    if (form.value.estimate) data.estimate = form.value.estimate
    if (form.value.due_date) data.due_date = form.value.due_date

    // Add board_id to metadata for board-specific filtering
    if (props.boardId && !isEdit.value) {
      data.metadata = { board_id: props.boardId }
    }

    if (isEdit.value && props.ticket) {
      // Update existing ticket
      const updated = await ticketStore.updateTicket(props.ticket.id, data)
      emit('updated', updated)
    } else {
      // Create new ticket
      const created = await ticketStore.createTicket(data as CreateTicketData)
      emit('created', created)
    }

    emit('close')
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to save ticket'
    console.error('Form submission error:', err)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.create-ticket-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999999;
  padding: var(--space-xl);
  /* Ensure no parent transforms affect this */
  transform: none !important;
  isolation: isolate;
}

.modal-content {
  position: relative;
  z-index: 1000000;
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: var(--panel-shadow);
  /* Ensure no clipping */
  contain: layout;
  border: 1px solid var(--border-color);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-xl) var(--space-xl);
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-heading);
}

.close-btn {
  background: none;
  border: none;
  font-size: 2rem;
  line-height: 1;
  cursor: pointer;
  color: var(--text-muted);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--panel-radius) / 2);
  transition: all 0.2s;
}

.close-btn:hover {
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.ticket-form {
  padding: var(--space-xl);
}

.form-group {
  margin-bottom: var(--space-xl);
}

.form-group label {
  display: block;
  margin-bottom: calc(var(--space-xs) * 1.5);
  font-weight: 600;
  color: var(--text-primary);
  font-size: 0.9rem;
}

.required {
  color: var(--accent-danger);
}

.form-group input[type='text'],
.form-group input[type='number'],
.form-group input[type='date'],
.form-group select,
.form-group textarea {
  width: 100%;
  padding: calc(var(--space-sm) * 1.25) var(--space-md);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.95rem;
  font-family: inherit;
  background: var(--bg-panel);
  color: var(--text-primary);
  transition: all 0.2s;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-group textarea {
  resize: vertical;
  min-height: 120px;
}

.help-text {
  display: block;
  margin-top: calc(var(--space-xs));
  font-size: 0.8rem;
  color: var(--text-muted);
}

/* Type options */
.type-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  gap: var(--space-sm);
}

.type-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--space-md) var(--space-sm);
  border: 2px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  cursor: pointer;
  transition: all 0.2s;
  background: var(--bg-panel);
}

.type-option:hover {
  border-color: var(--accent-primary);
  background: var(--bg-panel-header);
}

.type-option.active {
  border-color: var(--accent-primary);
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

.type-option input {
  display: none;
}

.type-icon {
  font-size: 1.5rem;
  margin-bottom: calc(var(--space-xs));
}

.type-label {
  font-size: 0.85rem;
  font-weight: 600;
}

/* Tags input */
.tags-input-wrapper {
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  padding: var(--space-sm);
  min-height: 42px;
  background: var(--bg-panel);
  transition: all 0.2s;
}

.tags-input-wrapper:focus-within {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.tags-display {
  display: flex;
  flex-wrap: wrap;
  gap: calc(var(--space-xs) * 1.5);
  margin-bottom: calc(var(--space-xs) * 1.5);
}

.tag {
  display: inline-flex;
  align-items: center;
  gap: calc(var(--space-xs));
  background: var(--accent-primary);
  color: var(--text-on-accent);
  padding: calc(var(--space-xs)) var(--space-sm);
  border-radius: calc(var(--panel-radius) / 4);
  font-size: 0.85rem;
}

.tag-remove {
  background: none;
  border: none;
  color: var(--text-on-accent);
  cursor: pointer;
  font-size: 1.2rem;
  line-height: 1;
  padding: 0;
  opacity: 0.8;
  transition: opacity 0.2s;
}

.tag-remove:hover {
  opacity: 1;
}

.tags-input-wrapper input {
  border: none;
  outline: none;
  padding: calc(var(--space-xs));
  font-size: 0.95rem;
  width: 100%;
  background: transparent;
  color: var(--text-primary);
}

/* Form row */
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-lg);
}

/* Actions */
.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-md);
  margin-top: var(--space-xl);
  padding-top: var(--space-xl);
  border-top: 1px solid var(--border-color);
}

.btn {
  padding: calc(var(--space-sm) * 1.25) var(--space-xl);
  border: none;
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-panel-header);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--border-color);
}

.btn-primary {
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

/* Error message */
.error-message {
  margin: 0 var(--space-xl) var(--space-xl);
  padding: var(--space-md);
  background: #fee;
  border: 1px solid #fcc;
  border-radius: calc(var(--panel-radius) / 2);
  color: var(--accent-danger);
  font-size: 0.9rem;
}
</style>
