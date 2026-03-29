import { request } from '@/utils/request'
import type { Category, PaginatedResponse, PaginationParams } from '@/types'

export const categoryApi = {
  // 获取分类列表
  getList(params?: PaginationParams) {
    return request.get<PaginatedResponse<Category>>('/categories', params)
  },
  
  // 获取所有分类(不分页)
  getAll() {
    return request.get<Category[]>('/categories/all')
  },
  
  // 获取分类详情
  getDetail(id: number) {
    return request.get<Category>(`/categories/${id}`)
  },
  
  // 创建分类
  create(data: Partial<Category>) {
    return request.post<Category>('/categories', data)
  },
  
  // 更新分类
  update(id: number, data: Partial<Category>) {
    return request.put<Category>(`/categories/${id}`, data)
  },
  
  // 删除分类
  delete(id: number) {
    return request.delete(`/categories/${id}`)
  },
  
  // 批量删除分类
  batchDelete(ids: number[]) {
    return request.post('/categories/batch-delete', { ids })
  },
  
  // 更新排序
  updateSort(data: { id: number; sort: number }[]) {
    return request.put('/categories/sort', data)
  }
}
