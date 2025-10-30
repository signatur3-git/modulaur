#!/usr/bin/env node
/**
 * Deploy Plugin Assets (Cross-platform)
 * Creates symlink for frontend serving
 * Modern plugins build directly to src-tauri/plugins/ (no copy needed)
 */

const fs = require('fs');
const path = require('path');

const rootDir = path.resolve(__dirname, '../..');
const tauriPluginsDir = path.join(rootDir, 'src-tauri', 'plugins');
const vitePublicPlugins = path.join(rootDir, 'src', 'public', 'plugins');

console.log('\n===============================================');
console.log('Plugin Deployment');
console.log('===============================================\n');

// Check tauri directory exists
if (!fs.existsSync(path.join(rootDir, 'src-tauri'))) {
  console.error('[X] ERROR: Could not find Tauri directory (src-tauri)');
  process.exit(1);
}

// Check frontend directory exists
if (!fs.existsSync(path.join(rootDir, 'src'))) {
  console.error('[X] ERROR: Could not find frontend directory (src)');
  process.exit(1);
}

// Create plugin directory if it doesn't exist
if (!fs.existsSync(tauriPluginsDir)) {
  fs.mkdirSync(tauriPluginsDir, { recursive: true });
  console.log('[OK] Created plugins directory');
}

// Check if symlink/junction already exists and is correct
let symlinkOk = false;

if (fs.existsSync(vitePublicPlugins)) {
  try {
    const stats = fs.lstatSync(vitePublicPlugins);

    if (stats.isSymbolicLink()) {
      const currentTarget = fs.readlinkSync(vitePublicPlugins);
      const resolvedTarget = path.resolve(path.dirname(vitePublicPlugins), currentTarget);

      if (resolvedTarget === tauriPluginsDir || currentTarget === tauriPluginsDir) {
        symlinkOk = true;
        console.log('[OK] Frontend plugins folder is correctly linked');
        console.log(`    ${vitePublicPlugins} -> ${tauriPluginsDir}`);
      } else {
        console.log('[WARN] Symlink points to wrong target');
        console.log(`    Current: ${currentTarget}`);
        console.log(`    Expected: ${tauriPluginsDir}`);
        console.log('    Recreating symlink...');

        fs.unlinkSync(vitePublicPlugins);
        fs.symlinkSync(tauriPluginsDir, vitePublicPlugins, 'junction');
        symlinkOk = true;
        console.log('[OK] Symlink recreated');
      }
    } else if (stats.isDirectory()) {
      console.log('[WARN] plugins is a regular directory (not a symlink)');
      console.log('    Converting to symlink for better dev experience...');

      // Remove directory and create symlink
      fs.rmSync(vitePublicPlugins, { recursive: true, force: true });
      fs.symlinkSync(tauriPluginsDir, vitePublicPlugins, 'junction');
      symlinkOk = true;
      console.log('[OK] Converted to symlink');
    }
  } catch (err) {
    console.error(`[X] Error checking symlink: ${err.message}`);
  }
} else {
  console.log('-> Creating frontend plugins symlink...');

  // Ensure parent directory exists
  const publicDir = path.dirname(vitePublicPlugins);
  if (!fs.existsSync(publicDir)) {
    fs.mkdirSync(publicDir, { recursive: true });
  }

  try {
    // Use 'junction' on Windows, 'dir' on Unix
    fs.symlinkSync(tauriPluginsDir, vitePublicPlugins, 'junction');
    symlinkOk = true;
    console.log('[OK] Created symlink: frontend/public/plugins -> tauri/plugins');
    console.log('    This eliminates copy overhead in dev mode!');
  } catch (err) {
    console.error(`[X] Failed to create symlink: ${err.message}`);
  }
}

// List deployed plugins
console.log('\n--- Deployed Plugins ---');
if (fs.existsSync(tauriPluginsDir)) {
  const plugins = fs.readdirSync(tauriPluginsDir).filter(f => {
    const pluginPath = path.join(tauriPluginsDir, f);
    return fs.statSync(pluginPath).isDirectory();
  });

  if (plugins.length === 0) {
    console.log('  (no plugins found)');
  } else {
    plugins.forEach(plugin => {
      const manifestPath = path.join(tauriPluginsDir, plugin, 'manifest.json');
      if (fs.existsSync(manifestPath)) {
        console.log(`  ✓ ${plugin}`);
      } else {
        console.log(`  ? ${plugin} (no manifest.json)`);
      }
    });
  }
}

console.log('\n===============================================');
console.log(symlinkOk ? '[OK] Plugin deployment complete!' : '[WARN] Deployment completed with warnings');
console.log('===============================================\n');

process.exit(symlinkOk ? 0 : 0); // Don't fail on symlink issues

