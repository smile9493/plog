<template>
  <div class="settings-page">
    <div class="page-header">
      <h1 class="page-title">⚙️ 系统设置</h1>
    </div>
    
    <div class="settings-tabs">
      <div
        v-for="tab in tabs"
        :key="tab.key"
        :class="['tab-item', { active: activeTab === tab.key }]"
        @click="activeTab = tab.key"
      >
        {{ tab.name }}
      </div>
    </div>
    
    <div class="card">
      <div v-if="activeTab === 'basic'" class="settings-section">
        <h3 class="section-title">基本设置</h3>
        <el-form :model="settings" label-width="120px" class="settings-form">
          <el-form-item label="站点名称">
            <el-input v-model="settings.siteName" placeholder="我的博客" />
          </el-form-item>
          
          <el-form-item label="站点描述">
            <el-input
              v-model="settings.siteDescription"
              type="textarea"
              :rows="2"
              placeholder="站点描述"
            />
          </el-form-item>
          
          <el-form-item label="站点关键词">
            <el-input v-model="settings.siteKeywords" placeholder="博客, 技术, 分享" />
          </el-form-item>
          
          <el-form-item label="站点URL">
            <el-input v-model="settings.siteUrl" placeholder="https://example.com" />
          </el-form-item>
        </el-form>
      </div>
      
      <div v-if="activeTab === 'content'" class="settings-section">
        <h3 class="section-title">内容设置</h3>
        <el-form :model="settings" label-width="120px" class="settings-form">
          <el-form-item label="每页文章数">
            <el-input-number v-model="settings.pageSize" :min="5" :max="50" />
          </el-form-item>
          
          <el-form-item label="文章摘要长度">
            <el-input-number v-model="settings.excerptLength" :min="50" :max="500" />
          </el-form-item>
          
          <el-form-item label="启用评论">
            <el-switch v-model="settings.commentsEnabled" />
          </el-form-item>
          
          <el-form-item label="评论需审核">
            <el-switch v-model="settings.commentsNeedApproval" />
          </el-form-item>
          
          <el-form-item label="启用文章浏览量">
            <el-switch v-model="settings.viewsEnabled" />
          </el-form-item>
        </el-form>
      </div>
      
      <div v-if="activeTab === 'user'" class="settings-section">
        <h3 class="section-title">用户设置</h3>
        <el-form :model="settings" label-width="120px" class="settings-form">
          <el-form-item label="开放注册">
            <el-switch v-model="settings.registrationEnabled" />
          </el-form-item>
          
          <el-form-item label="默认角色">
            <el-select v-model="settings.defaultRole" placeholder="选择默认角色">
              <el-option label="作者" value="author" />
              <el-option label="普通用户" value="user" />
            </el-select>
          </el-form-item>
          
          <el-form-item label="用户名最小长度">
            <el-input-number v-model="settings.usernameMinLength" :min="3" :max="20" />
          </el-form-item>
          
          <el-form-item label="密码最小长度">
            <el-input-number v-model="settings.passwordMinLength" :min="6" :max="20" />
          </el-form-item>
        </el-form>
      </div>
      
      <div v-if="activeTab === 'email'" class="settings-section">
        <h3 class="section-title">邮件设置</h3>
        <el-form :model="settings" label-width="120px" class="settings-form">
          <el-form-item label="SMTP服务器">
            <el-input v-model="settings.smtpHost" placeholder="smtp.example.com" />
          </el-form-item>
          
          <el-form-item label="SMTP端口">
            <el-input-number v-model="settings.smtpPort" :min="1" :max="65535" />
          </el-form-item>
          
          <el-form-item label="邮箱账号">
            <el-input v-model="settings.smtpUser" placeholder="your@email.com" />
          </el-form-item>
          
          <el-form-item label="邮箱密码">
            <el-input v-model="settings.smtpPassword" type="password" show-password />
          </el-form-item>
          
          <el-form-item label="发件人名称">
            <el-input v-model="settings.smtpFromName" placeholder="Plog 博客" />
          </el-form-item>
        </el-form>
      </div>
      
      <div v-if="activeTab === 'seo'" class="settings-section">
        <h3 class="section-title">SEO设置</h3>
        <el-form :model="settings" label-width="120px" class="settings-form">
          <el-form-item label="网站标题">
            <el-input v-model="settings.seoTitle" placeholder="我的博客 - 分享技术与生活" />
          </el-form-item>
          
          <el-form-item label="Meta描述">
            <el-input
              v-model="settings.seoDescription"
              type="textarea"
              :rows="2"
              placeholder="网站描述，用于搜索引擎展示"
            />
          </el-form-item>
          
          <el-form-item label="Meta关键词">
            <el-input v-model="settings.seoKeywords" placeholder="博客, 技术, 编程" />
          </el-form-item>
          
          <el-form-item label="启用Sitemap">
            <el-switch v-model="settings.sitemapEnabled" />
          </el-form-item>
          
          <el-form-item label="启用RSS">
            <el-switch v-model="settings.rssEnabled" />
          </el-form-item>
        </el-form>
      </div>
      
      <div class="form-actions">
        <el-button type="primary" :loading="saving" @click="handleSave">保存设置</el-button>
        <el-button @click="handleReset">重置</el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'

const activeTab = ref('basic')
const saving = ref(false)

const tabs = [
  { key: 'basic', name: '基本设置' },
  { key: 'content', name: '内容设置' },
  { key: 'user', name: '用户设置' },
  { key: 'email', name: '邮件设置' },
  { key: 'seo', name: 'SEO设置' }
]

const settings = reactive({
  siteName: '',
  siteDescription: '',
  siteKeywords: '',
  siteUrl: '',
  pageSize: 10,
  excerptLength: 200,
  commentsEnabled: true,
  commentsNeedApproval: false,
  viewsEnabled: true,
  registrationEnabled: false,
  defaultRole: 'author',
  usernameMinLength: 3,
  passwordMinLength: 8,
  smtpHost: '',
  smtpPort: 587,
  smtpUser: '',
  smtpPassword: '',
  smtpFromName: '',
  seoTitle: '',
  seoDescription: '',
  seoKeywords: '',
  sitemapEnabled: true,
  rssEnabled: true
})

const loadSettings = async () => {
  try {
    // TODO: 从API加载设置
    // const res = await settingsApi.get()
    // Object.assign(settings, res.data)
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

const handleSave = async () => {
  saving.value = true
  try {
    // TODO: 保存设置到API
    // await settingsApi.update(settings)
    await new Promise(resolve => setTimeout(resolve, 500))
    ElMessage.success('设置已保存')
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}

const handleReset = () => {
  ElMessage.info('已重置为默认值')
  loadSettings()
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.settings-page {
  max-width: 1000px;
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

.settings-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 24px;
  background: var(--card-bg);
  padding: 8px;
  border-radius: var(--radius);
}

.tab-item {
  padding: 10px 20px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  transition: all 0.2s ease;
}

.tab-item:hover {
  color: var(--text);
  background: #f8fafc;
}

.tab-item.active {
  color: #fff;
  background: var(--primary);
}

.card {
  background: var(--card-bg);
  border-radius: var(--radius);
  padding: 32px;
}

.settings-section {
  margin-bottom: 24px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 20px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border);
}

.settings-form {
  max-width: 600px;
}

.form-actions {
  margin-top: 24px;
  padding-top: 24px;
  border-top: 1px solid var(--border);
  display: flex;
  gap: 12px;
}
</style>