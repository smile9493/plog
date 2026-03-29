import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'
import { constantRoutes, asyncRoutes } from './routes'
import { useUserStore } from '@/store'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

NProgress.configure({ showSpinner: false })

const router = createRouter({
  history: createWebHashHistory(),
  routes: [...constantRoutes, ...asyncRoutes] as RouteRecordRaw[]
})

// 白名单路由
const whiteList = ['/login', '/404']

// 路由守卫
router.beforeEach(async (to, _from, next) => {
  NProgress.start()
  
  // 设置页面标题
  document.title = `${to.meta.title || 'Plog Admin'} - Plog Admin`
  
  const userStore = useUserStore()
  const hasToken = userStore.token
  
  if (hasToken) {
    if (to.path === '/login') {
      // 已登录，跳转到首页
      next({ path: '/' })
      NProgress.done()
    } else {
      // 判断是否已获取用户信息
      if (userStore.user) {
        next()
      } else {
        try {
          // 获取用户信息
          await userStore.fetchCurrentUser()
          next()
        } catch (error) {
          // 获取用户信息失败，清除 token 并跳转到登录页
          await userStore.logout()
          next(`/login?redirect=${to.path}`)
          NProgress.done()
        }
      }
    }
  } else {
    // 没有 token
    if (whiteList.includes(to.path)) {
      // 在白名单中，直接进入
      next()
    } else {
      // 不在白名单中，跳转到登录页
      next(`/login?redirect=${to.path}`)
      NProgress.done()
    }
  }
})

router.afterEach(() => {
  NProgress.done()
})

export default router
