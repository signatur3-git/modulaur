#!/usr/bin/env node
// ⚠️ DEPRECATED - DO NOT USE
// This script is deprecated and should NOT be used.
// The public/plugins directory should be a junction to src-tauri/plugins,
// not a regular directory with copied files.
// Use: npm run plugins:deploy (which creates the junction)
//
// If you run this script, it will break the junction and replace it with
// a regular directory, which will cause plugin loading issues.

console.error('❌ This script is deprecated and should not be used.');
console.error('   Use "npm run plugins:deploy" to set up the plugins junction.');
process.exit(1);

/* DEPRECATED CODE - DO NOT UNCOMMENT
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const pluginsSource = path.resolve(__dirname, '../plugins/featured');
const pluginsTarget = path.resolve(__dirname, 'public/plugins');

console.log('📦 Copying plugins to public directory...');
console.log(`   Source: ${pluginsSource}`);
console.log(`   Target: ${pluginsTarget}`);

// Remove old plugins directory if it exists
if (fs.existsSync(pluginsTarget)) {
  fs.rmSync(pluginsTarget, { recursive: true, force: true });
  console.log('   🗑️  Removed old plugins');
}

// Copy plugins
function copyDir(src, dest) {
  fs.mkdirSync(dest, { recursive: true });
  const entries = fs.readdirSync(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);

    if (entry.isDirectory()) {
      copyDir(srcPath, destPath);
    } else {
      fs.copyFileSync(srcPath, destPath);
    }
  }
}

try {
  copyDir(pluginsSource, pluginsTarget);
  console.log('   ✅ Plugins copied successfully!');
  console.log(`   📁 Available at: http://localhost:5173/plugins/`);
} catch (error) {
  console.error('   ❌ Failed to copy plugins:', error.message);
  process.exit(1);
}
*/

