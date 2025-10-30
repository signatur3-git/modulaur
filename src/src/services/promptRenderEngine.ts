// Prompt Rendering Engine
// Renders prompt templates with variables, context, and natural language support

import type {
  PromptContent,
  PromptTemplate,
  RenderRequest,
  RenderResult,
  RenderedPrompt,
  RenderError,
  RenderOptions,
  SeparatorRules,
  FormatOptions,
  Condition,
  PromptSection,
} from '@/types/promptTypes'
import { usePromptStore } from '@/stores/promptStore'

// Built-in separator rules
const BUILTIN_SEPARATORS: Record<string, SeparatorRules> = {
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
    many: { item_separator: ', ', last_separator: ', ', template: '{items}{last_separator}{last}' },
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
    many: { item_separator: '\n', last_separator: '\n', template: '{items}{last_separator}{last}' },
  },
}

/**
 * Escape special regex characters in a string
 */
function escapeRegExp(string: string): string {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

/**
 * Main rendering function
 */
export async function renderPrompt(request: RenderRequest): Promise<RenderResult> {
  const startTime = Date.now()
  const store = usePromptStore()
  const errors: RenderError[] = []
  const outputs: RenderedPrompt[] = []
  const variablesUsed = new Set<string>()
  const sectionsUsed = new Set<string>()

  try {
    // Get entry point (section with is_entry_point=true)
    // First try sections (new unified model), then fall back to templates (deprecated)
    let entryPoint: PromptSection | PromptTemplate | undefined = store.sections.find(
      s => store.getIdString(s.id) === request.template_id && s.is_entry_point
    )

    // Fallback to templates for backward compatibility
    if (!entryPoint) {
      entryPoint = store.templates.find(t => store.getIdString(t.id) === request.template_id)
    }

    if (!entryPoint) {
      throw new Error(`Entry point not found: ${request.template_id}`)
    }

    // Generate requested number of outputs
    for (let i = 0; i < request.options.count; i++) {
      try {
        const context: RenderContext = {
          variables: { ...request.context.variables },
          system: request.context.system || {},
          constraints: request.context.constraints || {},
          options: request.options,
          variablesUsed,
          sectionsUsed,
          errors: [],
        }

        const text = renderContent(entryPoint.content, context)

        outputs.push({
          text,
          variable_values: request.context.variables,
          warnings: context.errors.length > 0 ? context.errors.map(e => e.message) : undefined,
        })
      } catch (e) {
        errors.push({
          code: 'RENDER_ERROR',
          message: e instanceof Error ? e.message : String(e),
          location: `output ${i + 1}`,
        })
      }
    }

    return {
      success: errors.length === 0,
      outputs,
      errors: errors.length > 0 ? errors : undefined,
      metadata: {
        template_id: request.template_id,
        timestamp: new Date().toISOString(),
        duration_ms: Date.now() - startTime,
        variables_used: Array.from(variablesUsed),
        sections_used: Array.from(sectionsUsed),
      },
    }
  } catch (e) {
    return {
      success: false,
      outputs: [],
      errors: [
        {
          code: 'FATAL_ERROR',
          message: e instanceof Error ? e.message : String(e),
        },
      ],
      metadata: {
        template_id: request.template_id,
        timestamp: new Date().toISOString(),
        duration_ms: Date.now() - startTime,
        variables_used: [],
        sections_used: [],
      },
    }
  }
}

// Internal rendering context
interface RenderContext {
  variables: Record<string, any>
  system: Record<string, any>
  constraints: Record<string, any>
  options: RenderOptions
  variablesUsed: Set<string>
  sectionsUsed: Set<string>
  errors: RenderError[]
}

/**
 * Render content recursively
 */
function renderContent(content: PromptContent, context: RenderContext): string {
  switch (content.type) {
    case 'text':
      return content.value

    case 'variable':
      return renderVariable(content, context)

    case 'section-ref':
      return renderSectionRef(content, context)

    case 'composite':
      return content.parts.map(part => renderContent(part, context)).join('')

    case 'conditional':
      return renderConditional(content, context)

    case 'list':
      return renderList(content, context)

    case 'context':
      return renderContextValue(content, context)

    case 'plural':
      return renderPlural(content, context)

    case 'article':
      return renderArticle(content, context)

    case 'count-switch':
      return renderCountSwitch(content, context)

    case 'switch':
      return renderSwitch(content, context)

    // Random selection types
    case 'pick-one':
      return renderPickOne(content, context)

    case 'pick-many':
      return renderPickMany(content, context)

    case 'random-value':
      return renderRandomValue(content, context)

    case 'weighted-pick':
      return renderWeightedPick(content, context)

    case 'shuffle':
      return renderShuffle(content, context)

    default:
      context.errors.push({
        code: 'UNKNOWN_CONTENT_TYPE',
        message: `Unknown content type: ${(content as any).type}`,
        severity: 'error',
      } as any)
      return ''
  }
}

/**
 * Render a variable reference
 */
function renderVariable(
  content: { type: 'variable'; variable_id: string; path?: string; format?: FormatOptions },
  context: RenderContext
): string {
  context.variablesUsed.add(content.variable_id)

  let value = context.variables[content.variable_id]

  // Handle nested path
  if (content.path && value !== undefined) {
    const parts = content.path.split('.')
    for (const part of parts) {
      if (value && typeof value === 'object') {
        value = value[part]
      } else {
        value = undefined
        break
      }
    }
  }

  // Handle missing value
  if (value === undefined || value === null) {
    if (content.format?.placeholder) {
      return content.format.placeholder
    }
    return ''
  }

  // Convert to string
  let result = String(value)

  // Apply formatting
  if (content.format) {
    result = applyFormat(result, content.format)
  }

  return result
}

/**
 * Apply format options to a string
 */
function applyFormat(value: string, format: FormatOptions): string {
  let result = value

  // Trim
  if (format.trim) {
    result = result.trim()
  }

  // Case transformation
  if (format.case) {
    switch (format.case) {
      case 'upper':
        result = result.toUpperCase()
        break
      case 'lower':
        result = result.toLowerCase()
        break
      case 'title':
        result = result.replace(
          /\w\S*/g,
          txt => txt.charAt(0).toUpperCase() + txt.substring(1).toLowerCase()
        )
        break
      case 'sentence':
        result = result.charAt(0).toUpperCase() + result.substring(1).toLowerCase()
        break
    }
  }

  // Replace patterns
  if (format.replace) {
    for (const { from, to } of format.replace) {
      // Use replace with global regex for compatibility
      result = result.replace(new RegExp(escapeRegExp(from), 'g'), to)
    }
  }

  // Truncate
  if (format.truncate && result.length > format.truncate) {
    result = result.substring(0, format.truncate - 3) + '...'
  }

  // Prefix/suffix
  if (format.prefix) {
    result = format.prefix + result
  }
  if (format.suffix) {
    result = result + format.suffix
  }

  return result
}

/**
 * Render a section reference
 */
function renderSectionRef(
  content: { type: 'section-ref'; section_id: string },
  context: RenderContext
): string {
  const store = usePromptStore()
  const section = store.resolveSection(content.section_id)

  if (!section) {
    context.errors.push({
      code: 'SECTION_NOT_FOUND',
      message: `Section not found: ${content.section_id}`,
      severity: 'error',
    } as any)
    return ''
  }

  context.sectionsUsed.add(content.section_id)
  return renderContent(section.content, context)
}

/**
 * Render a conditional block
 */
function renderConditional(
  content: {
    type: 'conditional'
    condition: Condition
    then_content: PromptContent
    else_content?: PromptContent
  },
  context: RenderContext
): string {
  const result = evaluateCondition(content.condition, context)

  if (result) {
    return renderContent(content.then_content, context)
  } else if (content.else_content) {
    return renderContent(content.else_content, context)
  }

  return ''
}

/**
 * Evaluate a condition
 */
function evaluateCondition(condition: Condition, context: RenderContext): boolean {
  // Compound conditions
  if (condition.and) {
    return condition.and.every(c => evaluateCondition(c, context))
  }
  if (condition.or) {
    return condition.or.some(c => evaluateCondition(c, context))
  }
  if (condition.not) {
    return !evaluateCondition(condition.not, context)
  }

  // Get the value to check
  let value: any
  if (condition.variable) {
    value = context.variables[condition.variable]
    if (condition.path) {
      const parts = condition.path.split('.')
      for (const part of parts) {
        if (value && typeof value === 'object') {
          value = value[part]
        } else {
          value = undefined
          break
        }
      }
    }
  } else if (condition.context_key) {
    const [category, key] = condition.context_key.split('.')
    if (category === 'system') {
      value = context.system[key]
    } else if (category === 'constraints') {
      value = context.constraints[key]
    }
  }

  // Evaluate operator
  switch (condition.operator) {
    case 'exists':
      return value !== undefined && value !== null && value !== ''

    case 'not_exists':
      return value === undefined || value === null || value === ''

    case 'equals':
      return value === condition.value

    case 'not_equals':
      return value !== condition.value

    case 'contains':
      if (Array.isArray(value)) {
        return value.includes(condition.value)
      }
      if (typeof value === 'string') {
        return value.includes(condition.value)
      }
      return false

    case 'starts_with':
      return typeof value === 'string' && value.startsWith(condition.value)

    case 'ends_with':
      return typeof value === 'string' && value.endsWith(condition.value)

    case 'greater_than':
      return typeof value === 'number' && value > condition.value

    case 'less_than':
      return typeof value === 'number' && value < condition.value

    case 'has_items':
      return Array.isArray(value) && value.length > 0

    case 'is_empty':
      if (Array.isArray(value)) return value.length === 0
      if (typeof value === 'string') return value.length === 0
      return !value

    case 'matches':
      if (typeof value === 'string' && condition.value) {
        return new RegExp(condition.value).test(value)
      }
      return false

    default:
      return false
  }
}

/**
 * Render a list with separator set
 */
function renderList(
  content: {
    type: 'list'
    variable_id: string
    separator_set_id: string
    item_template?: PromptContent
  },
  context: RenderContext
): string {
  context.variablesUsed.add(content.variable_id)

  const items = context.variables[content.variable_id]
  if (!Array.isArray(items) || items.length === 0) {
    return ''
  }

  // Get separator rules
  const rules = getSeparatorRules(content.separator_set_id)
  if (!rules) {
    context.errors.push({
      code: 'SEPARATOR_NOT_FOUND',
      message: `Separator set not found: ${content.separator_set_id}`,
      severity: 'error',
    } as any)
    return items.join(', ') // Fallback
  }

  // Render each item
  const renderedItems = items.map((item, index) => {
    if (content.item_template) {
      // Create a sub-context with the item as a variable
      const itemContext: RenderContext = {
        ...context,
        variables: {
          ...context.variables,
          item,
          index,
          // Allow access to item properties directly
          ...(typeof item === 'object' ? item : {}),
        },
      }
      return renderContent(content.item_template, itemContext)
    }
    return String(item)
  })

  return joinWithSeparators(renderedItems, rules)
}

/**
 * Get separator rules by ID
 */
function getSeparatorRules(setId: string): SeparatorRules | null {
  // Check built-in separators first
  if (BUILTIN_SEPARATORS[setId]) {
    return BUILTIN_SEPARATORS[setId]
  }

  // Check store
  const store = usePromptStore()
  const set = store.resolveSeparatorSet(setId)
  if (set) {
    return set.rules as SeparatorRules
  }

  return null
}

/**
 * Join items using separator rules
 */
function joinWithSeparators(items: string[], rules: SeparatorRules): string {
  if (items.length === 0) {
    return ''
  }

  if (items.length === 1) {
    return rules.single.template.replace('{item}', items[0])
  }

  if (items.length === 2) {
    return rules.two.template
      .replace('{first}', items[0])
      .replace('{separator}', rules.two.separator)
      .replace('{second}', items[1])
  }

  // 3+ items
  if (rules.many.numbered) {
    // Numbered list
    return items.map((item, i) => `${i + 1}. ${item}`).join('\n')
  }

  const allButLast = items.slice(0, -1)
  const last = items[items.length - 1]

  return rules.many.template
    .replace('{items}', allButLast.join(rules.many.item_separator))
    .replace('{last_separator}', rules.many.last_separator)
    .replace('{last}', last)
}

/**
 * Render context value
 */
function renderContextValue(
  content: { type: 'context'; context_key: string; fallback?: PromptContent },
  context: RenderContext
): string {
  const [category, ...keyParts] = content.context_key.split('.')
  const key = keyParts.join('.')

  let value: any
  if (category === 'system') {
    value = key ? context.system[key] : context.system
  } else if (category === 'constraints') {
    value = key ? context.constraints[key] : context.constraints
  } else if (category === 'variables') {
    value = key ? context.variables[key] : context.variables
  }

  if (value !== undefined && value !== null) {
    return String(value)
  }

  if (content.fallback) {
    return renderContent(content.fallback, context)
  }

  return ''
}

/**
 * Render pluralized content based on count
 */
function renderPlural(
  content: {
    type: 'plural'
    count_variable: string
    zero?: string
    one: string
    two?: string
    few?: string
    many?: string
    other: string
  },
  context: RenderContext
): string {
  context.variablesUsed.add(content.count_variable)

  const value = context.variables[content.count_variable]
  let count: number

  if (Array.isArray(value)) {
    count = value.length
  } else if (typeof value === 'number') {
    count = value
  } else {
    count = 0
  }

  let template: string
  if (count === 0 && content.zero !== undefined) {
    template = content.zero
  } else if (count === 1) {
    template = content.one
  } else if (count === 2 && content.two !== undefined) {
    template = content.two
  } else if (count >= 3 && count <= 4 && content.few !== undefined) {
    template = content.few
  } else if (count >= 5 && content.many !== undefined) {
    template = content.many
  } else {
    template = content.other
  }

  // Replace {count} placeholder
  return template.replace(/\{count\}/g, String(count))
}

/**
 * Words that take "an" despite not starting with a vowel letter
 */
const AN_WORDS = new Set([
  'hour',
  'hours',
  'honest',
  'honor',
  'honour',
  'heir',
  'heirloom',
  'herb',
  'herbs', // American English
  // Acronyms pronounced with vowel sounds
  'html',
  'http',
  'sql',
  'mri',
  'fbi',
  'nba',
  'nfl',
  'mba',
])

/**
 * Words that take "a" despite starting with a vowel letter
 */
const A_WORDS = new Set([
  'one',
  'once',
  'university',
  'unicorn',
  'uniform',
  'union',
  'unique',
  'unit',
  'united',
  'universe',
  'universal',
  'use',
  'used',
  'useful',
  'user',
  'usual',
  'usually',
  'utility',
  'utensil',
  'euphoria',
  'european',
  'eucalyptus',
  'ewe',
])

/**
 * Render article (a/an/the) based on following word
 */
function renderArticle(
  content: {
    type: 'article'
    word_variable?: string
    word_content?: PromptContent
    style: 'indefinite' | 'definite'
    capitalize?: boolean
  },
  context: RenderContext
): string {
  // Get the word to check
  let word: string
  if (content.word_variable) {
    context.variablesUsed.add(content.word_variable)
    word = String(context.variables[content.word_variable] || '')
  } else if (content.word_content) {
    word = renderContent(content.word_content, context)
  } else {
    return ''
  }

  word = word.trim().toLowerCase()
  if (!word) return ''

  let article: string

  if (content.style === 'definite') {
    article = 'the'
  } else {
    // Indefinite article logic (a/an)
    const firstWord = word.split(/\s+/)[0]

    // Check exception lists first
    if (AN_WORDS.has(firstWord)) {
      article = 'an'
    } else if (A_WORDS.has(firstWord)) {
      article = 'a'
    } else {
      // Default vowel check
      const firstChar = firstWord.charAt(0)
      article = 'aeiou'.includes(firstChar) ? 'an' : 'a'
    }
  }

  if (content.capitalize) {
    article = article.charAt(0).toUpperCase() + article.slice(1)
  }

  return article
}

/**
 * Render count-based switch
 */
function renderCountSwitch(
  content: {
    type: 'count-switch'
    count_variable: string
    cases: { count: number | 'zero' | 'one' | 'other'; content: PromptContent }[]
    default_content?: PromptContent
  },
  context: RenderContext
): string {
  context.variablesUsed.add(content.count_variable)

  const value = context.variables[content.count_variable]
  let count: number

  if (Array.isArray(value)) {
    count = value.length
  } else if (typeof value === 'number') {
    count = value
  } else {
    count = 0
  }

  // Find matching case
  for (const c of content.cases) {
    if (c.count === count) {
      return renderContent(c.content, context)
    }
    if (c.count === 'zero' && count === 0) {
      return renderContent(c.content, context)
    }
    if (c.count === 'one' && count === 1) {
      return renderContent(c.content, context)
    }
    if (c.count === 'other' && count > 1) {
      return renderContent(c.content, context)
    }
  }

  if (content.default_content) {
    return renderContent(content.default_content, context)
  }

  return ''
}

/**
 * Render general switch/case
 */
function renderSwitch(
  content: {
    type: 'switch'
    variable_id: string
    cases: { value: string | number | boolean; content: PromptContent }[]
    default_content?: PromptContent
  },
  context: RenderContext
): string {
  context.variablesUsed.add(content.variable_id)

  const value = context.variables[content.variable_id]

  // Find matching case
  for (const c of content.cases) {
    if (c.value === value) {
      return renderContent(c.content, context)
    }
  }

  if (content.default_content) {
    return renderContent(content.default_content, context)
  }

  return ''
}

// ============================================
// RANDOM SELECTION FUNCTIONS
// ============================================

/**
 * Seeded random number generator for reproducible results
 */
function seededRandom(seed: number): () => number {
  return function () {
    seed = (seed * 1103515245 + 12345) & 0x7fffffff
    return seed / 0x7fffffff
  }
}

/**
 * Get random function from context (seeded or Math.random)
 */
function getRandomFn(context: RenderContext): () => number {
  if (context.options.randomize?.seed !== undefined) {
    return seededRandom(context.options.randomize.seed)
  }
  return Math.random
}

/**
 * Pick one random item from candidates
 */
function renderPickOne(
  content: {
    type: 'pick-one'
    candidates: PromptContent[]
    weights?: number[]
  },
  context: RenderContext
): string {
  if (content.candidates.length === 0) {
    return ''
  }

  const random = getRandomFn(context)
  let selectedIndex: number

  if (content.weights && content.weights.length === content.candidates.length) {
    // Weighted selection
    const totalWeight = content.weights.reduce((sum, w) => sum + w, 0)
    let r = random() * totalWeight
    selectedIndex = 0
    for (let i = 0; i < content.weights.length; i++) {
      r -= content.weights[i]
      if (r <= 0) {
        selectedIndex = i
        break
      }
    }
  } else {
    // Uniform selection
    selectedIndex = Math.floor(random() * content.candidates.length)
  }

  return renderContent(content.candidates[selectedIndex], context)
}

/**
 * Pick multiple random items from candidates
 */
function renderPickMany(
  content: {
    type: 'pick-many'
    candidates: PromptContent[]
    count: number | { min: number; max: number }
    separator_set_id?: string
    allow_duplicates?: boolean
  },
  context: RenderContext
): string {
  if (content.candidates.length === 0) {
    return ''
  }

  const random = getRandomFn(context)

  // Determine count to pick
  let pickCount: number
  if (typeof content.count === 'number') {
    pickCount = content.count
  } else {
    const range = content.count.max - content.count.min + 1
    pickCount = content.count.min + Math.floor(random() * range)
  }

  // Collect picked items
  const picked: PromptContent[] = []
  const availableIndices = content.candidates.map((_, i) => i)

  for (let i = 0; i < pickCount && availableIndices.length > 0; i++) {
    const randIndex = Math.floor(random() * availableIndices.length)
    const selectedIndex = availableIndices[randIndex]
    picked.push(content.candidates[selectedIndex])

    if (!content.allow_duplicates) {
      availableIndices.splice(randIndex, 1)
    }
  }

  // Render each picked item
  const renderedItems = picked.map(item => renderContent(item, context))

  // Join with separator
  const separatorId = content.separator_set_id || 'oxford-comma'
  const rules = getSeparatorRules(separatorId)
  if (rules) {
    return joinWithSeparators(renderedItems, rules)
  }
  return renderedItems.join(', ')
}

/**
 * Pick a random value from a pool
 */
function renderRandomValue(
  content: {
    type: 'random-value'
    pool?: string[]
    pool_variable?: string
    data_type_id?: string
    format?: FormatOptions
  },
  context: RenderContext
): string {
  const random = getRandomFn(context)
  let pool: string[] = []

  // Get pool from various sources
  if (content.pool) {
    pool = content.pool
  } else if (content.pool_variable) {
    context.variablesUsed.add(content.pool_variable)
    const varPool = context.variables[content.pool_variable]
    if (Array.isArray(varPool)) {
      pool = varPool.map(String)
    }
  } else if (content.data_type_id) {
    // Get enum values from data type
    const store = usePromptStore()
    const dataType = store.resolveDataType(content.data_type_id)
    if (dataType?.validation?.enum_values) {
      pool = dataType.validation.enum_values
    }
  }

  if (pool.length === 0) {
    return ''
  }

  // Pick random value
  const selectedIndex = Math.floor(random() * pool.length)
  let value = pool[selectedIndex]

  // Apply formatting
  if (content.format) {
    value = applyFormat(value, content.format)
  }

  return value
}

/**
 * Weighted random selection
 */
function renderWeightedPick(
  content: {
    type: 'weighted-pick'
    options: { weight: number; content: PromptContent }[]
  },
  context: RenderContext
): string {
  if (content.options.length === 0) {
    return ''
  }

  const random = getRandomFn(context)
  const totalWeight = content.options.reduce((sum, opt) => sum + opt.weight, 0)
  let r = random() * totalWeight

  for (const option of content.options) {
    r -= option.weight
    if (r <= 0) {
      return renderContent(option.content, context)
    }
  }

  // Fallback to last option
  return renderContent(content.options[content.options.length - 1].content, context)
}

/**
 * Shuffle array and optionally pick subset
 */
function renderShuffle(
  content: {
    type: 'shuffle'
    variable_id: string
    count?: number
    separator_set_id?: string
    item_template?: PromptContent
  },
  context: RenderContext
): string {
  context.variablesUsed.add(content.variable_id)

  const items = context.variables[content.variable_id]
  if (!Array.isArray(items) || items.length === 0) {
    return ''
  }

  const random = getRandomFn(context)

  // Fisher-Yates shuffle
  const shuffled = [...items]
  for (let i = shuffled.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1))
    ;[shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]]
  }

  // Take subset if count specified
  const selected = content.count ? shuffled.slice(0, content.count) : shuffled

  // Render each item
  const renderedItems = selected.map((item, index) => {
    if (content.item_template) {
      const itemContext: RenderContext = {
        ...context,
        variables: {
          ...context.variables,
          item,
          index,
          ...(typeof item === 'object' ? item : {}),
        },
      }
      return renderContent(content.item_template, itemContext)
    }
    return String(item)
  })

  // Join with separator
  const separatorId = content.separator_set_id || 'oxford-comma'
  const rules = getSeparatorRules(separatorId)
  if (rules) {
    return joinWithSeparators(renderedItems, rules)
  }
  return renderedItems.join(', ')
}

/**
 * Quick render function for simple templates
 */
export function quickRender(
  templateContent: PromptContent,
  variables: Record<string, any>
): string {
  const context: RenderContext = {
    variables,
    system: {},
    constraints: {},
    options: { count: 1 },
    variablesUsed: new Set(),
    sectionsUsed: new Set(),
    errors: [],
  }

  return renderContent(templateContent, context)
}

/**
 * Preview separator set with example items
 */
export function previewSeparator(rules: SeparatorRules, items: string[]): string {
  return joinWithSeparators(items, rules)
}

/**
 * Validate template content
 */
export function validateContent(content: PromptContent): string[] {
  const errors: string[] = []

  function validate(c: PromptContent, path: string) {
    if (!c || typeof c !== 'object') {
      errors.push(`${path}: Invalid content`)
      return
    }

    switch (c.type) {
      case 'text':
        if (typeof c.value !== 'string') {
          errors.push(`${path}: Text value must be a string`)
        }
        break

      case 'variable':
        if (!c.variable_id) {
          errors.push(`${path}: Variable must have variable_id`)
        }
        break

      case 'section-ref':
        if (!c.section_id) {
          errors.push(`${path}: Section reference must have section_id`)
        }
        break

      case 'composite':
        if (!Array.isArray(c.parts)) {
          errors.push(`${path}: Composite must have parts array`)
        } else {
          c.parts.forEach((part, i) => validate(part, `${path}.parts[${i}]`))
        }
        break

      case 'conditional':
        if (!c.condition) {
          errors.push(`${path}: Conditional must have condition`)
        }
        if (!c.then_content) {
          errors.push(`${path}: Conditional must have then_content`)
        } else {
          validate(c.then_content, `${path}.then`)
        }
        if (c.else_content) {
          validate(c.else_content, `${path}.else`)
        }
        break

      case 'list':
        if (!c.variable_id) {
          errors.push(`${path}: List must have variable_id`)
        }
        if (!c.separator_set_id) {
          errors.push(`${path}: List must have separator_set_id`)
        }
        if (c.item_template) {
          validate(c.item_template, `${path}.item_template`)
        }
        break

      case 'context':
        if (!c.context_key) {
          errors.push(`${path}: Context must have context_key`)
        }
        if (c.fallback) {
          validate(c.fallback, `${path}.fallback`)
        }
        break

      default:
        errors.push(`${path}: Unknown content type: ${(c as any).type}`)
    }
  }

  validate(content, 'root')
  return errors
}
