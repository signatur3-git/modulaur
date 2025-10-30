<template>
  <div class="text-panel">
    <p v-if="panel.config.content">{{ panel.config.content }}</p>
    <p v-else class="placeholder">📝 Text panel placeholder. Edit this panel to add content.</p>
  </div>
</template>

<script setup lang="ts">
import type { Panel } from '../../stores/dashboardStore'

interface Props {
  panel: Panel
}

defineProps<Props>()
</script>

<style scoped>
.text-panel {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder {
  color: #95a5a6;
  font-style: italic;
  text-align: center;
}
</style>
