import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { themes, type ThemeName, type ThemeConfig } from '@/styles/theme-config'

export const useThemeStore = defineStore('theme', () => {
  // 当前主题名称
  const currentTheme = ref<ThemeName>('modern')
  
  // 获取当前主题配置
  const themeConfig = ref<ThemeConfig>(themes[currentTheme.value])
  
  // 设置主题
  function setTheme(themeName: ThemeName) {
    if (themes[themeName]) {
      currentTheme.value = themeName
      themeConfig.value = themes[themeName]
      localStorage.setItem('plog-theme', themeName)
      applyTheme()
    }
  }
  
  // 应用主题到 CSS 变量
  function applyTheme() {
    const root = document.documentElement
    const colors = themeConfig.value.colors
    
    root.style.setProperty('--primary', colors.primary)
    root.style.setProperty('--bg', colors.background)
    root.style.setProperty('--card-bg', colors.cardBg)
    root.style.setProperty('--text', colors.text)
    root.style.setProperty('--text-secondary', colors.textSecondary)
    root.style.setProperty('--border', colors.border)
    root.style.setProperty('--sidebar-bg', colors.sidebarBg)
    root.style.setProperty('--sidebar-text', colors.sidebarText)
    root.style.setProperty('--radius', themeConfig.value.radius)
  }
  
  // 初始化主题
  function initTheme() {
    const savedTheme = localStorage.getItem('plog-theme') as ThemeName
    if (savedTheme && themes[savedTheme]) {
      setTheme(savedTheme)
    } else {
      applyTheme()
    }
  }
  
  // 获取所有主题列表
  function getThemes() {
    return Object.entries(themes).map(([key, config]) => ({
      key,
      name: config.name
    }))
  }
  
  // 监听主题变化
  watch(currentTheme, () => {
    applyTheme()
  })
  
  return {
    currentTheme,
    themeConfig,
    setTheme,
    initTheme,
    getThemes
  }
})