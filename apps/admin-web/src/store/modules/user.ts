import { defineStore } from 'pinia'
import type { User, LoginForm } from '@/types'
import { authApi } from '@/api/auth'
import { ElMessage } from 'element-plus'

interface UserState {
  token: string
  user: User | null
}

export const useUserStore = defineStore('user', {
  state: (): UserState => ({
    token: localStorage.getItem('token') || '',
    user: JSON.parse(localStorage.getItem('user') || 'null')
  }),
  
  getters: {
    isLoggedIn: (state) => !!state.token && !!state.user,
    username: (state) => state.user?.username || '',
    nickname: (state) => state.user?.nickname || state.user?.username || '',
    avatar: (state) => state.user?.avatar || '',
    role: (state) => state.user?.role || 'guest'
  },
  
  actions: {
    // 登录
    async login(loginForm: LoginForm) {
      try {
        const { token, user } = await authApi.login(loginForm)
        
        this.token = token
        this.user = user
        
        // 持久化存储
        localStorage.setItem('token', token)
        localStorage.setItem('user', JSON.stringify(user))
        
        ElMessage.success('登录成功')
        return true
      } catch (error) {
        return false
      }
    },
    
    // 登出
    async logout() {
      try {
        await authApi.logout()
      } catch (error) {
        // 即使接口失败也清除本地状态
      } finally {
        this.token = ''
        this.user = null
        localStorage.removeItem('token')
        localStorage.removeItem('user')
      }
    },
    
    // 获取当前用户信息
    async fetchCurrentUser() {
      try {
        const user = await authApi.getCurrentUser()
        this.user = user
        localStorage.setItem('user', JSON.stringify(user))
        return user
      } catch (error) {
        return null
      }
    },
    
    // 更新用户信息
    async updateProfile(data: Partial<User>) {
      try {
        const user = await authApi.updateProfile(data)
        this.user = user
        localStorage.setItem('user', JSON.stringify(user))
        ElMessage.success('更新成功')
        return true
      } catch (error) {
        return false
      }
    },
    
    // 修改密码
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
