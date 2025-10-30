import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { resolve } from 'path';
import { copyFileSync, existsSync, mkdirSync } from 'node:fs';

// Build directly to src-tauri/plugins/ to eliminate copy step
// IMPORTANT: When running this plugin build from within plugins/**, relative paths can
// accidentally resolve to plugins/src-tauri. Prefer the repo root from MODULAUR_ROOT.
const repoRoot = process.env.MODULAUR_ROOT
  ? resolve(process.env.MODULAUR_ROOT)
  : resolve(__dirname, '../../../../');

const pluginName = 'time-tracker';
const targetDir = resolve(repoRoot, 'src-tauri/plugins', pluginName);

export default defineConfig({
  plugins: [
    vue(),
    // Copy manifest.json after build
    {
      name: 'copy-manifest',
      writeBundle() {
        const manifestSrc = resolve(__dirname, '../manifest.json');
        const manifestDest = resolve(targetDir, 'manifest.json');

        if (existsSync(manifestSrc)) {
          mkdirSync(targetDir, { recursive: true });
          copyFileSync(manifestSrc, manifestDest);
          console.log('✓ Copied manifest.json to', targetDir);
        }
      }
    }
  ],
  build: {
    outDir: resolve(targetDir, 'frontend/dist'),
    emptyOutDir: true,
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'TimeTrackerPlugin',
      fileName: (format) => `time-tracker.${format}.js`,
      formats: ['umd']
    },
    rollupOptions: {
      external: ['vue'],
      output: {
        globals: {
          vue: 'Vue'
        },
        exports: 'named'
      }
    }
  }
});
