<template>
  <div
    class="ticket-card"
    :class="[
      `priority-${ticket.priority}`,
      `source-${ticket.source}`,
      { 'is-selected': isSelected },
    ]"
    @click="$emit('click', ticket)"
  >
    <!-- Header -->
    <div class="ticket-header">
      <span class="ticket-type-icon">{{ typeIcon }}</span>
      <span class="ticket-id">{{ ticketId }}</span>
      <span class="source-badge" :class="`source-${ticket.source}`">
        {{ ticket.source }}
      </span>
      <button
        v-if="!isReadOnly"
        class="ticket-menu-btn"
        @click.stop="$emit('menu', ticket)"
        title="Options"
      >
        ⋮
      </button>
    </div>

    <!-- Title -->
    <div class="ticket-title">{{ ticket.title }}</div>

    <!-- Metadata -->
    <div class="ticket-metadata">
      <!-- Assignee -->
      <div v-if="ticket.assignee" class="metadata-item" title="Assignee">
        <span class="icon">👤</span>
        <span class="text">{{ ticket.assignee }}</span>
      </div>

      <!-- Tags -->
      <div v-if="ticket.tags.length > 0" class="metadata-item tags">
        <span class="icon">🏷️</span>
        <span class="tags-list">
          <span v-for="tag in ticket.tags.slice(0, 2)" :key="tag" class="tag">
            {{ tag }}
          </span>
          <span v-if="ticket.tags.length > 2" class="tag more">
            +{{ ticket.tags.length - 2 }}
          </span>
        </span>
      </div>

      <!-- Due Date -->
      <div v-if="ticket.due_date" class="metadata-item" :class="{ overdue: isOverdue }">
        <span class="icon">⏰</span>
        <span class="text">{{ formattedDueDate }}</span>
      </div>

      <!-- Estimate -->
      <div v-if="ticket.estimate" class="metadata-item" title="Estimate">
        <span class="icon">📊</span>
        <span class="text">{{ ticket.estimate }} pts</span>
      </div>

      <!-- Comments -->
      <div v-if="ticket.comments.length > 0" class="metadata-item" title="Comments">
        <span class="icon">💬</span>
        <span class="text">{{ ticket.comments.length }}</span>
      </div>
    </div>

    <!-- Priority indicator -->
    <div class="priority-bar" :class="`priority-${ticket.priority}`"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Ticket } from '@/stores/ticketStore'

const props = defineProps<{
  ticket: Ticket
  isSelected?: boolean
  isReadOnly?: boolean
}>()

defineEmits<{
  click: [ticket: Ticket]
  menu: [ticket: Ticket]
}>()

// Type icon mapping
const typeIcon = computed(() => {
  const icons: Record<string, string> = {
    task: '✓',
    bug: '🐛',
    feature: '✨',
    epic: '🎯',
    story: '📖',
  }
  return icons[props.ticket.ticket_type] || '•'
})

// Ticket ID (short form)
const ticketId = computed(() => {
  if (props.ticket.source_id) {
    return props.ticket.source_id
  }
  // Extract last part of ID for native tickets
  // Handle both string IDs and object IDs from SurrealDB
  const idString = typeof props.ticket.id === 'string' ? props.ticket.id : String(props.ticket.id)

  const parts = idString.split(':')
  return parts[parts.length - 1].substring(0, 8)
})

// Due date formatting
const formattedDueDate = computed(() => {
  if (!props.ticket.due_date) return ''

  const due = new Date(props.ticket.due_date)
  const now = new Date()
  const diffDays = Math.ceil((due.getTime() - now.getTime()) / (1000 * 60 * 60 * 24))

  if (diffDays < 0) {
    return `${Math.abs(diffDays)}d overdue`
  } else if (diffDays === 0) {
    return 'Due today'
  } else if (diffDays === 1) {
    return 'Due tomorrow'
  } else if (diffDays < 7) {
    return `${diffDays}d left`
  } else {
    return due.toLocaleDateString()
  }
})

const isOverdue = computed(() => {
  if (!props.ticket.due_date) return false
  return new Date(props.ticket.due_date) < new Date()
})
</script>

<style scoped>
.ticket-card {
  background: var(--bg-panel);
  border: 1px solid var(--border-subtle);
  border-radius: calc(var(--panel-radius) / 2);
  padding: var(--space-md);
  margin-bottom: var(--space-sm);
  cursor: grab; /* Show that card can be dragged */
  transition: all 0.2s;
  position: relative;
  overflow: hidden;
  /* Prevent text selection during drag */
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.ticket-card:active {
  cursor: grabbing; /* Show that card is being dragged */
}

.ticket-card:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
  border-color: var(--border-color);
}

.ticket-card.is-selected {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 2px rgba(102, 126, 234, 0.2);
}

/* Header */
.ticket-header {
  display: flex;
  align-items: center;
  gap: calc(var(--space-xs) * 1.5);
  margin-bottom: var(--space-sm);
  font-size: 0.85rem;
}

.ticket-type-icon {
  font-size: 1rem;
}

.ticket-id {
  color: var(--text-muted);
  font-family: 'Courier New', monospace;
  font-size: 0.8rem;
}

.source-badge {
  padding: calc(var(--space-xs) * 0.5) calc(var(--space-sm) * 0.75);
  border-radius: 3px;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
}

.source-badge.source-native {
  background: var(--accent-primary);
  color: var(--text-on-accent);
}

.source-badge.source-jira {
  background: var(--accent-success);
  color: var(--text-on-accent);
}

.source-badge.source-gitlab {
  background: var(--accent-warning);
  color: var(--text-on-accent);
}

.source-badge.source-github {
  background: var(--accent-secondary);
  color: var(--text-on-accent);
}

.ticket-menu-btn {
  margin-left: auto;
  background: none;
  border: none;
  font-size: 1.2rem;
  cursor: pointer;
  padding: 0 calc(var(--space-xs));
  color: var(--text-muted);
  line-height: 1;
  border-radius: 3px;
  transition: all 0.2s;
}

.ticket-menu-btn:hover {
  color: var(--text-primary);
  background: var(--bg-panel-header);
}

/* Title */
.ticket-title {
  font-weight: 600;
  font-size: 0.95rem;
  line-height: 1.4;
  margin-bottom: var(--space-sm);
  color: var(--text-primary);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

/* Metadata */
.ticket-metadata {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.metadata-item {
  display: flex;
  align-items: center;
  gap: calc(var(--space-xs));
}

.metadata-item .icon {
  font-size: 0.9rem;
}

.metadata-item.overdue {
  color: var(--accent-danger);
  font-weight: 600;
}

/* Tags */
.tags-list {
  display: flex;
  gap: calc(var(--space-xs));
  flex-wrap: wrap;
}

.tag {
  background: var(--bg-panel-header);
  padding: calc(var(--space-xs) * 0.5) calc(var(--space-sm) * 0.75);
  border-radius: 3px;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.tag.more {
  background: var(--border-color);
  font-weight: 600;
  color: var(--text-primary);
}

/* Priority bar */
.priority-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 3px;
}

.priority-bar.priority-low {
  background: var(--accent-success);
}

.priority-bar.priority-medium {
  background: var(--accent-warning);
}

.priority-bar.priority-high {
  background: #fd7e14; /* Keep orange for high priority */
}

.priority-bar.priority-critical {
  background: var(--accent-danger);
}

/* Priority-based card styling */
.ticket-card.priority-critical {
  border-left: 3px solid var(--accent-danger);
}

.ticket-card.priority-high {
  border-left: 3px solid #fd7e14; /* Keep orange for high priority */
}

/* Read-only styling */
.ticket-card.source-gitlab,
.ticket-card.source-jira {
  opacity: 0.95;
}
</style>
