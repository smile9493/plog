// 用户类型定义
export interface User {
  uid: number
  username: string
  nickname: string
  email: string
  photo?: string
  description?: string
  role: 'admin' | 'editor' | 'user'
  create_time: number
  avatar?: string
}

export interface UserListParams {
  page?: number
  per_page?: number
  username?: string
  role?: string
}

export interface UserListResponse {
  items: User[]
  pagination: {
    page: number
    per_page: number
    total: number
    total_pages: number
    has_more: boolean
  }
}
