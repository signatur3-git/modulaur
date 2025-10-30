// Prompt Generator Types
// Complete type definitions for the prompt generator system

// ============================================
// CORE ENTITIES
// ============================================

/**
 * A shareable collection of prompt components with a namespace
 */
export interface PromptPackage {
  id: string
  namespace: string // Primary namespace, e.g., "creative-writing"
  additional_namespaces?: string[] // Additional namespaces this package defines
  name: string // Display name
  version: string // Semantic version
  description: string
  author: string

  // Dependencies on other packages
  dependencies: string[] // Array of package namespaces

  // What this package explicitly exports for cross-namespace use
  exports: string[] // Component IDs that are public

  created_at: string
  updated_at: string
}

/**
 * PromptSection - The unified content unit
 *
 * A section can be:
 * - An entry point (is_entry_point=true): A renderable "template" with variables and examples
 * - A reusable fragment (is_entry_point=false): A building block for composition
 *
 * The `content` field IS the template - there's no separate template entity.
 */
export interface PromptSection {
  id: string
  package_id: string
  namespace: string
  name: string
  description: string

  // The "template" - content logic lives here
  content: PromptContent

  // Whether this section is a top-level entry point for rendering
  // Entry points appear in the renderer's template selector
  is_entry_point: boolean

  // Whether other packages can reference this section
  exportable: boolean

  // Variables this section requires (from parent or user input)
  required_variables: string[]

  // === Entry Point Fields (only used when is_entry_point=true) ===

  // Variable definitions with types, defaults, validation
  variables?: VariableDefinition[]

  // Tags for categorization/filtering
  tags?: string[]

  // Example renderings for documentation
  examples?: PromptExample[]

  created_at: string
  updated_at: string
}

/**
 * @deprecated Use PromptSection with is_entry_point=true instead
 * Kept for backward compatibility during migration
 */
export interface PromptTemplate {
  id: string
  package_id: string
  namespace: string
  name: string
  description: string
  content: PromptContent
  variables: VariableDefinition[]
  tags: string[]
  examples: PromptExample[]
  created_at: string
  updated_at: string
}

/**
 * Defines how to join lists naturally
 */
export interface SeparatorSet {
  id: string
  package_id: string
  namespace: string
  name: string
  description: string

  rules: SeparatorRules

  created_at: string
  updated_at: string
}

/**
 * Custom data type for variables
 */
export interface PromptDataType {
  id: string
  package_id: string
  namespace: string
  name: string
  description: string

  base_type: BaseType
  validation?: ValidationRules
  format?: FormatOptions
  examples: any[]

  created_at: string
  updated_at: string
}

/**
 * Tag for categorization
 */
export interface PromptTag {
  id: string
  package_id: string
  namespace: string
  name: string
  description: string
  color?: string
  parent?: string // For hierarchical tags

  created_at: string
  updated_at: string
}

// ============================================
// CONTENT TYPES
// ============================================

/**
 * Union type for all possible content types
 */
export type PromptContent =
  | TextContent
  | VariableContent
  | SectionRefContent
  | CompositeContent
  | ConditionalContent
  | ListContent
  | ContextContent
  | PluralContent
  | ArticleContent
  | CountSwitchContent
  | SwitchContent
  // Random selection types
  | PickOneContent
  | PickManyContent
  | RandomValueContent
  | WeightedPickContent
  | ShuffleContent

export interface TextContent {
  type: 'text'
  value: string
}

export interface VariableContent {
  type: 'variable'
  variable_id: string
  path?: string // For object variables: "address.city"
  format?: FormatOptions
}

export interface SectionRefContent {
  type: 'section-ref'
  section_id: string // Can be "namespace:section-id" for cross-namespace
}

export interface CompositeContent {
  type: 'composite'
  parts: PromptContent[]
}

export interface ConditionalContent {
  type: 'conditional'
  condition: Condition
  then_content: PromptContent
  else_content?: PromptContent
}

export interface ListContent {
  type: 'list'
  variable_id: string // Variable containing the array
  separator_set_id: string // Which separator set to use
  item_template?: PromptContent // Optional template for each item
}

export interface ContextContent {
  type: 'context'
  context_key: string // e.g., "system.role", "constraints.style"
  fallback?: PromptContent
}

/**
 * Pluralization based on count
 * Handles: 0 items, 1 item, 2 items, many items
 */
export interface PluralContent {
  type: 'plural'
  count_variable: string // Variable containing the count (or array to count)
  zero?: string // "no items"
  one: string // "1 item" or "an item"
  two?: string // "2 items" (optional, uses 'other' if not provided)
  few?: string // For languages with 'few' form (3-4 in some)
  many?: string // For languages with 'many' form
  other: string // "X items" - use {count} placeholder
}

/**
 * Article selection (a/an) based on the following word
 * Handles vowel sounds, silent h, special cases
 */
export interface ArticleContent {
  type: 'article'
  word_variable?: string // Variable containing the word to check
  word_content?: PromptContent // Or content that produces the word
  style: 'indefinite' | 'definite' // a/an vs the
  capitalize?: boolean // Whether to capitalize (A vs a)
}

/**
 * Switch based on count (for more complex count-based logic)
 */
export interface CountSwitchContent {
  type: 'count-switch'
  count_variable: string
  cases: {
    count: number | 'zero' | 'one' | 'other'
    content: PromptContent
  }[]
  default_content?: PromptContent
}

/**
 * General switch/case based on variable value
 */
export interface SwitchContent {
  type: 'switch'
  variable_id: string
  cases: {
    value: string | number | boolean
    content: PromptContent
  }[]
  default_content?: PromptContent
}

/**
 * Randomly pick ONE item from a list of candidates
 * Each candidate can be a section-ref, text, or any content
 */
export interface PickOneContent {
  type: 'pick-one'
  candidates: PromptContent[] // Pick one of these randomly
  weights?: number[] // Optional weights for non-uniform distribution
}

/**
 * Randomly pick MULTIPLE items from a list of candidates
 */
export interface PickManyContent {
  type: 'pick-many'
  candidates: PromptContent[]
  count: number | { min: number; max: number } // How many to pick
  separator_set_id?: string // How to join them (default: oxford-comma)
  allow_duplicates?: boolean // Can pick same item twice (default: false)
}

/**
 * Pick a random value from a data pool (inline array or variable reference)
 */
export interface RandomValueContent {
  type: 'random-value'
  pool?: string[] // Inline pool of values
  pool_variable?: string // Or reference a variable containing the pool
  data_type_id?: string // Or reference a data type with enum values
  format?: FormatOptions // Optional formatting
}

/**
 * Weighted random selection with explicit weights
 */
export interface WeightedPickContent {
  type: 'weighted-pick'
  options: {
    weight: number // Higher = more likely
    content: PromptContent
  }[]
}

/**
 * Shuffle and use items from a list (pick without replacement)
 */
export interface ShuffleContent {
  type: 'shuffle'
  variable_id: string // Variable containing array to shuffle
  count?: number // How many to use (default: all)
  separator_set_id?: string // How to join them
  item_template?: PromptContent
}

// ============================================
// VARIABLE DEFINITIONS
// ============================================

export interface VariableDefinition {
  id: string
  name: string
  description?: string

  // Type information
  type: VariableType
  data_type_id?: string // Reference to custom data type

  // Constraints
  required: boolean
  default_value?: any

  // For enum types
  enum_values?: string[]

  // For array types
  item_type?: VariableType
  min_items?: number
  max_items?: number

  // For object types
  properties?: Record<string, VariableDefinition>
}

export type VariableType = 'string' | 'number' | 'boolean' | 'array' | 'object' | 'enum' | 'custom' // References a PromptDataType

// ============================================
// SEPARATOR RULES
// ============================================

export interface SeparatorRules {
  // For single item
  single: SingleItemRule

  // For two items: "word1 and word2"
  two: TwoItemRule

  // For 3+ items: "word1, word2, and word3"
  many: ManyItemRule

  // Optional: custom formats for specific counts
  custom?: CustomCountRule[]
}

export interface SingleItemRule {
  template: string // "{item}"
}

export interface TwoItemRule {
  separator: string // " and "
  template: string // "{first}{separator}{second}"
}

export interface ManyItemRule {
  item_separator: string // ", "
  last_separator: string // ", and "
  template: string // "{items}{last_separator}{last}"
  numbered?: boolean // Use numbered format
}

export interface CustomCountRule {
  count: number
  template: string
}

// ============================================
// CONDITIONS
// ============================================

export interface Condition {
  // What to check
  variable?: string
  path?: string // For nested: "user.address"
  context_key?: string // For context: "constraints.style"

  // Operator
  operator: ConditionOperator

  // Value to compare (for some operators)
  value?: any

  // Compound conditions
  and?: Condition[]
  or?: Condition[]
  not?: Condition
}

export type ConditionOperator =
  | 'exists' // Variable/path has a value
  | 'not_exists' // Variable/path is missing or null
  | 'equals' // Equals value
  | 'not_equals' // Not equals value
  | 'contains' // String/array contains value
  | 'starts_with' // String starts with value
  | 'ends_with' // String ends with value
  | 'greater_than' // Number comparison
  | 'less_than' // Number comparison
  | 'has_items' // Array has at least one item
  | 'is_empty' // Array/string is empty
  | 'matches' // Regex match

// ============================================
// VALIDATION & FORMATTING
// ============================================

export type BaseType = 'string' | 'number' | 'boolean' | 'array' | 'object' | 'enum'

export interface ValidationRules {
  pattern?: string // Regex pattern for strings
  min?: number // Min value (number) or length (string/array)
  max?: number // Max value (number) or length (string/array)
  enum_values?: string[] // Valid values for enum
  required?: boolean
}

export interface FormatOptions {
  case?: 'upper' | 'lower' | 'title' | 'sentence'
  prefix?: string
  suffix?: string
  truncate?: number // Max length with ellipsis
  placeholder?: string // If value is missing
  trim?: boolean
  replace?: { from: string; to: string }[]
}

// ============================================
// RENDERING
// ============================================

/**
 * Request to render prompts
 */
export interface RenderRequest {
  // What to render
  template_id: string

  // Context for rendering
  context: RenderingContext

  // Options
  options: RenderOptions
}

export interface RenderingContext {
  // User-provided variable values
  variables: Record<string, any>

  // System context
  system?: {
    timestamp?: string
    locale?: string
    random_seed?: number
    role?: string
  }

  // Constraints for the output
  constraints?: {
    max_length?: number
    min_length?: number
    style?: string
    audience?: string
    tone?: string
  }

  // Tag-based filtering
  include_tags?: string[]
  exclude_tags?: string[]
}

export interface RenderOptions {
  // Number of variations to generate
  count: number

  // Package namespace for resolving references
  rule_package?: string

  // Output constraints
  constraints?: {
    max_length?: number
    min_length?: number
    must_include?: string[]
    must_exclude?: string[]
  }

  // Randomization
  randomize?: {
    seed?: number
    variable_order?: boolean
    list_order?: boolean
  }

  // Output format
  format?: 'text' | 'markdown' | 'json'
}

/**
 * Result of rendering
 */
export interface RenderResult {
  success: boolean
  outputs: RenderedPrompt[]
  errors?: RenderError[]
  metadata: RenderMetadata
}

export interface RenderedPrompt {
  text: string
  variable_values: Record<string, any>
  warnings?: string[]
}

export interface RenderError {
  code: string
  message: string
  location?: string
  suggestion?: string
}

export interface RenderMetadata {
  template_id: string
  timestamp: string
  duration_ms: number
  variables_used: string[]
  sections_used: string[]
}

// ============================================
// EXAMPLES & DOCUMENTATION
// ============================================

export interface PromptExample {
  name: string
  description?: string
  variables: Record<string, any>
  expected_output: string
}

// ============================================
// PACKAGE EXPORT/IMPORT
// ============================================

export interface PackageExport {
  format_version: string // Export format version
  exported_at: string
  package: PromptPackage
  sections: PromptSection[] // Unified: includes both entry points and fragments
  separator_sets: SeparatorSet[]
  data_types: PromptDataType[]
  tags: PromptTag[]

  // @deprecated - kept for backward compatibility during migration
  templates?: PromptTemplate[]
}

export interface PackageValidationResult {
  valid: boolean
  errors: ValidationError[]
  warnings: ValidationWarning[]
  dependencies: DependencyInfo[]
}

export interface ValidationError {
  code: string
  message: string
  path: string
  severity: 'error'
}

export interface ValidationWarning {
  code: string
  message: string
  path: string
  severity: 'warning'
}

export interface DependencyInfo {
  namespace: string
  required: boolean
  available: boolean
  version_required?: string
  version_available?: string
}

// ============================================
// BUILT-IN SEPARATOR SETS
// ============================================

export const BUILTIN_SEPARATOR_SETS: Record<string, SeparatorRules> = {
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
    two: { separator: '\n• ', template: '• {first}{separator}{second}' },
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
  space: {
    single: { template: '{item}' },
    two: { separator: ' ', template: '{first}{separator}{second}' },
    many: { item_separator: ' ', last_separator: ' ', template: '{items}{last_separator}{last}' },
  },
}
