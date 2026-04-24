import { request } from '@/utils/request'
import type { User, LoginForm } from '@/types'

export const authApi = {
  // 用户登录
  login(data: LoginForm) {
    return request.post<{ user: User; expires_in: number }>('/auth/login', data)
  },
  
  // 用户登出
  logout() {
    return request.post('/auth/logout')
  },
  
  // 获取当前用户信息
  getCurrentUser() {
    return request.get<User>('/auth/me')
  },
  
  // 刷新 token
  refreshToken() {
    return request.post<{ token: string }>('/auth/refresh')
  },
  
  // 修改密码
  changePassword(data: { old_password: string; new_password: string }) {
    return request.post('/auth/password', data)
  },
  
  // 修改用户信息
  updateProfile(data: Partial<User>) {
    return request.put<User>('/auth/profile', data)
  }
}
