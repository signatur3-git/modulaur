// Plugin System Test Functions
// Open DevTools console (F12) and run these functions

import { invoke } from '@tauri-apps/api/core'

// Test 1: Get all installed plugins
export async function testGetPlugins() {
  console.log('🔍 Testing: Get Installed Plugins...')
  try {
    const plugins = await invoke('get_installed_plugins')
    console.log('✅ Plugins found:', plugins)
    return plugins
  } catch (error) {
    console.error('❌ Error:', error)
  }
}

// Test 2: Get specific plugin info
export async function testGetPluginInfo(name: string = 'example-adapter') {
  console.log(`🔍 Testing: Get Plugin Info for '${name}'...`)
  try {
    const info = await invoke('get_plugin_info', { name })
    console.log('✅ Plugin info:', info)
    return info
  } catch (error) {
    console.error('❌ Error:', error)
  }
}

// Test 3: Test plugin fetch
export async function testPluginFetch(pluginName: string = 'example-adapter') {
  console.log(`🔍 Testing: Plugin Fetch for '${pluginName}'...`)
  try {
    const result = await invoke('test_plugin_fetch', { pluginName })
    console.log('✅ Fetch result:', result)
    return result
  } catch (error) {
    console.error('❌ Error:', error)
  }
}

// Test 4: Reload plugins
export async function testReloadPlugins() {
  console.log('🔍 Testing: Reload Plugins...')
  try {
    const count = await invoke('reload_plugins')
    console.log(`✅ Reloaded ${count} plugins`)
    return count
  } catch (error) {
    console.error('❌ Error:', error)
  }
}

// Run all tests
export async function runAllTests() {
  console.log('🚀 Running all plugin tests...\n')

  await testGetPlugins()
  console.log('\n')

  await testGetPluginInfo()
  console.log('\n')

  await testPluginFetch()
  console.log('\n')

  await testReloadPlugins()
  console.log('\n')

  console.log('✅ All tests complete!')
}

// Make functions available in console
if (typeof window !== 'undefined') {
  ;(window as any).pluginTests = {
    testGetPlugins,
    testGetPluginInfo,
    testPluginFetch,
    testReloadPlugins,
    runAllTests,
  }
  console.log('📦 Plugin test functions loaded! Usage:')
  console.log('  pluginTests.testGetPlugins()')
  console.log('  pluginTests.testGetPluginInfo("example-adapter")')
  console.log('  pluginTests.testPluginFetch("example-adapter")')
  console.log('  pluginTests.testReloadPlugins()')
  console.log('  pluginTests.runAllTests()')
}
