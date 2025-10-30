<template>
  <div class="project-manager-panel">
    <div class="panel-header">
      <h2>📁 Project Management</h2>
      <button @click="showCreateForm = true" class="btn-primary">
        ➕ New Project
      </button>
    </div>

    <!-- Create/Edit Project Form -->
    <div v-if="showCreateForm || editingProject" class="project-form">
      <h3>{{ editingProject ? 'Edit Project' : 'Create New Project' }}</h3>

      <div class="form-group">
        <label>Project Name *</label>
        <input
          v-model="formData.name"
          type="text"
          placeholder="Enter project name"
          required
        />
      </div>

      <div class="form-group">
        <label>Description</label>
        <textarea
          v-model="formData.description"
          placeholder="Project description"
          rows="3"
        />
      </div>

      <div class="form-row">
        <div class="form-group">
          <label>Billable</label>
          <input
            v-model="formData.billable"
            type="checkbox"
          />
        </div>

        <div class="form-group" v-if="formData.billable">
          <label>Hourly Rate ($)</label>
          <input
            v-model.number="formData.hourlyRate"
            type="number"
            min="0"
            step="0.01"
            placeholder="0.00"
          />
        </div>
      </div>

      <div class="form-group">
        <label>Tags (comma-separated)</label>
        <input
          v-model="formData.tags"
          type="text"
          placeholder="web, frontend, client-name"
        />
      </div>

      <div class="form-group">
        <label>Status</label>
        <select v-model="formData.status">
          <option value="active">🟢 Active</option>
          <option value="on-hold">🟡 On Hold</option>
          <option value="completed">✅ Completed</option>
          <option value="archived">📦 Archived</option>
        </select>
      </div>

      <div class="form-actions">
        <button @click="handleSaveProject" class="btn-primary" :disabled="!formData.name">
          {{ editingProject ? 'Update' : 'Create' }} Project
        </button>
        <button @click="handleCancelForm" class="btn-secondary">
          Cancel
        </button>
      </div>

      <div v-if="error" class="error-message">
        ❌ {{ error }}
      </div>
    </div>

    <!-- Project List -->
    <div v-if="!showCreateForm && !editingProject" class="project-list">
      <!-- Filter Bar -->
      <div class="filter-bar">
        <select v-model="statusFilter" class="filter-select">
          <option value="">All Statuses</option>
          <option value="active">🟢 Active</option>
          <option value="on-hold">🟡 On Hold</option>
          <option value="completed">✅ Completed</option>
          <option value="archived">📦 Archived</option>
        </select>

        <input
          v-model="searchQuery"
          type="text"
          placeholder="🔍 Search projects..."
          class="search-input"
        />
      </div>

      <!-- Projects Grid -->
      <div class="projects-grid">
        <div
          v-for="project in filteredProjects"
          :key="project.name"
          class="project-card"
          :class="`status-${project.status}`"
        >
          <div class="project-header">
            <h3>{{ project.name }}</h3>
            <span class="status-badge" :class="`status-${project.status}`">
              {{ getStatusIcon(project.status) }} {{ project.status }}
            </span>
          </div>

          <p class="project-description">
            {{ project.description || 'No description' }}
          </p>

          <div class="project-meta">
            <div v-if="project.billable" class="meta-item">
              💰 ${{ project.hourlyRate || 0 }}/hr
            </div>
            <div class="meta-item">
              ⏱️ {{ formatDurationShort(project.totalTime || 0) }}
            </div>
            <div v-if="project.tags && project.tags.length > 0" class="meta-item tags">
              <span v-for="tag in project.tags" :key="tag" class="tag">
                {{ tag }}
              </span>
            </div>
          </div>

          <div class="project-actions">
            <button @click="startEditProject(project)" class="btn-icon" title="Edit">
              ✏️
            </button>
            <button @click="handleDeleteProject(project.name)" class="btn-icon btn-danger" title="Delete">
              🗑️
            </button>
          </div>
        </div>

        <div v-if="filteredProjects.length === 0" class="empty-state">
          <p>No projects found</p>
          <button @click="showCreateForm = true" class="btn-primary">
            Create Your First Project
          </button>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading">Loading projects...</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useProjectManager } from '../composables/useProjectManager';
import { formatDurationShort } from '../utils/duration';

const props = defineProps<{
  panel: {
    i: string;
    [key: string]: any;
  };
}>();

const {
  projects,
  loading,
  error,
  loadProjects,
  createProject,
  updateProject,
  deleteProject
} = useProjectManager();

const showCreateForm = ref(false);
const editingProject = ref<any>(null);
const originalProjectName = ref<string | null>(null);
const statusFilter = ref('');
const searchQuery = ref('');

const formData = ref({
  name: '',
  description: '',
  billable: false,
  hourlyRate: 0,
  tags: '',
  status: 'active'
});

// Filtered projects
const filteredProjects = computed(() => {
  let filtered = projects.value;

  // Status filter
  if (statusFilter.value) {
    filtered = filtered.filter(p => p.status === statusFilter.value);
  }

  // Search filter
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(p =>
      p.name.toLowerCase().includes(query) ||
      (p.description && p.description.toLowerCase().includes(query)) ||
      (p.tags && p.tags.some((t: string) => t.toLowerCase().includes(query)))
    );
  }

  return filtered;
});

function getStatusIcon(status: string): string {
  const icons: Record<string, string> = {
    active: '🟢',
    'on-hold': '🟡',
    completed: '✅',
    archived: '📦'
  };
  return icons[status] || '⚪';
}

function startEditProject(project: any) {
  editingProject.value = project;
  originalProjectName.value = project.name;
  formData.value = {
    name: project.name,
    description: project.description || '',
    billable: project.billable || false,
    hourlyRate: project.hourlyRate || 0,
    tags: project.tags ? project.tags.join(', ') : '',
    status: project.status || 'active'
  };
}

async function handleSaveProject() {
  try {
    const projectData = {
      name: formData.value.name.trim(),
      description: formData.value.description.trim() || undefined,
      billable: formData.value.billable,
      hourlyRate: formData.value.billable ? formData.value.hourlyRate : undefined,
      tags: formData.value.tags
        .split(',')
        .map(t => t.trim())
        .filter(t => t.length > 0),
      status: formData.value.status
    };

    if (editingProject.value && originalProjectName.value) {
      await updateProject(originalProjectName.value, projectData);
    } else {
      await createProject(projectData);
    }

    handleCancelForm();
    await loadProjects();
  } catch (err) {
    console.error('Failed to save project:', err);
  }
}

function handleCancelForm() {
  showCreateForm.value = false;
  editingProject.value = null;
  originalProjectName.value = null;
  formData.value = {
    name: '',
    description: '',
    billable: false,
    hourlyRate: 0,
    tags: '',
    status: 'active'
  };
}

async function handleDeleteProject(projectName: string) {
  if (!confirm(`Delete project "${projectName}"? This will not delete time entries.`)) {
    return;
  }

  try {
    await deleteProject(projectName);
    await loadProjects();
  } catch (err) {
    console.error('Failed to delete project:', err);
  }
}

onMounted(() => {
  loadProjects();
});
</script>

<style scoped>
.project-manager-panel {
  padding: var(--panel-padding);
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  color: var(--text-primary);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2xl);
}

.panel-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-heading);
}

/* Form Styles */
.project-form {
  background: var(--bg-panel-header);
  padding: var(--panel-padding);
  border-radius: var(--panel-radius);
  margin-bottom: var(--space-2xl);
}

.project-form h3 {
  margin: 0 0 var(--space-xl) 0;
  color: var(--text-heading);
}

.form-group {
  margin-bottom: var(--space-lg);
}

.form-group label {
  display: block;
  margin-bottom: var(--space-sm);
  font-weight: 500;
  color: var(--text-primary);
}

.form-group input[type="text"],
.form-group input[type="number"],
.form-group textarea,
.form-group select {
  width: 100%;
  padding: var(--space-sm);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 1rem;
  background: var(--bg-panel);
  color: var(--text-primary);
}

.form-group input[type="checkbox"] {
  width: auto;
  margin-right: var(--space-sm);
}

.form-row {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: var(--space-lg);
  align-items: start;
}

.form-actions {
  display: flex;
  gap: var(--space-lg);
  margin-top: var(--space-xl);
}

/* Filter Bar */
.filter-bar {
  display: flex;
  gap: var(--space-lg);
  margin-bottom: var(--space-xl);
}

.filter-select,
.search-input {
  padding: var(--space-sm);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 1rem;
  background: var(--bg-panel-header);
  color: var(--text-primary);
}

.search-input {
  flex: 1;
}

/* Projects Grid */
.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--panel-gap);
}

.project-card {
  background: var(--bg-panel-header);
  padding: var(--panel-padding);
  border-radius: var(--panel-radius);
  border-left: 4px solid var(--border-color);
  transition: transform 0.2s, box-shadow 0.2s;
}

.project-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--panel-shadow);
}

.project-card.status-active {
  border-left-color: var(--accent-success);
}

.project-card.status-on-hold {
  border-left-color: var(--accent-warning);
}

.project-card.status-completed {
  border-left-color: var(--accent-primary);
}

.project-card.status-archived {
  border-left-color: var(--text-muted);
}

.project-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: var(--space-lg);
}

.project-header h3 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-heading);
}

.status-badge {
  padding: var(--space-xs) var(--space-md);
  border-radius: 12px;
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: capitalize;
}

.status-badge.status-active {
  background: var(--accent-success);
  color: var(--text-on-accent);
  opacity: 0.8;
}

.status-badge.status-on-hold {
  background: var(--accent-warning);
  color: var(--text-primary);
  opacity: 0.8;
}

.status-badge.status-completed {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  opacity: 0.8;
}

.status-badge.status-archived {
  background: var(--text-muted);
  color: var(--text-on-accent);
  opacity: 0.8;
}

.project-description {
  color: var(--text-secondary);
  margin-bottom: var(--space-lg);
  font-size: 0.95rem;
}

.project-meta {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-lg);
  margin-bottom: var(--space-lg);
  font-size: 0.9rem;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  color: var(--text-secondary);
}

.meta-item.tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
}

.tag {
  padding: var(--space-xs) var(--space-sm);
  background: var(--bg-app);
  border-radius: 4px;
  font-size: 0.875rem;
  color: var(--text-primary);
  border: 1px solid var(--border-subtle);
}

.project-actions {
  display: flex;
  gap: var(--space-sm);
  justify-content: flex-end;
  margin-top: var(--space-lg);
  padding-top: var(--space-lg);
  border-top: 1px solid var(--border-subtle);
}

/* Common Button Styles */
.btn-primary,
.btn-secondary,
.btn-icon {
  padding: var(--space-sm) var(--space-lg);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.2s;
}

.btn-primary {
  background: var(--bg-button);
  color: var(--text-on-accent);
}

.btn-primary:hover {
  background: var(--bg-button-hover);
}

.btn-primary:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
  opacity: 0.6;
}

.btn-secondary {
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
}

.btn-secondary:hover {
  background: var(--bg-button-secondary-hover);
}

.btn-icon {
  padding: var(--space-sm);
  background: transparent;
  border: 1px solid var(--border-color);
}

.btn-icon:hover {
  background: var(--bg-panel-header);
}

.btn-icon.btn-danger:hover {
  background: var(--accent-danger);
  color: var(--text-on-accent);
  border-color: var(--accent-danger);
}

.empty-state {
  grid-column: 1 / -1;
  text-align: center;
  padding: var(--space-2xl) var(--space-lg);
  color: var(--text-secondary);
}

.error-message {
  margin-top: var(--space-lg);
  padding: var(--space-md);
  background: var(--accent-danger);
  color: var(--text-on-accent);
  border-radius: 4px;
  opacity: 0.9;
}

.loading {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--text-secondary);
}
</style>
