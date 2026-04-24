<template>
  <div class="tags-page">
    <div class="page-header">
      <h1 class="page-title">标签管理</h1>
    </div>
    
    <div class="card">
      <el-button type="primary" @click="handleCreate">+ 新建标签</el-button>
      
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="tags.length === 0" class="empty-state">暂无标签</div>
      <table v-else class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>名称</th>
            <th>使用次数</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="tag in tags" :key="tag.tid">
            <td>{{ tag.tid }}</td>
            <td><span class="tag">{{ tag.tagname }}</span></td>
            <td>{{ tag.usenum || 0 }}</td>
            <td>
              <el-button link type="primary" @click="handleEdit(tag)">编辑</el-button>
              <el-button link type="danger" @click="handleDelete(tag)">删除</el-button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="名称" required>
          <el-input v-model="form.tagname" placeholder="标签名称" />
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
import { tagApi } from '@/api/tag'

interface TagItem {
  tid: number
  tagname: string
  usenum: number
}

const dialogVisible = ref(false)
const dialogTitle = ref('新建标签')
const loading = ref(false)
const submitting = ref(false)

const form = reactive({
  tid: 0,
  tagname: ''
})

const tags = ref<TagItem[]>([])

const loadTags = async () => {
  loading.value = true
  try {
    const res = await tagApi.getList()
    tags.value = res.data || []
  } catch (error) {
    console.error('加载标签失败:', error)
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  dialogTitle.value = '新建标签'
  Object.assign(form, { tid: 0, tagname: '' })
  dialogVisible.value = true
}

const handleEdit = (tag: TagItem) => {
  dialogTitle.value = '编辑标签'
  Object.assign(form, {
    tid: tag.tid,
    tagname: tag.tagname
  })
  dialogVisible.value = true
}

const handleDelete = async (tag: TagItem) => {
  try {
    await ElMessageBox.confirm(`确定要删除标签"${tag.tagname}"吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await tagApi.delete(tag.tid)
    ElMessage.success('删除成功')
    loadTags()
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const handleSubmit = async () => {
  if (!form.tagname) {
    ElMessage.warning('请输入标签名称')
    return
  }
  
  submitting.value = true
  try {
    if (form.tid) {
      await tagApi.update(form.tid, { tagname: form.tagname })
      ElMessage.success('更新成功')
    } else {
      await tagApi.create({ tagname: form.tagname })
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadTags()
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    submitting.value = false
  }
}

onMounted(() => {
  loadTags()
})
</script>

<style scoped>
.tags-page {
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

.tag {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
  background: var(--primary);
  color: #fff;
}
</style>
