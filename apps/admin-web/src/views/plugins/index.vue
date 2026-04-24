<template>
  <div class="plugins-page">
    <div class="page-header">
      <h1 class="page-title">🔌 插件管理</h1>
    </div>
    
    <div class="plugins-tabs">
      <div
        v-for="tab in tabs"
        :key="tab.key"
        :class="['tab-item', { active: activeTab === tab.key }]"
        @click="activeTab = tab.key"
      >
        {{ tab.name }}
      </div>
    </div>
    
    <div v-if="activeTab === 'installed'" class="card">
      <div class="toolbar">
        <el-button type="primary" @click="handleInstall">+ 安装插件</el-button>
      </div>
      
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="plugins.length === 0" class="empty-state">
        <div class="empty-icon">📦</div>
        <p>暂无已安装的插件</p>
        <el-button type="primary" @click="handleInstall">安装插件</el-button>
      </div>
      <div v-else class="plugins-grid">
        <div v-for="plugin in plugins" :key="plugin.id" class="plugin-card">
          <div class="plugin-header">
            <div class="plugin-icon">{{ plugin.icon || '🔌' }}</div>
            <div class="plugin-info">
              <h3 class="plugin-name">{{ plugin.name }}</h3>
              <p class="plugin-version">v{{ plugin.version }}</p>
            </div>
            <el-switch
              v-model="plugin.enabled"
              @change="handleToggle(plugin)"
            />
          </div>
          
          <p class="plugin-description">{{ plugin.description }}</p>
          
          <div class="plugin-meta">
            <span class="plugin-author">
              <span class="meta-label">作者：</span>
              {{ plugin.author }}
            </span>
          </div>
          
          <div class="plugin-actions">
            <el-button
              v-if="plugin.hasConfig"
              size="small"
              @click="handleConfig(plugin)"
            >
              配置
            </el-button>
            <el-button
              size="small"
              type="danger"
              @click="handleUninstall(plugin)"
            >
              卸载
            </el-button>
          </div>
        </div>
      </div>
    </div>
    
    <div v-if="activeTab === 'themes'" class="card">
      <div class="section-header">
        <h3 class="section-title">前端主题</h3>
        <p class="section-desc">管理前台网站的主题样式，用户可以在前台自行切换</p>
      </div>
      
      <div class="themes-grid">
        <div
          v-for="theme in themes"
          :key="theme.key"
          :class="['theme-card', { active: theme.enabled }]"
        >
          <div class="theme-preview" :style="{ background: theme.preview }">
            <div class="theme-preview-content">
              <div class="preview-header"></div>
              <div class="preview-body">
                <div class="preview-line"></div>
                <div class="preview-line short"></div>
              </div>
            </div>
          </div>
          <div class="theme-info">
            <h4 class="theme-name">{{ theme.name }}</h4>
            <p class="theme-desc">{{ theme.description }}</p>
            <div class="theme-actions">
              <el-button
                v-if="!theme.enabled"
                type="primary"
                size="small"
                @click="handleEnableTheme(theme)"
              >
                启用
              </el-button>
              <el-button
                v-else
                size="small"
                disabled
              >
                已启用
              </el-button>
              <el-button
                size="small"
                @click="handlePreviewTheme(theme)"
              >
                预览
              </el-button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <div v-if="activeTab === 'market'" class="card">
      <div class="market-header">
        <el-input
          v-model="searchQuery"
          placeholder="搜索插件..."
          class="search-input"
          clearable
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
      </div>
      
      <div class="market-empty">
        <div class="empty-icon">🏪</div>
        <p>插件市场开发中...</p>
        <p class="empty-desc">即将上线，敬请期待</p>
      </div>
    </div>
    
    <el-dialog v-model="installDialogVisible" title="安装插件" width="600px">
      <div class="install-options">
        <div class="install-option" @click="handleUploadPlugin">
          <div class="option-icon">📁</div>
          <div class="option-title">上传插件包</div>
          <div class="option-desc">上传 .zip 或 .tar.gz 格式的插件包</div>
        </div>
        <div class="install-option" @click="handleFromMarket">
          <div class="option-icon">🏪</div>
          <div class="option-title">插件市场</div>
          <div class="option-desc">从官方插件市场浏览和安装</div>
        </div>
      </div>
      <input
        ref="fileInput"
        type="file"
        accept=".zip,.tar.gz"
        style="display: none"
        @change="handleFileSelect"
      />
    </el-dialog>
    
    <el-dialog v-model="configDialogVisible" :title="`配置 - ${currentPlugin?.name}`" width="600px">
      <el-form :model="configForm" label-width="120px">
        <el-form-item
          v-for="(value, key) in configForm"
          :key="key"
          :label="key"
        >
          <el-input
            v-if="typeof value === 'string'"
            v-model="configForm[key]"
          />
          <el-switch
            v-else-if="typeof value === 'boolean'"
            v-model="configForm[key]"
          />
          <el-input-number
            v-else-if="typeof value === 'number'"
            v-model="configForm[key]"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="configDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="savingConfig" @click="handleSaveConfig">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search } from '@element-plus/icons-vue'

const activeTab = ref('installed')
const loading = ref(false)
const installDialogVisible = ref(false)
const configDialogVisible = ref(false)
const savingConfig = ref(false)
const currentPlugin = ref<any>(null)
const fileInput = ref<HTMLInputElement>()
const searchQuery = ref('')

const configForm = reactive<Record<string, any>>({})

const tabs = [
  { key: 'installed', name: '已安装' },
  { key: 'themes', name: '主题管理' },
  { key: 'market', name: '插件市场' }
]

const plugins = ref<any[]>([])

const themes = ref([
  {
    key: 'light',
    name: '明亮主题',
    description: '清爽明亮的默认主题',
    preview: 'linear-gradient(135deg, #fafaf9 0%, #ffffff 100%)',
    enabled: true
  },
  {
    key: 'dark',
    name: '暗色主题',
    description: '护眼的暗色主题',
    preview: 'linear-gradient(135deg, #1c1917 0%, #292524 100%)',
    enabled: true
  },
  {
    key: 'sepia',
    name: '护眼主题',
    description: '温暖的护眼主题',
    preview: 'linear-gradient(135deg, #f5f0e6 0%, #faf6ed 100%)',
    enabled: true
  }
])

const loadPlugins = async () => {
  loading.value = true
  try {
    // TODO: 从API加载插件列表
    // const res = await pluginApi.getList()
    // plugins.value = res.data || []
    plugins.value = []
  } catch (error) {
    console.error('加载插件失败:', error)
  } finally {
    loading.value = false
  }
}

const handleToggle = async (plugin: any) => {
  try {
    // TODO: 调用API启用/禁用插件
    // await pluginApi.toggle(plugin.id, plugin.enabled)
    ElMessage.success(plugin.enabled ? '插件已启用' : '插件已禁用')
  } catch (error) {
    plugin.enabled = !plugin.enabled
    ElMessage.error('操作失败')
  }
}

const handleConfig = async (plugin: any) => {
  currentPlugin.value = plugin
  
  try {
    // TODO: 从API加载插件配置
    // const res = await pluginApi.getConfig(plugin.id)
    // Object.assign(configForm, res.data)
    Object.keys(configForm).forEach(key => delete configForm[key])
  } catch (error) {
    ElMessage.error('加载配置失败')
    return
  }
  
  configDialogVisible.value = true
}

const handleSaveConfig = async () => {
  savingConfig.value = true
  try {
    // TODO: 保存配置到API
    // await pluginApi.updateConfig(currentPlugin.value.id, configForm)
    ElMessage.success('配置已保存')
    configDialogVisible.value = false
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    savingConfig.value = false
  }
}

const handleUninstall = async (plugin: any) => {
  try {
    await ElMessageBox.confirm(
      `确定要卸载插件"${plugin.name}"吗？卸载后相关数据将被删除。`,
      '警告',
      {
        confirmButtonText: '确定卸载',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // TODO: 调用API卸载插件
    // await pluginApi.uninstall(plugin.id)
    plugins.value = plugins.value.filter(p => p.id !== plugin.id)
    ElMessage.success('插件已卸载')
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('卸载失败')
    }
  }
}

const handleInstall = () => {
  installDialogVisible.value = true
}

const handleUploadPlugin = () => {
  fileInput.value?.click()
  installDialogVisible.value = false
}

const handleFromMarket = () => {
  activeTab.value = 'market'
  installDialogVisible.value = false
}

const handleFileSelect = async (e: Event) => {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) return
  
  try {
    // TODO: 上传并安装插件
    // const formData = new FormData()
    // formData.append('file', file)
    // await pluginApi.install(formData)
    ElMessage.success('插件安装成功')
    loadPlugins()
  } catch (error) {
    ElMessage.error('安装失败')
  } finally {
    if (fileInput.value) {
      fileInput.value.value = ''
    }
  }
}

const handleEnableTheme = async (theme: any) => {
  try {
    // TODO: 调用API启用主题
    // await pluginApi.enableTheme(theme.key)
    theme.enabled = true
    ElMessage.success(`已启用${theme.name}`)
  } catch (error) {
    ElMessage.error('启用失败')
  }
}

const handlePreviewTheme = (theme: any) => {
  window.open(`http://localhost:8082?theme=${theme.key}`, '_blank')
}

onMounted(() => {
  loadPlugins()
})
</script>

<style scoped>
.plugins-page {
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

.plugins-tabs {
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
  padding: 24px;
}

.toolbar {
  margin-bottom: 24px;
}

.loading-state,
.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-state p {
  margin-bottom: 20px;
  font-size: 14px;
}

.plugins-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
}

.plugin-card {
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 20px;
  transition: box-shadow 0.2s ease;
}

.plugin-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.plugin-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.plugin-icon {
  width: 48px;
  height: 48px;
  background: #f8fafc;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.plugin-info {
  flex: 1;
}

.plugin-name {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 4px 0;
  color: var(--text);
}

.plugin-version {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

.plugin-description {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0 0 12px 0;
  line-height: 1.6;
}

.plugin-meta {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.meta-label {
  color: var(--text-secondary);
}

.plugin-actions {
  display: flex;
  gap: 8px;
}

.section-header {
  margin-bottom: 24px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 8px 0;
}

.section-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0;
}

.themes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.theme-card {
  border: 2px solid var(--border);
  border-radius: 12px;
  overflow: hidden;
  transition: all 0.2s ease;
}

.theme-card:hover {
  border-color: var(--primary);
}

.theme-card.active {
  border-color: var(--primary);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.theme-preview {
  height: 160px;
  padding: 20px;
  position: relative;
}

.theme-preview-content {
  background: rgba(255, 255, 255, 0.9);
  border-radius: 8px;
  height: 100%;
  padding: 12px;
}

.preview-header {
  height: 20px;
  background: #e5e7eb;
  border-radius: 4px;
  margin-bottom: 12px;
}

.preview-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview-line {
  height: 12px;
  background: #f3f4f6;
  border-radius: 4px;
}

.preview-line.short {
  width: 60%;
}

.theme-info {
  padding: 16px;
  background: #fff;
}

.theme-name {
  font-size: 15px;
  font-weight: 600;
  margin: 0 0 8px 0;
}

.theme-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0 0 12px 0;
}

.theme-actions {
  display: flex;
  gap: 8px;
}

.market-header {
  margin-bottom: 24px;
}

.search-input {
  max-width: 400px;
}

.market-empty {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-secondary);
}

.market-empty .empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.market-empty p {
  margin: 0 0 8px 0;
}

.empty-desc {
  font-size: 13px;
  color: var(--text-secondary);
}

.install-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.install-option {
  padding: 24px;
  border: 2px dashed var(--border);
  border-radius: 12px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.install-option:hover {
  border-color: var(--primary);
  background: rgba(99, 102, 241, 0.02);
}

.option-icon {
  font-size: 32px;
  margin-bottom: 12px;
}

.option-title {
  font-size: 15px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--text);
}

.option-desc {
  font-size: 13px;
  color: var(--text-secondary);
}

@media (max-width: 768px) {
  .plugins-grid {
    grid-template-columns: 1fr;
  }
  
  .themes-grid {
    grid-template-columns: 1fr;
  }
  
  .install-options {
    grid-template-columns: 1fr;
  }
}
</style>