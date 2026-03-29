import { request } from '@/utils/request'
import type { Post, PostForm, PostQueryParams, PaginatedResponse } from '@/types'

export const postApi = {
  // 获取文章列表
  getList(params?: PostQueryParams) {
    return request.get<PaginatedResponse<Post>>('/posts', params)
  },
  
  // 获取文章详情
  getDetail(id: number) {
    return request.get<Post>(`/posts/${id}`)
  },
  
  // 创建文章
  create(data: PostForm) {
    return request.post<Post>('/posts', data)
  },
  
  // 更新文章
  update(id: number, data: Partial<PostForm>) {
    return request.put<Post>(`/posts/${id}`, data)
  },
  
  // 删除文章
  delete(id: number) {
    return request.delete(`/posts/${id}`)
  },
  
  // 批量删除文章
  batchDelete(ids: number[]) {
    return request.post('/posts/batch-delete', { ids })
  },
  
  // 发布文章
  publish(id: number) {
    return request.put(`/posts/${id}/publish`)
  },
  
  // 归档文章
  archive(id: number) {
    return request.put(`/posts/${id}/archive`)
  },
  
  // 批量发布
  batchPublish(ids: number[]) {
    return request.post('/posts/batch-publish', { ids })
  },
  
  // 批量归档
  batchArchive(ids: number[]) {
    return request.post('/posts/batch-archive', { ids })
  }
}
