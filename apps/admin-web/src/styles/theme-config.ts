// 主题配置类型
export interface ThemeConfig {
  // 主题名称
  name: string
  // 颜色变量
  colors: {
    primary: string
    background: string
    cardBg: string
    text: string
    textSecondary: string
    border: string
    sidebarBg: string
    sidebarText: string
  }
  // 圆角
  radius: string
}

// 预设主题
export const themes: Record<string, ThemeConfig> = {
  modern: {
    name: '现代简约',
    colors: {
      primary: '#6366f1',
      background: '#f0f2f5',
      cardBg: '#fff',
      text: '#1a1a2e',
      textSecondary: '#6b7280',
      border: '#e5e7eb',
      sidebarBg: '#0f1729',
      sidebarText: '#cbd5e1'
    },
    radius: '12px'
  },
  light: {
    name: '明亮清新',
    colors: {
      primary: '#3b82f6',
      background: '#f8fafc',
      cardBg: '#ffffff',
      text: '#1e293b',
      textSecondary: '#64748b',
      border: '#e2e8f0',
      sidebarBg: '#1e293b',
      sidebarText: '#cbd5e1'
    },
    radius: '8px'
  },
  dark: {
    name: '暗色科技',
    colors: {
      primary: '#818cf8',
      background: '#0f172a',
      cardBg: '#1e293b',
      text: '#f1f5f9',
      textSecondary: '#94a3b8',
      border: '#334155',
      sidebarBg: '#020617',
      sidebarText: '#cbd5e1'
    },
    radius: '12px'
  },
  green: {
    name: '清新绿意',
    colors: {
      primary: '#10b981',
      background: '#f0fdf4',
      cardBg: '#ffffff',
      text: '#1e3a29',
      textSecondary: '#6b7280',
      border: '#d1fae5',
      sidebarBg: '#064e3b',
      sidebarText: '#d1fae5'
    },
    radius: '10px'
  }
}

export type ThemeName = keyof typeof themes