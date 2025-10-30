<template>
  <div>
    <div v-for="field in schema.fields" :key="field.key" class="form-group">
      <label>{{ field.label }}</label>

      <!-- Text input -->
      <input
        v-if="field.type === 'text'"
        :value="config[field.key]"
        @input="updateField(field.key, ($event.target as HTMLInputElement).value)"
        :type="field.type"
        class="form-control"
        :placeholder="field.placeholder"
        :required="field.required"
      />

      <!-- Textarea -->
      <textarea
        v-else-if="field.type === 'textarea'"
        :value="config[field.key]"
        @input="updateField(field.key, ($event.target as HTMLTextAreaElement).value)"
        class="form-control"
        :placeholder="field.placeholder"
        :required="field.required"
        :rows="field.rows || 4"
      ></textarea>

      <!-- Select dropdown -->
      <select
        v-else-if="field.type === 'select'"
        :value="config[field.key]"
        @change="updateField(field.key, ($event.target as HTMLSelectElement).value)"
        class="form-control"
        :required="field.required"
      >
        <option v-for="option in field.options" :key="String(option.value)" :value="option.value">
          {{ option.label }}
        </option>
      </select>

      <!-- Number input -->
      <input
        v-else-if="field.type === 'number'"
        :value="config[field.key]"
        @input="updateField(field.key, Number(($event.target as HTMLInputElement).value))"
        type="number"
        class="form-control"
        :placeholder="field.placeholder"
        :required="field.required"
        :min="field.min"
        :max="field.max"
      />

      <!-- Checkbox -->
      <label v-else-if="field.type === 'checkbox'">
        <input
          :checked="config[field.key]"
          @change="updateField(field.key, ($event.target as HTMLInputElement).checked)"
          type="checkbox"
          class="form-check-input"
        />
        {{ field.label }}
      </label>

      <!-- Help text -->
      <small v-if="field.help" class="form-text">{{ field.help }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ConfigField {
  key: string
  label: string
  type: 'text' | 'textarea' | 'select' | 'number' | 'checkbox'
  options?: { value: string | number | boolean; label: string }[]
  placeholder?: string
  required?: boolean
  min?: number
  max?: number
  rows?: number
  help?: string
}

interface ConfigSchema {
  fields: ConfigField[]
}

interface Props {
  schema: ConfigSchema
  config: Record<string, any>
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:config': [config: Record<string, any>]
}>()

function updateField(key: string, value: any) {
  emit('update:config', { ...props.config, [key]: value })
}
</script>

<style scoped>
.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #2c3e50;
}

.form-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: #6c757d;
}

.form-control {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 0.875rem;
}

.form-control:focus {
  outline: none;
  border-color: #3498db;
}

.form-check-input {
  margin-right: 0.5rem;
}
</style>
