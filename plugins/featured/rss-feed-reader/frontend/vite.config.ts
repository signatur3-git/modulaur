import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { resolve } from 'path';
import { copyFileSync, existsSync, mkdirSync } from 'node:fs';

const repoRoot = process.env.MODULAUR_ROOT
  ? resolve(process.env.MODULAUR_ROOT)
  : resolve(__dirname, '../../../../');

const pluginName = 'rss-feed-reader';
const targetDir = resolve(repoRoot, 'src-tauri/plugins', pluginName);

export default defineConfig({
  plugins: [
    vue(),
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
      name: 'RssFeedReaderPlugin',
      fileName: (format) => `rss-feed-reader.${format}.js`,
      formats: ['umd']
    },
    rollupOptions: {
      external: ['vue'],
      output: {
        globals: {
          vue: 'Vue'
        },
        assetFileNames: 'rss-feed-reader.[ext]',
        entryFileNames: 'rss-feed-reader.umd.js'
      }
    }
  }
});
