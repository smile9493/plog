# Plog Admin Web

Plog CMS 后台管理前端应用

## 技术栈

- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全
- **Vite** - 下一代构建工具
- **Element Plus** - Vue 3 组件库
- **Pinia** - Vue 3 状态管理
- **Vue Router 4** - 官方路由管理器
- **Axios** - HTTP 请求库

## 开发

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build

# 预览生产构建
npm run preview
```

## 项目结构

```
src/
├── api/               # API 接口封装
├── assets/            # 静态资源
├── components/        # 公共组件
│   ├── common/       # 通用组件
│   └── business/     # 业务组件
├── composables/       # 组合式函数
├── directives/        # 自定义指令
├── layouts/           # 布局组件
├── router/            # 路由配置
├── store/             # 状态管理
├── styles/            # 样式文件
├── types/             # TypeScript 类型定义
├── utils/             # 工具函数
├── views/             # 页面组件
├── App.vue
└── main.ts
```

## 功能模块

- ✅ 用户登录/登出
- ✅ 路由守卫和权限控制
- 🚧 文章管理 (开发中)
- 🚧 分类管理 (开发中)
- 🚧 标签管理 (开发中)
- 🚧 媒体管理 (开发中)
- 🚧 评论管理 (开发中)
- 🚧 用户管理 (开发中)
- 🚧 系统设置 (开发中)

## 环境配置

创建 `.env.development` 文件用于开发环境:

```env
VITE_APP_TITLE=Plog Admin
VITE_API_BASE_URL=http://localhost:8000
VITE_API_PREFIX=/api
```

创建 `.env.production` 文件用于生产环境:

```env
VITE_APP_TITLE=Plog Admin
VITE_API_BASE_URL=
VITE_API_PREFIX=/api
```

## 开发规范

### 组件命名

- 组件文件: PascalCase (如: `PostList.vue`)
- 组件注册: kebab-case (如: `<post-list />`)

### 目录命名

- 目录名: kebab-case (如: `post-list/`)

### 变量命名

- 变量: camelCase (如: `postList`)
- 常量: UPPER_SNAKE_CASE (如: `API_BASE_URL`)
- 类型/接口: PascalCase (如: `PostList`)

### Git 提交规范

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

## 浏览器支持

- Chrome (最新版本)
- Firefox (最新版本)
- Safari (最新版本)
- Edge (最新版本)

## License

MIT
