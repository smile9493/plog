import { request } from '@/utils/request'

export const categoryApi = {
  getList() {
    return request.get('/categories')
  },
  
  getAll() {
    return request.get('/categories')
      .then(res => res.data || [])
  },
  
  getDetail(id: number) {
    return request.get(`/categories/${id}`)
      .then(res => res.data)
  },
  
  create(data: Record<string, any>) {
    return request.post('/categories', data)
      .then(res => res.data)
  },
  
  update(id: number, data: Record<string, any>) {
    return request.put(`/categories/${id}`, data)
      .then(res => res.data)
  },
  
  delete(id: number) {
    return request.delete(`/categories/${id}`)
  },
  
  updateSort(data: { sid: number; sortorder: number }[]) {
    return request.put('/categories/sort', data)
  }
}
