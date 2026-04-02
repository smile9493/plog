<template>
  <div class="tag-manage">
    <!-- 操作栏 -->
    <el-card class="toolbar-card">
      <el-button type="primary" @click="handleCreate">
        <el-icon><Plus /></el-icon>
        添加标签
      </el-button>
    </el-card>

    <!-- 标签列表 -->
    <el-card class="table-card">
      <el-table v-loading="loading" :data="tagList" style="width: 100%">
        <el-table-column prop="name" label="标签名称" min-width="150" />
        <el-table-column prop="slug" label="别名" width="150" />
        <el-table-column prop="post_count" label="文章数" width="100" />
        <el-table-column prop="created_at" label="创建时间" width="180">
          <template #default="{ row }">
            {{ row ? formatDate(row.created_at) : '' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button v-if="row" type="primary" link @click="handleEdit(row)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button v-if="row" type="warning" link @click="handleMerge(row)">
              <el-icon><Connection /></el-icon>
              合并
            </el-button>
            <el-button v-if="row" type="danger" link @click="handleDelete(row.id)">
              <el-icon><Delete /></el-icon>
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <el-pagination
        v-model:current-page="queryParams.page"
        v-model:page-size="queryParams.per_page"
        :page-sizes="[10, 20, 50]"
        :total="total"
        layout="total, sizes, prev, pager, next"
        class="pagination"
        @size-change="fetchTags"
        @current-change="fetchTags"
      />
    </el-card>

    <!-- 编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑标签' : '添加标签'"
      width="500px"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="80px"
      >
        <el-form-item label="名称" prop="name">
          <el-input v-model="form.name" placeholder="请输入标签名称" />
        </el-form-item>
        <el-form-item label="别名" prop="slug">
          <el-input v-model="form.slug" placeholder="请输入标签别名（可选）" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="submitting">
          确定
        </el-button>
      </template>
    </el-dialog>

    <!-- 合并对话框 -->
    <el-dialog
      v-model="mergeDialogVisible"
      title="合并标签"
      width="500px"
    >
      <el-form label-width="100px">
        <el-form-item label="源标签">
          <el-tag>{{ mergeSource?.name }}</el-tag>
        </el-form-item>
        <el-form-item label="目标标签">
          <el-select v-model="mergeTargetId" placeholder="选择目标标签">
            <el-option
              v-for="tag in tagList.filter(t => t.id !== mergeSource?.id)"
              :key="tag.id"
              :label="tag.name"
              :value="tag.id"
            />
          </el-select>
        </el-form-item>
        <el-alert
          type="warning"
          :closable="false"
          show-icon
        >
          合并后，源标签将被删除，其下的所有文章将转移到目标标签
        </el-alert>
      </el-form>
      <template #footer>
        <el-button @click="mergeDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleMergeSubmit" :loading="merging">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Edit, Delete, Connection } from '@element-plus/icons-vue'
import { tagApi } from '@/api/tag'
import type { Tag, PaginationParams } from '@/types'
import dayjs from 'dayjs'

// 加载状态
const loading = ref(false)
const submitting = ref(false)
const merging = ref(false)

// 标签列表
const tagList = ref<Tag[]>([])
const total = ref(0)

// 查询参数
const queryParams = reactive<PaginationParams>({
  page: 1,
  per_page: 10
})

// 编辑对话框
const dialogVisible = ref(false)
const isEdit = ref(false)
const editId = ref<number>()

// 合并对话框
const mergeDialogVisible = ref(false)
const mergeSource = ref<Tag>()
const mergeTargetId = ref<number>()

// 表单引用
const formRef = ref<FormInstance>()

// 表单数据
const form = reactive({
  name: '',
  slug: ''
})

// 表单验证规则
const rules: FormRules = {
  name: [
    { required: true, message: '请输入标签名称', trigger: 'blur' },
    { min: 2, max: 50, message: '名称长度在 2 到 50 个字符', trigger: 'blur' }
  ],
  slug: [
    { pattern: /^[a-z0-9-]+$/, message: '别名只能包含小写字母、数字和连字符', trigger: 'blur' }
  ]
}

// 获取标签列表
const fetchTags = async () => {
  loading.value = true
  try {
    const res = await tagApi.getList(queryParams)
    tagList.value = res.items
    total.value = res.total
  } catch (error) {
    
  } finally {
    loading.value = false
  }
}

// 创建标签
const handleCreate = () => {
  isEdit.value = false
  editId.value = undefined
  form.name = ''
  form.slug = ''
  dialogVisible.value = true
}

// 编辑标签
const handleEdit = (tag: Tag) => {
  isEdit.value = true
  editId.value = tag.id
  form.name = tag.name
  form.slug = tag.slug
  dialogVisible.value = true
}

// 提交表单
const handleSubmit = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      submitting.value = true
      try {
        if (isEdit.value && editId.value) {
          await tagApi.update(editId.value, form)
          ElMessage.success('更新成功')
        } else {
          await tagApi.create(form)
          ElMessage.success('创建成功')
        }
        dialogVisible.value = false
        fetchTags()
      } catch (error) {
      } finally {
        submitting.value = false
      }
    }
  })
}

// 删除标签
const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个标签吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await tagApi.delete(id)
    ElMessage.success('删除成功')
    fetchTags()
  } catch (error) {
    // 取消或失败
  }
}

// 合并标签
const handleMerge = (tag: Tag) => {
  mergeSource.value = tag
  mergeTargetId.value = undefined
  mergeDialogVisible.value = true
}

// 提交合并
const handleMergeSubmit = async () => {
  if (!mergeSource.value || !mergeTargetId.value) {
    return
  }

  merging.value = true
  try {
    await tagApi.merge(mergeSource.value.id, mergeTargetId.value)
    ElMessage.success('合并成功')
    mergeDialogVisible.value = false
    fetchTags()
  } catch (error) {
  } finally {
    merging.value = false
  }
}

// 格式化日期
const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

// 初始化
onMounted(() => {
  fetchTags()
})
</script>

<style scoped lang="scss">
.tag-manage {
  .toolbar-card,
  .table-card {
    margin-bottom: 20px;
  }

  .pagination {
    margin-top: 20px;
    display: flex;
    justify-content: flex-end;
  }
}
</style>
