<template>
  <div class="init-container">
    <div class="init-card">
      <div class="init-icon">🚀</div>
      <h2 class="init-title">系统初始化</h2>
      <p class="init-subtitle">设置站点与管理员</p>
      
      <div v-if="checking" class="init-checking">
        <el-icon class="is-loading"><Loading /></el-icon>
        <p>正在检查系统状态...</p>
      </div>
      
      <div v-else-if="initialized" class="init-already">
        <el-icon color="#67C23A" :size="48"><SuccessFilled /></el-icon>
        <p>系统已初始化</p>
        <el-button type="primary" @click="goToLogin">前往登录</el-button>
      </div>
      
      <el-form
        v-else
        ref="formRef"
        :model="initForm"
        :rules="rules"
        class="init-form"
        @submit.prevent="handleInit"
      >
        <el-form-item prop="siteName">
          <label class="form-label">站点名称</label>
          <el-input
            v-model="initForm.siteName"
            placeholder="我的博客"
            size="large"
          />
        </el-form-item>
        
        <el-form-item prop="username">
          <label class="form-label">管理员用户名</label>
          <el-input
            v-model="initForm.username"
            placeholder="admin"
            size="large"
          />
        </el-form-item>
        
        <el-form-item prop="password">
          <label class="form-label">管理员密码</label>
          <el-input
            v-model="initForm.password"
            type="password"
            placeholder="至少8位"
            size="large"
            show-password
          />
        </el-form-item>
        
        <el-form-item prop="confirmPassword">
          <label class="form-label">确认密码</label>
          <el-input
            v-model="initForm.confirmPassword"
            type="password"
            placeholder="再次输入密码"
            size="large"
            show-password
          />
        </el-form-item>
        
        <el-button
          type="primary"
          size="large"
          :loading="loading"
          class="init-button"
          native-type="submit"
        >
          {{ loading ? '初始化中...' : '完成初始化' }}
        </el-button>
      </el-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { Loading, SuccessFilled } from '@element-plus/icons-vue'
import { initApi } from '@/api/init'

const router = useRouter()

const formRef = ref<FormInstance>()
const loading = ref(false)
const checking = ref(true)
const initialized = ref(false)

const initForm = reactive({
  siteName: '',
  username: '',
  password: '',
  confirmPassword: ''
})

const validatePassword = (_rule: any, value: any, callback: any) => {
  if (value !== initForm.password) {
    callback(new Error('两次输入的密码不一致'))
  } else {
    callback()
  }
}

const rules: FormRules = {
  siteName: [
    { required: true, message: '请输入站点名称', trigger: 'blur' }
  ],
  username: [
    { required: true, message: '请输入管理员用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 8, message: '密码至少8个字符', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    { validator: validatePassword, trigger: 'blur' }
  ]
}

const checkInitStatus = async () => {
  checking.value = true
  try {
    const response = await fetch('/api/init/status', { method: 'POST' })
    const data = await response.json()
    
    if (data.initialized) {
      initialized.value = true
    } else {
      initialized.value = false
    }
  } catch (error) {
    console.error('检查初始化状态失败:', error)
    initialized.value = false
  } finally {
    checking.value = false
  }
}

const handleInit = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    
    loading.value = true
    
    const response = await initApi.createAdmin({
      username: initForm.username,
      password: initForm.password,
      nickname: initForm.username
    })
    
    if (response.success) {
      ElMessage.success('初始化成功！')
      setTimeout(() => {
        router.push('/login')
      }, 1000)
    } else {
      ElMessage.error(response.message || '初始化失败')
    }
  } catch (error) {
    console.error('初始化失败:', error)
    ElMessage.error('初始化失败，请稍后重试')
  } finally {
    loading.value = false
  }
}

const goToLogin = () => {
  router.push('/login')
}

onMounted(() => {
  checkInitStatus()
})
</script>

<style scoped>
.init-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: 20px;
}

.init-card {
  width: 460px;
  background: var(--card-bg);
  border-radius: var(--radius);
  padding: 40px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.1);
}

.init-icon {
  text-align: center;
  font-size: 48px;
  margin-bottom: 12px;
}

.init-title {
  text-align: center;
  font-size: 24px;
  font-weight: 700;
  color: var(--text);
  margin-bottom: 8px;
}

.init-subtitle {
  text-align: center;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 28px;
}

.init-checking,
.init-already {
  text-align: center;
  padding: 40px 0;
}

.init-checking .el-icon {
  font-size: 32px;
  color: var(--primary);
  margin-bottom: 16px;
}

.init-checking p,
.init-already p {
  color: var(--text-secondary);
  margin-bottom: 20px;
}

.init-form {
  text-align: left;
}

.form-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 6px;
  color: var(--text-secondary);
}

.init-form :deep(.el-input__wrapper) {
  border-radius: 8px;
}

.init-form :deep(.el-input__inner) {
  font-size: 14px;
}

.init-button {
  width: 100%;
  height: 44px;
  font-size: 15px;
  font-weight: 600;
  border-radius: 8px;
  margin-top: 8px;
  background: var(--primary);
  border-color: var(--primary);
}

.init-button:hover {
  opacity: 0.9;
}

@media (max-width: 480px) {
  .init-card {
    width: 100%;
    padding: 32px 24px;
  }
}
</style>