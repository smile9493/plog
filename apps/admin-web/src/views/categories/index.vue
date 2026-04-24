<template>
  <div class="categories-page">
    <div class="page-header">
      <h1 class="page-title">分类管理</h1>
    </div>
    
    <div class="card">
      <el-button type="primary" @click="handleCreate">+ 新建分类</el-button>
      
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="categories.length === 0" class="empty-state">暂无分类</div>
      <table v-else class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>名称</th>
            <th>别名</th>
            <th>描述</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="category in categories" :key="category.sid">
            <td>{{ category.sid }}</td>
            <td>{{ category.sortname }}</td>
            <td>{{ category.alias || '-' }}</td>
            <td>{{ category.description || '-' }}</td>
            <td>
              <el-button link type="primary" @click="handleEdit(category)">编辑</el-button>
              <el-button link type="danger" @click="handleDelete(category)">删除</el-button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="名称" required>
          <el-input v-model="form.sortname" placeholder="分类名称" />
        </el-form-item>
        <el-form-item label="别名">
          <el-input v-model="form.alias" placeholder="url-slug" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="form.description" type="textarea" :rows="3" />
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
import { categoryApi } from '@/api/category'

interface CategoryItem {
  sid: number
  sortname: string
  alias: string | null
  description: string | null
  pid: number
  sortorder: number
}

const dialogVisible = ref(false)
const dialogTitle = ref('新建分类')
const loading = ref(false)
const submitting = ref(false)

const form = reactive({
  sid: 0,
  sortname: '',
  alias: '',
  description: ''
})

const categories = ref<CategoryItem[]>([])

const loadCategories = async () => {
  loading.value = true
  try {
    const res = await categoryApi.getList()
    categories.value = res.data || []
  } catch (error) {
    console.error('加载分类失败:', error)
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  dialogTitle.value = '新建分类'
  Object.assign(form, { sid: 0, sortname: '', alias: '', description: '' })
  dialogVisible.value = true
}

const handleEdit = (category: CategoryItem) => {
  dialogTitle.value = '编辑分类'
  Object.assign(form, {
    sid: category.sid,
    sortname: category.sortname,
    alias: category.alias || '',
    description: category.description || ''
  })
  dialogVisible.value = true
}

const handleDelete = async (category: CategoryItem) => {
  try {
    await ElMessageBox.confirm(`确定要删除分类"${category.sortname}"吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await categoryApi.delete(category.sid)
    ElMessage.success('删除成功')
    loadCategories()
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const handleSubmit = async () => {
  if (!form.sortname) {
    ElMessage.warning('请输入分类名称')
    return
  }
  
  submitting.value = true
  try {
    if (form.sid) {
      await categoryApi.update(form.sid, {
        sortname: form.sortname,
        alias: form.alias || undefined,
        description: form.description || undefined
      })
      ElMessage.success('更新成功')
    } else {
      await categoryApi.create({
        sortname: form.sortname,
        alias: form.alias || undefined,
        description: form.description || undefined
      })
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadCategories()
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    submitting.value = false
  }
}

onMounted(() => {
  loadCategories()
})
</script>

<style scoped>
.categories-page {
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
  margin-top: 16px;
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
</style>
