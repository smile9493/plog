<template>
  <div class="role-manage">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>角色管理</span>
          <el-button type="primary" @click="handleCreate">
            <el-icon><Plus /></el-icon>
            新增角色
          </el-button>
        </div>
      </template>

      <el-table v-loading="loading" :data="roleList" style="width: 100%">
        <el-table-column prop="name" label="角色名称" min-width="150" />
        <el-table-column prop="slug" label="标识" min-width="120" />
        <el-table-column prop="description" label="描述" min-width="200" />
        <el-table-column prop="user_count" label="用户数" width="100" />
        <el-table-column prop="created_at" label="创建时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="handleEdit(row)">编辑</el-button>
            <el-button type="warning" link @click="handlePermissions(row)">权限</el-button>
            <el-button type="danger" link :disabled="row.user_count > 0" @click="handleDelete(row)">
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 角色编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500px">
      <el-form ref="formRef" :model="formData" :rules="rules" label-width="80px">
        <el-form-item label="角色名称" prop="name">
          <el-input v-model="formData.name" placeholder="请输入角色名称" />
        </el-form-item>
        <el-form-item label="标识" prop="slug">
          <el-input v-model="formData.slug" placeholder="例如: editor, admin" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="formData.description" type="textarea" placeholder="请输入角色描述" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit">确定</el-button>
      </template>
    </el-dialog>

    <!-- 权限分配对话框 -->
    <el-dialog v-model="permDialogVisible" title="权限分配" width="600px">
      <el-tree
        ref="permTreeRef"
        :data="permTreeData"
        :props="{ label: 'name', children: 'children' }"
        show-checkbox
        node-key="id"
        :default-checked-keys="selectedPermIds"
      />
      <template #footer>
        <el-button @click="permDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSavePermissions">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { roleApi, permissionApi, type Role, type Permission } from '@/api/role'
import dayjs from 'dayjs'

const loading = ref(false)
const roleList = ref<Role[]>([])
const dialogVisible = ref(false)
const dialogTitle = ref('')
const isEdit = ref(false)
const editId = ref(0)
const formRef = ref<FormInstance>()

const formData = reactive({
  name: '',
  slug: '',
  description: ''
})

const rules = reactive<FormRules>({
  name: [{ required: true, message: '请输入角色名称', trigger: 'blur' }],
  slug: [{ required: true, message: '请输入标识', trigger: 'blur' }]
})

// 权限分配
const permDialogVisible = ref(false)
const currentRoleId = ref(0)
const permTreeData = ref<any[]>([])
const selectedPermIds = ref<number[]>([])
const permTreeRef = ref()

const fetchRoles = async () => {
  loading.value = true
  try {
    roleList.value = await roleApi.getList()
  } catch (error) {
    ElMessage.error('获取角色列表失败')
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  isEdit.value = false
  dialogTitle.value = '新增角色'
  formData.name = ''
  formData.slug = ''
  formData.description = ''
  dialogVisible.value = true
}

const handleEdit = (row: Role) => {
  isEdit.value = true
  editId.value = row.id
  dialogTitle.value = '编辑角色'
  formData.name = row.name
  formData.slug = row.slug
  formData.description = row.description
  dialogVisible.value = true
}

const handleSubmit = async () => {
  if (!formRef.value) return
  await formRef.value.validate(async (valid) => {
    if (!valid) return
    try {
      if (isEdit.value) {
        await roleApi.update(editId.value, formData)
        ElMessage.success('更新成功')
      } else {
        await roleApi.create(formData)
        ElMessage.success('创建成功')
      }
      dialogVisible.value = false
      fetchRoles()
    } catch (error) {
      ElMessage.error('操作失败')
    }
  })
}

const handleDelete = async (row: Role) => {
  try {
    await ElMessageBox.confirm(`确定要删除角色 "${row.name}" 吗？`, '警告', {
      type: 'warning'
    })
    await roleApi.delete(row.id)
    ElMessage.success('删除成功')
    fetchRoles()
  } catch (error) {
    // 取消或失败
  }
}

const handlePermissions = async (row: Role) => {
  currentRoleId.value = row.id
  permDialogVisible.value = true
  try {
    const perms = await permissionApi.getList()
    permTreeData.value = buildPermTree(perms)
    selectedPermIds.value = row.permissions.map(p => p.id)
  } catch (error) {
    ElMessage.error('获取权限列表失败')
  }
}

const buildPermTree = (perms: Permission[]) => {
  const modules: Record<string, any> = {}
  for (const perm of perms) {
    if (!modules[perm.module]) {
      modules[perm.module] = { id: `m_${perm.module}`, name: perm.module, children: [] }
    }
    modules[perm.module].children.push({ id: perm.id, name: perm.description || perm.name })
  }
  return Object.values(modules)
}

const handleSavePermissions = async () => {
  const checked = permTreeRef.value?.getCheckedKeys() || []
  const leafChecked = checked.filter((k: any) => typeof k === 'number')
  try {
    await roleApi.assignPermissions(currentRoleId.value, leafChecked)
    ElMessage.success('权限分配成功')
    permDialogVisible.value = false
    fetchRoles()
  } catch (error) {
    ElMessage.error('权限分配失败')
  }
}

const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

onMounted(() => {
  fetchRoles()
})
</script>

<style scoped lang="scss">
.role-manage {
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
}
</style>
