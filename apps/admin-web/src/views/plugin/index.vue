<template>
  <div class="plugin-manage">
    <!-- 操作栏 -->
    <el-card class="toolbar-card">
      <el-button type="primary" @click="handleInstall">
        <el-icon><Upload /></el-icon>
        安装插件
      </el-button>
      <el-button @click="handleRefresh">
        <el-icon><Refresh /></el-icon>
        刷新
      </el-button>
    </el-card>

    <!-- 插件列表 -->
    <el-card class="table-card">
      <el-table v-loading="loading" :data="pluginList" style="width: 100%">
        <el-table-column prop="name" label="插件名称" min-width="150">
          <template #default="{ row }">
            <div v-if="row" class="plugin-name">
              <strong>{{ row.name }}</strong>
              <el-tag v-if="row.has_update" type="warning" size="small" style="margin-left: 8px;">
                有更新
              </el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="version" label="版本" width="100" />
        <el-table-column prop="description" label="描述" min-width="200" />
        <el-table-column prop="author" label="作者" width="120" />
        <el-table-column prop="enabled" label="状态" width="100">
          <template #default="{ row }">
            <el-switch
              v-if="row"
              v-model="row.enabled"
              :disabled="!row.installed"
              @change="handleToggleEnable(row)"
            />
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button v-if="row" type="primary" link @click="handleConfig(row)">
              <el-icon><Setting /></el-icon>
              配置
            </el-button>
            <el-button v-if="row" type="danger" link @click="handleUninstall(row)">
              <el-icon><Delete /></el-icon>
              卸载
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 安装对话框 -->
    <el-dialog v-model="installDialogVisible" title="安装插件" width="500px">
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
          拖拽插件ZIP文件到此处或 <em>点击上传</em>
        </div>
        <template #tip>
          <div class="el-upload__tip">
            仅支持ZIP格式,插件包必须包含 plugin.json 文件
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
    <el-dialog v-model="configDialogVisible" title="插件配置" width="600px">
      <el-form v-if="currentPlugin" label-width="120px">
        <el-form-item label="插件名称">
          <el-input :value="currentPlugin.name" disabled />
        </el-form-item>
        <el-form-item label="版本">
          <el-input :value="currentPlugin.version" disabled />
        </el-form-item>
        <el-form-item label="描述">
          <el-input :value="currentPlugin.description" type="textarea" :rows="3" disabled />
        </el-form-item>
        
        <el-divider>插件配置</el-divider>
        
        <el-alert type="info" :closable="false" style="margin-bottom: 20px;">
          此插件暂无可配置项
        </el-alert>
      </el-form>
      <template #footer>
        <el-button @click="configDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Upload, Refresh, Setting, Delete, UploadFilled } from '@element-plus/icons-vue'
import { pluginApi, type Plugin } from '@/api/plugin'

// 加载状态
const loading = ref(false)
const uploading = ref(false)

// 插件列表
const pluginList = ref<Plugin[]>([])

// 对话框
const installDialogVisible = ref(false)
const configDialogVisible = ref(false)
const currentPlugin = ref<Plugin | null>(null)

// 上传文件
const uploadFile = ref<File | null>(null)

// 获取插件列表
const fetchPlugins = async () => {
  loading.value = true
  try {
    // 模拟数据
    pluginList.value = [
      {
        name: 'seo-plugin',
        version: '1.0.0',
        description: 'SEO优化插件 - 提供SEO元数据管理、sitemap生成等功能',
        author: 'Plog Team',
        enabled: true,
        installed: true,
        has_update: false
      },
      {
        name: 'stats-plugin',
        version: '1.0.0',
        description: '统计插件 - 提供访问统计、数据可视化等功能',
        author: 'Plog Team',
        enabled: false,
        installed: true,
        has_update: true
      }
    ]
  } catch (error) {
    ElMessage.error('获取插件列表失败')
  } finally {
    loading.value = false
  }
}

// 刷新列表
const handleRefresh = () => {
  fetchPlugins()
}

// 安装插件
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
    ElMessage.warning('请选择插件文件')
    return
  }

  uploading.value = true
  try {
    const formData = new FormData()
    formData.append('file', uploadFile.value)
    
    await pluginApi.install(formData)
    ElMessage.success('插件安装成功')
    installDialogVisible.value = false
    fetchPlugins()
  } catch (error) {
    ElMessage.error('插件安装失败')
  } finally {
    uploading.value = false
  }
}

// 切换启用状态
const handleToggleEnable = async (plugin: Plugin) => {
  try {
    if (plugin.enabled) {
      await pluginApi.enable(plugin.name)
      ElMessage.success('插件已启用')
    } else {
      await pluginApi.disable(plugin.name)
      ElMessage.success('插件已禁用')
    }
  } catch (error) {
    // 恢复状态
    plugin.enabled = !plugin.enabled
    ElMessage.error('操作失败')
  }
}

// 配置插件
const handleConfig = (plugin: Plugin) => {
  currentPlugin.value = plugin
  configDialogVisible.value = true
}

// 卸载插件
const handleUninstall = async (plugin: Plugin) => {
  try {
    await ElMessageBox.confirm(
      `确定要卸载插件 "${plugin.name}" 吗？卸载后将无法恢复。`,
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    await pluginApi.uninstall(plugin.name)
    ElMessage.success('插件已卸载')
    fetchPlugins()
  } catch (error) {
    // 取消或失败
  }
}

// 初始化
onMounted(() => {
  fetchPlugins()
})
</script>

<style scoped lang="scss">
.plugin-manage {
  .toolbar-card,
  .table-card {
    margin-bottom: 20px;
  }

  .plugin-name {
    display: flex;
    align-items: center;
  }
}
</style>
