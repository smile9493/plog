import request from '@/utils/request'
import type { User, UserListParams, UserListResponse } from '@/types/user'

export const userApi = {
  // 获取用户列表
  getList(params: UserListParams) {
    return request.get<UserListResponse>('/users', { params })
  },

  // 获取用户详情
  getById(id: number) {
    return request.get<User>(`/api/users/${id}`)
  },

  // 创建用户
  create(data: Partial<User> & { password?: string }) {
    return request.post<User>('/users', data)
  },

  // 更新用户
  update(id: number, data: Partial<User>) {
    return request.put<User>(`/api/users/${id}`, data)
  },

  // 删除用户
  delete(id: number) {
    return request.delete(`/api/users/${id}`)
  },

  // 重置密码
  resetPassword(id: number, password: string) {
    return request.post(`/api/users/${id}/reset-password`, { password })
  }
}
