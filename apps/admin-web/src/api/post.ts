import { request } from '@/utils/request'

export const postApi = {
  getList(params?: Record<string, any>) {
    return request.get('/posts', params)
  },
  
  getDetail(id: number) {
    return request.get(`/posts/${id}`)
  },
  
  create(data: Record<string, any>) {
    return request.post('/posts', data)
  },
  
  update(id: number, data: Record<string, any>) {
    return request.put(`/posts/${id}`, data)
  },
  
  delete(id: number) {
    return request.delete(`/posts/${id}`)
  },
  
  publish(id: number) {
    return request.put(`/posts/${id}`, { hide: 'n' })
  },
  
  archive(id: number) {
    return request.put(`/posts/${id}`, { hide: 'y' })
  }
}
