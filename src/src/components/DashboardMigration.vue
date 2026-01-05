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
  background: var(--bg-panel);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 30px;
}

.migration-card h3 {
  margin: 0 0 15px 0;
  font-size: 24px;
  color: var(--text-heading);
}

.migration-card p {
  margin: 0 0 20px 0;
  color: var(--text-secondary);
  line-height: 1.6;
}

.warning-box {
  background: color-mix(in srgb, var(--accent-warning) 14%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-warning) 40%, transparent);
  border-radius: 6px;
  padding: 15px;
  margin-bottom: 20px;
  color: var(--text-primary);
}

.warning-box strong {
  display: block;
  margin-bottom: 10px;
  color: var(--accent-warning);
}

.warning-box ul {
  margin: 0;
  padding-left: 20px;
  color: var(--text-primary);
}

.result-box {
  padding: 15px;
  border-radius: 6px;
  margin-bottom: 20px;
}

.result-box.success {
  background: color-mix(in srgb, var(--accent-success) 14%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-success) 40%, transparent);
  color: var(--accent-success);
}

.result-box.error {
  background: color-mix(in srgb, var(--accent-danger) 14%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-danger) 40%, transparent);
  color: var(--accent-danger);
}

.result-box p {
  margin: 0;
}

.btn-migrate {
  padding: 12px 24px;
  background: var(--bg-button);
  color: var(--text-on-accent);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-migrate:hover:not(:disabled) {
  background: var(--bg-button-hover);
}

.btn-migrate:disabled {
  background: var(--bg-button-secondary);
  cursor: not-allowed;
  opacity: 0.8;
}

/* Remove any leftover hardcoded white styles (theme drives these now). */
</style>
