<template>
  <div class="snippets-page">
    <SnippetsPanel :panel="{ i: 'snippets-page' }" />
  </div>
</template>

<script setup lang="ts">
import SnippetsPanel from './components/SnippetsPanel.vue';
</script>

<style scoped>
.snippets-page {
  height: 100%;
}
</style>

