import { request } from '@/utils/request'
import type { PaginatedResponse, PaginationParams } from '@/types'

export interface Theme {
  name: string
  version: string
  description: string
  author: string
  active: boolean
  installed: boolean
  has_update: boolean
  screenshot?: string
  config?: Record<string, any>
}

export interface ThemeDetail extends Theme {
  entry: string
  templates: string[]
  assets: {
    css: string[]
    js: string[]
  }
}

export const themeApi = {
  // 获取主题列表
  getList(params?: PaginationParams) {
    return request.get<PaginatedResponse<Theme>>('/themes', params)
  },
  
  // 获取主题详情
  getDetail(name: string) {
    return request.get<ThemeDetail>(`/themes/${name}`)
  },
  
  // 安装主题
  install(data: FormData) {
    return request.post<{ name: string; message: string }>('/themes/install', data, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })
  },
  
  // 卸载主题
  uninstall(name: string) {
    return request.delete(`/themes/${name}`)
  },
  
  // 激活主题
  activate(name: string) {
    return request.post(`/themes/${name}/activate`)
  },
  
  // 更新主题配置
  updateConfig(name: string, config: Record<string, any>) {
    return request.put(`/themes/${name}/config`, config)
  },
  
  // 获取主题配置
  getConfig(name: string) {
    return request.get<Record<string, any>>(`/themes/${name}/config`)
  }
}
