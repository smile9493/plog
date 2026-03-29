<template>
  <div class="theme-manage">
    <!-- 操作栏 -->
    <el-card class="toolbar-card">
      <el-button type="primary" @click="handleInstall">
        <el-icon><Upload /></el-icon>
        安装主题
      </el-button>
      <el-button @click="handleRefresh">
        <el-icon><Refresh /></el-icon>
        刷新
      </el-button>
    </el-card>

    <!-- 主题列表 -->
    <el-card class="theme-card">
      <div class="theme-grid">
        <div
          v-for="theme in themeList"
          :key="theme.name"
          class="theme-item"
          :class="{ active: theme.active }"
        >
          <div class="theme-preview">
            <img
              v-if="theme.screenshot"
              :src="theme.screenshot"
              :alt="theme.name"
            />
            <div v-else class="no-preview">
              <el-icon><Picture /></el-icon>
              <span>无预览图</span>
            </div>
            <div v-if="theme.active" class="active-badge">
              <el-tag type="success">当前主题</el-tag>
            </div>
          </div>
          
          <div class="theme-info">
            <h3 class="theme-name">{{ theme.name }}</h3>
            <p class="theme-description">{{ theme.description }}</p>
            <div class="theme-meta">
              <span>版本: {{ theme.version }}</span>
              <span>作者: {{ theme.author }}</span>
            </div>
          </div>
          
          <div class="theme-actions">
            <el-button
              v-if="!theme.active"
              type="primary"
              size="small"
              @click="handleActivate(theme)"
            >
              激活
            </el-button>
            <el-button
              v-else
              type="success"
              size="small"
              disabled
            >
              已激活
            </el-button>
            <el-button size="small" @click="handleConfig(theme)">
              <el-icon><Setting /></el-icon>
              配置
            </el-button>
            <el-button
              v-if="!theme.active"
              type="danger"
              size="small"
              @click="handleUninstall(theme)"
            >
              卸载
            </el-button>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 安装对话框 -->
    <el-dialog v-model="installDialogVisible" title="安装主题" width="500px">
      <el-upload
        ref="uploadRef"
        :auto-upload="false"
        :limit="1"
        accept=".zip"
        :on-change="handleFileChange"
        drag
      >
        <el-icon class="el-icon--upload"><UploadFilled /></el-icon>
        <div class="el-upload__text">
          拖拽主题ZIP文件到此处或 <em>点击上传</em>
        </div>
        <template #tip>
          <div class="el-upload__tip">
            仅支持ZIP格式,主题包必须包含 theme.json 文件
          </div>
        </template>
      </el-upload>
      <template #footer>
        <el-button @click="installDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleUpload" :loading="uploading">
          安装
        </el-button>
      </template>
    </el-dialog>

    <!-- 配置对话框 -->
    <el-dialog v-model="configDialogVisible" title="主题配置" width="700px">
      <el-form v-if="currentTheme" label-width="120px">
        <el-form-item label="主题名称">
          <el-input :value="currentTheme.name" disabled />
        </el-form-item>
        <el-form-item label="版本">
          <el-input :value="currentTheme.version" disabled />
        </el-form-item>
        <el-form-item label="描述">
          <el-input :value="currentTheme.description" type="textarea" :rows="3" disabled />
        </el-form-item>
        
        <el-divider>主题配置</el-divider>
        
        <el-tabs>
          <el-tab-pane label="布局设置">
            <el-form-item label="显示侧边栏">
              <el-switch v-model="themeConfig.layout.sidebar" />
            </el-form-item>
            <el-form-item label="显示页脚">
              <el-switch v-model="themeConfig.layout.footer" />
            </el-form-item>
            <el-form-item label="显示页头">
              <el-switch v-model="themeConfig.layout.header" />
            </el-form-item>
          </el-tab-pane>
          
          <el-tab-pane label="颜色设置">
            <el-form-item label="主色调">
              <el-color-picker v-model="themeConfig.colors.primary" />
            </el-form-item>
            <el-form-item label="辅助色">
              <el-color-picker v-model="themeConfig.colors.secondary" />
            </el-form-item>
            <el-form-item label="文字颜色">
              <el-color-picker v-model="themeConfig.colors.text" />
            </el-form-item>
            <el-form-item label="背景颜色">
              <el-color-picker v-model="themeConfig.colors.background" />
            </el-form-item>
          </el-tab-pane>
          
          <el-tab-pane label="文章设置">
            <el-form-item label="每页文章数">
              <el-input-number v-model="themeConfig.posts.per_page" :min="1" :max="50" />
            </el-form-item>
            <el-form-item label="显示摘要">
              <el-switch v-model="themeConfig.posts.show_excerpt" />
            </el-form-item>
            <el-form-item label="显示作者">
              <el-switch v-model="themeConfig.posts.show_author" />
            </el-form-item>
            <el-form-item label="显示日期">
              <el-switch v-model="themeConfig.posts.show_date" />
            </el-form-item>
          </el-tab-pane>
        </el-tabs>
      </el-form>
      <template #footer>
        <el-button @click="configDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSaveConfig" :loading="saving">
          保存
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Upload, Refresh, Setting, Picture, UploadFilled } from '@element-plus/icons-vue'
import { themeApi, type Theme } from '@/api/theme'

// 加载状态
const loading = ref(false)
const uploading = ref(false)
const saving = ref(false)

// 主题列表
const themeList = ref<Theme[]>([])

// 对话框
const installDialogVisible = ref(false)
const configDialogVisible = ref(false)
const currentTheme = ref<Theme | null>(null)

// 上传文件
const uploadFile = ref<File | null>(null)

// 主题配置
const themeConfig = reactive({
  layout: {
    sidebar: true,
    footer: true,
    header: true
  },
  colors: {
    primary: '#409eff',
    secondary: '#67c23a',
    text: '#333333',
    background: '#ffffff'
  },
  posts: {
    per_page: 10,
    show_excerpt: true,
    show_author: true,
    show_date: true
  }
})

// 获取主题列表
const fetchThemes = async () => {
  loading.value = true
  try {
    // 模拟数据
    themeList.value = [
      {
        name: 'default',
        version: '1.0.0',
        description: 'Plog默认主题 - 简洁优雅的博客主题',
        author: 'Plog Team',
        active: true,
        installed: true,
        has_update: false,
        screenshot: '/themes/default/screenshot.png'
      },
      {
        name: 'minimal',
        version: '1.0.0',
        description: '简约主题 - 极简设计,专注内容',
        author: 'Plog Team',
        active: false,
        installed: true,
        has_update: false,
        screenshot: '/themes/minimal/screenshot.png'
      }
    ]
  } catch (error) {
    ElMessage.error('获取主题列表失败')
  } finally {
    loading.value = false
  }
}

// 刷新列表
const handleRefresh = () => {
  fetchThemes()
}

// 安装主题
const handleInstall = () => {
  installDialogVisible.value = true
}

// 文件选择
const handleFileChange = (file: any) => {
  uploadFile.value = file.raw
}

// 上传安装
const handleUpload = async () => {
  if (!uploadFile.value) {
    ElMessage.warning('请选择主题文件')
    return
  }

  uploading.value = true
  try {
    const formData = new FormData()
    formData.append('file', uploadFile.value)
    
    await themeApi.install(formData)
    ElMessage.success('主题安装成功')
    installDialogVisible.value = false
    fetchThemes()
  } catch (error) {
    ElMessage.error('主题安装失败')
  } finally {
    uploading.value = false
  }
}

// 激活主题
const handleActivate = async (theme: Theme) => {
  try {
    await ElMessageBox.confirm(
      `确定要激活主题 "${theme.name}" 吗？`,
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'info'
      }
    )
    
    await themeApi.activate(theme.name)
    ElMessage.success('主题已激活')
    fetchThemes()
  } catch (error) {
    // 取消或失败
  }
}

// 配置主题
const handleConfig = (theme: Theme) => {
  currentTheme.value = theme
  configDialogVisible.value = true
}

// 保存配置
const handleSaveConfig = async () => {
  saving.value = true
  try {
    if (currentTheme.value) {
      await themeApi.updateConfig(currentTheme.value.name, themeConfig)
      ElMessage.success('配置已保存')
      configDialogVisible.value = false
    }
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    saving.value = false
  }
}

// 卸载主题
const handleUninstall = async (theme: Theme) => {
  try {
    await ElMessageBox.confirm(
      `确定要卸载主题 "${theme.name}" 吗？卸载后将无法恢复。`,
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    await themeApi.uninstall(theme.name)
    ElMessage.success('主题已卸载')
    fetchThemes()
  } catch (error) {
    // 取消或失败
  }
}

// 初始化
onMounted(() => {
  fetchThemes()
})
</script>

<style scoped lang="scss">
.theme-manage {
  .toolbar-card,
  .theme-card {
    margin-bottom: 20px;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;

    .theme-item {
      border: 1px solid #dcdfe6;
      border-radius: 8px;
      overflow: hidden;
      transition: all 0.3s;

      &:hover {
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      }

      &.active {
        border-color: #409eff;
        box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
      }

      .theme-preview {
        position: relative;
        width: 100%;
        height: 200px;
        background-color: #f5f7fa;

        img {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }

        .no-preview {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          height: 100%;
          color: #909399;

          .el-icon {
            font-size: 48px;
            margin-bottom: 10px;
          }
        }

        .active-badge {
          position: absolute;
          top: 10px;
          right: 10px;
        }
      }

      .theme-info {
        padding: 15px;

        .theme-name {
          font-size: 18px;
          margin: 0 0 10px;
        }

        .theme-description {
          color: #666;
          font-size: 14px;
          margin: 0 0 10px;
          line-height: 1.5;
        }

        .theme-meta {
          display: flex;
          gap: 15px;
          color: #999;
          font-size: 12px;
        }
      }

      .theme-actions {
        padding: 15px;
        border-top: 1px solid #ebeef5;
        display: flex;
        gap: 10px;
        flex-wrap: wrap;
      }
    }
  }
}
</style>
