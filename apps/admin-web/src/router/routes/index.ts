import type { RouteRecordRaw } from 'vue-router'

export const constantRoutes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/login/index.vue'),
    meta: {
      title: '登录',
      hidden: true
    }
  },
  {
    path: '/404',
    name: '404',
    component: () => import('@/views/error/404.vue'),
    meta: {
      title: '404',
      hidden: true
    }
  },
  {
    path: '/init',
    name: 'Init',
    component: () => import('@/views/init/index.vue'),
    meta: {
      title: '初始化',
      hidden: true
    }
  }
]

export const asyncRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/layouts/MainLayout.vue'),
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('@/views/dashboard/index.vue'),
        meta: {
          title: '仪表盘',
          icon: 'Odometer'
        }
      },
      {
        path: 'website',
        name: 'Website',
        component: () => import('@/views/website/index.vue'),
        meta: {
          title: '前台网站',
          icon: 'Monitor'
        }
      },
      {
        path: 'posts',
        name: 'Posts',
        component: () => import('@/views/posts/index.vue'),
        meta: {
          title: '文章管理',
          icon: 'Document'
        }
      },
      {
        path: 'categories',
        name: 'Categories',
        component: () => import('@/views/categories/index.vue'),
        meta: {
          title: '分类管理',
          icon: 'Folder'
        }
      },
      {
        path: 'tags',
        name: 'Tags',
        component: () => import('@/views/tags/index.vue'),
        meta: {
          title: '标签管理',
          icon: 'PriceTag'
        }
      },
      {
        path: 'comments',
        name: 'Comments',
        component: () => import('@/views/comments/index.vue'),
        meta: {
          title: '评论管理',
          icon: 'ChatDotSquare'
        }
      },
      {
        path: 'users',
        name: 'Users',
        component: () => import('@/views/users/index.vue'),
        meta: {
          title: '用户管理',
          icon: 'User'
        }
      },
      {
        path: 'plugins',
        name: 'Plugins',
        component: () => import('@/views/plugins/index.vue'),
        meta: {
          title: '插件管理',
          icon: 'Connection'
        }
      },
      {
        path: 'settings',
        name: 'Settings',
        component: () => import('@/views/settings/index.vue'),
        meta: {
          title: '系统设置',
          icon: 'Setting'
        }
      }
    ]
  }
]