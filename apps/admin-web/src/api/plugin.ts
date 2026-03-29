import { request } from '@/utils/request'
import type { PaginatedResponse, PaginationParams } from '@/types'

export interface Plugin {
  name: string
  version: string
  description: string
  author: string
  enabled: boolean
  installed: boolean
  has_update: boolean
  config?: Record<string, any>
}

export interface PluginDetail extends Plugin {
  entry: string
  dependencies: Record<string, string>
  permissions: string[]
  hooks: {
    actions: string[]
    filters: string[]
  }
  routes: any[]
  menus: any[]
}

export const pluginApi = {
  // 获取插件列表
  getList(params?: PaginationParams) {
    return request.get<PaginatedResponse<Plugin>>('/plugins', params)
  },
  
  // 获取插件详情
  getDetail(name: string) {
    return request.get<PluginDetail>(`/plugins/${name}`)
  },
  
  // 安装插件
  install(data: FormData) {
    return request.post<{ name: string; message: string }>('/plugins/install', data, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })
  },
  
  // 卸载插件
  uninstall(name: string) {
    return request.delete(`/plugins/${name}`)
  },
  
  // 启用插件
  enable(name: string) {
    return request.post(`/plugins/${name}/enable`)
  },
  
  // 禁用插件
  disable(name: string) {
    return request.post(`/plugins/${name}/disable`)
  },
  
  // 更新插件配置
  updateConfig(name: string, config: Record<string, any>) {
    return request.put(`/plugins/${name}/config`, config)
  },
  
  // 获取插件配置
  getConfig(name: string) {
    return request.get<Record<string, any>>(`/plugins/${name}/config`)
  }
}
