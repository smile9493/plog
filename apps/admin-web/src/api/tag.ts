import { request } from '@/utils/request'

export const tagApi = {
  getList() {
    return request.get('/tags')
  },
  
  getAll() {
    return request.get('/tags')
      .then(res => res.data || [])
  },
  
  getPopular(limit: number = 20) {
    return request.get('/tags', { popular: true, limit })
      .then(res => res.data || [])
  },
  
  getDetail(id: number) {
    return request.get(`/tags/${id}`)
      .then(res => res.data)
  },
  
  create(data: { tagname: string }) {
    return request.post('/tags', data)
      .then(res => res.data)
  },
  
  update(id: number, data: { tagname: string }) {
    return request.put(`/tags/${id}`, data)
      .then(res => res.data)
  },
  
  delete(id: number) {
    return request.delete(`/tags/${id}`)
  }
}
