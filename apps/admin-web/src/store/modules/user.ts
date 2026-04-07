import { defineStore } from 'pinia'
import type { User, LoginForm } from '@/types'
import { authApi } from '@/api/auth'
import { ElMessage } from 'element-plus'

// Cookie 工具函数
function getCookie(name: string): string {
  const match = document.cookie.match(new RegExp('(^| )' + name + '=([^;]+)'))
  return match ? decodeURIComponent(match[2]) : ''
}

function deleteCookie(name: string) {
  document.cookie = name + '=; Path=/; Max-Age=0; SameSite=Strict'
}

interface UserState {
  token: string
  user: User | null
}

export const useUserStore = defineStore('user', {
  state: (): UserState => ({
    token: getCookie('token') || '',
    user: sessionStorage.getItem('user') ? JSON.parse(sessionStorage.getItem('user')!) : null
  }),
  
  getters: {
    isLoggedIn: (state) => !!state.token && !!state.user,
    username: (state) => state.user?.username || '',
    nickname: (state) => state.user?.nickname || state.user?.username || '',
    avatar: (state) => state.user?.avatar || '',
    role: (state) => state.user?.role || 'guest'
  },
  
  actions: {
    async login(loginForm: LoginForm) {
      try {
        const { token, user } = await authApi.login(loginForm)
        
        this.token = token
        this.user = user
        
        // Token 通过 httpOnly cookie 存储，用户信息存储在 sessionStorage
        sessionStorage.setItem('user', JSON.stringify(user))
        
        ElMessage.success('登录成功')
        return true
      } catch (error) {
        return false
      }
    },
    
    async logout() {
      try {
        await authApi.logout()
      } catch (error) {
        // 即使接口失败也清除本地状态
      } finally {
        this.token = ''
        this.user = null
        deleteCookie('token')
        sessionStorage.removeItem('user')
      }
    },
    
    async fetchCurrentUser() {
      try {
        const user = await authApi.getCurrentUser()
        this.user = user
        sessionStorage.setItem('user', JSON.stringify(user))
        return user
      } catch (error) {
        return null
      }
    },
    
    async updateProfile(data: Partial<User>) {
      try {
        const user = await authApi.updateProfile(data)
        this.user = user
        sessionStorage.setItem('user', JSON.stringify(user))
        ElMessage.success('更新成功')
        return true
      } catch (error) {
        return false
      }
    },
    
    async changePassword(data: { old_password: string; new_password: string }) {
      try {
        await authApi.changePassword(data)
        ElMessage.success('密码修改成功，请重新登录')
        await this.logout()
        return true
      } catch (error) {
        return false
      }
    }
  }
})
