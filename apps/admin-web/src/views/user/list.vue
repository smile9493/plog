<template>
  <div class="user-list">
    <!-- 搜索和筛选 -->
    <el-card class="search-card">
      <el-form :model="searchForm" inline>
        <el-form-item label="用户名">
          <el-input
            v-model="searchForm.username"
            placeholder="请输入用户名"
            clearable
            @keyup.enter="handleSearch"
          />
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="searchForm.role" placeholder="请选择角色" clearable>
            <el-option label="管理员" value="admin" />
            <el-option label="编辑" value="editor" />
            <el-option label="用户" value="user" />
          </el-select>
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
          <span>用户列表</span>
          <el-button type="primary" @click="handleCreate">
            <el-icon><Plus /></el-icon>
            新建用户
          </el-button>
        </div>
      </template>

      <!-- 用户表格 -->
      <el-table
        v-loading="loading"
        :data="userList"
        border
        stripe
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="uid" label="ID" width="80" />
        <el-table-column prop="username" label="用户名" min-width="120" />
        <el-table-column prop="nickname" label="昵称" min-width="120" />
        <el-table-column prop="email" label="邮箱" min-width="180" />
        <el-table-column prop="role" label="角色" width="100">
          <template #default="{ row }">
            <el-tag :type="getRoleTagType(row.role)">
              {{ getRoleLabel(row.role) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.create_time) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="handleEdit(row)">
              编辑
            </el-button>
            <el-button type="warning" link @click="handleResetPassword(row)">
              重置密码
            </el-button>
            <el-popconfirm
              title="确定要删除这个用户吗？"
              @confirm="handleDelete(row)"
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

    <!-- 用户表单对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-width="80px"
      >
        <el-form-item label="用户名" prop="username">
          <el-input v-model="formData.username" placeholder="请输入用户名" />
        </el-form-item>
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="formData.nickname" placeholder="请输入昵称" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="formData.email" placeholder="请输入邮箱" />
        </el-form-item>
        <el-form-item label="角色" prop="role">
          <el-select v-model="formData.role" placeholder="请选择角色">
            <el-option label="管理员" value="admin" />
            <el-option label="编辑" value="editor" />
            <el-option label="用户" value="user" />
          </el-select>
        </el-form-item>
        <el-form-item v-if="!formData.uid" label="密码" prop="password">
          <el-input
            v-model="formData.password"
            type="password"
            placeholder="请输入密码"
            show-password
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="handleSubmit">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Search, Refresh, Plus } from '@element-plus/icons-vue'
import { userApi } from '@/api/user'
import type { User } from '@/types/user'

// 搜索表单
const searchForm = reactive({
  username: '',
  role: ''
})

// 分页
const pagination = reactive({
  page: 1,
  per_page: 20,
  total: 0
})

// 用户列表
const userList = ref<User[]>([])
const loading = ref(false)
const selectedUsers = ref<User[]>([])

// 对话框
const dialogVisible = ref(false)
const dialogTitle = computed(() => formData.uid ? '编辑用户' : '新建用户')
const submitting = ref(false)

// 表单
const formRef = ref<FormInstance>()
const formData = reactive<Partial<User> & { password?: string }>({
  username: '',
  nickname: '',
  email: '',
  role: 'user',
  password: ''
})

// 表单验证规则
const formRules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  role: [
    { required: true, message: '请选择角色', trigger: 'change' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少 6 个字符', trigger: 'blur' }
  ]
}

// 获取用户列表
const fetchUserList = async () => {
  loading.value = true
  try {
    const res = await userApi.getList({
      page: pagination.page,
      per_page: pagination.per_page,
      ...searchForm
    })
    userList.value = res.data.items
    pagination.total = res.data.pagination.total
  } catch (error) {
    
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  pagination.page = 1
  fetchUserList()
}

// 重置
const handleReset = () => {
  searchForm.username = ''
  searchForm.role = ''
  handleSearch()
}

// 分页
const handleSizeChange = () => {
  fetchUserList()
}

const handlePageChange = () => {
  fetchUserList()
}

// 选择
const handleSelectionChange = (selection: User[]) => {
  selectedUsers.value = selection
}

// 新建
const handleCreate = () => {
  Object.assign(formData, {
    uid: undefined,
    username: '',
    nickname: '',
    email: '',
    role: 'user',
    password: ''
  })
  dialogVisible.value = true
}

// 编辑
const handleEdit = (row: User) => {
  Object.assign(formData, {
    uid: row.uid,
    username: row.username,
    nickname: row.nickname,
    email: row.email,
    role: row.role,
    password: ''
  })
  dialogVisible.value = true
}

// 提交
const handleSubmit = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (!valid) return
    
    submitting.value = true
    try {
      if (formData.uid) {
        await userApi.update(formData.uid, formData)
        ElMessage.success('更新成功')
      } else {
        await userApi.create(formData)
        ElMessage.success('创建成功')
      }
      dialogVisible.value = false
      fetchUserList()
    } catch (error) {
    } finally {
      submitting.value = false
    }
  })
}

// 删除
const handleDelete = async (row: User) => {
  try {
    await userApi.delete(row.uid)
    ElMessage.success('删除成功')
    fetchUserList()
  } catch (error) {
  }
}

// 重置密码
const handleResetPassword = async (row: User) => {
  try {
    const { value: password } = await ElMessageBox.prompt('请输入新密码', '重置密码', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      inputType: 'password',
      inputValidator: (value) => {
        if (!value || value.length < 6) {
          return '密码至少 6 个字符'
        }
        return true
      }
    })
    
    await userApi.resetPassword(row.uid, password)
    ElMessage.success('密码重置成功')
  } catch (error) {
    if (error !== 'cancel') {
    }
  }
}

// 工具函数
const getRoleTagType = (role: string) => {
  const map: Record<string, string> = {
    admin: 'danger',
    editor: 'warning',
    user: 'info'
  }
  return map[role] || 'info'
}

const getRoleLabel = (role: string) => {
  const map: Record<string, string> = {
    admin: '管理员',
    editor: '编辑',
    user: '用户'
  }
  return map[role] || role
}

const formatDate = (timestamp: number) => {
  return new Date(timestamp * 1000).toLocaleString()
}

// 初始化
onMounted(() => {
  fetchUserList()
})
</script>

<style scoped lang="scss">
.user-list {
  .search-card {
    margin-bottom: 16px;
  }

  .table-card {
    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .pagination {
      margin-top: 16px;
      display: flex;
      justify-content: flex-end;
    }
  }
}
</style>
