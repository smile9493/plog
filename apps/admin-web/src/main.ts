import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

// Element Plus - 完整引入
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

// Element Plus 图标
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

// 样式
import './styles/index.scss'

const app = createApp(App)

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

// 使用 Element Plus
app.use(ElementPlus)
app.use(store)
app.use(router)

app.mount('#app')
