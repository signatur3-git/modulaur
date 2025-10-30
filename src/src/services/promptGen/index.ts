// Prompt Generator Page Types Registration
// Registers the prompt generator page types with the page type registry

import { getPageTypeRegistry } from '@/services/pageTypeRegistry'
import { markRaw } from 'vue'

// Import page type components
import PromptAuthoringPage from '@/components/pageTypes/PromptAuthoringPage.vue'
import PromptRendererPage from '@/components/pageTypes/PromptRendererPage.vue'

/**
 * Register prompt generator page types
 */
export function registerPromptGenPageTypes(): void {
  const registry = getPageTypeRegistry()

  // Register Prompt Authoring Page Type
  registry.register({
    id: 'prompt-authoring',
    name: 'Prompt Authoring',
    icon: '🎨',
    description:
      'Create and manage prompt templates, sections, separator sets, and packages. Build complex prompts with variables, conditions, and natural language list handling.',
    component: markRaw(PromptAuthoringPage),
    supportsConfig: false,
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'prompt-generator',
      plugin: 'prompt-gen',
    },
  })

  // Register Prompt Renderer Page Type
  registry.register({
    id: 'prompt-renderer',
    name: 'Prompt Renderer',
    icon: '🎯',
    description:
      'Generate prompts from templates with variables and context. Render multiple variations and copy results to clipboard.',
    component: markRaw(PromptRendererPage),
    supportsConfig: false,
    metadata: {
      version: '1.0.0',
      author: 'Modulaur',
      category: 'prompt-generator',
      plugin: 'prompt-gen',
    },
  })

  console.log('✅ Registered prompt generator page types (Authoring, Renderer)')
}

/**
 * Unregister prompt generator page types
 */
export function unregisterPromptGenPageTypes(): void {
  const registry = getPageTypeRegistry()
  registry.unregister('prompt-authoring')
  registry.unregister('prompt-renderer')
  console.log('🗑️ Unregistered prompt generator page types')
}
