<template>
  <div class="system-setting">
    <el-tabs v-model="activeTab">
      <!-- 基本设置 -->
      <el-tab-pane label="基本设置" name="basic">
        <el-card>
          <el-form
            ref="basicFormRef"
            :model="basicForm"
            :rules="basicRules"
            label-width="120px"
            class="setting-form"
          >
            <el-form-item label="站点名称" prop="site_name">
              <el-input v-model="basicForm.site_name" placeholder="请输入站点名称" />
            </el-form-item>
            <el-form-item label="站点描述" prop="site_description">
              <el-input
                v-model="basicForm.site_description"
                type="textarea"
                :rows="3"
                placeholder="请输入站点描述"
              />
            </el-form-item>
            <el-form-item label="站点URL" prop="site_url">
              <el-input v-model="basicForm.site_url" placeholder="请输入站点URL" />
            </el-form-item>
            <el-form-item label="管理员邮箱" prop="admin_email">
              <el-input v-model="basicForm.admin_email" placeholder="请输入管理员邮箱" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleSaveBasic" :loading="saving">
                保存设置
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-tab-pane>

      <!-- SEO设置 -->
      <el-tab-pane label="SEO设置" name="seo">
        <el-card>
          <el-form
            ref="seoFormRef"
            :model="seoForm"
            label-width="120px"
            class="setting-form"
          >
            <el-form-item label="SEO标题">
              <el-input v-model="seoForm.seo_title" placeholder="请输入SEO标题" />
            </el-form-item>
            <el-form-item label="SEO关键词">
              <el-input
                v-model="seoForm.seo_keywords"
                type="textarea"
                :rows="3"
                placeholder="请输入SEO关键词，多个关键词用逗号分隔"
              />
            </el-form-item>
            <el-form-item label="SEO描述">
              <el-input
                v-model="seoForm.seo_description"
                type="textarea"
                :rows="3"
                placeholder="请输入SEO描述"
              />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleSaveSeo" :loading="saving">
                保存设置
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-tab-pane>

      <!-- 邮件设置 -->
      <el-tab-pane label="邮件设置" name="email">
        <el-card>
          <el-form
            ref="emailFormRef"
            :model="emailForm"
            label-width="120px"
            class="setting-form"
          >
            <el-form-item label="SMTP服务器">
              <el-input v-model="emailForm.smtp_host" placeholder="smtp.example.com" />
            </el-form-item>
            <el-form-item label="SMTP端口">
              <el-input-number v-model="emailForm.smtp_port" :min="1" :max="65535" />
            </el-form-item>
            <el-form-item label="SMTP用户名">
              <el-input v-model="emailForm.smtp_username" placeholder="请输入SMTP用户名" />
            </el-form-item>
            <el-form-item label="SMTP密码">
              <el-input
                v-model="emailForm.smtp_password"
                type="password"
                placeholder="请输入SMTP密码"
                show-password
              />
            </el-form-item>
            <el-form-item label="发件人邮箱">
              <el-input v-model="emailForm.from_email" placeholder="noreply@example.com" />
            </el-form-item>
            <el-form-item label="发件人名称">
              <el-input v-model="emailForm.from_name" placeholder="Plog CMS" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleSaveEmail" :loading="saving">
                保存设置
              </el-button>
              <el-button @click="handleTestEmail" :loading="testing">
                发送测试邮件
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-tab-pane>

      <!-- 上传设置 -->
      <el-tab-pane label="上传设置" name="upload">
        <el-card>
          <el-form
            ref="uploadFormRef"
            :model="uploadForm"
            label-width="120px"
            class="setting-form"
          >
            <el-form-item label="最大上传大小">
              <el-input-number
                v-model="uploadForm.max_upload_size"
                :min="1"
                :max="100"
              />
              <span style="margin-left: 10px;">MB</span>
            </el-form-item>
            <el-form-item label="允许的文件类型">
              <el-input
                v-model="uploadForm.allowed_types"
                placeholder="jpg,jpeg,png,gif,webp"
              />
            </el-form-item>
            <el-form-item label="图片压缩">
              <el-switch v-model="uploadForm.image_compress" />
            </el-form-item>
            <el-form-item v-if="uploadForm.image_compress" label="压缩质量">
              <el-slider v-model="uploadForm.compress_quality" :min="1" :max="100" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleSaveUpload" :loading="saving">
                保存设置
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-tab-pane>

      <!-- 系统信息 -->
      <el-tab-pane label="系统信息" name="system">
        <el-card>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="系统版本">1.0.0</el-descriptions-item>
            <el-descriptions-item label="PHP版本">8.0</el-descriptions-item>
            <el-descriptions-item label="数据库版本">MySQL 8.0</el-descriptions-item>
            <el-descriptions-item label="服务器系统">Linux</el-descriptions-item>
            <el-descriptions-item label="安装时间">2026-03-28</el-descriptions-item>
            <el-descriptions-item label="最后更新">2026-03-28</el-descriptions-item>
          </el-descriptions>

          <div style="margin-top: 20px;">
            <el-button type="primary" @click="handleClearCache">
              <el-icon><Delete /></el-icon>
              清除缓存
            </el-button>
            <el-button type="warning" @click="handleBackup">
              <el-icon><Download /></el-icon>
              数据备份
            </el-button>
          </div>
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { Delete, Download } from '@element-plus/icons-vue'

// 当前标签
const activeTab = ref('basic')

// 保存状态
const saving = ref(false)
const testing = ref(false)

// 基本设置表单
const basicFormRef = ref<FormInstance>()
const basicForm = reactive({
  site_name: 'Plog CMS',
  site_description: '一个简洁的内容管理系统',
  site_url: '',
  admin_email: ''
})

const basicRules: FormRules = {
  site_name: [
    { required: true, message: '请输入站点名称', trigger: 'blur' }
  ],
  site_url: [
    { type: 'url', message: '请输入有效的URL', trigger: 'blur' }
  ],
  admin_email: [
    { type: 'email', message: '请输入有效的邮箱地址', trigger: 'blur' }
  ]
}

// SEO设置表单
const seoForm = reactive({
  seo_title: '',
  seo_keywords: '',
  seo_description: ''
})

// 邮件设置表单
const emailForm = reactive({
  smtp_host: '',
  smtp_port: 587,
  smtp_username: '',
  smtp_password: '',
  from_email: '',
  from_name: 'Plog CMS'
})

// 上传设置表单
const uploadForm = reactive({
  max_upload_size: 5,
  allowed_types: 'jpg,jpeg,png,gif,webp',
  image_compress: false,
  compress_quality: 80
})

// 保存基本设置
const handleSaveBasic = async () => {
  if (!basicFormRef.value) return
  
  await basicFormRef.value.validate(async (valid) => {
    if (valid) {
      saving.value = true
      try {
        // TODO: 调用API保存设置
        await new Promise(resolve => setTimeout(resolve, 500))
        ElMessage.success('保存成功')
      } catch (error) {
      } finally {
        saving.value = false
      }
    }
  })
}

// 保存SEO设置
const handleSaveSeo = async () => {
  saving.value = true
  try {
    // TODO: 调用API保存设置
    await new Promise(resolve => setTimeout(resolve, 500))
    ElMessage.success('保存成功')
  } catch (error) {
  } finally {
    saving.value = false
  }
}

// 保存邮件设置
const handleSaveEmail = async () => {
  saving.value = true
  try {
    // TODO: 调用API保存设置
    await new Promise(resolve => setTimeout(resolve, 500))
    ElMessage.success('保存成功')
  } catch (error) {
  } finally {
    saving.value = false
  }
}

// 发送测试邮件
const handleTestEmail = async () => {
  testing.value = true
  try {
    // TODO: 调用API发送测试邮件
    await new Promise(resolve => setTimeout(resolve, 1000))
    ElMessage.success('测试邮件已发送')
  } catch (error) {
  } finally {
    testing.value = false
  }
}

// 保存上传设置
const handleSaveUpload = async () => {
  saving.value = true
  try {
    // TODO: 调用API保存设置
    await new Promise(resolve => setTimeout(resolve, 500))
    ElMessage.success('保存成功')
  } catch (error) {
  } finally {
    saving.value = false
  }
}

// 清除缓存
const handleClearCache = async () => {
  try {
    // TODO: 调用API清除缓存
    await new Promise(resolve => setTimeout(resolve, 500))
    ElMessage.success('缓存已清除')
  } catch (error) {
  }
}

// 数据备份
const handleBackup = async () => {
  try {
    // TODO: 调用API备份数据
    await new Promise(resolve => setTimeout(resolve, 1000))
    ElMessage.success('备份已开始，完成后将发送邮件通知')
  } catch (error) {
  }
}

// 初始化
onMounted(() => {
  // TODO: 从API加载设置数据
})
</script>

<style scoped lang="scss">
.system-setting {
  .setting-form {
    max-width: 600px;
  }
}
</style>
