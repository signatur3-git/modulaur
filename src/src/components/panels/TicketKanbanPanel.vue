<template>
  <div class="ticket-kanban-panel">
    <!-- Compact header for panel -->
    <div class="panel-controls">
      <button class="btn-sm btn-primary" @click="showCreateForm = true" title="New Ticket">
        + New
      </button>
      <button class="btn-sm" @click="showFilters = !showFilters" title="Filters">⚙️</button>
      <button class="btn-sm" @click="refreshTickets" title="Refresh">🔄</button>
    </div>

    <!-- Filters (collapsible) -->
    <div v-if="showFilters" class="filters-compact">
      <select v-model="filters.source" @change="applyFilters" class="filter-select">
        <option :value="undefined">All Sources</option>
        <option value="native">Native</option>
        <option value="jira">Jira</option>
        <option value="gitlab">GitLab</option>
        <option value="github">GitHub</option>
      </select>

      <select v-model="filters.ticket_type" @change="applyFilters" class="filter-select">
        <option :value="undefined">All Types</option>
        <option value="task">Task</option>
        <option value="bug">Bug</option>
        <option value="feature">Feature</option>
      </select>

      <select v-model="filters.priority" @change="applyFilters" class="filter-select">
        <option :value="undefined">All Priorities</option>
        <option value="low">Low</option>
        <option value="medium">Medium</option>
        <option value="high">High</option>
        <option value="critical">Critical</option>
      </select>

      <button class="btn-sm" @click="clearFilters">Clear</button>
    </div>

    <!-- Loading state -->
    <div v-if="ticketStore.loading" class="loading-state">
      <div class="spinner"></div>
      <p>Loading...</p>
    </div>

    <!-- Error state -->
    <div v-if="ticketStore.error" class="error-state">
      <p>{{ ticketStore.error }}</p>
      <button @click="refreshTickets" class="btn-sm">Retry</button>
    </div>

    <!-- Kanban columns -->
    <div v-if="!ticketStore.loading" class="kanban-columns">
      <div v-for="column in ticketStore.columns" :key="column.id" class="kanban-column">
        <!-- Column header -->
        <div class="column-header" :style="{ borderTopColor: column.color }">
          <h4>{{ column.name }}</h4>
          <span class="ticket-count">{{ getColumnTicketCount(column.id) }}</span>
          <button class="add-ticket-btn" @click="quickAddTicket(column.id)" title="Quick add">
            +
          </button>
        </div>

        <!-- Column tickets - with drag and drop -->
        <div class="draggable-container" @mousedown.stop @touchstart.stop>
          <VueDraggable
            :model-value="getColumnTickets(column.id)"
            @update:model-value="tickets => handleColumnUpdate(column.id, tickets)"
            :group="{ name: 'tickets', pull: true, put: true }"
            class="column-tickets"
            :animation="200"
            ghost-class="ticket-ghost"
            :force-fallback="true"
            :fallback-tolerance="3"
            @start="onDragStart"
            @end="onDragEnd"
          >
            <TicketCard
              v-for="ticket in getColumnTickets(column.id)"
              :key="ticket.id"
              :ticket="ticket"
              :isSelected="selectedTicketId === ticket.id"
              :isReadOnly="ticket.source !== 'native'"
              @click="selectTicket(ticket)"
              @menu="t => showTicketMenu(t)"
            />
          </VueDraggable>
        </div>

        <!-- Empty state -->
        <div v-if="getColumnTicketCount(column.id) === 0" class="empty-column">
          <p>No tickets</p>
          <button @click="quickAddTicket(column.id)" class="btn-link">+ Add</button>
        </div>
      </div>
    </div>

    <!-- Create/Edit Ticket Form Modal - Teleport to body to avoid z-index issues -->
    <Teleport to="body">
      <CreateTicketForm
        v-if="showCreateForm"
        :ticket="editingTicket"
        :board-id="boardId"
        @close="closeCreateForm"
        @created="handleTicketCreated"
        @updated="handleTicketUpdated"
      />
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { VueDraggable } from 'vue-draggable-plus'
import { useTicketStore, type Ticket, type TicketFilters } from '@/stores/ticketStore'
import { useToast } from '@/composables/useToast'
import TicketCard from '../tickets/TicketCard.vue'
import CreateTicketForm from '../tickets/CreateTicketForm.vue'

const props = defineProps<{
  panel: {
    i: string
    [key: string]: any
  }
}>()

// Each panel has its own board_id (using panel.i as the unique identifier)
const boardId = computed(() => props.panel.i)

const ticketStore = useTicketStore()
const toast = useToast()

// UI state
const showCreateForm = ref(false)
const showFilters = ref(false)
const editingTicket = ref<Ticket | undefined>(undefined)
const selectedTicketId = ref<string | null>(null)
const quickAddColumn = ref<string | null>(null)

// Filters
const filters = ref<TicketFilters>({})

// Column tickets reactive arrays for drag-and-drop
const columnTicketsMap = ref<Map<string, Ticket[]>>(new Map())

// Initialize columns with empty arrays
ticketStore.columns.forEach(column => {
  columnTicketsMap.value.set(column.id, [])
})

// Filter tickets for this board only
const boardTickets = computed(() => {
  return ticketStore.filteredTickets.filter(ticket => {
    // Check if ticket belongs to this board via metadata
    const ticketBoardId = ticket.metadata?.board_id
    return ticketBoardId === boardId.value
  })
})

// Update column tickets map when store changes
function updateColumnTicketsMap() {
  const byStatus = new Map<string, Ticket[]>()

  // Initialize all columns
  ticketStore.columns.forEach(col => {
    byStatus.set(col.id, [])
  })

  // Group board-specific tickets by status
  boardTickets.value.forEach(ticket => {
    if (!byStatus.has(ticket.status)) {
      byStatus.set(ticket.status, [])
    }
    byStatus.get(ticket.status)!.push(ticket)
  })

  // Update map
  byStatus.forEach((tickets, status) => {
    columnTicketsMap.value.set(status, [...tickets])
  })
}

// Get tickets for a column
function getColumnTickets(columnId: string): Ticket[] {
  if (!columnTicketsMap.value.has(columnId)) {
    columnTicketsMap.value.set(columnId, [])
  }
  return columnTicketsMap.value.get(columnId)!
}

// Get ticket count
function getColumnTicketCount(columnId: string): number {
  return getColumnTickets(columnId).length
}

// Update column tickets (for v-model binding)
function updateColumnTickets(columnId: string, tickets: Ticket[]) {
  columnTicketsMap.value.set(columnId, tickets)
}

// Handle column update (when tickets array changes due to drag-and-drop)
async function handleColumnUpdate(columnId: string, newTickets: Ticket[]) {
  console.log('🎯 ==========handleColumnUpdate called==========')
  console.log('  Column:', columnId)
  console.log(
    '  New tickets:',
    newTickets.map(t => ({ id: t.id, status: t.status }))
  )

  const oldTickets = getColumnTickets(columnId)
  console.log(
    '  Old tickets:',
    oldTickets.map(t => ({ id: t.id, status: t.status }))
  )

  // Update the map first (optimistic update)
  updateColumnTickets(columnId, newTickets)

  // Find tickets that were added to this column (present in new but not in old)
  const addedTickets = newTickets.filter(
    newTicket => !oldTickets.some(oldTicket => oldTicket.id === newTicket.id)
  )

  console.log(
    '  Added tickets:',
    addedTickets.map(t => ({ id: t.id, status: t.status }))
  )

  // Process each added ticket
  for (const ticket of addedTickets) {
    if (ticket.status !== columnId) {
      console.log(`  📤 Moving ticket ${ticket.id} from ${ticket.status} to ${columnId}`)

      try {
        const updated = await ticketStore.moveTicket(ticket.id, columnId)
        console.log('  ✅ Backend returned:', updated)

        // Show success feedback
        toast.success(`Ticket moved to ${columnId}`)
      } catch (err) {
        console.error('  ❌ Failed to move ticket:', err)
        toast.error('Failed to move ticket: ' + (err instanceof Error ? err.message : String(err)))

        // Revert by refreshing
        await refreshTickets()
        return // Exit early to avoid double refresh
      }
    } else {
      console.log(`  ⏭️ Ticket ${ticket.id} already has correct status`)
    }
  }

  // Force full refresh to ensure UI is in sync with database
  console.log('  🔄 Forcing full refresh...')
  await refreshTickets()
  console.log('  ✅ Refresh complete')
  console.log('🎯 ==========handleColumnUpdate complete==========')
}

// Drag start/end handlers to prevent grid-layout interference
function onDragStart(event: any) {
  // Stop event from bubbling to grid-layout
  event.stopPropagation?.()
  // Add body class to prevent text selection globally
  document.body.classList.add('dragging')
  console.log('🎯 Ticket drag started')
}

function onDragEnd(event: any) {
  // Stop event from bubbling to grid-layout
  event.stopPropagation?.()
  // Remove body class to restore normal selection
  document.body.classList.remove('dragging')
  console.log('🎯 Ticket drag ended')
}

// Select ticket
function selectTicket(ticket: Ticket) {
  selectedTicketId.value = ticket.id
  ticketStore.selectTicket(ticket)
}

// Show ticket menu
function showTicketMenu(ticket: Ticket) {
  if (ticket.source === 'native') {
    editingTicket.value = ticket
    showCreateForm.value = true
  }
}

// Quick add ticket
function quickAddTicket(columnId: string) {
  quickAddColumn.value = columnId
  editingTicket.value = undefined
  showCreateForm.value = true
}

// Close create form
function closeCreateForm() {
  showCreateForm.value = false
  editingTicket.value = undefined
  quickAddColumn.value = null
}

// Handle ticket created
async function handleTicketCreated(ticket: Ticket) {
  if (quickAddColumn.value && ticket.status !== quickAddColumn.value) {
    await ticketStore.moveTicket(ticket.id, quickAddColumn.value)
  }
  updateColumnTicketsMap()
  closeCreateForm()
}

// Handle ticket updated
function handleTicketUpdated() {
  updateColumnTicketsMap()
  closeCreateForm()
}

// Apply filters
function applyFilters() {
  ticketStore.setFilters(filters.value)
  updateColumnTicketsMap()
}

// Clear filters
function clearFilters() {
  filters.value = {}
  ticketStore.clearFilters()
  updateColumnTicketsMap()
}

// Refresh tickets
async function refreshTickets() {
  try {
    await ticketStore.loadTickets(filters.value)
    updateColumnTicketsMap()
  } catch (err) {
    console.error('Failed to refresh tickets:', err)
  }
}

// Load tickets on mount
onMounted(() => {
  refreshTickets()
})

// Watch for changes to board tickets and update columns
watch(
  () => boardTickets.value,
  () => {
    updateColumnTicketsMap()
  },
  { deep: true }
)

// Watch for changes to the global ticket store to refresh board tickets
watch(
  () => ticketStore.tickets.size,
  () => {
    updateColumnTicketsMap()
  }
)
</script>

<style>
.ticket-kanban-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-panel);
}

/* Compact panel controls */
.panel-controls {
  display: flex;
  gap: 6px;
  padding: 8px;
  background: var(--bg-panel);
  border-bottom: 1px solid var(--border-color);
}

.btn-sm {
  padding: 6px 10px;
  border: 1px solid #dee2e6;
  background: var(--bg-panel);
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-sm:hover {
  background: var(--bg-panel);
}

.btn-primary {
  background: var(--accent-primary);
  color: white;
  border-color: var(--accent-primary);
}

.btn-primary:hover {
  background: var(--accent-hover);
}

/* Compact filters */
.filters-compact {
  display: flex;
  gap: 6px;
  padding: 8px;
  background: var(--bg-panel);
  border-bottom: 1px solid var(--border-color);
  flex-wrap: wrap;
}

.filter-select {
  padding: 4px 8px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 0.85rem;
  background: var(--bg-panel);
}

/* Loading/Error states */
.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.spinner {
  width: 24px;
  height: 24px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid var(--accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 8px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.error-state {
  color: #dc3545;
}

/* Kanban columns */
.kanban-columns {
  flex: 1;
  display: flex;
  gap: 8px;
  padding: 8px;
  overflow-x: auto;
  min-height: 0;
  /* Prevent text selection during drag operations */
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

.kanban-column {
  flex: 1;
  min-width: 200px;
  max-width: 280px;
  display: flex;
  flex-direction: column;
  background: var(--bg-panel-header);
  border-radius: 6px;
  overflow: hidden;
}

/* Column header */
.column-header {
  background: var(--bg-panel);
  padding: 8px 10px;
  display: flex;
  align-items: center;
  gap: 6px;
  border-top: 3px solid var(--accent-primary);
}

.column-header h4 {
  flex: 1;
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-heading);
}

.ticket-count {
  background: var(--bg-panel-header);
  padding: 2px 6px;
  border-radius: 10px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary);
}

.add-ticket-btn {
  width: 24px;
  height: 24px;
  border: none;
  background: var(--accent-primary);
  color: white;
  border-radius: 3px;
  font-size: 1rem;
  line-height: 1;
  cursor: pointer;
  transition: all 0.2s;
}

.add-ticket-btn:hover {
  background: var(--accent-hover);
}

/* Column tickets */
.draggable-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 50px;
  /* Prevent grid-layout from capturing events */
  position: relative;
  z-index: 2;
}

.column-tickets {
  flex: 1;
  padding: 8px;
  overflow-y: auto;
  min-height: 50px;
  /* Ensure ticket dragging works independently */
  position: relative;
  z-index: 1;
  pointer-events: auto;
}

/* Ensure ticket cards can be dragged */
.column-tickets > * {
  pointer-events: auto;
  touch-action: none; /* Allow dragging on touch devices */
}

.ticket-ghost {
  opacity: 0.5;
  background: var(--accent-primary);
}

/* Empty column */
.empty-column {
  text-align: center;
  padding: 20px 10px;
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.empty-column p {
  margin: 0 0 8px 0;
}

.btn-link {
  background: none;
  border: none;
  color: var(--accent-primary);
  font-size: 0.85rem;
  cursor: pointer;
  text-decoration: underline;
}

.btn-link:hover {
  color: var(--accent-hover);
}

/* Scrollbar styling */
.kanban-columns::-webkit-scrollbar,
.column-tickets::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.kanban-columns::-webkit-scrollbar-track,
.column-tickets::-webkit-scrollbar-track {
  background: var(--bg-panel-header);
}

.kanban-columns::-webkit-scrollbar-thumb,
.column-tickets::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
}

.kanban-columns::-webkit-scrollbar-thumb:hover,
.column-tickets::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}
</style>
