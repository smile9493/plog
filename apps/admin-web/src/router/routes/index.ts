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
  }
]

export const asyncRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/layouts/MainLayout.vue'),
    redirect: '/dashboard',
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
        path: 'post',
        name: 'Post',
        redirect: '/post/list',
        meta: {
          title: '文章管理',
          icon: 'Document'
        },
        children: [
          {
            path: 'list',
            name: 'PostList',
            component: () => import('@/views/post/list.vue'),
            meta: {
              title: '文章列表',
              icon: 'List'
            }
          },
          {
            path: 'create',
            name: 'PostCreate',
            component: () => import('@/views/post/edit.vue'),
            meta: {
              title: '写文章',
              icon: 'Edit',
              hidden: true
            }
          },
          {
            path: 'edit/:id',
            name: 'PostEdit',
            component: () => import('@/views/post/edit.vue'),
            meta: {
              title: '编辑文章',
              icon: 'Edit',
              hidden: true
            }
          }
        ]
      },
      {
        path: 'category',
        name: 'Category',
        component: () => import('@/views/category/index.vue'),
        meta: {
          title: '分类管理',
          icon: 'Folder'
        }
      },
      {
        path: 'tag',
        name: 'Tag',
        component: () => import('@/views/tag/index.vue'),
        meta: {
          title: '标签管理',
          icon: 'PriceTag'
        }
      },
      {
        path: 'media',
        name: 'Media',
        component: () => import('@/views/media/index.vue'),
        meta: {
          title: '媒体库',
          icon: 'Picture'
        }
      },
      {
        path: 'comment',
        name: 'Comment',
        component: () => import('@/views/comment/index.vue'),
        meta: {
          title: '评论管理',
          icon: 'ChatDotSquare'
        }
      },
      {
        path: 'plugin',
        name: 'Plugin',
        component: () => import('@/views/plugin/index.vue'),
        meta: {
          title: '插件管理',
          icon: 'Connection'
        }
      },
      {
        path: 'theme',
        name: 'Theme',
        component: () => import('@/views/theme/index.vue'),
        meta: {
          title: '主题管理',
          icon: 'Brush'
        }
      },
      {
        path: 'user',
        name: 'User',
        redirect: '/user/list',
        meta: {
          title: '用户管理',
          icon: 'User'
        },
        children: [
          {
            path: 'list',
            name: 'UserList',
            component: () => import('@/views/user/list.vue'),
            meta: {
              title: '用户列表',
              icon: 'UserFilled'
            }
          },
          {
            path: 'profile',
            name: 'UserProfile',
            component: () => import('@/views/user/profile.vue'),
            meta: {
              title: '个人中心',
              icon: 'UserFilled',
              hidden: true
            }
          }
        ]
      },
      {
        path: 'setting',
        name: 'Setting',
        component: () => import('@/views/setting/index.vue'),
        meta: {
          title: '系统设置',
          icon: 'Setting'
        }
      }
    ]
  }
]
