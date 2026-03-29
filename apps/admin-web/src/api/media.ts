import { request } from '@/utils/request'
import type { Media, PaginatedResponse, PaginationParams } from '@/types'

export const mediaApi = {
  // 获取媒体列表
  getList(params?: PaginationParams & { type?: string }) {
    return request.get<PaginatedResponse<Media>>('/media', params)
  },
  
  // 获取媒体详情
  getDetail(id: number) {
    return request.get<Media>(`/media/${id}`)
  },
  
  // 删除媒体
  delete(id: number) {
    return request.delete(`/media/${id}`)
  },
  
  // 批量删除媒体
  batchDelete(ids: number[]) {
    return request.post('/media/batch-delete', { ids })
  },
  
  // 更新媒体信息
  update(id: number, data: Partial<Media>) {
    return request.put<Media>(`/media/${id}`, data)
  }
}
