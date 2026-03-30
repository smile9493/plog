import { request } from '@/utils/request'
import type { Tag, ApiResponse } from '@/types'

export const tagApi = {
  // 获取标签列表
  getList() {
    return request.get<ApiResponse<Tag[]>>('/api/tags')
  },
  
  // 获取所有标签(不分页)
  getAll() {
    return request.get<ApiResponse<Tag[]>>('/api/tags')
      .then(res => res.data || [])
  },
  
  // 获取热门标签
  getPopular(limit: number = 20) {
    return request.get<ApiResponse<Tag[]>>('/api/tags', { 
      params: { popular: true, limit } 
    }).then(res => res.data || [])
  },
  
  // 获取标签详情
  getDetail(id: number) {
    return request.get<ApiResponse<Tag>>(`/api/tags/${id}`)
      .then(res => res.data)
  },
  
  // 创建标签
  create(data: { tagname: string }) {
    return request.post<ApiResponse<Tag>>('/api/tags', data)
      .then(res => res.data)
  },
  
  // 更新标签
  update(id: number, data: { tagname: string }) {
    return request.put<ApiResponse<Tag>>(`/api/tags/${id}`, data)
      .then(res => res.data)
  },
  
  // 删除标签
  delete(id: number) {
    return request.delete<ApiResponse<void>>(`/api/tags/${id}`)
  }
}
