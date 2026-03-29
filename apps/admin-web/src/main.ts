import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

// Element Plus - 按需引入样式
import 'element-plus/dist/index.css'

// 样式
import './styles/index.scss'

const app = createApp(App)

// Element Plus 图标 - 按需引入
import {
  Odometer,
  Document,
  List,
  Edit,
  Folder,
  PriceTag,
  Picture,
  ChatDotSquare,
  Connection,
  Brush,
  User,
  UserFilled,
  Setting,
  SwitchButton,
  Expand,
  Fold,
  Upload,
  Refresh,
  Delete,
  Download,
  Plus,
  Search,
  Close,
  Check,
  Warning,
  InfoFilled,
  SuccessFilled,
  CircleCheck,
  CircleClose,
  UploadFilled,
  Lock
} from '@element-plus/icons-vue'

// 注册常用图标
const icons = {
  Odometer,
  Document,
  List,
  Edit,
  Folder,
  PriceTag,
  Picture,
  ChatDotSquare,
  Connection,
  Brush,
  User,
  UserFilled,
  Setting,
  SwitchButton,
  Expand,
  Fold,
  Upload,
  Refresh,
  Delete,
  Download,
  Plus,
  Search,
  Close,
  Check,
  Warning,
  InfoFilled,
  SuccessFilled,
  CircleCheck,
  CircleClose,
  UploadFilled,
  Lock
}

for (const [key, component] of Object.entries(icons)) {
  app.component(key, component)
}

app.use(store)
app.use(router)

app.mount('#app')
