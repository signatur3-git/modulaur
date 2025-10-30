import { defineStore } from 'pinia'

export interface ThemeConfig {
  name: string
  displayName: string
  rowHeight: number
  compact: boolean
}

export const themes: Record<string, ThemeConfig> = {
  'light-comfortable': {
    name: 'light-comfortable',
    displayName: 'Light (Comfortable)',
    rowHeight: 60,
    compact: false,
  },
  'light-compact': {
    name: 'light-compact',
    displayName: 'Light (Compact)',
    rowHeight: 40,
    compact: true,
  },
  'dark-comfortable': {
    name: 'dark-comfortable',
    displayName: 'Dark (Comfortable)',
    rowHeight: 60,
    compact: false,
  },
  'dark-compact': {
    name: 'dark-compact',
    displayName: 'Dark (Compact)',
    rowHeight: 40,
    compact: true,
  },
}

export const useThemeStore = defineStore('theme', {
  state: () => ({
    currentTheme: 'light-comfortable' as string,
  }),

  getters: {
    theme: (state): ThemeConfig => themes[state.currentTheme] || themes['light-comfortable'],
    availableThemes: () => Object.values(themes),
  },

  actions: {
    setTheme(themeName: string) {
      if (themes[themeName]) {
        this.currentTheme = themeName
        document.documentElement.setAttribute('data-theme', themeName)
        localStorage.setItem('app-theme', themeName)
        console.log(`🎨 Theme changed to: ${themes[themeName].displayName}`)
      }
    },

    loadSavedTheme() {
      const saved = localStorage.getItem('app-theme')
      if (saved && themes[saved]) {
        this.setTheme(saved)
        console.log(`🎨 Loaded saved theme: ${themes[saved].displayName}`)
      } else {
        // Set default theme
        this.setTheme('light-comfortable')
      }
    },
  },
})
