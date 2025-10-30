// Quick Diagnostic for Plugin Issues
// Run this in browser console to debug

async function diagnosePluginsIssue() {
  console.log('🔍 Starting Plugin Diagnostics...\n')

  try {
    // Check what backend knows about
    console.log('1️⃣ Checking backend plugins:')
    const { invoke } = await import('@tauri-apps/api/core')
    const backendPlugins = (await invoke('get_installed_plugins')) as any[]
    console.log(`   Found ${backendPlugins.length} plugins:`)
    backendPlugins.forEach((p: any) => {
      console.log(`   - ${p.name}`)
      console.log(`     hasFrontend: ${!!p.frontend}`)
      if (p.frontend) {
        console.log(`     frontend.enabled: ${p.frontend.enabled}`)
      }
    })

    // Check what frontend loaded
    console.log('\n2️⃣ Checking frontend plugins:')
    const { pluginLoader } = await import('./services/pluginLoader')
    const frontendPlugins = pluginLoader.getAvailablePanelTypes()
    console.log(`   Loaded ${frontendPlugins.length} frontend plugins:`)
    frontendPlugins.forEach((p: any) => {
      console.log(`   - ${p.name}`)
    })

    // Check directory access
    console.log('\n3️⃣ Testing file access:')
    try {
      const response = await fetch('/plugins/hello-panel/frontend/index.js')
      if (response.ok) {
        console.log('   ✅ hello-panel files are accessible')
      } else {
        console.log(`   ❌ hello-panel NOT accessible (${response.status})`)
      }
    } catch (e: any) {
      console.log(`   ❌ Error accessing files: ${e.message}`)
    }

    // Summary
    console.log('\n📊 Summary:')
    if (backendPlugins.length === 1) {
      console.log('   ⚠️  Backend only knows about 1 plugin')
      console.log('   💡 Solution: Backend needs restart to discover hello-panel')
      console.log('   Run: Close app, then "cargo tauri dev"')
    }

    if (frontendPlugins.length === 0 && backendPlugins.some((p: any) => p.frontend?.enabled)) {
      console.log("   ⚠️  Frontend can't load plugins")
      console.log('   💡 Solution: Check file access (test 3 above)')
      console.log('   Run: fix-plugin-access.bat')
    }

    console.log('\n✅ Diagnostics complete!')
  } catch (error) {
    console.error('❌ Diagnostic failed:', error)
  }
}

// Auto-run
diagnosePluginsIssue()

// Export for manual use
window.diagnosePluginsIssue = diagnosePluginsIssue
