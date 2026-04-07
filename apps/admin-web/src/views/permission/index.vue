<template>
  <div class="permission-manage">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>权限管理</span>
          <div class="header-actions">
            <el-select v-model="filterModule" placeholder="按模块筛选" clearable style="width: 150px; margin-right: 16px;" @change="handleFilter">
              <el-option v-for="mod in modules" :key="mod" :label="mod" :value="mod" />
            </el-select>
            <el-button type="primary" @click="handleCreate">
              <el-icon><Plus /></el-icon>
              新增权限
            </el-button>
          </div>
        </div>
      </template>

      <el-table v-loading="loading" :data="filteredPermissions" style="width: 100%">
        <el-table-column prop="name" label="权限名称" min-width="150" />
        <el-table-column prop="slug" label="标识" min-width="150" />
        <el-table-column prop="module" label="模块" width="120">
          <template #default="{ row }">
            <el-tag size="small">{{ row.module }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="action" label="操作类型" width="100">
          <template #default="{ row }">
            <el-tag :type="getActionType(row.action)" size="small">{{ row.action }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="description" label="描述" min-width="200" />
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="handleEdit(row)">编辑</el-button>
            <el-button type="danger" link @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 权限编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500px">
      <el-form ref="formRef" :model="formData" :rules="rules" label-width="100px">
        <el-form-item label="权限名称" prop="name">
          <el-input v-model="formData.name" placeholder="请输入权限名称" />
        </el-form-item>
        <el-form-item label="标识" prop="slug">
          <el-input v-model="formData.slug" placeholder="例如: post.create" />
        </el-form-item>
        <el-form-item label="模块" prop="module">
          <el-select v-model="formData.module" placeholder="请选择模块" style="width: 100%;">
            <el-option label="文章" value="post" />
            <el-option label="分类" value="category" />
            <el-option label="标签" value="tag" />
            <el-option label="评论" value="comment" />
            <el-option label="媒体" value="media" />
            <el-option label="用户" value="user" />
            <el-option label="角色" value="role" />
            <el-option label="插件" value="plugin" />
            <el-option label="主题" value="theme" />
            <el-option label="设置" value="setting" />
          </el-select>
        </el-form-item>
        <el-form-item label="操作类型" prop="action">
          <el-select v-model="formData.action" placeholder="请选择操作类型" style="width: 100%;">
            <el-option label="查看" value="read" />
            <el-option label="创建" value="create" />
            <el-option label="编辑" value="update" />
            <el-option label="删除" value="delete" />
            <el-option label="管理" value="manage" />
          </el-select>
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="formData.description" type="textarea" placeholder="请输入权限描述" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { permissionApi, type Permission } from '@/api/role'
import { request } from '@/utils/request'

const loading = ref(false)
const permissionList = ref<Permission[]>([])
const filterModule = ref('')
const dialogVisible = ref(false)
const dialogTitle = ref('')
const isEdit = ref(false)
const editId = ref(0)
const formRef = ref<FormInstance>()

const formData = reactive({
  name: '',
  slug: '',
  module: '',
  action: '',
  description: ''
})

const rules = reactive<FormRules>({
  name: [{ required: true, message: '请输入权限名称', trigger: 'blur' }],
  slug: [{ required: true, message: '请输入标识', trigger: 'blur' }],
  module: [{ required: true, message: '请选择模块', trigger: 'change' }],
  action: [{ required: true, message: '请选择操作类型', trigger: 'change' }]
})

const modules = computed(() => {
  const mods = new Set(permissionList.value.map(p => p.module))
  return Array.from(mods)
})

const filteredPermissions = computed(() => {
  if (!filterModule.value) return permissionList.value
  return permissionList.value.filter(p => p.module === filterModule.value)
})

const getActionType = (action: string) => {
  const map: Record<string, string> = {
    read: 'info',
    create: 'success',
    update: 'warning',
    delete: 'danger',
    manage: ''
  }
  return map[action] || 'info'
}

const fetchPermissions = async () => {
  loading.value = true
  try {
    permissionList.value = await permissionApi.getList()
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  } finally {
    loading.value = false
  }
}

const handleFilter = () => {
  // computed handles filtering
}

const handleCreate = () => {
  isEdit.value = false
  dialogTitle.value = '新增权限'
  formData.name = ''
  formData.slug = ''
  formData.module = ''
  formData.action = ''
  formData.description = ''
  dialogVisible.value = true
}

const handleEdit = (row: Permission) => {
  isEdit.value = true
  editId.value = row.id
  dialogTitle.value = '编辑权限'
  formData.name = row.name
  formData.slug = row.slug
  formData.module = row.module
  formData.action = row.action
  formData.description = row.description
  dialogVisible.value = true
}

const handleSubmit = async () => {
  if (!formRef.value) return
  await formRef.value.validate(async (valid) => {
    if (!valid) return
    try {
      if (isEdit.value) {
        await request.put(`/permissions/${editId.value}`, formData)
        ElMessage.success('更新成功')
      } else {
        await request.post('/permissions', formData)
        ElMessage.success('创建成功')
      }
      dialogVisible.value = false
      fetchPermissions()
    } catch (error) {
      ElMessage.error('操作失败')
    }
  })
}

const handleDelete = async (row: Permission) => {
  try {
    await ElMessageBox.confirm(`确定要删除权限 "${row.name}" 吗？`, '警告', {
      type: 'warning'
    })
    await request.delete(`/permissions/${row.id}`)
    ElMessage.success('删除成功')
    fetchPermissions()
  } catch (error) {
    // 取消或失败
  }
}

onMounted(() => {
  fetchPermissions()
})
</script>

<style scoped lang="scss">
.permission-manage {
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .header-actions {
      display: flex;
      align-items: center;
    }
  }
}
</style>
