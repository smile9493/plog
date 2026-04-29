<template>
  <div class="layout-container">
    <aside class="sidebar">
      <div class="logo">
        <span class="logo-icon">📝</span>
        <span class="logo-text">Plog 管理</span>
      </div>
      
      <nav class="nav-menu">
        <router-link
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="nav-item"
          :class="{ active: isActive(item.path) }"
        >
          <span class="nav-icon">{{ item.icon }}</span>
          <span class="nav-text">{{ item.name }}</span>
        </router-link>
      </nav>
      
      <div class="sidebar-footer">
        <div class="theme-switcher">
          <el-dropdown trigger="click" @command="handleThemeChange">
            <span class="theme-trigger">
              <span class="theme-icon">🎨</span>
              <span class="theme-text">{{ currentThemeName }}</span>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item
                  v-for="theme in themes"
                  :key="theme.key"
                  :command="theme.key"
                  :disabled="themeStore.currentTheme === theme.key"
                >
                  {{ theme.name }}
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
        
        <div class="user-info">
          <el-dropdown trigger="click" @command="handleUserCommand">
            <span class="user-trigger">
              <span class="user-avatar">{{ userStore.user?.nickname?.[0] || 'U' }}</span>
              <span class="user-name">{{ userStore.user?.nickname || '用户' }}</span>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="profile">个人设置</el-dropdown-item>
                <el-dropdown-item command="logout" divided>退出登录</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </div>
    </aside>
    
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessageBox, ElMessage } from 'element-plus'
import { useUserStore } from '@/store/modules/user'
import { useThemeStore } from '@/store/modules/theme'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const themeStore = useThemeStore()

const navItems = [
  { path: '/dashboard', name: '仪表盘', icon: '📊' },
  { path: '/website', name: '前台网站', icon: '🌐' },
  { path: '/posts', name: '文章管理', icon: '📄' },
  { path: '/categories', name: '分类管理', icon: '📂' },
  { path: '/tags', name: '标签管理', icon: '🏷️' },
  { path: '/comments', name: '评论管理', icon: '💬' },
  { path: '/users', name: '用户管理', icon: '👥' },
  { path: '/plugins', name: '插件管理', icon: '🔌' },
  { path: '/settings', name: '系统设置', icon: '⚙️' }
]

const themes = computed(() => themeStore.getThemes())
const currentThemeName = computed(() => themeStore.themeConfig.name)

const isActive = (path: string) => {
  return route.path === path || route.path.startsWith(path + '/')
}

const handleThemeChange = (themeName: string) => {
  themeStore.setTheme(themeName as any)
  ElMessage.success(`已切换到${themeStore.themeConfig.name}主题`)
}

const handleUserCommand = async (command: string) => {
  if (command === 'logout') {
    try {
      await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      })
      
      await userStore.logout()
      ElMessage.success('已退出登录')
      router.push('/login')
    } catch {
      // 用户取消
    }
  } else if (command === 'profile') {
    router.push('/profile')
  }
}
</script>

<style scoped>
.layout-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.sidebar {
  width: 240px;
  height: 100vh;
  position: fixed;
  left: 0;
  top: 0;
  background: var(--sidebar-bg);
  color: var(--sidebar-text);
  padding: 24px 0;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  overflow-x: hidden;
  z-index: 100;
}

.logo {
  padding: 0 20px 24px;
  font-size: 18px;
  font-weight: 700;
  color: #fff;
  display: flex;
  align-items: center;
  gap: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  margin-bottom: 12px;
  flex-shrink: 0;
}

.logo-icon {
  font-size: 20px;
}

.nav-menu {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.nav-item {
  padding: 12px 20px;
  margin: 4px 12px;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: 0.15s;
  text-decoration: none;
  color: inherit;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.06);
}

.nav-item.active {
  background: rgba(99, 102, 241, 0.3);
  color: #fff;
  font-weight: 600;
}

.nav-icon {
  font-size: 16px;
}

.sidebar-footer {
  padding: 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  flex-shrink: 0;
}

.theme-switcher {
  margin-bottom: 12px;
}

.theme-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: 0.15s;
}

.theme-trigger:hover {
  background: rgba(255, 255, 255, 0.06);
}

.theme-icon {
  font-size: 16px;
}

.user-info {
  padding-top: 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

.user-trigger {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: 0.15s;
}

.user-trigger:hover {
  background: rgba(255, 255, 255, 0.06);
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--primary);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
}

.user-name {
  font-size: 14px;
  color: #fff;
}

.main-content {
  flex: 1;
  margin-left: 240px;
  padding: 24px;
  background: var(--bg);
  overflow-y: auto;
  height: 100vh;
  box-sizing: border-box;
}

@media (max-width: 768px) {
  .sidebar {
    width: 64px;
    overflow: hidden;
  }
  
  .main-content {
    margin-left: 64px;
  }
  
  .logo-text,
  .nav-text,
  .theme-text,
  .user-name {
    display: none;
  }
  
  .nav-item {
    justify-content: center;
    padding: 12px;
  }
  
  .user-trigger {
    justify-content: center;
  }
}
</style>