<template>
  <div class="website-page">
    <div class="page-header">
      <h1 class="page-title">🌐 前台网站管理</h1>
    </div>
    
    <div class="card">
      <div class="card-title">📊 网站统计</div>
      <div class="stats-grid">
        <div class="stat-item">
          <span class="stat-label">文章总数</span>
          <span class="stat-value">{{ stats.posts }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">评论总数</span>
          <span class="stat-value">{{ stats.comments }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">访问量</span>
          <span class="stat-value">{{ stats.views }}</span>
        </div>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">⚙️ 网站配置</div>
      <el-form :model="config" label-width="120px" class="config-form">
        <el-form-item label="网站标题">
          <el-input v-model="config.title" placeholder="我的博客" />
        </el-form-item>
        <el-form-item label="网站副标题">
          <el-input v-model="config.subtitle" placeholder="一个简约的博客" />
        </el-form-item>
        <el-form-item label="网站关键词">
          <el-input v-model="config.keywords" placeholder="博客, 技术, 分享" />
        </el-form-item>
        <el-form-item label="网站描述">
          <el-input
            v-model="config.description"
            type="textarea"
            :rows="3"
            placeholder="网站描述"
          />
        </el-form-item>
        <el-form-item label="页脚信息">
          <el-input
            v-model="config.footer"
            type="textarea"
            :rows="2"
            placeholder="版权信息"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="saving" @click="handleSave">保存配置</el-button>
        </el-form-item>
      </el-form>
    </div>
    
    <div class="card">
      <div class="card-title">🔗 快速访问</div>
      <div class="quick-links">
        <a :href="frontendUrl" target="_blank" class="quick-link">
          <span class="link-icon">🏠</span>
          <span class="link-text">访问前台首页</span>
        </a>
        <a :href="`${frontendUrl}/posts`" target="_blank" class="quick-link">
          <span class="link-icon">📄</span>
          <span class="link-text">查看所有文章</span>
        </a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'

const frontendUrl = 'http://localhost:8082'
const saving = ref(false)

const stats = reactive({
  posts: 0,
  comments: 0,
  views: '0'
})

const config = reactive({
  title: '',
  subtitle: '',
  keywords: '',
  description: '',
  footer: ''
})

const handleSave = async () => {
  saving.value = true
  try {
    // TODO: 保存配置到API
    await new Promise(resolve => setTimeout(resolve, 500))
    ElMessage.success('配置已保存')
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}

const loadStats = async () => {
  // TODO: 从API加载统计数据
  stats.posts = 0
  stats.comments = 0
  stats.views = '0'
}

const loadConfig = async () => {
  // TODO: 从API加载配置
}

onMounted(() => {
  loadStats()
  loadConfig()
})
</script>

<style scoped>
.website-page {
  max-width: 1300px;
}

.page-header {
  background: var(--card-bg);
  padding: 20px;
  border-radius: var(--radius);
  margin-bottom: 24px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--primary);
  margin: 0;
}

.card {
  background: var(--card-bg);
  border-radius: var(--radius);
  padding: 24px;
  margin-bottom: 24px;
}

.card-title {
  font-size: 17px;
  font-weight: 700;
  margin-bottom: 16px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 20px;
  background: #f8fafc;
  border-radius: 12px;
  text-align: center;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 13px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--text);
}

.config-form {
  max-width: 600px;
}

.quick-links {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.quick-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: #f8fafc;
  border-radius: 8px;
  text-decoration: none;
  color: var(--text);
  transition: 0.2s;
}

.quick-link:hover {
  background: #f1f5f9;
  transform: translateY(-2px);
}

.link-icon {
  font-size: 24px;
}

.link-text {
  font-size: 14px;
  font-weight: 500;
}

@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .quick-links {
    grid-template-columns: 1fr;
  }
}
</style>