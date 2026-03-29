<template>
  <div class="post-list">
    <!-- 搜索和操作栏 -->
    <el-card class="search-card">
      <el-form :inline="true" :model="queryParams" class="search-form">
        <el-form-item label="关键词">
          <el-input
            v-model="queryParams.keyword"
            placeholder="搜索文章标题"
            clearable
            @clear="handleSearch"
            @keyup.enter="handleSearch"
          />
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="queryParams.status" placeholder="选择状态" clearable>
            <el-option label="全部" value="" />
            <el-option label="已发布" value="published" />
            <el-option label="草稿" value="draft" />
            <el-option label="已归档" value="archived" />
          </el-select>
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="queryParams.category_id" placeholder="选择分类" clearable>
            <el-option label="全部" :value="undefined" />
            <el-option
              v-for="category in categories"
              :key="category.id"
              :label="category.name"
              :value="category.id"
            />
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

    <!-- 操作按钮 -->
    <el-card class="toolbar-card">
      <el-button type="primary" @click="handleCreate">
        <el-icon><Plus /></el-icon>
        写文章
      </el-button>
      <el-button
        type="danger"
        :disabled="selectedIds.length === 0"
        @click="handleBatchDelete"
      >
        <el-icon><Delete /></el-icon>
        批量删除
      </el-button>
      <el-button
        type="success"
        :disabled="selectedIds.length === 0"
        @click="handleBatchPublish"
      >
        <el-icon><Upload /></el-icon>
        批量发布
      </el-button>
    </el-card>

    <!-- 文章列表 -->
    <el-card class="table-card">
      <el-table
        v-loading="loading"
        :data="postList"
        style="width: 100%"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="title" label="标题" min-width="200">
          <template #default="{ row }">
            <el-link v-if="row" type="primary" @click="handleEdit(row.id)">{{ row.title }}</el-link>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag v-if="row" :type="getStatusType(row.status)">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="category" label="分类" width="120">
          <template #default="{ row }">
            {{ row?.category?.name || '未分类' }}
          </template>
        </el-table-column>
        <el-table-column prop="views" label="浏览" width="80" />
        <el-table-column prop="created_at" label="发布时间" width="180">
          <template #default="{ row }">
            {{ row ? formatDate(row.created_at) : '' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button v-if="row" type="primary" link @click="handleEdit(row.id)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button
              v-if="row && row.status === 'draft'"
              type="success"
              link
              @click="handlePublish(row.id)"
            >
              <el-icon><Upload /></el-icon>
              发布
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
        :page-sizes="[10, 20, 50, 100]"
        :total="total"
        layout="total, sizes, prev, pager, next, jumper"
        class="pagination"
        @size-change="handleSearch"
        @current-change="handleSearch"
      />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Refresh, Plus, Delete, Upload, Edit } from '@element-plus/icons-vue'
import { postApi } from '@/api/post'
import { categoryApi } from '@/api/category'
import type { Post, Category, PostQueryParams } from '@/types'
import dayjs from 'dayjs'

const router = useRouter()

// 加载状态
const loading = ref(false)

// 文章列表
const postList = ref<Post[]>([])
const total = ref(0)

// 分类列表
const categories = ref<Category[]>([])

// 选中的文章ID
const selectedIds = ref<number[]>([])

// 查询参数
const queryParams = reactive<PostQueryParams>({
  page: 1,
  per_page: 10,
  keyword: '',
  status: '',
  category_id: undefined
})

// 获取文章列表
const fetchPosts = async () => {
  loading.value = true
  try {
    const params = { ...queryParams }
    // 移除空值参数
    Object.keys(params).forEach((key) => {
      if (params[key as keyof PostQueryParams] === '' || params[key as keyof PostQueryParams] === undefined) {
        delete params[key as keyof PostQueryParams]
      }
    })
    
    const res = await postApi.getList(params)
    postList.value = res.items
    total.value = res.total
  } catch (error) {
    ElMessage.error('获取文章列表失败')
  } finally {
    loading.value = false
  }
}

// 获取分类列表
const fetchCategories = async () => {
  try {
    const res = await categoryApi.getAll()
    categories.value = res
  } catch (error) {
    // 静默失败
  }
}

// 搜索
const handleSearch = () => {
  queryParams.page = 1
  fetchPosts()
}

// 重置
const handleReset = () => {
  queryParams.keyword = ''
  queryParams.status = ''
  queryParams.category_id = undefined
  queryParams.page = 1
  fetchPosts()
}

// 创建文章
const handleCreate = () => {
  router.push('/post/create')
}

// 编辑文章
const handleEdit = (id: number) => {
  router.push(`/post/edit/${id}`)
}

// 发布文章
const handlePublish = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要发布这篇文章吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await postApi.publish(id)
    ElMessage.success('发布成功')
    fetchPosts()
  } catch (error) {
    // 取消或失败
  }
}

// 删除文章
const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这篇文章吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await postApi.delete(id)
    ElMessage.success('删除成功')
    fetchPosts()
  } catch (error) {
    // 取消或失败
  }
}

// 批量删除
const handleBatchDelete = async () => {
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedIds.value.length} 篇文章吗？`, '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await postApi.batchDelete(selectedIds.value)
    ElMessage.success('批量删除成功')
    selectedIds.value = []
    fetchPosts()
  } catch (error) {
    // 取消或失败
  }
}

// 批量发布
const handleBatchPublish = async () => {
  try {
    await ElMessageBox.confirm(`确定要发布选中的 ${selectedIds.value.length} 篇文章吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await postApi.batchPublish(selectedIds.value)
    ElMessage.success('批量发布成功')
    selectedIds.value = []
    fetchPosts()
  } catch (error) {
    // 取消或失败
  }
}

// 选择变化
const handleSelectionChange = (selection: Post[]) => {
  selectedIds.value = selection.map((item) => item.id)
}

// 获取状态类型
const getStatusType = (status: string) => {
  const types: Record<string, string> = {
    published: 'success',
    draft: 'info',
    archived: 'warning'
  }
  return types[status] || 'info'
}

// 获取状态文本
const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    published: '已发布',
    draft: '草稿',
    archived: '已归档'
  }
  return texts[status] || status
}

// 格式化日期
const formatDate = (date: string) => {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

// 初始化
onMounted(() => {
  fetchPosts()
  fetchCategories()
})
</script>

<style scoped lang="scss">
.post-list {
  .search-card,
  .toolbar-card,
  .table-card {
    margin-bottom: 20px;
  }

  .search-form {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }

  .pagination {
    margin-top: 20px;
    display: flex;
    justify-content: flex-end;
  }
}
</style>
