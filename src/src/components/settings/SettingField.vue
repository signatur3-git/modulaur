<template>
  <div class="setting-field" :class="{ 'has-error': error }">
    <label :for="field.id" class="field-label">
      {{ field.label }}
      <span v-if="field.required" class="required">*</span>
    </label>

    <p v-if="field.description" class="field-description">{{ field.description }}</p>

    <!-- Text Input -->
    <input
      v-if="field.type === 'text'"
      :id="field.id"
      type="text"
      :value="modelValue"
      @input="handleInput"
      :required="field.required"
      class="field-input"
    />

    <!-- Number Input -->
    <input
      v-else-if="field.type === 'number'"
      :id="field.id"
      type="number"
      :value="modelValue"
      @input="handleInput"
      :required="field.required"
      class="field-input"
    />

    <!-- Checkbox -->
    <label v-else-if="field.type === 'checkbox'" class="checkbox-label">
      <input
        :id="field.id"
        type="checkbox"
        :checked="modelValue"
        @change="handleCheckbox"
        class="field-checkbox"
      />
      <span>{{ field.checkboxLabel || 'Enable' }}</span>
    </label>

    <!-- Select Dropdown -->
    <select
      v-else-if="field.type === 'select'"
      :id="field.id"
      :value="modelValue"
      @change="handleInput"
      :required="field.required"
      class="field-select"
    >
      <option value="">-- Select --</option>
      <option v-for="option in field.options" :key="option.value" :value="option.value">
        {{ option.label }}
      </option>
    </select>

    <!-- Textarea -->
    <textarea
      v-else-if="field.type === 'textarea'"
      :id="field.id"
      :value="modelValue"
      @input="handleInput"
      :required="field.required"
      class="field-textarea"
      :rows="field.rows || 4"
    />

    <!-- Color Picker -->
    <input
      v-else-if="field.type === 'color'"
      :id="field.id"
      type="color"
      :value="modelValue"
      @input="handleInput"
      class="field-color"
    />

    <!-- Range Slider -->
    <div v-else-if="field.type === 'range'" class="range-container">
      <input
        :id="field.id"
        type="range"
        :value="modelValue"
        @input="handleInput"
        :min="field.min || 0"
        :max="field.max || 100"
        :step="field.step || 1"
        class="field-range"
      />
      <span class="range-value">{{ modelValue }}</span>
    </div>

    <!-- Unknown Type -->
    <div v-else class="field-error">Unknown field type: {{ field.type }}</div>

    <p v-if="error" class="error-message">{{ error }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

/**
 * Setting Field Component
 *
 * Renders a single form field based on field type.
 * Supports various input types with validation.
 */

interface Field {
  id: string
  type: string
  label: string
  description?: string
  default?: any
  required?: boolean
  checkboxLabel?: string
  options?: Array<{ value: any; label: string }>
  rows?: number
  min?: number
  max?: number
  step?: number
  validation?: {
    pattern?: string
    min?: number
    max?: number
    minLength?: number
    maxLength?: number
  }
}

const props = defineProps<{
  field: Field
  value?: any
}>()

const emit = defineEmits<{
  update: [fieldId: string, value: any]
}>()

const modelValue = ref(props.value)
const error = ref<string | null>(null)

watch(
  () => props.value,
  newValue => {
    modelValue.value = newValue
  }
)

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
  let value: any = target.value

  // Convert to appropriate type
  if (props.field.type === 'number') {
    value = parseFloat(value)
  } else if (props.field.type === 'range') {
    value = parseFloat(value)
  }

  modelValue.value = value
  error.value = validate(value)

  if (!error.value) {
    emit('update', props.field.id, value)
  }
}

function handleCheckbox(event: Event) {
  const target = event.target as HTMLInputElement
  const value = target.checked
  modelValue.value = value
  emit('update', props.field.id, value)
}

function validate(value: any): string | null {
  const validation = props.field.validation

  if (!validation) return null

  // Required check
  if (props.field.required && (value === undefined || value === null || value === '')) {
    return 'This field is required'
  }

  // Pattern check
  if (validation.pattern && typeof value === 'string') {
    const regex = new RegExp(validation.pattern)
    if (!regex.test(value)) {
      return 'Invalid format'
    }
  }

  // Number range
  if (props.field.type === 'number' || props.field.type === 'range') {
    if (validation.min !== undefined && value < validation.min) {
      return `Minimum value is ${validation.min}`
    }
    if (validation.max !== undefined && value > validation.max) {
      return `Maximum value is ${validation.max}`
    }
  }

  // String length
  if (typeof value === 'string') {
    if (validation.minLength !== undefined && value.length < validation.minLength) {
      return `Minimum length is ${validation.minLength} characters`
    }
    if (validation.maxLength !== undefined && value.length > validation.maxLength) {
      return `Maximum length is ${validation.maxLength} characters`
    }
  }

  return null
}
</script>

<style scoped>
.setting-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.field-label {
  font-weight: 600;
  color: #212529;
  font-size: 0.95rem;
}

.required {
  color: #dc3545;
  margin-left: 0.25rem;
}

.field-description {
  margin: 0;
  font-size: 0.85rem;
  color: #6c757d;
}

.field-input,
.field-select,
.field-textarea {
  padding: 0.5rem 0.75rem;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.field-input:focus,
.field-select:focus,
.field-textarea:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.has-error .field-input,
.has-error .field-select,
.has-error .field-textarea {
  border-color: #dc3545;
}

.field-textarea {
  resize: vertical;
  font-family: inherit;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-weight: normal;
}

.field-checkbox {
  width: 1.25rem;
  height: 1.25rem;
  cursor: pointer;
}

.field-color {
  width: 80px;
  height: 40px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  cursor: pointer;
}

.range-container {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.field-range {
  flex: 1;
}

.range-value {
  min-width: 3rem;
  text-align: center;
  font-weight: 600;
  color: #495057;
}

.error-message {
  margin: 0;
  color: #dc3545;
  font-size: 0.85rem;
}

.field-error {
  padding: 0.75rem;
  background: #fff3cd;
  border: 1px solid #ffc107;
  border-radius: 4px;
  color: #856404;
  font-size: 0.9rem;
}
</style>
