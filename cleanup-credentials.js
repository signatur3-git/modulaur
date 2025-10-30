/**
 * Cleanup script for orphaned encrypted credentials
 * Run this in browser console to clean up partial migration artifacts
 *
 * USE CASE: If credential migration ran partially and left corrupted data
 */

console.log('🧹 Credential Cleanup Script - Starting...')

// Step 1: Check for orphaned encrypted credentials
const secureKeys = Object.keys(localStorage).filter(k => k.startsWith('secure_'))
console.log(`Found ${secureKeys.length} encrypted credential keys:`, secureKeys)

// Step 2: Check data sources for ***SECURED*** tokens
let dataSources = []
try {
  const stored = localStorage.getItem('data-sources')
  if (stored) {
    dataSources = JSON.parse(stored)
    console.log(`Found ${dataSources.length} data sources`)
  }
} catch (e) {
  console.error('Failed to parse data sources:', e)
}

// Step 3: Find corrupted tokens
const corrupted = dataSources.filter(ds => {
  if (!ds.auth) return false
  return ds.auth.token === '***SECURED***' ||
         ds.auth.password === '***SECURED***' ||
         ds.auth.client_secret === '***SECURED***' ||
         ds.auth.api_key === '***SECURED***'
})

console.log(`Found ${corrupted.length} data sources with corrupted credentials:`)
corrupted.forEach(ds => console.log(`  - ${ds.name}: ${ds.auth.type}`))

// Step 4: Ask for confirmation before cleaning
if (secureKeys.length > 0 || corrupted.length > 0) {
  console.warn('⚠️  WARNING: Found orphaned/corrupted credentials!')
  console.warn('⚠️  Orphaned encrypted keys:', secureKeys.length)
  console.warn('⚠️  Corrupted data sources:', corrupted.length)
  console.log('')
  console.log('To clean up, run:')
  console.log('  cleanupCredentials()')
  console.log('')
  console.log('NOTE: This will DELETE encrypted keys but KEEP data sources.')
  console.log('You will need to re-enter tokens for corrupted data sources.')
} else {
  console.log('✅ No orphaned credentials found - all clean!')
}

// Step 5: Cleanup function (user must call explicitly)
window.cleanupCredentials = function() {
  console.log('🧹 Starting cleanup...')

  // Remove encrypted credential keys
  let removed = 0
  secureKeys.forEach(key => {
    localStorage.removeItem(key)
    removed++
    console.log(`  Removed: ${key}`)
  })

  console.log(`✅ Removed ${removed} encrypted credential keys`)

  // Show corrupted data sources (don't auto-fix, user must re-enter)
  if (corrupted.length > 0) {
    console.warn('⚠️  Still have corrupted data sources - need manual token re-entry:')
    corrupted.forEach(ds => {
      console.warn(`  - ${ds.name}: Edit and re-enter ${ds.auth.type} credentials`)
    })
  }

  console.log('✅ Cleanup complete!')
  console.log('Next: Re-enter credentials for corrupted data sources in the UI')
}

// Done
console.log('🧹 Credential Cleanup Script - Ready')
console.log('Run cleanupCredentials() to perform cleanup')

