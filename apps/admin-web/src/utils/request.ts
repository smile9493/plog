import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosResponse, type AxiosError, type InternalAxiosRequestConfig } from 'axios'
import { ElMessage } from 'element-plus'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

// 配置 NProgress
NProgress.configure({ showSpinner: false })

// 创建 axios 实例
const service: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_PREFIX || '/api',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json;charset=utf-8'
  }
})

// 请求拦截器
service.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    NProgress.start()
    
    // 从 localStorage 获取 token
    const token = localStorage.getItem('token')
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`
    }
    
    return config
  },
  (error: AxiosError) => {
    NProgress.done()
    return Promise.reject(error)
  }
)

// 响应拦截器
service.interceptors.response.use(
  (response: AxiosResponse) => {
    NProgress.done()
    
    const { data } = response
    
    // 如果返回 success: true，说明接口请求成功
    if (data.success === true) {
      return data.data
    }
    
    // 如果返回的状态码为 200 或 0，说明接口请求成功
    if (data.code === 200 || data.code === 0) {
      return data.data
    }
    
    // 其他状态码静默处理
    return Promise.reject(new Error(data.message || data.error?.message || '请求失败'))
  },
  (error: AxiosError) => {
    NProgress.done()
    
    // 处理 HTTP 错误状态码
    if (error.response) {
      const { status, data } = error.response as any
      
      switch (status) {
        case 401:
          ElMessage.error('登录已过期，请重新登录')
          // 清除 token
          localStorage.removeItem('token')
          localStorage.removeItem('user')
          // 跳转到登录页
          window.location.href = '/login'
          break
        default:
          break
      }
    } else if (error.request) {
      // 网络错误静默处理
    } else {
      // 请求配置错误静默处理
    }
    
    return Promise.reject(error)
  }
)

// 封装请求方法
export const request = {
  get<T = any>(url: string, params?: any, config?: AxiosRequestConfig): Promise<T> {
    return service.get(url, { params, ...config })
  },
  
  post<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    return service.post(url, data, config)
  },
  
  put<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    return service.put(url, data, config)
  },
  
  delete<T = any>(url: string, params?: any, config?: AxiosRequestConfig): Promise<T> {
    return service.delete(url, { params, ...config })
  },
  
  patch<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    return service.patch(url, data, config)
  }
}

export default service
