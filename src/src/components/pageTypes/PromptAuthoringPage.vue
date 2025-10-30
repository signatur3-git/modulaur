<template>
  <div class="prompt-authoring-page">
    <div class="page-header">
      <h1>🎨 Prompt Authoring</h1>
      <p>Create and manage prompt templates, sections, and packages</p>
    </div>

    <div class="authoring-layout">
      <!-- Package Sidebar -->
      <aside class="package-sidebar">
        <div class="sidebar-header">
          <h3>📦 Packages</h3>
          <button @click="showCreatePackage = true" class="btn-icon" title="Create Package">
            ➕
          </button>
        </div>

        <div class="package-list">
          <div
            v-for="pkg in packages"
            :key="getIdString(pkg.id)"
            class="package-item"
            :class="{ selected: selectedPackageId === getIdString(pkg.id) }"
            @click="selectPackage(getIdString(pkg.id))"
          >
            <div class="package-info">
              <span class="package-name">{{ pkg.name }}</span>
              <span class="package-namespace">{{ pkg.namespace }}</span>
            </div>
            <div class="package-actions-inline">
              <span class="package-version">v{{ pkg.version }}</span>
              <button
                @click.stop="deletePackage(getIdString(pkg.id), pkg.name)"
                class="btn-icon-small btn-danger"
                title="Delete Package"
              >
                🗑️
              </button>
            </div>
          </div>

          <div v-if="packages.length === 0" class="empty-state">
            <p>No packages yet</p>
            <button @click="showCreatePackage = true" class="btn-small">Create Package</button>
            <button
              @click="seedExamples"
              class="btn-small btn-secondary"
              style="margin-left: 0.5rem"
            >
              🌱 Seed Examples
            </button>
          </div>
        </div>

        <!-- Seed Examples Button (always visible at bottom) -->
        <div class="sidebar-footer">
          <button @click="seedExamples" class="btn-seed" title="Load example prompts">
            🌱 Load Examples
          </button>
          <button
            @click="seedText2ImageCommon"
            class="btn-seed btn-t2i"
            title="Load Text2Image common library"
          >
            🎨 Text2Image Library
          </button>
        </div>
      </aside>

      <!-- Main Content -->
      <main class="main-content">
        <div v-if="!selectedPackage" class="no-selection">
          <h2>👈 Select a Package</h2>
          <p>Choose a package from the sidebar to start editing</p>
        </div>

        <div v-else class="package-editor">
          <!-- Package Header -->
          <div class="package-header">
            <div class="package-title">
              <h2>{{ selectedPackage.name }}</h2>
              <span class="badge">{{ selectedPackage.namespace }}</span>
            </div>
            <div class="package-actions">
              <button @click="exportCurrentPackage" class="btn-secondary" title="Export Package">
                📤 Export
              </button>
              <button @click="showEditPackage = true" class="btn-secondary" title="Edit Package">
                ✏️ Edit
              </button>
            </div>
          </div>

          <!-- Tabs -->
          <div class="tabs">
            <button
              v-for="tab in tabs"
              :key="tab.id"
              class="tab"
              :class="{ active: activeTab === tab.id }"
              @click="activeTab = tab.id"
            >
              {{ tab.icon }} {{ tab.label }}
              <span class="count">{{ tab.count }}</span>
            </button>
          </div>

          <!-- Tab Content -->
          <div class="tab-content">
            <!-- Entry Points Tab (formerly Templates) -->
            <div v-if="activeTab === 'entry-points'" class="entry-points-tab">
              <div class="tab-header">
                <h3>Entry Points</h3>
                <button @click="showCreateEntryPoint = true" class="btn-primary">
                  ➕ New Entry Point
                </button>
              </div>
              <p class="tab-description">
                Entry points are renderable sections that appear in the Prompt Renderer.
              </p>

              <div class="item-grid">
                <div
                  v-for="section in packageEntryPoints"
                  :key="getIdString(section.id)"
                  class="item-card entry-point-card"
                  @click="editSection(section)"
                >
                  <h4>{{ section.name }}</h4>
                  <p>{{ section.description }}</p>
                  <div class="item-meta">
                    <span class="tag" v-for="tag in (section.tags || []).slice(0, 3)" :key="tag">{{
                      tag
                    }}</span>
                    <span v-if="(section.variables || []).length > 0" class="variable-count">
                      {{ section.variables?.length }} variables
                    </span>
                    <span v-if="section.exportable" class="badge exportable">Exportable</span>
                  </div>
                </div>

                <div v-if="packageEntryPoints.length === 0" class="empty-grid">
                  <p>No entry points in this package</p>
                  <p class="hint">Entry points are sections marked as renderable</p>
                </div>
              </div>
            </div>

            <!-- Sections Tab (Fragments) -->
            <div v-if="activeTab === 'sections'" class="sections-tab">
              <div class="tab-header">
                <h3>Sections (Fragments)</h3>
                <button @click="showCreateSection = true" class="btn-primary">
                  ➕ New Section
                </button>
              </div>
              <p class="tab-description">
                Reusable building blocks that can be composed into entry points.
              </p>

              <div class="item-grid">
                <div
                  v-for="section in packageFragments"
                  :key="getIdString(section.id)"
                  class="item-card"
                  @click="editSection(section)"
                >
                  <h4>{{ section.name }}</h4>
                  <p>{{ section.description }}</p>
                  <div class="item-meta">
                    <span v-if="section.exportable" class="badge exportable">Exportable</span>
                    <span v-if="section.required_variables.length > 0" class="variable-count">
                      {{ section.required_variables.length }} required vars
                    </span>
                  </div>
                </div>

                <div v-if="packageFragments.length === 0" class="empty-grid">
                  <p>No fragment sections in this package</p>
                </div>
              </div>
            </div>

            <!-- Separator Sets Tab -->
            <div v-if="activeTab === 'separators'" class="separators-tab">
              <div class="tab-header">
                <h3>Separator Sets</h3>
                <button @click="showCreateSeparator = true" class="btn-primary">
                  ➕ New Separator Set
                </button>
              </div>

              <div class="separator-preview">
                <h4>Built-in Separators</h4>
                <div class="builtin-separators">
                  <div
                    v-for="(name, key) in builtinSeparatorNames"
                    :key="key"
                    class="separator-demo"
                  >
                    <strong>{{ name }}</strong>
                    <code>{{ previewSeparator(key, ['apple', 'banana', 'cherry']) }}</code>
                  </div>
                </div>
              </div>

              <div class="item-grid">
                <div
                  v-for="sep in packageSeparatorSets"
                  :key="getIdString(sep.id)"
                  class="item-card"
                >
                  <h4>{{ sep.name }}</h4>
                  <p>{{ sep.description }}</p>
                  <code class="separator-example">
                    {{ previewSeparatorSet(sep) }}
                  </code>
                </div>
              </div>
            </div>

            <!-- Data Types Tab -->
            <div v-if="activeTab === 'datatypes'" class="datatypes-tab">
              <div class="tab-header">
                <h3>Data Types</h3>
                <button @click="showCreateDataType = true" class="btn-primary">
                  ➕ New Data Type
                </button>
              </div>

              <div class="item-grid">
                <div v-for="dt in packageDataTypes" :key="getIdString(dt.id)" class="item-card">
                  <h4>{{ dt.name }}</h4>
                  <p>{{ dt.description }}</p>
                  <div class="item-meta">
                    <span class="badge">{{ dt.base_type }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Tags Tab -->
            <div v-if="activeTab === 'tags'" class="tags-tab">
              <div class="tab-header">
                <h3>Tags</h3>
                <button @click="showCreateTag = true" class="btn-primary">➕ New Tag</button>
              </div>

              <div class="tag-list">
                <div
                  v-for="tag in packageTags"
                  :key="getIdString(tag.id)"
                  class="tag-item"
                  :style="{ borderColor: tag.color || '#6c757d' }"
                >
                  <span class="tag-color" :style="{ background: tag.color || '#6c757d' }"></span>
                  <span class="tag-name">{{ tag.name }}</span>
                  <span v-if="tag.parent" class="tag-parent">↳ {{ tag.parent }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>

    <!-- Create Package Modal -->
    <div v-if="showCreatePackage" class="modal-overlay" @click="showCreatePackage = false">
      <div class="modal" @click.stop>
        <h3>Create New Package</h3>
        <form @submit.prevent="createPackage">
          <div class="form-group">
            <label>Namespace *</label>
            <input
              v-model="newPackage.namespace"
              required
              placeholder="my-prompts"
              pattern="[a-z0-9-]+"
            />
            <span class="hint">Lowercase letters, numbers, and hyphens only</span>
          </div>
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newPackage.name" required placeholder="My Prompt Package" />
          </div>
          <div class="form-group">
            <label>Version</label>
            <input v-model="newPackage.version" placeholder="1.0.0" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="newPackage.description"
              placeholder="Describe your package..."
              rows="3"
            />
          </div>
          <div class="form-group">
            <label>Author</label>
            <input v-model="newPackage.author" placeholder="Your name" />
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreatePackage = false" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">Create Package</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Create Entry Point Modal -->
    <div v-if="showCreateEntryPoint" class="modal-overlay" @click="showCreateEntryPoint = false">
      <div class="modal modal-large" @click.stop>
        <h3>Create New Entry Point</h3>
        <p class="modal-description">
          Entry points are renderable sections that appear in the Prompt Renderer.
        </p>
        <form @submit.prevent="createEntryPoint">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newEntryPoint.name" required placeholder="My Prompt Template" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="newEntryPoint.description"
              placeholder="What does this prompt do?"
              rows="2"
            />
          </div>
          <div class="form-group">
            <label>Content (Simple Text)</label>
            <textarea
              v-model="newEntryPoint.content.value"
              placeholder="Enter your prompt template text. Use {variable_name} for variables."
              rows="6"
              class="monospace"
            />
            <span class="hint"
              >Use {'{'}variable_name{'}'} syntax for variables. Complex content can be edited
              later.</span
            >
          </div>
          <div class="form-group">
            <label>Tags (comma-separated)</label>
            <input
              :value="newEntryPoint.tags.join(', ')"
              @input="
                newEntryPoint.tags = ($event.target as HTMLInputElement).value
                  .split(',')
                  .map(t => t.trim())
                  .filter(Boolean)
              "
              placeholder="creative, writing, characters"
            />
          </div>
          <div class="form-group checkbox-group">
            <label>
              <input type="checkbox" v-model="newEntryPoint.exportable" />
              Exportable (can be referenced by other packages)
            </label>
          </div>
          <div class="form-group">
            <label>Required Variables (comma-separated)</label>
            <input
              :value="newEntryPoint.required_variables.join(', ')"
              @input="
                newEntryPoint.required_variables = ($event.target as HTMLInputElement).value
                  .split(',')
                  .map(t => t.trim())
                  .filter(Boolean)
              "
              placeholder="name, topic, style"
            />
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreateEntryPoint = false" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">Create Entry Point</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Create Section (Fragment) Modal -->
    <div v-if="showCreateSection" class="modal-overlay" @click="showCreateSection = false">
      <div class="modal modal-large" @click.stop>
        <h3>Create New Section (Fragment)</h3>
        <p class="modal-description">
          Fragments are reusable building blocks that can be composed into entry points.
        </p>
        <form @submit.prevent="createSection">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newSection.name" required placeholder="My Section" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="newSection.description"
              placeholder="What is this section for?"
              rows="2"
            />
          </div>
          <div class="form-group">
            <label>Content (Simple Text)</label>
            <textarea
              v-model="newSection.content.value"
              placeholder="Enter the section content. Use {variable_name} for variables."
              rows="6"
              class="monospace"
            />
          </div>
          <div class="form-group checkbox-group">
            <label>
              <input type="checkbox" v-model="newSection.exportable" />
              Exportable (can be used by other packages)
            </label>
          </div>
          <div class="form-group">
            <label>Required Variables (comma-separated)</label>
            <input
              :value="newSection.required_variables.join(', ')"
              @input="
                newSection.required_variables = ($event.target as HTMLInputElement).value
                  .split(',')
                  .map(t => t.trim())
                  .filter(Boolean)
              "
              placeholder="name, traits, occupation"
            />
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreateSection = false" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">Create Section</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Create Separator Set Modal -->
    <div v-if="showCreateSeparator" class="modal-overlay" @click="showCreateSeparator = false">
      <div class="modal modal-large" @click.stop>
        <h3>Create New Separator Set</h3>
        <form @submit.prevent="createSeparatorSet">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newSeparator.name" required placeholder="My Separator Set" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="newSeparator.description"
              placeholder="How should lists be joined?"
              rows="2"
            />
          </div>

          <div class="separator-config">
            <h4>Single Item</h4>
            <div class="form-group">
              <label>Template</label>
              <input v-model="newSeparator.rules.single.template" placeholder="{item}" />
              <span class="hint">Use {'{'}item{'}'} for the item</span>
            </div>

            <h4>Two Items</h4>
            <div class="form-row">
              <div class="form-group">
                <label>Separator</label>
                <input v-model="newSeparator.rules.two.separator" placeholder=" and " />
              </div>
              <div class="form-group">
                <label>Template</label>
                <input
                  v-model="newSeparator.rules.two.template"
                  placeholder="{first}{separator}{second}"
                />
              </div>
            </div>

            <h4>Three+ Items</h4>
            <div class="form-row">
              <div class="form-group">
                <label>Item Separator</label>
                <input v-model="newSeparator.rules.many.item_separator" placeholder=", " />
              </div>
              <div class="form-group">
                <label>Last Separator</label>
                <input v-model="newSeparator.rules.many.last_separator" placeholder=", and " />
              </div>
            </div>
            <div class="form-group">
              <label>Template</label>
              <input
                v-model="newSeparator.rules.many.template"
                placeholder="{items}{last_separator}{last}"
              />
            </div>
          </div>

          <div class="separator-preview-section">
            <h4>Preview</h4>
            <div class="preview-examples">
              <div class="preview-item">
                <span class="preview-label">1 item:</span>
                <code>{{ previewCustomSeparator(newSeparator.rules, ['apple']) }}</code>
              </div>
              <div class="preview-item">
                <span class="preview-label">2 items:</span>
                <code>{{ previewCustomSeparator(newSeparator.rules, ['apple', 'banana']) }}</code>
              </div>
              <div class="preview-item">
                <span class="preview-label">3 items:</span>
                <code>{{
                  previewCustomSeparator(newSeparator.rules, ['apple', 'banana', 'cherry'])
                }}</code>
              </div>
            </div>
          </div>

          <div class="modal-actions">
            <button type="button" @click="showCreateSeparator = false" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">Create Separator Set</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Create Data Type Modal -->
    <div v-if="showCreateDataType" class="modal-overlay" @click="showCreateDataType = false">
      <div class="modal" @click.stop>
        <h3>Create New Data Type</h3>
        <form @submit.prevent="createDataType">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newDataType.name" required placeholder="PersonName" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="newDataType.description"
              placeholder="What kind of data is this?"
              rows="2"
            />
          </div>
          <div class="form-group">
            <label>Base Type *</label>
            <select v-model="newDataType.base_type" required>
              <option value="string">String</option>
              <option value="number">Number</option>
              <option value="boolean">Boolean</option>
              <option value="array">Array</option>
              <option value="object">Object</option>
              <option value="enum">Enum</option>
            </select>
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreateDataType = false" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">Create Data Type</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Create Tag Modal -->
    <div v-if="showCreateTag" class="modal-overlay" @click="showCreateTag = false">
      <div class="modal" @click.stop>
        <h3>Create New Tag</h3>
        <form @submit.prevent="createTag">
          <div class="form-group">
            <label>Name *</label>
            <input v-model="newTag.name" required placeholder="creative" />
          </div>
          <div class="form-group">
            <label>Description</label>
            <textarea
              v-model="newTag.description"
              placeholder="What does this tag represent?"
              rows="2"
            />
          </div>
          <div class="form-group">
            <label>Color</label>
            <div class="color-input">
              <input type="color" v-model="newTag.color" />
              <input type="text" v-model="newTag.color" placeholder="#6c757d" />
            </div>
          </div>
          <div class="form-group">
            <label>Parent Tag (optional)</label>
            <select v-model="newTag.parent">
              <option value="">None</option>
              <option v-for="tag in packageTags" :key="getIdString(tag.id)" :value="tag.name">
                {{ tag.name }}
              </option>
            </select>
            <span class="hint">Create hierarchical tags</span>
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreateTag = false" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">Create Tag</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Loading/Error -->
    <div v-if="loading" class="loading-overlay">
      <p>Loading...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { usePromptStore } from '@/stores/promptStore'
import { previewSeparator as previewSep } from '@/services/promptRenderEngine'
import type { PromptSection, SeparatorSet } from '@/types/promptTypes'

const store = usePromptStore()

// State
const activeTab = ref('entry-points')
const showCreatePackage = ref(false)
const showEditPackage = ref(false)
const showCreateEntryPoint = ref(false)
// const showCreateTemplate = ref(false) // @deprecated - commented out
const showCreateSection = ref(false)
const showCreateSeparator = ref(false)
const showCreateDataType = ref(false)
const showCreateTag = ref(false)

const newPackage = ref({
  namespace: '',
  name: '',
  version: '1.0.0',
  description: '',
  author: '',
  additional_namespaces: [] as string[],
  dependencies: [] as string[],
  exports: [] as string[],
})

// Entry point form (section with is_entry_point=true)
const newEntryPoint = ref({
  name: '',
  description: '',
  content: { type: 'text', value: '' } as any,
  is_entry_point: true,
  exportable: false,
  required_variables: [] as string[],
  variables: [] as any[],
  tags: [] as string[],
  examples: [] as any[],
})

// @deprecated - use newEntryPoint
// Commented out to avoid unused variable warnings
/*
const newTemplate = ref({
  name: '',
  description: '',
  content: { type: 'text', value: '' } as any,
  variables: [] as any[],
  tags: [] as string[],
  examples: [] as any[],
})
*/

// Fragment section form (is_entry_point=false)
const newSection = ref({
  name: '',
  description: '',
  content: { type: 'text', value: '' } as any,
  is_entry_point: false,
  exportable: false,
  required_variables: [] as string[],
})

const newSeparator = ref({
  name: '',
  description: '',
  rules: {
    single: { template: '{item}' },
    two: { separator: ' and ', template: '{first}{separator}{second}' },
    many: {
      item_separator: ', ',
      last_separator: ', and ',
      template: '{items}{last_separator}{last}',
    },
  },
})

const newDataType = ref({
  name: '',
  description: '',
  base_type: 'string' as 'string' | 'number' | 'boolean' | 'array' | 'object' | 'enum',
  validation: {} as any,
  format: {} as any,
  examples: [] as any[],
})

const newTag = ref({
  name: '',
  description: '',
  color: '#6c757d',
  parent: '',
})

// Computed
const packages = computed(() => store.packages)
const loading = computed(() => store.loading)
const selectedPackageId = computed(() => store.selectedPackageId)
const selectedPackage = computed(() => store.selectedPackage)

// Entry points (sections with is_entry_point=true)
const packageEntryPoints = computed(() =>
  store.sections.filter(s => s.package_id === selectedPackageId.value && s.is_entry_point)
)

// Fragments (sections with is_entry_point=false)
const packageFragments = computed(() =>
  store.sections.filter(s => s.package_id === selectedPackageId.value && !s.is_entry_point)
)

const packageSeparatorSets = computed(() =>
  store.separatorSets.filter(s => s.package_id === selectedPackageId.value)
)
const packageDataTypes = computed(() =>
  store.dataTypes.filter(d => d.package_id === selectedPackageId.value)
)
const packageTags = computed(() => store.tags.filter(t => t.package_id === selectedPackageId.value))

const tabs = computed(() => [
  { id: 'entry-points', label: 'Entry Points', icon: '🎯', count: packageEntryPoints.value.length },
  { id: 'sections', label: 'Sections', icon: '📋', count: packageFragments.value.length },
  { id: 'separators', label: 'Separators', icon: '🔗', count: packageSeparatorSets.value.length },
  { id: 'datatypes', label: 'Data Types', icon: '📊', count: packageDataTypes.value.length },
  { id: 'tags', label: 'Tags', icon: '🏷️', count: packageTags.value.length },
])

const builtinSeparatorNames: Record<string, string> = {
  'oxford-comma': 'Oxford Comma',
  'simple-comma': 'Simple Comma',
  'or-list': 'Or List',
  'and-list-no-oxford': 'And (No Oxford)',
  'bullet-list': 'Bullet List',
  'numbered-list': 'Numbered List',
  newline: 'Newline',
}

// Methods
function getIdString(id: any): string {
  return store.getIdString(id)
}

function selectPackage(id: string) {
  store.selectPackage(id)
}

async function deletePackage(id: string, name: string) {
  if (
    !confirm(
      `Delete package "${name}" and all its contents?\n\nThis will delete all entry points, sections, separator sets, data types, and tags in this package.`
    )
  ) {
    return
  }
  try {
    await store.deletePackage(id)
    alert(`Package "${name}" deleted successfully.`)
  } catch (e) {
    alert('Failed to delete package: ' + e)
  }
}

async function createPackage() {
  try {
    await store.createPackage(newPackage.value)
    showCreatePackage.value = false
    newPackage.value = {
      namespace: '',
      name: '',
      version: '1.0.0',
      description: '',
      author: '',
      additional_namespaces: [],
      dependencies: [],
      exports: [],
    }
  } catch (e) {
    alert('Failed to create package: ' + e)
  }
}

// Create entry point (section with is_entry_point=true)
async function createEntryPoint() {
  if (!selectedPackage.value) return
  try {
    await store.createSection({
      ...newEntryPoint.value,
      is_entry_point: true,
      package_id: selectedPackageId.value!,
      namespace: selectedPackage.value.namespace,
    })
    showCreateEntryPoint.value = false
    newEntryPoint.value = {
      name: '',
      description: '',
      content: { type: 'text', value: '' },
      is_entry_point: true,
      exportable: false,
      required_variables: [],
      variables: [],
      tags: [],
      examples: [],
    }
  } catch (e) {
    alert('Failed to create entry point: ' + e)
  }
}

// @deprecated - use createEntryPoint
// Commented out to avoid unused function warnings
/*
async function createTemplate() {
  if (!selectedPackage.value) return
  try {
    await store.createTemplate({
      ...newTemplate.value,
      package_id: selectedPackageId.value!,
      namespace: selectedPackage.value.namespace,
    })
    showCreateTemplate.value = false
    newTemplate.value = {
      name: '',
      description: '',
      content: { type: 'text', value: '' },
      variables: [],
      tags: [],
      examples: [],
    }
  } catch (e) {
    alert('Failed to create template: ' + e)
  }
}
*/

// Create fragment section (is_entry_point=false)
async function createSection() {
  if (!selectedPackage.value) return
  try {
    await store.createSection({
      ...newSection.value,
      is_entry_point: false,
      package_id: selectedPackageId.value!,
      namespace: selectedPackage.value.namespace,
    })
    showCreateSection.value = false
    newSection.value = {
      name: '',
      description: '',
      content: { type: 'text', value: '' },
      is_entry_point: false,
      exportable: false,
      required_variables: [],
    }
  } catch (e) {
    alert('Failed to create section: ' + e)
  }
}

async function createSeparatorSet() {
  if (!selectedPackage.value) return
  try {
    await store.createSeparatorSet({
      ...newSeparator.value,
      package_id: selectedPackageId.value!,
      namespace: selectedPackage.value.namespace,
    })
    showCreateSeparator.value = false
    newSeparator.value = {
      name: '',
      description: '',
      rules: {
        single: { template: '{item}' },
        two: { separator: ' and ', template: '{first}{separator}{second}' },
        many: {
          item_separator: ', ',
          last_separator: ', and ',
          template: '{items}{last_separator}{last}',
        },
      },
    }
  } catch (e) {
    alert('Failed to create separator set: ' + e)
  }
}

async function createDataType() {
  if (!selectedPackage.value) return
  try {
    await store.createDataType({
      name: newDataType.value.name,
      description: newDataType.value.description,
      base_type: newDataType.value.base_type as
        | 'string'
        | 'number'
        | 'boolean'
        | 'array'
        | 'object'
        | 'enum',
      validation: newDataType.value.validation,
      format: newDataType.value.format,
      examples: newDataType.value.examples,
      package_id: selectedPackageId.value!,
      namespace: selectedPackage.value.namespace,
    })
    showCreateDataType.value = false
    newDataType.value = {
      name: '',
      description: '',
      base_type: 'string' as 'string' | 'number' | 'boolean' | 'array' | 'object' | 'enum',
      validation: {},
      format: {},
      examples: [],
    }
  } catch (e) {
    alert('Failed to create data type: ' + e)
  }
}

async function createTag() {
  if (!selectedPackage.value) return
  try {
    await store.createTag({
      ...newTag.value,
      package_id: selectedPackageId.value!,
      namespace: selectedPackage.value.namespace,
    })
    showCreateTag.value = false
    newTag.value = {
      name: '',
      description: '',
      color: '#6c757d',
      parent: '',
    }
  } catch (e) {
    alert('Failed to create tag: ' + e)
  }
}

async function exportCurrentPackage() {
  if (!selectedPackageId.value) return

  try {
    const exportData = await store.exportPackage(selectedPackageId.value)
    const json = JSON.stringify(exportData, null, 2)
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${selectedPackage.value?.namespace || 'package'}-export.json`
    a.click()
    URL.revokeObjectURL(url)
  } catch (e) {
    alert('Failed to export: ' + e)
  }
}

async function seedExamples() {
  try {
    const result = await store.seedExamplePackages()
    alert(result)
  } catch (e) {
    alert('Failed to seed examples: ' + e)
  }
}

async function seedText2ImageCommon() {
  try {
    const result = await store.seedText2ImageCommonPackage()
    alert(result)
  } catch (e) {
    alert('Failed to seed text2image common package: ' + e)
  }
}

// Commented out to avoid unused function warnings
/*
function editTemplate(template: PromptTemplate) {
  // TODO: Open template editor
  console.log('Edit template:', template)
}
*/

function editSection(section: PromptSection) {
  // TODO: Open section editor
  console.log('Edit section:', section)
}

function previewSeparator(key: string, items: string[]): string {
  const rules = {
    'oxford-comma': {
      single: { template: '{item}' },
      two: { separator: ' and ', template: '{first}{separator}{second}' },
      many: {
        item_separator: ', ',
        last_separator: ', and ',
        template: '{items}{last_separator}{last}',
      },
    },
    'simple-comma': {
      single: { template: '{item}' },
      two: { separator: ', ', template: '{first}{separator}{second}' },
      many: {
        item_separator: ', ',
        last_separator: ', ',
        template: '{items}{last_separator}{last}',
      },
    },
    'or-list': {
      single: { template: '{item}' },
      two: { separator: ' or ', template: '{first}{separator}{second}' },
      many: {
        item_separator: ', ',
        last_separator: ', or ',
        template: '{items}{last_separator}{last}',
      },
    },
    'and-list-no-oxford': {
      single: { template: '{item}' },
      two: { separator: ' and ', template: '{first}{separator}{second}' },
      many: {
        item_separator: ', ',
        last_separator: ' and ',
        template: '{items}{last_separator}{last}',
      },
    },
    'bullet-list': {
      single: { template: '• {item}' },
      two: { separator: '\n', template: '• {first}\n• {second}' },
      many: {
        item_separator: '\n• ',
        last_separator: '\n• ',
        template: '• {items}{last_separator}{last}',
      },
    },
    'numbered-list': {
      single: { template: '1. {item}' },
      two: { separator: '\n', template: '1. {first}\n2. {second}' },
      many: { item_separator: '\n', last_separator: '\n', template: '{numbered}', numbered: true },
    },
    newline: {
      single: { template: '{item}' },
      two: { separator: '\n', template: '{first}{separator}{second}' },
      many: {
        item_separator: '\n',
        last_separator: '\n',
        template: '{items}{last_separator}{last}',
      },
    },
  } as any

  return previewSep(rules[key], items)
}

function previewSeparatorSet(sep: SeparatorSet): string {
  try {
    return previewSep(sep.rules as any, ['item1', 'item2', 'item3'])
  } catch {
    return 'Error rendering preview'
  }
}

function previewCustomSeparator(rules: any, items: string[]): string {
  try {
    if (items.length === 0) return ''
    if (items.length === 1) {
      return (rules.single?.template || '{item}').replace('{item}', items[0])
    }
    if (items.length === 2) {
      const sep = rules.two?.separator || ' and '
      const template = rules.two?.template || '{first}{separator}{second}'
      return template
        .replace('{first}', items[0])
        .replace('{separator}', sep)
        .replace('{second}', items[1])
    }
    // 3+ items
    const itemSep = rules.many?.item_separator || ', '
    const lastSep = rules.many?.last_separator || ', and '
    const allButLast = items.slice(0, -1)
    const last = items[items.length - 1]
    return allButLast.join(itemSep) + lastSep + last
  } catch {
    return 'Preview error'
  }
}

// Lifecycle
onMounted(async () => {
  await store.loadAll()
})
</script>

<style scoped>
.prompt-authoring-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #f8f9fa;
}

.page-header {
  padding: 1.5rem 2rem;
  background: #ffffff;
  border-bottom: 1px solid #dee2e6;
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 1.5rem;
}

.page-header p {
  margin: 0;
  color: #6c757d;
}

.authoring-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.package-sidebar {
  width: 280px;
  background: #ffffff;
  border-right: 1px solid #dee2e6;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid #dee2e6;
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1rem;
}

.btn-icon {
  padding: 0.25rem 0.5rem;
  background: none;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  cursor: pointer;
}

.btn-icon:hover {
  background: #f8f9fa;
}

.package-list {
  flex: 1;
  overflow: auto;
  padding: 0.5rem;
}

.sidebar-footer {
  padding: 0.75rem;
  border-top: 1px solid #dee2e6;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.btn-seed {
  width: 100%;
  padding: 0.5rem;
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  color: #495057;
  transition: all 0.2s;
}

.btn-seed:hover {
  background: #e9ecef;
  border-color: #adb5bd;
}

.btn-seed.btn-t2i {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: 1px solid #667eea;
  font-weight: 500;
}

.btn-seed.btn-t2i:hover {
  background: linear-gradient(135deg, #5568d3 0%, #63398f 100%);
  border-color: #5568d3;
}

.package-item {
  padding: 0.75rem;
  border-radius: 4px;
  cursor: pointer;
  margin-bottom: 0.5rem;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.package-item:hover {
  background: #f8f9fa;
}

.package-item.selected {
  background: #e7f1ff;
  border: 1px solid #007bff;
}

.package-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  flex: 1;
  min-width: 0;
}

.package-name {
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.package-namespace {
  font-size: 0.85rem;
  color: #6c757d;
}

.package-actions-inline {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 0;
}

.package-version {
  font-size: 0.8rem;
  color: #6c757d;
}

.btn-icon-small {
  padding: 0.15rem 0.3rem;
  font-size: 0.75rem;
  background: transparent;
  border: none;
  cursor: pointer;
  opacity: 0.5;
  transition: opacity 0.2s;
}

.btn-icon-small:hover {
  opacity: 1;
}

.btn-danger:hover {
  color: #dc3545;
}

.main-content {
  flex: 1;
  overflow: auto;
  padding: 1.5rem;
}

.no-selection {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #6c757d;
}

.package-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1.5rem;
}

.package-title {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.package-title h2 {
  margin: 0;
}

.badge {
  padding: 0.25rem 0.5rem;
  background: #e9ecef;
  border-radius: 4px;
  font-size: 0.85rem;
  color: #495057;
}

.package-actions {
  display: flex;
  gap: 0.5rem;
}

.tabs {
  display: flex;
  gap: 0.5rem;
  border-bottom: 1px solid #dee2e6;
  margin-bottom: 1.5rem;
}

.tab {
  padding: 0.75rem 1rem;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.tab:hover {
  background: #f8f9fa;
}

.tab.active {
  border-bottom-color: #007bff;
  color: #007bff;
}

.count {
  background: #e9ecef;
  padding: 0.125rem 0.5rem;
  border-radius: 10px;
  font-size: 0.8rem;
}

.tab-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.tab-header h3 {
  margin: 0;
}

.item-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1rem;
}

.item-card {
  background: #ffffff;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.item-card:hover {
  border-color: #007bff;
  box-shadow: 0 2px 8px rgba(0, 123, 255, 0.1);
}

.item-card h4 {
  margin: 0 0 0.5rem 0;
}

.item-card p {
  margin: 0 0 0.75rem 0;
  color: #6c757d;
  font-size: 0.9rem;
}

.item-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag {
  padding: 0.125rem 0.5rem;
  background: #e9ecef;
  border-radius: 4px;
  font-size: 0.8rem;
}

.variable-count {
  font-size: 0.8rem;
  color: #6c757d;
}

.badge.exportable {
  background: #d4edda;
  color: #155724;
}

.empty-grid,
.empty-state {
  text-align: center;
  padding: 2rem;
  color: #6c757d;
}

.builtin-separators {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 0.75rem;
  margin-bottom: 2rem;
}

.separator-demo {
  background: #f8f9fa;
  padding: 0.75rem;
  border-radius: 4px;
}

.separator-demo strong {
  display: block;
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
}

.separator-demo code,
.separator-example {
  font-family: monospace;
  font-size: 0.85rem;
  color: #495057;
  word-break: break-all;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: #ffffff;
  border: 2px solid;
  border-radius: 20px;
}

.tag-color {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.tag-parent {
  font-size: 0.8rem;
  color: #6c757d;
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: #ffffff;
  padding: 2rem;
  border-radius: 8px;
  width: 100%;
  max-width: 500px;
  max-height: 90vh;
  overflow: auto;
}

.modal.modal-large {
  max-width: 700px;
}

.modal h3 {
  margin: 0 0 0.5rem 0;
}

.modal-description {
  margin: 0 0 1.5rem 0;
  color: #6c757d;
  font-size: 0.9rem;
}

.tab-description {
  margin: 0 0 1rem 0;
  color: #6c757d;
  font-size: 0.9rem;
}

.entry-point-card {
  border-left: 3px solid #007bff;
}

.modal h4 {
  margin: 1.5rem 0 0.75rem 0;
  font-size: 0.95rem;
  color: #495057;
  border-bottom: 1px solid #dee2e6;
  padding-bottom: 0.5rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.form-group input,
.form-group textarea,
.form-group select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  font-size: 1rem;
}

.form-group textarea.monospace {
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 0.9rem;
}

.form-group .hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.85rem;
  color: #6c757d;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.checkbox-group label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: normal;
  cursor: pointer;
}

.checkbox-group input[type='checkbox'] {
  width: auto;
}

.color-input {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.color-input input[type='color'] {
  width: 50px;
  height: 36px;
  padding: 2px;
  cursor: pointer;
}

.color-input input[type='text'] {
  flex: 1;
}

.separator-config {
  background: #f8f9fa;
  padding: 1rem;
  border-radius: 4px;
  margin-bottom: 1rem;
}

.separator-preview-section {
  background: #e9ecef;
  padding: 1rem;
  border-radius: 4px;
  margin-bottom: 1rem;
}

.separator-preview-section h4 {
  margin: 0 0 0.75rem 0;
  border: none;
  padding: 0;
}

.preview-examples {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.preview-item {
  display: flex;
  gap: 0.75rem;
  align-items: baseline;
}

.preview-label {
  font-size: 0.85rem;
  color: #6c757d;
  min-width: 60px;
}

.preview-item code {
  background: #ffffff;
  padding: 0.25rem 0.5rem;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.9rem;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.btn-primary,
.btn-secondary {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-small {
  padding: 0.25rem 0.75rem;
  font-size: 0.9rem;
}

.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1001;
}
</style>
