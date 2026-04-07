import { request } from '@/utils/request'

export interface Role {
  id: number
  name: string
  slug: string
  description: string
  permissions: Permission[]
  user_count: number
  created_at: string
  updated_at: string
}

export interface Permission {
  id: number
  name: string
  slug: string
  module: string
  action: string
  description: string
}

export const roleApi = {
  getList() {
    return request.get<Role[]>('/roles')
  },
  getDetail(id: number) {
    return request.get<Role>(`/roles/${id}`)
  },
  create(data: Partial<Role>) {
    return request.post<Role>('/roles', data)
  },
  update(id: number, data: Partial<Role>) {
    return request.put<Role>(`/roles/${id}`, data)
  },
  delete(id: number) {
    return request.delete(`/roles/${id}`)
  },
  assignPermissions(id: number, permissionIds: number[]) {
    return request.post(`/roles/${id}/permissions`, { permission_ids: permissionIds })
  }
}

export const permissionApi = {
  getList() {
    return request.get<Permission[]>('/permissions')
  },
  getByModule(module: string) {
    return request.get<Permission[]>(`/permissions?module=${module}`)
  }
}
