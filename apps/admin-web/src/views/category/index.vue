<template>
  <div class="category-manage">
    <!-- 操作栏 -->
    <el-card class="toolbar-card">
      <el-button type="primary" @click="handleCreate">
        <el-icon><Plus /></el-icon>
        添加分类
      </el-button>
    </el-card>

    <!-- 分类列表 -->
    <el-card class="table-card">
      <el-table v-loading="loading" :data="categoryList" style="width: 100%">
        <el-table-column prop="name" label="分类名称" min-width="150" />
        <el-table-column prop="slug" label="别名" width="150" />
        <el-table-column prop="description" label="描述" min-width="200">
          <template #default="scope">
            {{ scope?.row?.description || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="post_count" label="文章数" width="100" />
        <el-table-column prop="created_at" label="创建时间" width="180">
          <template #default="scope">
            {{ scope?.row ? formatDate(scope.row.created_at) : '' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="scope">
            <el-button v-if="scope?.row" type="primary" link @click="handleEdit(scope.row)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button v-if="scope?.row" type="danger" link @click="handleDelete(scope.row.id)">
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
        @size-change="fetchCategories"
        @current-change="fetchCategories"
      />
    </el-card>

    <!-- 编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑分类' : '添加分类'"
      width="500px"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="80px"
      >
        <el-form-item label="名称" prop="name">
          <el-input v-model="form.name" placeholder="请输入分类名称" />
        </el-form-item>
        <el-form-item label="别名" prop="slug">
          <el-input v-model="form.slug" placeholder="请输入分类别名（可选）" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input
            v-model="form.description"
            type="textarea"
            :rows="3"
            placeholder="请输入分类描述（可选）"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="submitting">
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
import { Plus, Edit, Delete } from '@element-plus/icons-vue'
import { categoryApi } from '@/api/category'
import type { Category, PaginationParams } from '@/types'
import dayjs from 'dayjs'

// 加载状态
const loading = ref(false)
const submitting = ref(false)

// 分类列表
const categoryList = ref<Category[]>([])
const total = ref(0)

// 查询参数
const queryParams = reactive<PaginationParams>({
  page: 1,
  per_page: 10
})

// 对话框
const dialogVisible = ref(false)
const isEdit = ref(false)
const editId = ref<number>()

// 表单引用
const formRef = ref<FormInstance>()

// 表单数据
const form = reactive({
  name: '',
  slug: '',
  description: ''
})

// 表单验证规则
const rules: FormRules = {
  name: [
    { required: true, message: '请输入分类名称', trigger: 'blur' },
    { min: 2, max: 50, message: '名称长度在 2 到 50 个字符', trigger: 'blur' }
  ],
  slug: [
    { pattern: /^[a-z0-9-]+$/, message: '别名只能包含小写字母、数字和连字符', trigger: 'blur' }
  ]
}

// 获取分类列表
const fetchCategories = async () => {
  loading.value = true
  try {
    const res = await categoryApi.getList(queryParams)
    categoryList.value = res.items
    total.value = res.total
  } catch (error) {
    
  } finally {
    loading.value = false
  }
}

// 创建分类
const handleCreate = () => {
  isEdit.value = false
  editId.value = undefined
  form.name = ''
  form.slug = ''
  form.description = ''
  dialogVisible.value = true
}

// 编辑分类
const handleEdit = (category: Category) => {
  isEdit.value = true
  editId.value = category.id
  form.name = category.name
  form.slug = category.slug
  form.description = category.description || ''
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
          await categoryApi.update(editId.value, form)
          ElMessage.success('更新成功')
        } else {
          await categoryApi.create(form)
          ElMessage.success('创建成功')
        }
        dialogVisible.value = false
        fetchCategories()
      } catch (error) {
      } finally {
        submitting.value = false
      }
    }
  })
}

// 删除分类
const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个分类吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await categoryApi.delete(id)
    ElMessage.success('删除成功')
    fetchCategories()
  } catch (error) {
    // 取消或失败
  }
}

// 格式化日期
const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

// 初始化
onMounted(() => {
  fetchCategories()
})
</script>

<style scoped lang="scss">
.category-manage {
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
