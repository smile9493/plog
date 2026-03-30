import { request } from '@/utils/request'
import type { User, LoginForm } from '@/types'

export const authApi = {
  // 用户登录
  login(data: LoginForm) {
    return request.post<{ token: string; user: User }>('/api/auth/login', data)
  },
  
  // 用户登出
  logout() {
    return request.post('/api/auth/logout')
  },
  
  // 获取当前用户信息
  getCurrentUser() {
    return request.get<User>('/api/auth/me')
  },
  
  // 刷新 token
  refreshToken() {
    return request.post<{ token: string }>('/api/auth/refresh')
  },
  
  // 修改密码
  changePassword(data: { old_password: string; new_password: string }) {
    return request.post('/api/auth/password', data)
  },
  
  // 修改用户信息
  updateProfile(data: Partial<User>) {
    return request.put<User>('/api/auth/profile', data)
  }
}
