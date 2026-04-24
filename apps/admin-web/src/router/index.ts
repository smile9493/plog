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
const whiteList = ['/login', '/404', '/init']

// 检查系统初始化状态
async function checkInitStatus(): Promise<boolean> {
  try {
    const response = await fetch('/api/init/status', { method: 'POST' })
    const data = await response.json()
    return !data.initialized
  } catch (error) {
    console.error('检查初始化状态失败:', error)
    return false
  }
}

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
    } else if (to.path === '/init') {
      // 已初始化，跳转到首页
      next({ path: '/' })
      NProgress.done()
    } else if (to.path === '/') {
      // 已登录访问根路径，跳转到仪表盘
      next('/dashboard')
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
    } else if (to.path === '/') {
      // 访问根路径，检查是否需要初始化
      try {
        const needsInit = await checkInitStatus()
        if (needsInit) {
          next('/init')
        } else {
          next('/login')
        }
      } catch (error) {
        next('/login')
      }
      NProgress.done()
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
