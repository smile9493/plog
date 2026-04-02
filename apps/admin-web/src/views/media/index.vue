<template>
  <div class="media-manage">
    <!-- 上传区域 -->
    <el-card class="upload-card">
      <el-upload
        :action="uploadUrl"
        :headers="uploadHeaders"
        multiple
        :show-file-list="false"
        :on-success="handleUploadSuccess"
        :on-error="handleUploadError"
        :before-upload="beforeUpload"
        drag
      >
        <el-icon class="el-icon--upload"><UploadFilled /></el-icon>
        <div class="el-upload__text">
          拖拽文件到此处或 <em>点击上传</em>
        </div>
        <template #tip>
          <div class="el-upload__tip">
            支持 jpg、png、gif、webp 格式图片，单个文件大小不超过 5MB
          </div>
        </template>
      </el-upload>
    </el-card>

    <!-- 工具栏 -->
    <el-card class="toolbar-card">
      <el-radio-group v-model="viewMode" @change="handleViewModeChange">
        <el-radio-button label="grid">
          <el-icon><Grid /></el-icon>
          网格视图
        </el-radio-button>
        <el-radio-button label="list">
          <el-icon><List /></el-icon>
          列表视图
        </el-radio-button>
      </el-radio-group>
      
      <el-select
        v-model="filterType"
        placeholder="文件类型"
        clearable
        style="margin-left: 20px; width: 150px;"
        @change="handleFilterChange"
      >
        <el-option label="全部" value="" />
        <el-option label="图片" value="image" />
        <el-option label="视频" value="video" />
        <el-option label="文档" value="document" />
      </el-select>

      <el-button
        type="danger"
        :disabled="selectedIds.length === 0"
        style="margin-left: auto;"
        @click="handleBatchDelete"
      >
        <el-icon><Delete /></el-icon>
        批量删除
      </el-button>
    </el-card>

    <!-- 媒体列表 - 网格视图 -->
    <el-card v-if="viewMode === 'grid'" class="grid-card">
      <div class="media-grid">
        <div
          v-for="media in mediaList"
          :key="media.id"
          class="media-item"
          :class="{ selected: selectedIds.includes(media.id) }"
          @click="handleSelect(media.id)"
        >
          <div class="media-preview">
            <img v-if="isImage(media.mimetype)" :src="media.filepath" :alt="media.filename" />
            <el-icon v-else class="file-icon"><Document /></el-icon>
          </div>
          <div class="media-info">
            <div class="media-name">{{ media.filename }}</div>
            <div class="media-size">{{ formatSize(media.filesize) }}</div>
          </div>
          <div class="media-actions">
            <el-button type="primary" link size="small" @click.stop="handlePreview(media)">
              <el-icon><View /></el-icon>
            </el-button>
            <el-button type="danger" link size="small" @click.stop="handleDelete(media.id)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 媒体列表 - 列表视图 -->
    <el-card v-else class="table-card">
      <el-table
        v-loading="loading"
        :data="mediaList"
        style="width: 100%"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column label="预览" width="100">
          <template #default="{ row }">
            <img
              v-if="row && isImage(row.mimetype)"
              :src="row.filepath"
              :alt="row.filename"
              style="width: 60px; height: 60px; object-fit: cover;"
            />
            <el-icon v-else-if="row" style="font-size: 40px;"><Document /></el-icon>
          </template>
        </el-table-column>
        <el-table-column prop="filename" label="文件名" min-width="200" />
        <el-table-column prop="filesize" label="大小" width="100">
          <template #default="{ row }">
            {{ row ? formatSize(row.filesize) : '' }}
          </template>
        </el-table-column>
        <el-table-column prop="mimetype" label="类型" width="150" />
        <el-table-column prop="created_at" label="上传时间" width="180">
          <template #default="{ row }">
            {{ row ? formatDate(row.created_at) : '' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button v-if="row" type="primary" link @click="handlePreview(row)">
              <el-icon><View /></el-icon>
              预览
            </el-button>
            <el-button v-if="row" type="danger" link @click="handleDelete(row.id)">
              <el-icon><Delete /></el-icon>
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 分页 -->
    <el-pagination
      v-model:current-page="queryParams.page"
      v-model:page-size="queryParams.per_page"
      :page-sizes="[20, 40, 60, 100]"
      :total="total"
      layout="total, sizes, prev, pager, next"
      class="pagination"
      @size-change="fetchMedia"
      @current-change="fetchMedia"
    />

    <!-- 预览对话框 -->
    <el-dialog v-model="previewVisible" title="文件预览" width="800px">
      <div v-if="previewMedia" class="preview-content">
        <img
          v-if="isImage(previewMedia.mimetype)"
          :src="previewMedia.filepath"
          :alt="previewMedia.filename"
          style="max-width: 100%;"
        />
        <div v-else class="file-info">
          <p><strong>文件名:</strong> {{ previewMedia.filename }}</p>
          <p><strong>文件大小:</strong> {{ formatSize(previewMedia.filesize) }}</p>
          <p><strong>文件类型:</strong> {{ previewMedia.mimetype }}</p>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { UploadFilled, Grid, List, Delete, View, Document } from '@element-plus/icons-vue'
import { mediaApi } from '@/api/media'
import type { Media, PaginationParams } from '@/types'
import dayjs from 'dayjs'

// 加载状态
const loading = ref(false)

// 媒体列表
const mediaList = ref<Media[]>([])
const total = ref(0)

// 视图模式
const viewMode = ref<'grid' | 'list'>('grid')

// 筛选类型
const filterType = ref('')

// 选中的媒体ID
const selectedIds = ref<number[]>([])

// 预览
const previewVisible = ref(false)
const previewMedia = ref<Media>()

// 查询参数
const queryParams = reactive<PaginationParams & { type?: string }>({
  page: 1,
  per_page: 20,
  type: undefined
})

// 上传配置
const uploadUrl = computed(() => import.meta.env.VITE_API_BASE_URL + '/api/media/upload')
const uploadHeaders = computed(() => ({
  Authorization: `Bearer ${localStorage.getItem('token')}`
}))

// 获取媒体列表
const fetchMedia = async () => {
  loading.value = true
  try {
    const params = { ...queryParams }
    if (filterType.value) {
      params.type = filterType.value
    }
    const res = await mediaApi.getList(params)
    mediaList.value = res.items
    total.value = res.total
  } catch (error) {
    
  } finally {
    loading.value = false
  }
}

// 上传前验证
const beforeUpload = (file: File) => {
  const isImage = file.type.startsWith('image/')
  const isLt5M = file.size / 1024 / 1024 < 5

  if (!isImage) {
    return false
  }
  if (!isLt5M) {
    return false
  }
  return true
}

// 上传成功
const handleUploadSuccess = (response: any) => {
  if (response.code === 200 || response.code === 0) {
    ElMessage.success('上传成功')
    fetchMedia()
  } else {
  }
}

// 上传失败
const handleUploadError = () => {
}

// 选择媒体
const handleSelect = (id: number) => {
  const index = selectedIds.value.indexOf(id)
  if (index > -1) {
    selectedIds.value.splice(index, 1)
  } else {
    selectedIds.value.push(id)
  }
}

// 选择变化
const handleSelectionChange = (selection: Media[]) => {
  selectedIds.value = selection.map((item) => item.id)
}

// 预览媒体
const handlePreview = (media: Media) => {
  previewMedia.value = media
  previewVisible.value = true
}

// 删除媒体
const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个文件吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await mediaApi.delete(id)
    ElMessage.success('删除成功')
    fetchMedia()
  } catch (error) {
    // 取消或失败
  }
}

// 批量删除
const handleBatchDelete = async () => {
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedIds.value.length} 个文件吗？`, '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await mediaApi.batchDelete(selectedIds.value)
    ElMessage.success('批量删除成功')
    selectedIds.value = []
    fetchMedia()
  } catch (error) {
    // 取消或失败
  }
}

// 视图模式变化
const handleViewModeChange = () => {
  selectedIds.value = []
}

// 筛选变化
const handleFilterChange = () => {
  queryParams.page = 1
  fetchMedia()
}

// 判断是否为图片
const isImage = (mimetype: string) => {
  return mimetype.startsWith('image/')
}

// 格式化文件大小
const formatSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

// 格式化日期
const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

// 初始化
onMounted(() => {
  fetchMedia()
})
</script>

<style scoped lang="scss">
.media-manage {
  .upload-card,
  .toolbar-card,
  .grid-card,
  .table-card {
    margin-bottom: 20px;
  }

  .toolbar-card {
    :deep(.el-card__body) {
      display: flex;
      align-items: center;
    }
  }

  .media-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 16px;

    .media-item {
      border: 1px solid #dcdfe6;
      border-radius: 4px;
      overflow: hidden;
      cursor: pointer;
      transition: all 0.3s;
      position: relative;

      &:hover {
        border-color: #409eff;
        box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
      }

      &.selected {
        border-color: #409eff;
        background-color: #ecf5ff;
      }

      .media-preview {
        width: 100%;
        height: 120px;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #f5f7fa;

        img {
          max-width: 100%;
          max-height: 100%;
          object-fit: contain;
        }

        .file-icon {
          font-size: 48px;
          color: #909399;
        }
      }

      .media-info {
        padding: 8px;
        border-top: 1px solid #ebeef5;

        .media-name {
          font-size: 12px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        .media-size {
          font-size: 12px;
          color: #909399;
          margin-top: 4px;
        }
      }

      .media-actions {
        position: absolute;
        top: 4px;
        right: 4px;
        display: none;
        gap: 4px;
      }

      &:hover .media-actions {
        display: flex;
      }
    }
  }

  .pagination {
    display: flex;
    justify-content: flex-end;
  }

  .preview-content {
    text-align: center;

    .file-info {
      text-align: left;
      padding: 20px;

      p {
        margin: 10px 0;
      }
    }
  }
}
</style>
