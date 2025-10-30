import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useTimeTracker } from './useTimeTracker';

export interface Project {
  name: string;
  description?: string;
  billable: boolean;
  hourlyRate?: number;
  tags: string[];
  status: 'active' | 'on-hold' | 'completed' | 'archived';
  totalTime?: number;
  createdAt?: string;
  updatedAt?: string;
}

export interface ProjectRecord {
  id: any;
  record_type: string;
  source: string;
  timestamp: string;
  data: Project;
  metadata?: any;
}

/**
 * Extract string ID from SurrealDB Thing object or return as-is if already string
 */
function extractId(id: any): string {
  if (typeof id === 'string') {
    return id;
  }
  if (id && typeof id === 'object') {
    if (id.id && typeof id.id === 'object' && id.id.String) {
      return `${id.tb}:${id.id.String}`;
    }
    if (id.id && typeof id.id === 'string') {
      return `${id.tb}:${id.id}`;
    }
  }
  return String(id);
}

/**
 * Extract ID for delete/update operations
 * SurrealDB.delete and update expects just the ID part, not "table:id"
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

export function useProjectManager() {
  const projects = ref<Project[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const { entries } = useTimeTracker();

  /**
   * Load all projects from the database
   */
  async function loadProjects() {
    loading.value = true;
    error.value = null;

    try {
      const records = await invoke<ProjectRecord[]>('get_records_by_type', {
        recordType: 'project'
      });

      // Calculate total time for each project
      const projectsWithTime = records.map(record => {
        const projectName = record.data.name;
        const projectEntries = entries.value.filter(
          entry => entry.data.project === projectName && entry.data.duration_seconds
        );
        const totalTime = projectEntries.reduce(
          (sum, entry) => sum + (entry.data.duration_seconds || 0),
          0
        );

        return {
          ...record.data,
          totalTime
        };
      });

      projects.value = projectsWithTime.sort((a, b) => {
        // Sort by status (active first), then by name
        const statusOrder = { active: 0, 'on-hold': 1, completed: 2, archived: 3 };
        const statusDiff = statusOrder[a.status] - statusOrder[b.status];
        if (statusDiff !== 0) return statusDiff;
        return a.name.localeCompare(b.name);
      });
    } catch (err) {
      console.error('Failed to load projects:', err);
      error.value = err instanceof Error ? err.message : 'Failed to load projects';
      projects.value = [];
    } finally {
      loading.value = false;
    }
  }

  /**
   * Create a new project
   */
  async function createProject(project: Omit<Project, 'createdAt' | 'updatedAt'>) {
    loading.value = true;
    error.value = null;

    try {
      const now = new Date().toISOString();
      const record = {
        record_type: 'project',
        source: 'time-tracker-plugin',
        timestamp: now,
        data: {
          ...project,
          createdAt: now,
          updatedAt: now
        },
        metadata: {
          tags: ['time-tracker', 'project'],
          status: 'active',
          title: project.name,
          description: project.description || null
        }
      };

      await invoke('upsert_record', { record });
      await loadProjects();
    } catch (err) {
      console.error('Failed to create project:', err);
      error.value = err instanceof Error ? err.message : 'Failed to create project';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Update an existing project
   */
  async function updateProject(
    projectName: string,
    updates: Partial<Omit<Project, 'createdAt' | 'updatedAt'>>
  ) {
    loading.value = true;
    error.value = null;

    try {
      // Find the project record
      const records = await invoke<ProjectRecord[]>('get_records_by_type', {
        recordType: 'project'
      });

      const projectRecord = records.find(r => r.data.name === projectName);
      if (!projectRecord) {
        throw new Error(`Project "${projectName}" not found`);
      }

      const recordId = extractIdForDelete(projectRecord.id);
      const now = new Date().toISOString();

      const updatedRecord = {
        record_type: 'project',
        source: 'time-tracker-plugin',
        timestamp: projectRecord.timestamp,
        data: {
          ...projectRecord.data,
          ...updates,
          updatedAt: now
        },
        metadata: {
          ...projectRecord.metadata,
          tags: ['time-tracker', 'project'],
          title: updates.name || projectRecord.data.name,
          description: updates.description !== undefined ? updates.description : projectRecord.data.description
        }
      };

      await invoke('update_record', {
        id: recordId,
        record: updatedRecord
      });

      await loadProjects();
    } catch (err) {
      console.error('Failed to update project:', err);
      error.value = err instanceof Error ? err.message : 'Failed to update project';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Delete a project
   */
  async function deleteProject(projectName: string) {
    loading.value = true;
    error.value = null;

    try {
      // Find the project record
      const records = await invoke<ProjectRecord[]>('get_records_by_type', {
        recordType: 'project'
      });

      const projectRecord = records.find(r => r.data.name === projectName);
      if (!projectRecord) {
        throw new Error(`Project "${projectName}" not found`);
      }

      const recordId = extractIdForDelete(projectRecord.id);
      await invoke('delete_record', { id: recordId });

      await loadProjects();
    } catch (err) {
      console.error('Failed to delete project:', err);
      error.value = err instanceof Error ? err.message : 'Failed to delete project';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Get project by name
   */
  function getProject(projectName: string): Project | undefined {
    return projects.value.find(p => p.name === projectName);
  }

  /**
   * Get active projects
   */
  const activeProjects = computed(() => {
    return projects.value.filter(p => p.status === 'active');
  });

  /**
   * Get billable projects
   */
  const billableProjects = computed(() => {
    return projects.value.filter(p => p.billable);
  });

  return {
    projects,
    loading,
    error,
    activeProjects,
    billableProjects,
    loadProjects,
    createProject,
    updateProject,
    deleteProject,
    getProject
  };
}

