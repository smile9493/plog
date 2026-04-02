import { request } from '@/utils/request'
import type { Category, ApiResponse } from '@/types'

export const categoryApi = {
  // 获取分类列表
  getList() {
    return request.get<ApiResponse<Category[]>>('/categories')
  },
  
  // 获取所有分类(不分页)
  getAll() {
    return request.get<ApiResponse<Category[]>>('/categories')
      .then(res => res.data || [])
  },
  
  // 获取分类详情
  getDetail(id: number) {
    return request.get<ApiResponse<Category>>(`/categories/${id}`)
      .then(res => res.data)
  },
  
  // 创建分类
  create(data: Partial<Category>) {
    return request.post<ApiResponse<Category>>('/categories', data)
      .then(res => res.data)
  },
  
  // 更新分类
  update(id: number, data: Partial<Category>) {
    return request.put<ApiResponse<Category>>(`/categories/${id}`, data)
      .then(res => res.data)
  },
  
  // 删除分类
  delete(id: number) {
    return request.delete<ApiResponse<void>>(`/categories/${id}`)
  },
  
  // 更新排序
  updateSort(data: { sid: number; sortorder: number }[]) {
    return request.put<ApiResponse<void>>('/categories/sort', data)
  }
}
