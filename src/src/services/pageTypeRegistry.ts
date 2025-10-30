// Page Type Registry Service
// Manages registration and retrieval of page types

import type { PageType } from '@/types/pageTypes'

/**
 * Page Type Registry
 * Central registry for all page types in the application
 * Supports runtime registration for plugin extensibility
 */
export class PageTypeRegistry {
  private types: Map<string, PageType>
  private listeners: Map<string, Set<Function>>

  constructor() {
    this.types = new Map()
    this.listeners = new Map()
  }

  /**
   * Register a new page type
   */
  register(type: PageType): void {
    if (this.types.has(type.id)) {
      console.warn(`Page type "${type.id}" is already registered. Overwriting.`)
    }

    // Validate page type
    this.validatePageType(type)

    this.types.set(type.id, type)
    this.emit('type-registered', type)

    console.log(`✅ Registered page type: ${type.id}`)
  }

  /**
   * Unregister a page type
   */
  unregister(typeId: string): boolean {
    const existed = this.types.delete(typeId)
    if (existed) {
      this.emit('type-unregistered', typeId)
      console.log(`🗑️ Unregistered page type: ${typeId}`)
    }
    return existed
  }

  /**
   * Get a page type by ID
   */
  get(typeId: string): PageType | undefined {
    return this.types.get(typeId)
  }

  /**
   * Get all registered page types
   */
  getAll(): PageType[] {
    return Array.from(this.types.values())
  }

  /**
   * Check if a page type is registered
   */
  has(typeId: string): boolean {
    return this.types.has(typeId)
  }

  /**
   * Get page types that match a predicate
   */
  // eslint-disable-next-line no-unused-vars
  filter(predicate: (pageType: PageType) => boolean): PageType[] {
    return this.getAll().filter(predicate)
  }

  /**
   * Clear all registered types (useful for testing)
   */
  clear(): void {
    this.types.clear()
    console.log('🗑️ Cleared all page types')
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
        console.error(`Error in page type registry listener for "${event}":`, e)
      }
    })
  }

  /**
   * Validate page type structure
   */
  private validatePageType(type: PageType): void {
    if (!type.id) {
      throw new Error('Page type must have an id')
    }
    if (!type.name) {
      throw new Error('Page type must have a name')
    }
    if (!type.component) {
      throw new Error('Page type must have a component')
    }
    if (type.supportsConfig && type.configSchema) {
      // Basic schema validation
      if (type.configSchema.type !== 'object') {
        throw new Error('Config schema must be of type "object"')
      }
    }
  }

  /**
   * Get registry statistics
   */
  getStats() {
    return {
      totalTypes: this.types.size,
      types: Array.from(this.types.keys()),
      listeners: Array.from(this.listeners.entries()).map(([event, callbacks]) => ({
        event,
        count: callbacks.size,
      })),
    }
  }
}

// Singleton instance
let registryInstance: PageTypeRegistry | null = null

/**
 * Get the global page type registry instance
 */
export function getPageTypeRegistry(): PageTypeRegistry {
  if (!registryInstance) {
    registryInstance = new PageTypeRegistry()
  }
  return registryInstance
}

/**
 * Reset the registry (useful for testing)
 */
export function resetPageTypeRegistry(): void {
  registryInstance = null
}
