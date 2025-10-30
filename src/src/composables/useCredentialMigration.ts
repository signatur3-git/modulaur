import { useSecureStorage } from './useSecureStorage'
import type { AuthConfig } from '../stores/dataSourceStore'

/**
 * Utility functions for migrating credentials to secure storage
 * M5.1: Added safety checks and rollback mechanism
 */

/**
 * Test if encryption/decryption works
 * Returns true if encryption is functional
 */
export async function testEncryption(): Promise<boolean> {
  try {
    const { storeCredential, getCredential } = useSecureStorage()

    // Test encrypt/decrypt round trip
    const testValue = 'test-' + Date.now()
    await storeCredential('__test__', testValue)
    const retrieved = await getCredential('__test__')

    // Cleanup
    localStorage.removeItem('secure___test__')

    return retrieved === testValue
  } catch (e) {
    console.error('Encryption test failed:', e)
    return false
  }
}

/**
 * Check if credentials have already been migrated
 */
export function areCredentialsMigrated(): boolean {
  return localStorage.getItem('credentials-migrated') === 'true'
}

/**
 * Mark credentials as migrated
 */
function markAsMigrated(): void {
  localStorage.setItem('credentials-migrated', 'true')
  localStorage.setItem('credentials-migrated-date', new Date().toISOString())
}

/**
 * Store auth credentials securely
 * Returns auth config with sensitive fields removed
 */
export async function secureAuthConfig(
  dataSourceId: string,
  auth: AuthConfig | null
): Promise<AuthConfig | null> {
  if (!auth) return null

  const { storeCredential } = useSecureStorage()
  const securedAuth = { ...auth }

  // Store each sensitive field separately
  if (auth.token) {
    await storeCredential(`ds_${dataSourceId}_token`, auth.token)
    securedAuth.token = '***SECURED***'
  }

  if (auth.password) {
    await storeCredential(`ds_${dataSourceId}_password`, auth.password)
    securedAuth.password = '***SECURED***'
  }

  if (auth.client_secret) {
    await storeCredential(`ds_${dataSourceId}_client_secret`, auth.client_secret)
    securedAuth.client_secret = '***SECURED***'
  }

  if (auth.api_key) {
    await storeCredential(`ds_${dataSourceId}_api_key`, auth.api_key)
    securedAuth.api_key = '***SECURED***'
  }

  return securedAuth
}

/**
 * Restore auth credentials from secure storage
 * Returns complete auth config with real credentials
 */
export async function restoreAuthConfig(
  dataSourceId: string,
  auth: AuthConfig | null
): Promise<AuthConfig | null> {
  if (!auth) return null

  const { getCredential } = useSecureStorage()
  const restoredAuth = { ...auth }

  // Restore each sensitive field
  if (auth.token === '***SECURED***') {
    const token = await getCredential(`ds_${dataSourceId}_token`)
    if (token) restoredAuth.token = token
  }

  if (auth.password === '***SECURED***') {
    const password = await getCredential(`ds_${dataSourceId}_password`)
    if (password) restoredAuth.password = password
  }

  if (auth.client_secret === '***SECURED***') {
    const clientSecret = await getCredential(`ds_${dataSourceId}_client_secret`)
    if (clientSecret) restoredAuth.client_secret = clientSecret
  }

  if (auth.api_key === '***SECURED***') {
    const apiKey = await getCredential(`ds_${dataSourceId}_api_key`)
    if (apiKey) restoredAuth.api_key = apiKey
  }

  return restoredAuth
}

/**
 * Check if auth config has secured credentials
 */
export function hasSecuredCredentials(auth: AuthConfig | null): boolean {
  if (!auth) return false

  return (
    auth.token === '***SECURED***' ||
    auth.password === '***SECURED***' ||
    auth.client_secret === '***SECURED***' ||
    auth.api_key === '***SECURED***'
  )
}

/**
 * Migrate all data sources to use secure storage
 * M5.1: Now with safety checks and rollback
 * Should be called explicitly by user (opt-in), not automatically
 */
export async function migrateDataSourcesToSecureStorage(): Promise<{
  success: boolean
  migrated: number
  error?: string
}> {
  // 1. Check if already migrated
  if (areCredentialsMigrated()) {
    return { success: true, migrated: 0, error: 'Already migrated' }
  }

  // 2. Test encryption works
  console.log('Testing encryption...')
  const canEncrypt = await testEncryption()
  if (!canEncrypt) {
    const error = 'Encryption test failed - cannot migrate safely'
    console.error(error)
    return { success: false, migrated: 0, error }
  }
  console.log('✅ Encryption test passed')

  // 3. Load data sources
  const stored = localStorage.getItem('data-sources')
  if (!stored) {
    return { success: true, migrated: 0, error: 'No data sources to migrate' }
  }

  // 4. Backup before migration
  console.log('Creating backup...')
  localStorage.setItem('data-sources-backup', stored)
  localStorage.setItem('data-sources-backup-date', new Date().toISOString())

  try {
    const dataSources = JSON.parse(stored)
    let migrated = 0

    console.log(`Migrating ${dataSources.length} data sources...`)

    // 5. Migrate each data source
    for (const ds of dataSources) {
      if (ds.auth && !hasSecuredCredentials(ds.auth)) {
        // Has plaintext credentials - migrate them
        console.log(`  Migrating: ${ds.name}`)
        ds.auth = await secureAuthConfig(ds.id, ds.auth)
        migrated++
      }
    }

    // 6. Save updated data sources
    if (migrated > 0) {
      localStorage.setItem('data-sources', JSON.stringify(dataSources))
      console.log(`✅ Migrated ${migrated} data sources to secure storage`)
    } else {
      console.log('No data sources needed migration')
    }

    // 7. Mark as successfully migrated
    markAsMigrated()

    return { success: true, migrated }
  } catch (e: any) {
    // 8. Rollback on error
    console.error('❌ Migration failed, rolling back:', e)

    const backup = localStorage.getItem('data-sources-backup')
    if (backup) {
      localStorage.setItem('data-sources', backup)
      console.log('✅ Rolled back to backup')
    }

    return { success: false, migrated: 0, error: e.message || e.toString() }
  }
}

/**
 * Rollback migration (restore from backup)
 */
export function rollbackMigration(): boolean {
  try {
    const backup = localStorage.getItem('data-sources-backup')
    if (!backup) {
      console.error('No backup found')
      return false
    }

    localStorage.setItem('data-sources', backup)
    localStorage.removeItem('credentials-migrated')
    localStorage.removeItem('credentials-migrated-date')

    console.log('✅ Rolled back migration successfully')
    return true
  } catch (e) {
    console.error('Failed to rollback:', e)
    return false
  }
}
