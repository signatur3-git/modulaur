import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

/**
 * Composable for secure credential storage
 * Encrypts sensitive data before storing in localStorage
 * Uses browser's SubtleCrypto API for encryption
 */

interface EncryptedCredential {
  encrypted: string
  iv: string
  salt: string
}

export function useSecureStorage() {
  const isSupported = ref(typeof window !== 'undefined' && window.crypto && window.crypto.subtle)

  /**
   * Generate a encryption key from a password
   */
  async function deriveKey(password: string, salt: Uint8Array): Promise<CryptoKey> {
    const encoder = new TextEncoder()
    const passwordBuffer = encoder.encode(password)

    // Import password as key material
    const keyMaterial = await window.crypto.subtle.importKey(
      'raw',
      passwordBuffer,
      'PBKDF2',
      false,
      ['deriveKey']
    )

    // Derive actual encryption key
    return await window.crypto.subtle.deriveKey(
      {
        name: 'PBKDF2',
        salt: salt,
        iterations: 100000,
        hash: 'SHA-256',
      },
      keyMaterial,
      { name: 'AES-GCM', length: 256 },
      false,
      ['encrypt', 'decrypt']
    )
  }

  /**
   * Get or create machine-specific password
   * In production, this could use Windows Credential Manager via Tauri
   */
  async function getMachinePassword(): Promise<string> {
    // Try to get from Tauri secure storage (if available)
    try {
      const stored = await invoke<string>('get_machine_password')
      if (stored) return stored
    } catch {
      // Fallback: use browser fingerprint
      console.warn('Tauri secure storage not available, using fallback')
    }

    // Fallback: Generate based on machine characteristics
    // This is not perfect security, but better than plaintext
    const fingerprint = [
      navigator.userAgent,
      navigator.language,
      new Date().getTimezoneOffset(),
      screen.width,
      screen.height,
    ].join('|')

    // Hash the fingerprint
    const encoder = new TextEncoder()
    const data = encoder.encode(fingerprint)
    const hashBuffer = await window.crypto.subtle.digest('SHA-256', data)
    const hashArray = Array.from(new Uint8Array(hashBuffer))
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
  }

  /**
   * Encrypt a value
   */
  async function encrypt(value: string): Promise<EncryptedCredential> {
    if (!isSupported.value) {
      console.warn('SubtleCrypto not supported, storing without encryption')
      return {
        encrypted: btoa(value), // Base64 encode (NOT SECURE)
        iv: '',
        salt: '',
      }
    }

    // Generate salt and IV
    const salt = window.crypto.getRandomValues(new Uint8Array(16))
    const iv = window.crypto.getRandomValues(new Uint8Array(12))

    // Get machine password and derive key
    const password = await getMachinePassword()
    const key = await deriveKey(password, salt)

    // Encrypt the value
    const encoder = new TextEncoder()
    const data = encoder.encode(value)
    const encrypted = await window.crypto.subtle.encrypt({ name: 'AES-GCM', iv: iv }, key, data)

    // Convert to base64 for storage
    return {
      encrypted: btoa(String.fromCharCode(...new Uint8Array(encrypted))),
      iv: btoa(String.fromCharCode(...iv)),
      salt: btoa(String.fromCharCode(...salt)),
    }
  }

  /**
   * Decrypt a value
   */
  async function decrypt(credential: EncryptedCredential): Promise<string> {
    if (!isSupported.value || !credential.iv || !credential.salt) {
      // Was stored without encryption (fallback)
      try {
        return atob(credential.encrypted)
      } catch (e) {
        console.error('Failed to decode credential:', e)
        return ''
      }
    }

    try {
      // Convert from base64
      const encrypted = Uint8Array.from(atob(credential.encrypted), c => c.charCodeAt(0))
      const iv = Uint8Array.from(atob(credential.iv), c => c.charCodeAt(0))
      const salt = Uint8Array.from(atob(credential.salt), c => c.charCodeAt(0))

      // Get machine password and derive key
      const password = await getMachinePassword()
      const key = await deriveKey(password, salt)

      // Decrypt
      const decrypted = await window.crypto.subtle.decrypt(
        { name: 'AES-GCM', iv: iv },
        key,
        encrypted
      )

      // Convert back to string
      const decoder = new TextDecoder()
      return decoder.decode(decrypted)
    } catch (e) {
      console.error('Failed to decrypt credential:', e)
      return ''
    }
  }

  /**
   * Store a credential securely
   */
  async function storeCredential(key: string, value: string): Promise<void> {
    const encrypted = await encrypt(value)
    localStorage.setItem(`secure_${key}`, JSON.stringify(encrypted))
  }

  /**
   * Retrieve a credential securely
   */
  async function getCredential(key: string): Promise<string | null> {
    const stored = localStorage.getItem(`secure_${key}`)
    if (!stored) return null

    try {
      const credential = JSON.parse(stored) as EncryptedCredential
      return await decrypt(credential)
    } catch (e) {
      console.error('Failed to retrieve credential:', e)
      return null
    }
  }

  /**
   * Remove a credential
   */
  function removeCredential(key: string): void {
    localStorage.removeItem(`secure_${key}`)
  }

  /**
   * Migrate existing plaintext credentials to encrypted storage
   */
  async function migrateCredentials(): Promise<number> {
    let migrated = 0
    const keysToMigrate: string[] = []

    // Find plaintext credentials (without 'secure_' prefix)
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (
        key &&
        !key.startsWith('secure_') &&
        (key.includes('token') || key.includes('password') || key.includes('key'))
      ) {
        keysToMigrate.push(key)
      }
    }

    // Migrate each credential
    for (const key of keysToMigrate) {
      const value = localStorage.getItem(key)
      if (value) {
        await storeCredential(key, value)
        localStorage.removeItem(key) // Remove plaintext version
        migrated++
      }
    }

    return migrated
  }

  return {
    isSupported,
    storeCredential,
    getCredential,
    removeCredential,
    migrateCredentials,
  }
}
