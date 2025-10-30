import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'
import { copyFileSync, existsSync, mkdirSync } from 'node:fs'

const repoRoot = process.env.MODULAUR_ROOT
  ? path.resolve(process.env.MODULAUR_ROOT)
  : path.resolve(__dirname, '../../../../');

const pluginName = 'text-converter';
const targetDir = path.resolve(repoRoot, 'src-tauri/plugins', pluginName);

export default defineConfig({
  plugins: [
    vue(),
    {
      name: 'copy-manifest',
      writeBundle() {
        const manifestSrc = path.resolve(__dirname, '../manifest.json');
        const manifestDest = path.resolve(targetDir, 'manifest.json');
        if (existsSync(manifestSrc)) {
          mkdirSync(targetDir, { recursive: true });
          copyFileSync(manifestSrc, manifestDest);
          console.log('✓ Copied manifest.json to', targetDir);
        }
      }
    }
  ],
  build: {
    outDir: path.resolve(targetDir, 'frontend/dist'),
    emptyOutDir: true,
    lib: {
      entry: path.resolve(__dirname, 'index.js'),
      name: 'TextConverterPlugin',
      formats: ['umd'],
      fileName: () => 'text-converter.umd.js'
    },
    rollupOptions: {
      external: ['vue'],
      output: {
        globals: {
          vue: 'Vue'
        },
        assetFileNames: 'text-converter.css'
      }
    }
  }
})

