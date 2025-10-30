// Layout Template Registry Service
// Manages registration and retrieval of layout templates

import type { LayoutTemplate } from '@/types/pageTypes'

/**
 * Layout Template Registry
 * Central registry for layout templates used by layout pages
 */
export class LayoutTemplateRegistry {
  private templates: Map<string, LayoutTemplate>
  private listeners: Map<string, Set<Function>>

  constructor() {
    this.templates = new Map()
    this.listeners = new Map()
  }

  /**
   * Register a new layout template
   */
  register(template: LayoutTemplate): void {
    if (this.templates.has(template.id)) {
      console.warn(`Layout template "${template.id}" is already registered. Overwriting.`)
    }

    // Validate template
    this.validateTemplate(template)

    this.templates.set(template.id, template)
    this.emit('layout-registered', template)

    console.log(`✅ Registered layout template: ${template.id}`)
  }

  /**
   * Unregister a layout template
   */
  unregister(templateId: string): boolean {
    const existed = this.templates.delete(templateId)
    if (existed) {
      this.emit('layout-unregistered', templateId)
      console.log(`🗑️ Unregistered layout template: ${templateId}`)
    }
    return existed
  }

  /**
   * Get a layout template by ID
   */
  get(templateId: string): LayoutTemplate | undefined {
    return this.templates.get(templateId)
  }

  /**
   * Get all registered layout templates
   */
  getAll(): LayoutTemplate[] {
    return Array.from(this.templates.values())
  }

  /**
   * Check if a layout template is registered
   */
  has(templateId: string): boolean {
    return this.templates.has(templateId)
  }

  /**
   * Get layout templates that match a predicate
   */
  // eslint-disable-next-line no-unused-vars
  filter(predicate: (template: LayoutTemplate) => boolean): LayoutTemplate[] {
    return this.getAll().filter(predicate)
  }

  /**
   * Get slots for a layout template
   */
  getSlots(templateId: string) {
    return this.templates.get(templateId)?.slots || []
  }

  /**
   * Clear all registered templates (useful for testing)
   */
  clear(): void {
    this.templates.clear()
    console.log('🗑️ Cleared all layout templates')
  }

  /**
   * Subscribe to registry events
   */
  on(event: string, callback: Function): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set())
    }
    this.listeners.get(event)!.add(callback)
  }

  /**
   * Unsubscribe from registry events
   */
  off(event: string, callback: Function): void {
    this.listeners.get(event)?.delete(callback)
  }

  /**
   * Emit a registry event
   */
  private emit(event: string, ...args: any[]): void {
    this.listeners.get(event)?.forEach(callback => {
      try {
        callback(...args)
      } catch (e) {
        console.error(`Error in layout template registry listener for "${event}":`, e)
      }
    })
  }

  /**
   * Validate layout template structure
   */
  private validateTemplate(template: LayoutTemplate): void {
    if (!template.id) {
      throw new Error('Layout template must have an id')
    }
    if (!template.name) {
      throw new Error('Layout template must have a name')
    }
    if (!template.component) {
      throw new Error('Layout template must have a component')
    }
    if (!template.slots || template.slots.length === 0) {
      throw new Error('Layout template must have at least one slot')
    }

    // Validate slot IDs are unique
    const slotIds = new Set<string>()
    for (const slot of template.slots) {
      if (!slot.id) {
        throw new Error('Layout slot must have an id')
      }
      if (slotIds.has(slot.id)) {
        throw new Error(`Duplicate slot ID "${slot.id}" in layout template "${template.id}"`)
      }
      slotIds.add(slot.id)
    }
  }

  /**
   * Get registry statistics
   */
  getStats() {
    return {
      totalTemplates: this.templates.size,
      templates: Array.from(this.templates.keys()),
      totalSlots: this.getAll().reduce((sum, t) => sum + t.slots.length, 0),
      listeners: Array.from(this.listeners.entries()).map(([event, callbacks]) => ({
        event,
        count: callbacks.size,
      })),
    }
  }
}

// Singleton instance
let registryInstance: LayoutTemplateRegistry | null = null

/**
 * Get the global layout template registry instance
 */
export function getLayoutTemplateRegistry(): LayoutTemplateRegistry {
  if (!registryInstance) {
    registryInstance = new LayoutTemplateRegistry()
  }
  return registryInstance
}

/**
 * Reset the registry (useful for testing)
 */
export function resetLayoutTemplateRegistry(): void {
  registryInstance = null
}
