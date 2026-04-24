<template>
  <div class="comments-page">
    <div class="page-header">
      <h1 class="page-title">💬 评论管理</h1>
    </div>
    
    <div class="card">
      <el-select v-model="statusFilter" placeholder="全部状态" class="status-select" @change="loadComments">
        <el-option label="全部状态" value="" />
        <el-option label="已通过" value="n" />
        <el-option label="待审核" value="y" />
        <el-option label="垃圾" value="spam" />
      </el-select>
      
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="comments.length === 0" class="empty-state">暂无评论</div>
      <table v-else class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>作者</th>
            <th>内容</th>
            <th>所属文章</th>
            <th>状态</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="comment in comments" :key="comment.id">
            <td>{{ comment.id }}</td>
            <td>{{ comment.author }}</td>
            <td class="comment-content">{{ comment.content }}</td>
            <td>{{ comment.post_title || '-' }}</td>
            <td>
              <span :class="['tag', getStatusClass(comment.hide)]">
                {{ getStatusText(comment.hide) }}
              </span>
            </td>
            <td>
              <template v-if="comment.hide === 'y'">
                <el-button link type="success" @click="handleApprove(comment)">通过</el-button>
              </template>
              <el-button link type="danger" @click="handleDelete(comment)">删除</el-button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="totalComments > pageSize" class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          :page-size="pageSize"
          :total="totalComments"
          layout="prev, pager, next"
          @current-change="loadComments"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { commentApi } from '@/api/comment'

const statusFilter = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const totalComments = ref(0)
const loading = ref(false)

const comments = ref<any[]>([])

const getStatusClass = (status: string) => {
  const map: Record<string, string> = {
    n: 'tag-success',
    y: 'tag-warning',
    spam: 'tag-danger'
  }
  return map[status] || 'tag-info'
}

const getStatusText = (status: string) => {
  const map: Record<string, string> = {
    n: '通过',
    y: '待审',
    spam: '垃圾'
  }
  return map[status] || status
}

const loadComments = async () => {
  loading.value = true
  try {
    const res = await commentApi.getList({
      page: currentPage.value,
      per_page: pageSize.value,
      status: statusFilter.value || undefined
    })
    
    if (res.data) {
      comments.value = res.data
      totalComments.value = res.pagination?.total || 0
    }
  } catch (error) {
    console.error('加载评论失败:', error)
  } finally {
    loading.value = false
  }
}

const handleApprove = async (comment: any) => {
  try {
    await commentApi.approve(comment.id)
    ElMessage.success('已通过')
    loadComments()
  } catch (error) {
    ElMessage.error('操作失败')
  }
}

const handleDelete = async (comment: any) => {
  try {
    await ElMessageBox.confirm('确定要删除这条评论吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await commentApi.delete(comment.id)
    ElMessage.success('删除成功')
    loadComments()
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

onMounted(() => {
  loadComments()
})
</script>

<style scoped>
.comments-page {
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
}

.status-select {
  width: 130px;
  margin-bottom: 16px;
}

.loading-state,
.empty-state {
  text-align: center;
  padding: 40px 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
}

.data-table th,
.data-table td {
  padding: 12px 10px;
  text-align: left;
  border-bottom: 1px solid var(--border);
}

.data-table th {
  background: #f8fafc;
  font-weight: 600;
  color: var(--text-secondary);
}

.data-table tr:hover td {
  background: #f8fafc;
}

.comment-content {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tag {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
}

.tag-success {
  background: #d1fae5;
  color: #065f46;
}

.tag-warning {
  background: #fef3c7;
  color: #92400e;
}

.tag-danger {
  background: #fee2e2;
  color: #991b1b;
}

.pagination {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}
</style>