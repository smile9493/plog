<template>
  <div class="comment-list">
    <!-- 搜索和筛选 -->
    <el-card class="search-card">
      <el-form :model="searchForm" inline>
        <el-form-item label="状态">
          <el-select v-model="searchForm.status" placeholder="选择状态" clearable>
            <el-option label="全部" value="" />
            <el-option label="待审核" value="pending" />
            <el-option label="已通过" value="approved" />
            <el-option label="垃圾" value="spam" />
          </el-select>
        </el-form-item>
        <el-form-item label="文章 ID">
          <el-input
            v-model="searchForm.post_id"
            placeholder="文章 ID"
            clearable
            @keyup.enter="handleSearch"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">
            <el-icon><Search /></el-icon>
            搜索
          </el-button>
          <el-button @click="handleReset">
            <el-icon><Refresh /></el-icon>
            重置
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 操作栏 -->
    <el-card class="table-card">
      <template #header>
        <div class="card-header">
          <span>评论列表</span>
          <div>
            <el-button type="success" :disabled="selectedIds.length === 0" @click="handleBatchApprove">
              <el-icon><Check /></el-icon>
              批量通过
            </el-button>
            <el-button type="danger" :disabled="selectedIds.length === 0" @click="handleBatchDelete">
              <el-icon><Delete /></el-icon>
              批量删除
            </el-button>
          </div>
        </div>
      </template>

      <!-- 评论表格 -->
      <el-table
        v-loading="loading"
        :data="commentList"
        border
        stripe
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="cid" label="ID" width="80" />
        <el-table-column label="评论者" width="150">
          <template #default="{ row }">
            <div>
              <div class="font-medium">{{ row.poster }}</div>
              <div class="text-gray-500 text-sm">{{ row.email }}</div>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="content" label="内容" min-width="300">
          <template #default="{ row }">
            <el-tooltip :content="row.content" placement="top">
              <div class="line-clamp-2">{{ row.content }}</div>
            </el-tooltip>
          </template>
        </el-table-column>
        <el-table-column prop="gid" label="文章 ID" width="100" />
        <el-table-column prop="ip" label="IP" width="140" />
        <el-table-column prop="hide" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.hide)">
              {{ getStatusText(row.hide) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="date" label="时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.date) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button
              v-if="row.hide === 'y'"
              type="success"
              link
              @click="handleApprove(row.cid)"
            >
              通过
            </el-button>
            <el-button
              v-if="row.hide === 'n'"
              type="warning"
              link
              @click="handleReject(row.cid)"
            >
              拒绝
            </el-button>
            <el-popconfirm
              title="确定要删除这条评论吗？"
              @confirm="handleDelete(row.cid)"
            >
              <template #reference>
                <el-button type="danger" link>删除</el-button>
              </template>
            </el-popconfirm>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination">
        <el-pagination
          v-model:current-page="pagination.page"
          v-model:page-size="pagination.per_page"
          :page-sizes="[10, 20, 50, 100]"
          :total="pagination.total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Refresh, Check, Delete } from '@element-plus/icons-vue'
import { commentApi } from '@/api/comment'
import type { Comment } from '@/types'

// 搜索表单
const searchForm = reactive({
  status: '',
  post_id: ''
})

// 分页
const pagination = reactive({
  page: 1,
  per_page: 20,
  total: 0
})

// 评论列表
const commentList = ref<Comment[]>([])
const loading = ref(false)
const selectedIds = ref<number[]>([])

// 获取评论列表
const fetchComments = async () => {
  loading.value = true
  try {
    const res = await commentApi.getList({
      page: pagination.page,
      per_page: pagination.per_page,
      ...searchForm
    })
    commentList.value = res.data?.items || []
    pagination.total = res.data?.pagination?.total || 0
  } catch (error) {
    
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  pagination.page = 1
  fetchComments()
}

// 重置
const handleReset = () => {
  searchForm.status = ''
  searchForm.post_id = ''
  handleSearch()
}

// 分页
const handleSizeChange = () => {
  fetchComments()
}

const handlePageChange = () => {
  fetchComments()
}

// 选择
const handleSelectionChange = (selection: Comment[]) => {
  selectedIds.value = selection.map(item => item.cid)
}

// 审核通过
const handleApprove = async (id: number) => {
  try {
    await commentApi.approve(id)
    ElMessage.success('已通过')
    fetchComments()
  } catch (error) {
  }
}

// 拒绝
const handleReject = async (id: number) => {
  try {
    await commentApi.reject(id)
    ElMessage.success('已拒绝')
    fetchComments()
  } catch (error) {
  }
}

// 删除
const handleDelete = async (id: number) => {
  try {
    await commentApi.delete(id)
    ElMessage.success('删除成功')
    fetchComments()
  } catch (error) {
  }
}

// 批量通过
const handleBatchApprove = async () => {
  try {
    await ElMessageBox.confirm(`确定要通过选中的 ${selectedIds.value.length} 条评论吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await commentApi.batchApprove(selectedIds.value)
    ElMessage.success('批量通过成功')
    selectedIds.value = []
    fetchComments()
  } catch (error) {
    if (error !== 'cancel') {
    }
  }
}

// 批量删除
const handleBatchDelete = async () => {
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedIds.value.length} 条评论吗？`, '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await commentApi.batchDelete(selectedIds.value)
    ElMessage.success('批量删除成功')
    selectedIds.value = []
    fetchComments()
  } catch (error) {
    if (error !== 'cancel') {
    }
  }
}

// 状态类型
const getStatusType = (hide: string) => {
  const map: Record<string, string> = {
    n: 'success',
    y: 'warning',
    spam: 'danger'
  }
  return map[hide] || 'info'
}

// 状态文本
const getStatusText = (hide: string) => {
  const map: Record<string, string> = {
    n: '已通过',
    y: '待审核',
    spam: '垃圾'
  }
  return map[hide] || hide
}

// 格式化日期
const formatDate = (timestamp: number) => {
  return new Date(timestamp * 1000).toLocaleString()
}

// 初始化
onMounted(() => {
  fetchComments()
})
</script>

<style scoped lang="scss">
.comment-list {
  .search-card {
    margin-bottom: 16px;
  }

  .table-card {
    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .line-clamp-2 {
      display: -webkit-box;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }

    .font-medium {
      font-weight: 500;
    }

    .text-gray-500 {
      color: #909399;
    }

    .text-sm {
      font-size: 12px;
    }

    .pagination {
      margin-top: 16px;
      display: flex;
      justify-content: flex-end;
    }
  }
}
</style>
