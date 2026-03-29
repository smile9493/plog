import { request } from '@/utils/request'
import type { Tag, PaginatedResponse, PaginationParams } from '@/types'

export const tagApi = {
  // 获取标签列表
  getList(params?: PaginationParams) {
    return request.get<PaginatedResponse<Tag>>('/tags', params)
  },
  
  // 获取所有标签(不分页)
  getAll() {
    return request.get<Tag[]>('/tags/all')
  },
  
  // 获取标签详情
  getDetail(id: number) {
    return request.get<Tag>(`/tags/${id}`)
  },
  
  // 创建标签
  create(data: Partial<Tag>) {
    return request.post<Tag>('/tags', data)
  },
  
  // 更新标签
  update(id: number, data: Partial<Tag>) {
    return request.put<Tag>(`/tags/${id}`, data)
  },
  
  // 删除标签
  delete(id: number) {
    return request.delete(`/tags/${id}`)
  },
  
  // 批量删除标签
  batchDelete(ids: number[]) {
    return request.post('/tags/batch-delete', { ids })
  },
  
  // 合并标签
  merge(sourceId: number, targetId: number) {
    return request.post('/tags/merge', { source_id: sourceId, target_id: targetId })
  }
}
