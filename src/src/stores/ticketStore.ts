// Ticket Store - State management for Kanban/Ticket system
// Handles CRUD operations for native and external tickets

import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// ============================================================================
// Types
// ============================================================================

export type TicketSource = 'native' | 'jira' | 'gitlab' | 'github'
export type TicketType = 'task' | 'bug' | 'feature' | 'epic' | 'story'
export type Priority = 'low' | 'medium' | 'high' | 'critical'

export interface Ticket {
  id: string
  source: TicketSource
  source_id?: string

  title: string
  description?: string
  ticket_type: TicketType

  status: string
  priority: Priority
  tags: string[]

  assignee?: string
  reporter?: string

  estimate?: number
  time_spent?: number
  due_date?: string

  created_at: string
  updated_at: string

  parent_id?: string
  linked_tickets: string[]

  comments: Comment[]
  metadata: Record<string, any>
}

export interface Comment {
  id: string
  author: string
  text: string
  created_at: string
}

export interface CreateTicketData {
  title: string
  description?: string
  ticket_type: TicketType
  priority?: Priority
  assignee?: string
  tags?: string[]
  estimate?: number
  due_date?: string
  metadata?: Record<string, any> // Allow passing metadata (e.g., board_id)
}

export interface UpdateTicketData {
  title?: string
  description?: string
  status?: string
  priority?: Priority
  assignee?: string
  tags?: string[]
  estimate?: number
  time_spent?: number
  due_date?: string
}

export interface TicketFilters {
  source?: TicketSource
  ticket_type?: TicketType
  status?: string
  priority?: Priority
  assignee?: string
  tags?: string[]
}

export interface KanbanColumn {
  id: string
  name: string
  order: number
  color: string
  is_start?: boolean
  is_done?: boolean
}

// ============================================================================
// Store
// ============================================================================

interface TicketState {
  tickets: Map<string, Ticket>
  columns: KanbanColumn[]
  filters: TicketFilters
  selectedTicket: Ticket | null
  loading: boolean
  error: string | null
}

export const useTicketStore = defineStore('tickets', {
  state: (): TicketState => ({
    tickets: new Map(),
    columns: [
      { id: 'backlog', name: 'Backlog', order: 0, color: '#6c757d' },
      { id: 'todo', name: 'To Do', order: 1, color: '#007bff', is_start: true },
      { id: 'in-progress', name: 'In Progress', order: 2, color: '#ffc107' },
      { id: 'review', name: 'Review', order: 3, color: '#6f42c1' },
      { id: 'done', name: 'Done', order: 4, color: '#28a745', is_done: true },
    ],
    filters: {},
    selectedTicket: null,
    loading: false,
    error: null,
  }),

  getters: {
    // Get all tickets as array
    allTickets(): Ticket[] {
      return Array.from(this.tickets.values())
    },

    // Get tickets grouped by status/column
    ticketsByStatus(): Map<string, Ticket[]> {
      const byStatus = new Map<string, Ticket[]>()

      // Initialize all columns
      this.columns.forEach(col => {
        byStatus.set(col.id, [])
      })

      // Group tickets
      this.tickets.forEach(ticket => {
        if (!byStatus.has(ticket.status)) {
          byStatus.set(ticket.status, [])
        }
        byStatus.get(ticket.status)!.push(ticket)
      })

      return byStatus
    },

    // Get filtered tickets
    filteredTickets(): Ticket[] {
      let tickets = this.allTickets

      if (this.filters.source) {
        tickets = tickets.filter(t => t.source === this.filters.source)
      }
      if (this.filters.ticket_type) {
        tickets = tickets.filter(t => t.ticket_type === this.filters.ticket_type)
      }
      if (this.filters.status) {
        tickets = tickets.filter(t => t.status === this.filters.status)
      }
      if (this.filters.priority) {
        tickets = tickets.filter(t => t.priority === this.filters.priority)
      }
      if (this.filters.assignee) {
        tickets = tickets.filter(t => t.assignee === this.filters.assignee)
      }
      if (this.filters.tags && this.filters.tags.length > 0) {
        tickets = tickets.filter(t => this.filters.tags!.some(tag => t.tags.includes(tag)))
      }

      return tickets
    },

    // Get filtered tickets grouped by status
    filteredTicketsByStatus(): Map<string, Ticket[]> {
      const byStatus = new Map<string, Ticket[]>()

      // Initialize all columns
      this.columns.forEach(col => {
        byStatus.set(col.id, [])
      })

      // Group filtered tickets
      this.filteredTickets.forEach(ticket => {
        if (!byStatus.has(ticket.status)) {
          byStatus.set(ticket.status, [])
        }
        byStatus.get(ticket.status)!.push(ticket)
      })

      return byStatus
    },

    // Get ticket count by status
    ticketCountByStatus(): Map<string, number> {
      const counts = new Map<string, number>()
      this.ticketsByStatus.forEach((tickets, status) => {
        counts.set(status, tickets.length)
      })
      return counts
    },

    // Get all unique assignees
    allAssignees(): string[] {
      const assignees = new Set<string>()
      this.tickets.forEach(ticket => {
        if (ticket.assignee) {
          assignees.add(ticket.assignee)
        }
      })
      return Array.from(assignees).sort()
    },

    // Get all unique tags
    allTags(): string[] {
      const tags = new Set<string>()
      this.tickets.forEach(ticket => {
        ticket.tags.forEach(tag => tags.add(tag))
      })
      return Array.from(tags).sort()
    },
  },

  actions: {
    // ========================================================================
    // Create
    // ========================================================================

    async createTicket(data: CreateTicketData): Promise<Ticket> {
      console.log('📝 Creating ticket:', data)
      this.loading = true
      this.error = null

      try {
        const ticket = await invoke<Ticket>('create_ticket', { ticket: data })
        this.tickets.set(ticket.id, ticket)
        console.log('✅ Ticket created:', ticket)
        return ticket
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        console.error('❌ Failed to create ticket:', error)
        this.error = error
        throw err
      } finally {
        this.loading = false
      }
    },

    // ========================================================================
    // Read
    // ========================================================================

    async loadTickets(filters?: TicketFilters): Promise<void> {
      console.log('📥 Loading tickets with filters:', filters)
      this.loading = true
      this.error = null

      try {
        const tickets = await invoke<Ticket[]>('get_tickets', { filters })

        // Clear existing and add new
        this.tickets.clear()
        tickets.forEach((ticket: Ticket) => {
          this.tickets.set(ticket.id, ticket)
        })

        console.log(`✅ Loaded ${tickets.length} tickets`)
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        console.error('❌ Failed to load tickets:', error)
        this.error = error
        throw err
      } finally {
        this.loading = false
      }
    },

    // ========================================================================
    // Update
    // ========================================================================

    async updateTicket(id: string, updates: UpdateTicketData): Promise<Ticket> {
      console.log('📝 Updating ticket:', id, updates)
      this.loading = true
      this.error = null

      try {
        const ticket = await invoke<Ticket>('update_ticket', { id, updates })
        this.tickets.set(ticket.id, ticket)

        // Update selected ticket if it's the one being updated
        if (this.selectedTicket?.id === id) {
          this.selectedTicket = ticket
        }

        console.log('✅ Ticket updated:', ticket)
        return ticket
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        console.error('❌ Failed to update ticket:', error)
        this.error = error
        throw err
      } finally {
        this.loading = false
      }
    },

    // ========================================================================
    // Delete
    // ========================================================================

    async deleteTicket(id: string): Promise<void> {
      console.log('🗑️ Deleting ticket:', id)
      this.loading = true
      this.error = null

      try {
        await invoke('delete_ticket', { id })
        this.tickets.delete(id)

        // Clear selected ticket if it's the one being deleted
        if (this.selectedTicket?.id === id) {
          this.selectedTicket = null
        }

        console.log('✅ Ticket deleted:', id)
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        console.error('❌ Failed to delete ticket:', error)
        this.error = error
        throw err
      } finally {
        this.loading = false
      }
    },

    // ========================================================================
    // Move (Change Status)
    // ========================================================================

    async moveTicket(id: string, newStatus: string): Promise<Ticket> {
      console.log('🔄 Moving ticket:', id, '→', newStatus)
      this.loading = true
      this.error = null

      try {
        const ticket = await invoke<Ticket>('move_ticket', {
          id,
          newStatus,
        })
        this.tickets.set(ticket.id, ticket)

        // Update selected ticket if it's the one being moved
        if (this.selectedTicket?.id === id) {
          this.selectedTicket = ticket
        }

        console.log('✅ Ticket moved:', ticket)
        return ticket
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        console.error('❌ Failed to move ticket:', error)
        this.error = error
        throw err
      } finally {
        this.loading = false
      }
    },

    // ========================================================================
    // Comments
    // ========================================================================

    async addComment(ticketId: string, author: string, text: string): Promise<Comment> {
      console.log('💬 Adding comment to ticket:', ticketId)
      this.loading = true
      this.error = null

      try {
        const comment = await invoke<Comment>('add_comment', {
          ticketId,
          comment: { author, text },
        })

        // Update local ticket with new comment
        const ticket = this.tickets.get(ticketId)
        if (ticket) {
          ticket.comments.push(comment)
          this.tickets.set(ticketId, ticket)

          // Update selected ticket if it's the one receiving comment
          if (this.selectedTicket?.id === ticketId) {
            this.selectedTicket = ticket
          }
        }

        console.log('✅ Comment added:', comment)
        return comment
      } catch (err) {
        const error = err instanceof Error ? err.message : String(err)
        console.error('❌ Failed to add comment:', error)
        this.error = error
        throw err
      } finally {
        this.loading = false
      }
    },

    // ========================================================================
    // UI State Management
    // ========================================================================

    selectTicket(ticket: Ticket | null) {
      this.selectedTicket = ticket
    },

    setFilters(filters: TicketFilters) {
      this.filters = filters
    },

    clearFilters() {
      this.filters = {}
    },

    clearError() {
      this.error = null
    },
  },
})
