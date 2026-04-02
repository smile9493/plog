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
          </el-select>
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="queryParams.category_id" placeholder="选择分类" clearable>
            <el-option label="全部" :value="undefined" />
            <el-option
              v-for="category in categories"
              :key="category.sid"
              :label="category.sortname"
              :value="category.sid"
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
            <el-link type="primary" @click="handleEdit(row.gid)">{{ row.title }}</el-link>
          </template>
        </el-table-column>
        <el-table-column prop="hide" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.hide)">
              {{ getStatusText(row.hide) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="sortid" label="分类" width="120">
          <template #default="{ row }">
            {{ getCategoryName(row.sortid) }}
          </template>
        </el-table-column>
        <el-table-column prop="views" label="浏览" width="80" />
        <el-table-column prop="comnum" label="评论" width="80" />
        <el-table-column prop="date" label="发布时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.date) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="handleEdit(row.gid)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button
              v-if="row.hide === 'y'"
              type="success"
              link
              @click="handlePublish(row.gid)"
            >
              <el-icon><Upload /></el-icon>
              发布
            </el-button>
            <el-button type="danger" link @click="handleDelete(row.gid)">
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
import type { Post, Category, PostListParams } from '@/types'

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
const queryParams = reactive<PostListParams>({
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
    const res = await postApi.getList(queryParams)
    postList.value = res.items
    total.value = res.pagination.total
  } catch (error) {
    
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
  selectedIds.value = selection.map((item) => item.gid)
}

// 获取状态类型
const getStatusType = (hide: string) => {
  return hide === 'n' ? 'success' : 'info'
}

// 获取状态文本
const getStatusText = (hide: string) => {
  return hide === 'n' ? '已发布' : '草稿'
}

// 获取分类名称
const getCategoryName = (sortid: number) => {
  const category = categories.value.find(c => c.sid === sortid)
  return category?.sortname || '未分类'
}

// 格式化日期
const formatDate = (timestamp: number) => {
  return new Date(timestamp * 1000).toLocaleString()
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
