import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { calculateDuration } from '../utils/duration';
import { calculateSummary, getTodayEntries, getThisWeekEntries } from '../utils/aggregation';
import { generateCSV, downloadCSV } from '../utils/csv';

export interface RecordMetadata {
  tags?: string[];
  status?: string | null;
  title?: string | null;
  description?: string | null;
}

export interface TimeEntry {
  id: any; // Can be string or SurrealDB Thing object
  record_type: string;
  source: string;
  timestamp: string;
  data: {
    ticket_id?: string | null;
    project: string;
    description: string;
    end_time?: string | null;
    duration_seconds?: number | null;
    tags?: string[];
    billable?: boolean;
    hourly_rate?: number | null;
    notes?: string | null;
  };
  metadata?: RecordMetadata;
}

/**
 * Extract string ID from SurrealDB Thing object or return as-is if already string
 * Handles Vue Proxy objects by accessing the underlying value
 */
function extractId(id: any): string {
  // If it's already a string, return it
  if (typeof id === 'string') {
    return id;
  }

  // Handle null/undefined
  if (!id) {
    return '';
  }

  // Unwrap Vue Proxy if needed
  const rawId = id && typeof id === 'object' ? JSON.parse(JSON.stringify(id)) : id;

  // SurrealDB Thing format: { tb: "records", id: { String: "xyz" } }
  if (rawId && typeof rawId === 'object') {
    // Check if it has tb and id properties (SurrealDB Thing)
    if (rawId.tb && rawId.id) {
      // Handle nested String wrapper: id: { String: "xyz" }
      if (typeof rawId.id === 'object' && rawId.id.String) {
        return `${rawId.tb}:${rawId.id.String}`;
      }
      // Handle direct string id: id: "xyz"
      if (typeof rawId.id === 'string') {
        return `${rawId.tb}:${rawId.id}`;
      }
    }
  }

  return String(rawId);
}

/**
 * Extract ID without table prefix for SurrealDB commands
 * SurrealDB.delete expects just the ID part, not "table:id"
 */
function extractIdForDelete(id: any): string {
  const fullId = extractId(id); // Get "records:xyz123"

  // Strip table prefix if present
  if (fullId.includes(':')) {
    const parts = fullId.split(':');
    return parts.slice(1).join(':'); // Return just "xyz123"
  }

  return fullId;
}

export interface TimeEntryFilters {
  project?: string;
  ticket_id?: string;
  startDate?: string;
  endDate?: string;
  billable?: boolean;
  tags?: string[];
}

export function useTimeTracker() {
  const entries = ref<TimeEntry[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Get active timer (entry without end_time)
  const activeTimer = computed(() => {
    return entries.value.find(e =>
      e.record_type === 'time_entry' &&
      e.data.end_time === null
    );
  });

  // Get today's entries
  const todayEntries = computed(() => getTodayEntries(entries.value));

  // Get this week's entries
  const weekEntries = computed(() => getThisWeekEntries(entries.value));

  // Get unique projects
  const projects = computed(() => {
    const projectSet = new Set(entries.value.map(e => e.data.project));
    return Array.from(projectSet).sort();
  });

  /**
   * Load all time entries
   */
  async function loadEntries() {
    loading.value = true;
    error.value = null;

    try {
      console.log('📥 Loading time entries from backend...');
      const records = await invoke<TimeEntry[]>('get_records_by_type', {
        recordType: 'time_entry'
      });

      console.log(`📥 Received ${records.length} time entries from backend`);

      // Log all entry IDs for debugging
      records.forEach((record, index) => {
        const id = extractId(record.id);
        console.log(`  [${index}] ID: ${id}, Project: ${record.data.project}, Time: ${record.timestamp}`);
      });

      entries.value = records.sort((a, b) =>
        new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime()
      );
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error('Failed to load time entries:', err);
    } finally {
      loading.value = false;
    }
  }

  /**
   * Start a timer
   */
  async function startTimer(
    project: string,
    description: string,
    ticketId?: string
  ): Promise<void> {
    // Check if there's already an active timer
    if (activeTimer.value) {
      throw new Error('Timer already running. Stop the current timer first.');
    }

    loading.value = true;
    error.value = null;

    try {
      const entry: Partial<TimeEntry> = {
        record_type: 'time_entry',
        source: 'time-tracker',
        timestamp: new Date().toISOString(),
        data: {
          project,
          description,
          ticket_id: ticketId || null,
          end_time: null,
          duration_seconds: null,
          tags: [],
          billable: true,
          hourly_rate: null,
          notes: null
        },
        metadata: {
          tags: [],
          status: 'active',
          title: `${project}: ${description}`,
          description: ticketId ? `Ticket: ${ticketId}` : null
        }
      };

      await invoke('upsert_record', { record: entry });
      await loadEntries();
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error('Failed to start timer:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Stop the active timer
   */
  async function stopTimer(): Promise<void> {
    if (!activeTimer.value) {
      throw new Error('No active timer');
    }

    loading.value = true;
    error.value = null;

    try {
      const endTime = new Date().toISOString();
      const durationSeconds = calculateDuration(activeTimer.value.timestamp, endTime);

      const updated: TimeEntry = {
        ...activeTimer.value,
        data: {
          ...activeTimer.value.data,
          end_time: endTime,
          duration_seconds: durationSeconds
        },
        metadata: {
          ...activeTimer.value.metadata,
          status: 'completed'
        }
      };

      await invoke('upsert_record', { record: updated });
      await loadEntries();
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error('Failed to stop timer:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Create a manual time entry
   */
  async function createEntry(data: {
    project: string;
    description: string;
    startTime: string;
    endTime?: string;
    ticketId?: string;
    tags?: string[];
    billable?: boolean;
    hourlyRate?: number;
    notes?: string;
  }): Promise<void> {
    loading.value = true;
    error.value = null;

    try {
      const durationSeconds = data.endTime
        ? calculateDuration(data.startTime, data.endTime)
        : null;

      const entry: Partial<TimeEntry> = {
        record_type: 'time_entry',
        source: 'time-tracker',
        timestamp: data.startTime,
        data: {
          project: data.project,
          description: data.description,
          ticket_id: data.ticketId || null,
          end_time: data.endTime || null,
          duration_seconds: durationSeconds,
          tags: data.tags || [],
          billable: data.billable ?? true,
          hourly_rate: data.hourlyRate || null,
          notes: data.notes || null
        },
        metadata: {
          tags: data.tags || [],
          status: data.endTime ? 'completed' : 'active',
          title: `${data.project}: ${data.description}`,
          description: data.ticketId ? `Ticket: ${data.ticketId}` : null
        }
      };

      await invoke('upsert_record', { record: entry });
      await loadEntries();
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error('Failed to create entry:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Update an existing entry
   */
  async function updateEntry(
    id: any,
    updates: Partial<TimeEntry['data']>
  ): Promise<void> {
    loading.value = true;
    error.value = null;

    try {
      const existing = entries.value.find(e => {
        const entryId = extractId(e.id);
        const targetId = extractId(id);
        return entryId === targetId;
      });

      if (!existing) {
        throw new Error(`Entry not found with ID: ${extractId(id)}`);
      }

      // Recalculate duration if times changed
      let durationSeconds = existing.data.duration_seconds;
      const startTime = existing.timestamp;

      if (updates.end_time && startTime) {
        durationSeconds = calculateDuration(startTime, updates.end_time);
      } else if (!updates.end_time && updates.end_time !== undefined) {
        // If end_time is explicitly set to null/undefined, clear duration
        durationSeconds = null;
      }

      // Backend update_record expects the bare id part, not "records:<id>"
      const recordId = extractIdForDelete(existing.id);
      if (!recordId) {
        throw new Error('Invalid or empty ID extracted');
      }

      // Updated record payload (do NOT include id; backend clears it anyway)
      const updatedRecord = {
        record_type: 'time_entry',
        source: 'time-tracker',
        timestamp: startTime,
        data: {
          ...existing.data,
          ...updates,
          duration_seconds: durationSeconds
        },
        metadata: {
          tags: updates.tags || existing.data.tags || [],
          status: updates.end_time ? 'completed' : (existing.metadata?.status || 'active'),
          title: `${updates.project || existing.data.project}: ${updates.description || existing.data.description}`,
          description: updates.ticket_id || existing.data.ticket_id || null
        }
      };

      await invoke('update_record', { id: recordId, record: updatedRecord });
      await loadEntries();
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error('Failed to update entry:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Delete an entry
   */
  async function deleteEntry(id: any): Promise<void> {
    loading.value = true;
    error.value = null;

    try {
      // Log the raw ID structure
      console.log('🗑️ Delete called with raw ID:', id);

      // Extract ID WITHOUT table prefix (SurrealDB delete needs just the ID)
      const recordId = extractIdForDelete(id);
      console.log('🗑️ Extracted ID for backend (table prefix stripped):', recordId);

      if (!recordId) {
        throw new Error('Invalid or empty ID extracted');
      }

      // Call backend delete with just the ID part
      await invoke('delete_record', { id: recordId });
      console.log('🗑️ Backend delete successful, reloading entries...');

      // Reload entries to refresh the list
      await loadEntries();
      console.log('🗑️ Entries reloaded, count:', entries.value.length);
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error('Failed to delete entry:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Filter entries
   */
  function filterEntries(filters: TimeEntryFilters): TimeEntry[] {
    return entries.value.filter(entry => {
      if (filters.project && entry.data.project !== filters.project) return false;
      if (filters.ticket_id && entry.data.ticket_id !== filters.ticket_id) return false;
      if (filters.startDate && entry.timestamp < filters.startDate) return false;
      if (filters.endDate && entry.timestamp > filters.endDate) return false;
      if (filters.billable !== undefined && entry.data.billable !== filters.billable) return false;

      if (filters.tags && filters.tags.length > 0) {
        const entryTags = entry.data.tags || [];
        const hasAllTags = filters.tags.every(tag => entryTags.includes(tag));
        if (!hasAllTags) return false;
      }

      return true;
    });
  }

  /**
   * Get summary for filtered entries
   */
  function getSummary(filteredEntries?: TimeEntry[]) {
    return calculateSummary(filteredEntries || entries.value);
  }

  /**
   * Export entries to CSV
   */
  function exportCSV(filteredEntries?: TimeEntry[], filename?: string) {
    const entriesToExport = filteredEntries || entries.value;
    const csv = generateCSV(entriesToExport);
    const defaultFilename = `timesheet-${new Date().toISOString().split('T')[0]}.csv`;
    downloadCSV(csv, filename || defaultFilename);
  }

  return {
    // State
    entries,
    loading,
    error,
    activeTimer,
    todayEntries,
    weekEntries,
    projects,

    // Actions
    loadEntries,
    startTimer,
    stopTimer,
    createEntry,
    updateEntry,
    deleteEntry,
    filterEntries,
    getSummary,
    exportCSV
  };
}

