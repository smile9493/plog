import request from '@/utils/request'

export const initApi = {
  createAdmin(data: {
    username: string
    password: string
    nickname: string
  }) {
    return request.post('/init/create-admin', data)
  }
}