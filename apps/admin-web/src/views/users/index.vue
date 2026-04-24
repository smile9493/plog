<template>
  <div class="users-page">
    <div class="page-header">
      <h1 class="page-title">👥 用户管理</h1>
    </div>
    
    <div class="card">
      <el-button type="primary" @click="handleCreate">+ 新建用户</el-button>
      
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="users.length === 0" class="empty-state">暂无用户</div>
      <table v-else class="data-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>用户名</th>
            <th>昵称</th>
            <th>角色</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in users" :key="user.id">
            <td>{{ user.id }}</td>
            <td>{{ user.username }}</td>
            <td>{{ user.nickname || '-' }}</td>
            <td>
              <span :class="['tag', getRoleClass(user.role)]">
                {{ getRoleText(user.role) }}
              </span>
            </td>
            <td>
              <el-button link type="primary" @click="handleEdit(user)">编辑</el-button>
              <el-button link type="danger" @click="handleDelete(user)">删除</el-button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="totalUsers > pageSize" class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          :page-size="pageSize"
          :total="totalUsers"
          layout="prev, pager, next"
          @current-change="loadUsers"
        />
      </div>
    </div>
    
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="用户名">
          <el-input v-model="form.username" :disabled="!!form.id" />
        </el-form-item>
        <el-form-item label="昵称">
          <el-input v-model="form.nickname" />
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="form.role">
            <el-option label="管理员" value="admin" />
            <el-option label="编辑" value="editor" />
            <el-option label="作者" value="author" />
            <el-option label="普通用户" value="user" />
          </el-select>
        </el-form-item>
        <el-form-item v-if="!form.id" label="密码">
          <el-input v-model="form.password" type="password" show-password />
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
import { userApi } from '@/api/user'

const dialogVisible = ref(false)
const dialogTitle = ref('新建用户')
const loading = ref(false)
const submitting = ref(false)
const currentPage = ref(1)
const pageSize = ref(10)
const totalUsers = ref(0)

const form = reactive({
  id: 0,
  username: '',
  nickname: '',
  role: 'user',
  password: ''
})

const users = ref<any[]>([])

const getRoleClass = (role: string) => {
  const map: Record<string, string> = {
    admin: 'tag-danger',
    editor: 'tag-warning',
    author: 'tag-info',
    user: 'tag-success'
  }
  return map[role] || 'tag-info'
}

const getRoleText = (role: string) => {
  const map: Record<string, string> = {
    admin: '管理员',
    editor: '编辑',
    author: '作者',
    user: '普通用户'
  }
  return map[role] || role
}

const loadUsers = async () => {
  loading.value = true
  try {
    const res = await userApi.getList({
      page: currentPage.value,
      per_page: pageSize.value
    })
    
    if (res.data) {
      users.value = res.data
      totalUsers.value = res.pagination?.total || 0
    }
  } catch (error) {
    console.error('加载用户失败:', error)
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  dialogTitle.value = '新建用户'
  Object.assign(form, { id: 0, username: '', nickname: '', role: 'user', password: '' })
  dialogVisible.value = true
}

const handleEdit = (user: any) => {
  dialogTitle.value = '编辑用户'
  Object.assign(form, { ...user, password: '' })
  dialogVisible.value = true
}

const handleDelete = async (user: any) => {
  try {
    await ElMessageBox.confirm(`确定要删除用户"${user.username}"吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await userApi.delete(user.id)
    ElMessage.success('删除成功')
    loadUsers()
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const handleSubmit = async () => {
  if (!form.username) {
    ElMessage.warning('请输入用户名')
    return
  }
  
  if (!form.id && !form.password) {
    ElMessage.warning('请输入密码')
    return
  }
  
  submitting.value = true
  try {
    if (form.id) {
      await userApi.update(form.id, {
        nickname: form.nickname,
        role: form.role
      })
      ElMessage.success('更新成功')
    } else {
      await userApi.create({
        username: form.username,
        nickname: form.nickname,
        role: form.role,
        password: form.password
      })
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    loadUsers()
  } catch (error) {
    ElMessage.error('保存失败')
  } finally {
    submitting.value = false
  }
}

onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.users-page {
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
}

.tag-danger {
  background: #fee2e2;
  color: #991b1b;
}

.tag-warning {
  background: #fef3c7;
  color: #92400e;
}

.tag-info {
  background: #dbeafe;
  color: #1e40af;
}

.tag-success {
  background: #d1fae5;
  color: #065f46;
}

.pagination {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}
</style>