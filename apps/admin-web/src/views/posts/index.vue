<template>
  <div class="posts-page">
    <div class="page-header">
      <h1 class="page-title">文章管理</h1>
    </div>
    
    <div class="card">
      <div class="toolbar">
        <el-input
          v-model="searchQuery"
          placeholder="搜索文章..."
          class="search-input"
          clearable
          @clear="loadPosts"
          @keyup.enter="loadPosts"
        />
        <el-select v-model="statusFilter" placeholder="全部状态" class="status-select" @change="loadPosts">
          <el-option label="全部状态" value="all" />
          <el-option label="已发布" value="n" />
          <el-option label="草稿" value="y" />
        </el-select>
        <el-button type="primary" @click="handleCreate">
          + 新建文章
        </el-button>
      </div>
      
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="posts.length === 0" class="empty-state">暂无文章</div>
      <table v-else class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>标题</th>
            <th>分类</th>
            <th>状态</th>
            <th>浏览</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="post in posts" :key="post.gid">
            <td>{{ post.gid }}</td>
            <td>{{ post.title }}</td>
            <td>{{ getCategoryName(post.sortid) }}</td>
            <td>
              <span :class="['tag', post.hide === 'n' ? 'tag-success' : 'tag-info']">
                {{ post.hide === 'n' ? '已发布' : '草稿' }}
              </span>
            </td>
            <td>{{ post.views || 0 }}</td>
            <td>{{ formatDate(post.date) }}</td>
            <td>
              <el-button link type="primary" @click="handleEdit(post)">编辑</el-button>
              <el-button link type="danger" @click="handleDelete(post)">删除</el-button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="totalPosts > pageSize" class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          :page-size="pageSize"
          :total="totalPosts"
          layout="prev, pager, next"
          @current-change="loadPosts"
        />
      </div>
    </div>
    
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="800px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="标题" required>
          <el-input v-model="form.title" placeholder="请输入文章标题" />
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="form.sortid" placeholder="选择分类" clearable>
            <el-option
              v-for="cat in categories"
              :key="cat.sid"
              :label="cat.sortname"
              :value="cat.sid"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="form.hide">
            <el-option label="草稿" value="y" />
            <el-option label="已发布" value="n" />
          </el-select>
        </el-form-item>
        <el-form-item label="摘要">
          <el-input v-model="form.excerpt" type="textarea" :rows="2" placeholder="文章摘要" />
        </el-form-item>
        <el-form-item label="内容" required>
          <el-input v-model="form.content" type="textarea" :rows="10" placeholder="文章内容（支持Markdown）" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="handleSubmit">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { postApi } from '@/api/post'
import { categoryApi } from '@/api/category'

interface PostItem {
  gid: number
  title: string
  content: string
  excerpt: string | null
  author: number
  sortid: number
  date: number
  hide: string
  type: string
  views: number
  comnum: number
  like_count: number
  top: string
  sortop: string
  allow_remark: string
  password: string | null
  cover: string | null
  alias: string | null
}

interface CategoryItem {
  sid: number
  sortname: string
  alias: string | null
  description: string | null
}

const searchQuery = ref('')
const statusFilter = ref('all')
const currentPage = ref(1)
const pageSize = ref(10)
const totalPosts = ref(0)
const dialogVisible = ref(false)
const dialogTitle = ref('新建文章')
const loading = ref(false)
const submitting = ref(false)

const form = reactive({
  gid: 0,
  title: '',
  sortid: 0,
  hide: 'n',
  content: '',
  excerpt: ''
})

const posts = ref<PostItem[]>([])
const categories = ref<CategoryItem[]>([])

const loadCategories = async () => {
  try {
    const res = await categoryApi.getList()
    categories.value = res.data || []
  } catch (error) {
    console.error('加载分类失败:', error)
  }
}

const loadPosts = async () => {
  loading.value = true
  try {
    const res = await postApi.getList({
      page: currentPage.value,
      per_page: pageSize.value,
      keyword: searchQuery.value || undefined,
      status: statusFilter.value || undefined
    })
    
    if (res.data) {
      posts.value = res.data.items || []
      totalPosts.value = res.data.pagination?.total || 0
    }
  } catch (error) {
    console.error('加载文章失败:', error)
  } finally {
    loading.value = false
  }
}

const getCategoryName = (sortid: number) => {
  const cat = categories.value.find(c => c.sid === sortid)
  return cat ? cat.sortname : '-'
}

const formatDate = (timestamp: number) => {
  if (!timestamp) return '-'
  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}

const handleCreate = () => {
  dialogTitle.value = '新建文章'
  Object.assign(form, { gid: 0, title: '', sortid: 0, hide: 'n', content: '', excerpt: '' })
  dialogVisible.value = true
}

const handleEdit = (post: PostItem) => {
  dialogTitle.value = '编辑文章'
  Object.assign(form, {
    gid: post.gid,
    title: post.title,
    sortid: post.sortid || 0,
    hide: post.hide,
    content: post.content || '',
    excerpt: post.excerpt || ''
  })
  dialogVisible.value = true
}

const handleDelete = async (post: PostItem) => {
  try {
    await ElMessageBox.confirm(`确定要删除文章"${post.title}"吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await postApi.delete(post.gid)
    ElMessage.success('删除成功')
    loadPosts()
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const handleSubmit = async () => {
  if (!form.title) {
    ElMessage.warning('请输入标题')
    return
  }
  if (!form.content) {
    ElMessage.warning('请输入内容')
    return
  }
  
  submitting.value = true
  try {
    if (form.gid) {
      await postApi.update(form.gid, {
        title: form.title,
        sortid: form.sortid || undefined,
        hide: form.hide,
        content: form.content,
        excerpt: form.excerpt || undefined
      })
      ElMessage.success('更新成功')
    } else {
      await postApi.create({
        title: form.title,
        sortid: form.sortid || undefined,
        hide: form.hide,
        content: form.content,
        excerpt: form.excerpt || undefined
      })
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadPosts()
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    submitting.value = false
  }
}

onMounted(() => {
  loadCategories()
  loadPosts()
})
</script>

<style scoped>
.posts-page {
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

.toolbar {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.search-input {
  width: 200px;
}

.status-select {
  width: 120px;
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

.tag-info {
  background: #dbeafe;
  color: #1e40af;
}

.pagination {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}
</style>
