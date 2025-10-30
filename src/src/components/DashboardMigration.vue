<template>
  <div class="dashboard-migration">
    <div class="migration-card">
      <h3>🔄 Migrate Legacy Dashboards</h3>
      <p>
        This will migrate all file-based dashboards (the old Home page dashboards) to a new
        "Dashboards" page using the dashboard-collection system.
      </p>

      <div class="warning-box">
        <strong>⚠️ Note:</strong>
        <ul>
          <li>This will create a new "Dashboards" page in your navigation</li>
          <li>All existing dashboards will be preserved in the new page</li>
          <li>The old Home button will remain for backward compatibility</li>
          <li>This migration can only be run once</li>
        </ul>
      </div>

      <div
        v-if="migrationResult"
        class="result-box"
        :class="migrationResult.success ? 'success' : 'error'"
      >
        <p>{{ migrationResult.message }}</p>
      </div>

      <button @click="runMigration" :disabled="migrating" class="btn-migrate">
        {{ migrating ? '⏳ Migrating...' : '▶️ Run Migration' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const migrating = ref(false)
const migrationResult = ref<{ success: boolean; message: string } | null>(null)

async function runMigration() {
  if (
    !confirm(
      'Migrate legacy dashboards to the new dashboard-collection system?\n\nThis will create a new "Dashboards" page in your navigation.'
    )
  ) {
    return
  }

  migrating.value = true
  migrationResult.value = null

  try {
    const result = await invoke<string>('migrate_legacy_dashboards_to_collection')
    migrationResult.value = {
      success: true,
      message: result,
    }
  } catch (error) {
    migrationResult.value = {
      success: false,
      message: `Migration failed: ${error}`,
    }
  } finally {
    migrating.value = false
  }
}
</script>

<style scoped>
.dashboard-migration {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.migration-card {
  background: white;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 30px;
}

.migration-card h3 {
  margin: 0 0 15px 0;
  font-size: 24px;
  color: #333;
}

.migration-card p {
  margin: 0 0 20px 0;
  color: #666;
  line-height: 1.6;
}

.warning-box {
  background: #fff3cd;
  border: 1px solid #ffc107;
  border-radius: 6px;
  padding: 15px;
  margin-bottom: 20px;
}

.warning-box strong {
  display: block;
  margin-bottom: 10px;
  color: #856404;
}

.warning-box ul {
  margin: 0;
  padding-left: 20px;
  color: #856404;
}

.warning-box li {
  margin: 5px 0;
}

.result-box {
  padding: 15px;
  border-radius: 6px;
  margin-bottom: 20px;
}

.result-box.success {
  background: #d4edda;
  border: 1px solid #c3e6cb;
  color: #155724;
}

.result-box.error {
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  color: #721c24;
}

.result-box p {
  margin: 0;
}

.btn-migrate {
  padding: 12px 24px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-migrate:hover:not(:disabled) {
  background: #0056b3;
}

.btn-migrate:disabled {
  background: #6c757d;
  cursor: not-allowed;
  opacity: 0.6;
}
</style>
