import { request } from '@/utils/request'
import type { Comment, ApiResponse } from '@/types'

export interface CommentListParams {
  page?: number
  per_page?: number
  post_id?: number | string
  status?: string
}

export interface CommentListResponse {
  items: Comment[]
  pagination: {
    page: number
    per_page: number
    total: number
    total_pages: number
    has_more: boolean
  }
}

export const commentApi = {
  // 获取评论列表
  getList(params?: CommentListParams) {
    return request.get<ApiResponse<CommentListResponse>>('/comments', { params })
  },

  // 获取评论详情
  getDetail(id: number) {
    return request.get<ApiResponse<Comment>>(`/comments/${id}`)
  },

  // 创建评论
  create(data: Partial<Comment>) {
    return request.post<ApiResponse<Comment>>('/comments', data)
  },

  // 更新评论
  update(id: number, data: Partial<Comment>) {
    return request.put<ApiResponse<Comment>>(`/comments/${id}`, data)
  },

  // 删除评论
  delete(id: number) {
    return request.delete<ApiResponse<void>>(`/comments/${id}`)
  },

  // 审核通过
  approve(id: number) {
    return request.post<ApiResponse<void>>(`/comments/${id}/approve`)
  },

  // 拒绝评论
  reject(id: number) {
    return request.put<ApiResponse<void>>(`/comments/${id}`, { hide: 'spam' })
  },

  // 批量通过
  batchApprove(ids: number[]) {
    return request.post<ApiResponse<void>>('/comments/batch-approve', { ids })
  },

  // 批量删除
  batchDelete(ids: number[]) {
    return request.post<ApiResponse<void>>('/comments/batch-delete', { ids })
  }
}
