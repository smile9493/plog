import { request } from '@/utils/request'
import type { Post, PostListParams, PostListResponse } from '@/types'

export const postApi = {
  // 获取文章列表
  getList(params?: PostListParams) {
    return request.get<PostListResponse>('/api/posts', { params })
  },
  
  // 获取文章详情
  getDetail(id: number) {
    return request.get<Post>(`/api/posts/${id}`)
  },
  
  // 创建文章
  create(data: Partial<Post>) {
    return request.post<Post>('/api/posts', data)
  },
  
  // 更新文章
  update(id: number, data: Partial<Post>) {
    return request.put<Post>(`/api/posts/${id}`, data)
  },
  
  // 删除文章
  delete(id: number) {
    return request.delete(`/api/posts/${id}`)
  },
  
  // 批量删除文章
  batchDelete(ids: number[]) {
    return request.post('/api/posts/batch-delete', { ids })
  },
  
  // 发布文章
  publish(id: number) {
    return request.put(`/api/posts/${id}`, { hide: 'n' })
  },
  
  // 归档文章
  archive(id: number) {
    return request.put(`/api/posts/${id}`, { hide: 'y' })
  },
  
  // 批量发布
  batchPublish(ids: number[]) {
    return request.post('/api/posts/batch-publish', { ids })
  },
  
  // 批量归档
  batchArchive(ids: number[]) {
    return request.post('/api/posts/batch-archive', { ids })
  }
}
