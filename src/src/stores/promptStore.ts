// Prompt Generator Store
// Pinia store for managing prompt packages and sections (unified model)

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type {
  PromptPackage,
  PromptTemplate,
  PromptSection,
  SeparatorSet,
  PromptDataType,
  PromptTag,
  PackageExport,
} from '@/types/promptTypes'

export const usePromptStore = defineStore('prompt', () => {
  // ============================================
  // STATE
  // ============================================

  const packages = ref<PromptPackage[]>([])
  // @deprecated - templates are now sections with is_entry_point=true
  const templates = ref<PromptTemplate[]>([])
  const sections = ref<PromptSection[]>([])
  const separatorSets = ref<SeparatorSet[]>([])
  const dataTypes = ref<PromptDataType[]>([])
  const tags = ref<PromptTag[]>([])

  const loading = ref(false)
  const error = ref<string | null>(null)

  const selectedPackageId = ref<string | null>(null)
  const selectedSectionId = ref<string | null>(null)
  // @deprecated
  const selectedTemplateId = ref<string | null>(null)

  // ============================================
  // COMPUTED
  // ============================================

  const selectedPackage = computed(() =>
    packages.value.find(p => getIdString(p.id) === selectedPackageId.value)
  )

  // Entry points are sections marked as is_entry_point=true
  const entryPoints = computed(() => sections.value.filter(s => s.is_entry_point))

  // Package entry points (renderable "templates")
  const packageEntryPoints = computed(() => {
    if (!selectedPackageId.value) return entryPoints.value
    return entryPoints.value.filter(s => s.package_id === selectedPackageId.value)
  })

  // Non-entry point sections (reusable fragments)
  const packageFragments = computed(() => {
    if (!selectedPackageId.value) return sections.value.filter(s => !s.is_entry_point)
    return sections.value.filter(s => s.package_id === selectedPackageId.value && !s.is_entry_point)
  })

  const selectedSection = computed(() =>
    sections.value.find(s => getIdString(s.id) === selectedSectionId.value)
  )

  // @deprecated - use selectedSection instead
  const selectedTemplate = computed(() =>
    templates.value.find(t => getIdString(t.id) === selectedTemplateId.value)
  )

  // @deprecated - use packageEntryPoints instead
  const packageTemplates = computed(() => {
    if (!selectedPackageId.value) return templates.value
    return templates.value.filter(t => t.package_id === selectedPackageId.value)
  })

  const packageSections = computed(() => {
    if (!selectedPackageId.value) return sections.value
    return sections.value.filter(s => s.package_id === selectedPackageId.value)
  })

  const namespaces = computed(() => {
    const ns = new Set<string>()
    packages.value.forEach(p => {
      ns.add(p.namespace)
      // Include additional namespaces
      p.additional_namespaces?.forEach(n => ns.add(n))
    })
    return Array.from(ns).sort()
  })

  // ============================================
  // HELPER FUNCTIONS
  // ============================================

  function getIdString(id: any): string {
    if (!id) return ''
    if (typeof id === 'string') return id
    if (typeof id === 'object' && 'id' in id) {
      const thingId = id.id
      if (typeof thingId === 'string') return thingId
      if (typeof thingId === 'object' && 'String' in thingId) return thingId.String
      return JSON.stringify(thingId)
    }
    return String(id)
  }

  // ============================================
  // PACKAGE ACTIONS
  // ============================================

  async function loadPackages() {
    loading.value = true
    error.value = null
    try {
      packages.value = await invoke('get_prompt_packages')
    } catch (e) {
      error.value = `Failed to load packages: ${e}`
      console.error('Failed to load packages:', e)
    } finally {
      loading.value = false
    }
  }

  async function createPackage(pkg: Omit<PromptPackage, 'id' | 'created_at' | 'updated_at'>) {
    loading.value = true
    error.value = null
    try {
      const created = await invoke<PromptPackage>('create_prompt_package', {
        package: { ...pkg, created_at: '', updated_at: '' },
      })
      packages.value.push(created)
      return created
    } catch (e) {
      error.value = `Failed to create package: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updatePackage(id: string, pkg: PromptPackage) {
    loading.value = true
    error.value = null
    try {
      const updated = await invoke<PromptPackage>('update_prompt_package', { id, package: pkg })
      const index = packages.value.findIndex(p => getIdString(p.id) === id)
      if (index >= 0) packages.value[index] = updated
      return updated
    } catch (e) {
      error.value = `Failed to update package: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deletePackage(id: string) {
    loading.value = true
    error.value = null
    try {
      await invoke('delete_prompt_package', { id })
      // Reload all data to reflect cascade deletions
      await loadAll()
      // Clear selection if deleted package was selected
      if (selectedPackageId.value === id) {
        selectedPackageId.value = null
      }
    } catch (e) {
      error.value = `Failed to delete package: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // TEMPLATE ACTIONS
  // ============================================

  async function loadTemplates(packageId?: string) {
    loading.value = true
    error.value = null
    try {
      templates.value = await invoke('get_prompt_templates', { packageId })
    } catch (e) {
      error.value = `Failed to load templates: ${e}`
      console.error('Failed to load templates:', e)
    } finally {
      loading.value = false
    }
  }

  async function createTemplate(
    template: Omit<PromptTemplate, 'id' | 'created_at' | 'updated_at'>
  ) {
    loading.value = true
    error.value = null
    try {
      const created = await invoke<PromptTemplate>('create_prompt_template', {
        template: { ...template, created_at: '', updated_at: '' },
      })
      templates.value.push(created)
      return created
    } catch (e) {
      error.value = `Failed to create template: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateTemplate(id: string, template: PromptTemplate) {
    loading.value = true
    error.value = null
    try {
      const updated = await invoke<PromptTemplate>('update_prompt_template', { id, template })
      const index = templates.value.findIndex(t => getIdString(t.id) === id)
      if (index >= 0) templates.value[index] = updated
      return updated
    } catch (e) {
      error.value = `Failed to update template: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteTemplate(id: string) {
    loading.value = true
    error.value = null
    try {
      await invoke('delete_prompt_template', { id })
      templates.value = templates.value.filter(t => getIdString(t.id) !== id)
    } catch (e) {
      error.value = `Failed to delete template: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // SECTION ACTIONS
  // ============================================

  async function loadSections(packageId?: string) {
    loading.value = true
    error.value = null
    try {
      sections.value = await invoke('get_prompt_sections', { packageId })
    } catch (e) {
      error.value = `Failed to load sections: ${e}`
      console.error('Failed to load sections:', e)
    } finally {
      loading.value = false
    }
  }

  async function createSection(section: Omit<PromptSection, 'id' | 'created_at' | 'updated_at'>) {
    loading.value = true
    error.value = null
    try {
      const created = await invoke<PromptSection>('create_prompt_section', {
        section: { ...section, created_at: '', updated_at: '' },
      })
      sections.value.push(created)
      return created
    } catch (e) {
      error.value = `Failed to create section: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // SEPARATOR SET ACTIONS
  // ============================================

  async function loadSeparatorSets(packageId?: string) {
    loading.value = true
    error.value = null
    try {
      separatorSets.value = await invoke('get_separator_sets', { packageId })
    } catch (e) {
      error.value = `Failed to load separator sets: ${e}`
      console.error('Failed to load separator sets:', e)
    } finally {
      loading.value = false
    }
  }

  async function createSeparatorSet(set: Omit<SeparatorSet, 'id' | 'created_at' | 'updated_at'>) {
    loading.value = true
    error.value = null
    try {
      const created = await invoke<SeparatorSet>('create_separator_set', {
        separatorSet: { ...set, created_at: '', updated_at: '' },
      })
      separatorSets.value.push(created)
      return created
    } catch (e) {
      error.value = `Failed to create separator set: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // DATA TYPE ACTIONS
  // ============================================

  async function loadDataTypes(packageId?: string) {
    loading.value = true
    error.value = null
    try {
      dataTypes.value = await invoke('get_prompt_data_types', { packageId })
    } catch (e) {
      error.value = `Failed to load data types: ${e}`
      console.error('Failed to load data types:', e)
    } finally {
      loading.value = false
    }
  }

  async function createDataType(dt: Omit<PromptDataType, 'id' | 'created_at' | 'updated_at'>) {
    loading.value = true
    error.value = null
    try {
      const created = await invoke<PromptDataType>('create_prompt_data_type', {
        dataType: { ...dt, created_at: '', updated_at: '' },
      })
      dataTypes.value.push(created)
      return created
    } catch (e) {
      error.value = `Failed to create data type: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // TAG ACTIONS
  // ============================================

  async function loadTags(packageId?: string) {
    loading.value = true
    error.value = null
    try {
      tags.value = await invoke('get_prompt_tags', { packageId })
    } catch (e) {
      error.value = `Failed to load tags: ${e}`
      console.error('Failed to load tags:', e)
    } finally {
      loading.value = false
    }
  }

  async function createTag(tag: Omit<PromptTag, 'id' | 'created_at' | 'updated_at'>) {
    loading.value = true
    error.value = null
    try {
      const created = await invoke<PromptTag>('create_prompt_tag', {
        tag: { ...tag, created_at: '', updated_at: '' },
      })
      tags.value.push(created)
      return created
    } catch (e) {
      error.value = `Failed to create tag: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // EXPORT/IMPORT
  // ============================================

  async function exportPackage(packageId: string): Promise<PackageExport> {
    loading.value = true
    error.value = null
    try {
      return await invoke<PackageExport>('export_prompt_package', { packageId })
    } catch (e) {
      error.value = `Failed to export package: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function importPackage(exportData: PackageExport): Promise<string> {
    loading.value = true
    error.value = null
    try {
      const packageId = await invoke<string>('import_prompt_package', { exportData })
      // Reload all data
      await loadPackages()
      await loadTemplates()
      await loadSections()
      await loadSeparatorSets()
      await loadDataTypes()
      await loadTags()
      return packageId
    } catch (e) {
      error.value = `Failed to import package: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function seedExamplePackages(): Promise<string> {
    loading.value = true
    error.value = null
    try {
      const result = await invoke<string>('seed_example_packages')
      // Reload all data after seeding
      await loadAll()
      return result
    } catch (e) {
      error.value = `Failed to seed examples: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  async function seedText2ImageCommonPackage(): Promise<string> {
    loading.value = true
    error.value = null
    try {
      const result = await invoke<string>('seed_text2image_common_package')
      // Reload all data after seeding
      await loadAll()
      return result
    } catch (e) {
      error.value = `Failed to seed text2image common package: ${e}`
      throw e
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // LOAD ALL
  // ============================================

  async function loadAll() {
    loading.value = true
    error.value = null
    try {
      await Promise.all([
        loadPackages(),
        loadTemplates(),
        loadSections(),
        loadSeparatorSets(),
        loadDataTypes(),
        loadTags(),
      ])
    } catch (e) {
      error.value = `Failed to load data: ${e}`
      console.error('Failed to load prompt data:', e)
    } finally {
      loading.value = false
    }
  }

  // ============================================
  // SELECTION
  // ============================================

  function selectPackage(id: string | null) {
    selectedPackageId.value = id
    selectedTemplateId.value = null
    selectedSectionId.value = null
  }

  function selectSection(id: string | null) {
    selectedSectionId.value = id
  }

  // @deprecated - use selectSection instead
  function selectTemplate(id: string | null) {
    selectedTemplateId.value = id
  }

  // ============================================
  // RESOLUTION
  // ============================================

  function resolveSection(sectionId: string): PromptSection | null {
    // Support namespace:id format
    if (sectionId.includes(':')) {
      const [namespace, id] = sectionId.split(':')
      return (
        sections.value.find(
          s => s.namespace === namespace && (getIdString(s.id) === id || s.name === id)
        ) || null
      )
    }

    // Search in current namespace first, then all
    const currentNs = selectedPackage.value?.namespace
    if (currentNs) {
      const inCurrentNs = sections.value.find(
        s => s.namespace === currentNs && (getIdString(s.id) === sectionId || s.name === sectionId)
      )
      if (inCurrentNs) return inCurrentNs
    }

    // Search all (only exportable)
    return (
      sections.value.find(
        s => s.exportable && (getIdString(s.id) === sectionId || s.name === sectionId)
      ) || null
    )
  }

  // Resolve an entry point by ID or name
  function resolveEntryPoint(entryPointId: string): PromptSection | null {
    const section = resolveSection(entryPointId)
    if (section && section.is_entry_point) return section
    return null
  }

  function resolveSeparatorSet(setId: string): SeparatorSet | null {
    // Support namespace:id format
    if (setId.includes(':')) {
      const [namespace, id] = setId.split(':')
      return (
        separatorSets.value.find(
          s => s.namespace === namespace && (getIdString(s.id) === id || s.name === id)
        ) || null
      )
    }

    return separatorSets.value.find(s => getIdString(s.id) === setId || s.name === setId) || null
  }

  function resolveDataType(typeId: string): PromptDataType | null {
    if (typeId.includes(':')) {
      const [namespace, id] = typeId.split(':')
      return (
        dataTypes.value.find(
          t => t.namespace === namespace && (getIdString(t.id) === id || t.name === id)
        ) || null
      )
    }

    return dataTypes.value.find(t => getIdString(t.id) === typeId || t.name === typeId) || null
  }

  // ============================================
  // RETURN
  // ============================================

  return {
    // State
    packages,
    templates, // @deprecated
    sections,
    separatorSets,
    dataTypes,
    tags,
    loading,
    error,
    selectedPackageId,
    selectedSectionId,
    selectedTemplateId, // @deprecated

    // Computed
    selectedPackage,
    selectedSection,
    selectedTemplate, // @deprecated
    entryPoints,
    packageEntryPoints,
    packageFragments,
    packageTemplates, // @deprecated
    packageSections,
    namespaces,

    // Helpers
    getIdString,

    // Package actions
    loadPackages,
    createPackage,
    updatePackage,
    deletePackage,

    // Template actions (@deprecated - use section with is_entry_point=true)
    loadTemplates,
    createTemplate,
    updateTemplate,
    deleteTemplate,

    // Section actions
    loadSections,
    createSection,

    // Separator set actions
    loadSeparatorSets,
    createSeparatorSet,

    // Data type actions
    loadDataTypes,
    createDataType,

    // Tag actions
    loadTags,
    createTag,

    // Export/import
    exportPackage,
    importPackage,
    seedExamplePackages,
    seedText2ImageCommonPackage,

    // Load all
    loadAll,

    // Selection
    selectPackage,
    selectSection,
    selectTemplate, // @deprecated

    // Resolution
    resolveSection,
    resolveEntryPoint,
    resolveSeparatorSet,
    resolveDataType,
  }
})
