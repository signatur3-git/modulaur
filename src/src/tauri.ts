// Tauri API helper - provides type-safe access to Tauri commands
// This file helps detect whether we're running in Tauri or browser mode

import { invoke as tauriInvoke } from '@tauri-apps/api/core'

// Check if we're running in Tauri context (Tauri v2)
export const isTauri = () => {
  return window !== undefined && '__TAURI_INTERNALS__' in window
}

// Safe invoke wrapper that checks Tauri availability
export const invoke = async <T>(command: string, args?: Record<string, unknown>): Promise<T> => {
  if (!isTauri()) {
    console.warn(
      `Tauri command "${command}" called but not in Tauri context. Running in browser mode.`
    )
    throw new Error('Not running in Tauri context. Use browser fallback or start with "cargo run".')
  }
  return tauriInvoke<T>(command, args)
}

// Export the check function for use in components
export const checkTauriContext = () => {
  console.log('=== Tauri Context Check ===')
  console.log('Is Tauri?', isTauri())
  console.log('window.__TAURI_INTERNALS__:', (window as any).__TAURI_INTERNALS__)
  console.log('Environment:', isTauri() ? 'Tauri App ✅' : 'Browser ❌')
  console.log('=========================')
  return isTauri()
}
