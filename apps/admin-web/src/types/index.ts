// 用户相关类型
export interface User {
  id: number
  username: string
  email: string
  nickname?: string
  avatar?: string
  role: string
  created_at: string
  updated_at: string
}

export interface LoginForm {
  username: string
  password: string
  remember?: boolean
}

// 文章相关类型
export interface Post {
  id: number
  title: string
  content: string
  excerpt?: string
  cover?: string
  author_id: number
  category_id?: number
  status: 'draft' | 'published' | 'archived'
  views: number
  created_at: string
  updated_at: string
  author?: User
  category?: Category
  tags?: Tag[]
}

export interface PostForm {
  title: string
  content: string
  excerpt?: string
  cover?: string
  category_id?: number
  tag_ids?: number[]
  status: 'draft' | 'published'
}

// 分类相关类型
export interface Category {
  id: number
  name: string
  slug: string
  description?: string
  parent_id?: number
  post_count: number
  created_at: string
  updated_at: string
}

// 标签相关类型
export interface Tag {
  id: number
  name: string
  slug: string
  post_count: number
  created_at: string
  updated_at: string
}

// 评论相关类型
export interface Comment {
  id: number
  post_id: number
  user_id?: number
  author: string
  email: string
  content: string
  status: 'pending' | 'approved' | 'spam'
  created_at: string
  updated_at: string
  post?: Post
  user?: User
}

// 媒体文件相关类型
export interface Media {
  id: number
  filename: string
  filepath: string
  filesize: number
  mimetype: string
  width?: number
  height?: number
  created_at: string
  updated_at: string
}

// API 响应类型
export interface ApiResponse<T = any> {
  code: number
  message: string
  data: T
}

export interface PaginatedResponse<T> {
  items: T[]
  total: number
  page: number
  per_page: number
  total_pages: number
}

// 分页参数
export interface PaginationParams {
  page?: number
  per_page?: number
}

// 文章查询参数
export interface PostQueryParams extends PaginationParams {
  keyword?: string
  status?: string
  category_id?: number
  tag_id?: number
  author_id?: number
  sort_by?: string
  sort_order?: 'asc' | 'desc'
}
