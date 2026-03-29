// 用户相关类型 - 匹配 Rust API
export interface User {
  uid: number
  username: string
  nickname: string
  email: string
  photo?: string
  description?: string
  role: 'admin' | 'editor' | 'user'
  create_time: number
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

export interface LoginForm {
  username: string
  password: string
  remember?: boolean
}

// 文章相关类型
export interface Post {
  gid: number
  title: string
  content: string
  excerpt?: string
  cover?: string
  author: number
  sortid: number
  date: number
  hide: string
  type: string
  views: number
  comnum: number
  like_count: number
  top: string
  sortop: string
  allow_remark: string
  password?: string
  alias?: string
}

export interface PostListParams {
  page?: number
  per_page?: number
  keyword?: string
  category_id?: number
  status?: string
  order?: string
}

export interface PostListResponse {
  items: Post[]
  pagination: {
    page: number
    per_page: number
    total: number
    total_pages: number
    has_more: boolean
  }
}

// 分类相关类型
export interface Category {
  sid: number
  sortname: string
  pid: number
  sortorder: number
  description?: string
  alias?: string
}

// 标签相关类型
export interface Tag {
  tid: number
  tagname: string
  usenum: number
}

// 评论相关类型
export interface Comment {
  cid: number
  gid: number
  pid: number
  content: string
  poster: string
  email: string
  url: string
  ip: string
  date: number
  hide: string
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
}

// API 响应类型
export interface ApiResponse<T = any> {
  success: boolean
  data: T
  error?: {
    code: string
    message: string
  }
  meta?: {
    request_id: string
    timestamp: string
  }
}

// 分页参数
export interface PaginationParams {
  page?: number
  per_page?: number
}
