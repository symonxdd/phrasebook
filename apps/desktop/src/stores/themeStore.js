import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  const theme = ref('system') // 'light', 'dark', or 'system'
  let hasSetupSystemListener = false

  const applyTheme = () => {
    const root = document.documentElement
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    const themeToApply =
      theme.value === 'system' ? (systemPrefersDark ? 'dark' : 'light') : theme.value

    root.classList.remove('theme-light', 'theme-dark')
    root.classList.add(`theme-${themeToApply}`)

    if (!hasSetupSystemListener) {
      window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        if (theme.value === 'system') {
          applyTheme()
        }
      })
      hasSetupSystemListener = true
    }
  }

  const initTheme = () => {
    const saved = localStorage.getItem('app-theme')
    theme.value = saved || 'system'
    applyTheme()
  }

  const setTheme = (newTheme) => {
    theme.value = newTheme
    localStorage.setItem('app-theme', newTheme)
    applyTheme()
  }

  return {
    theme,
    initTheme,
    setTheme,
    applyTheme,
  }
})
