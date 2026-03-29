<template>
  <div class="post-edit">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>{{ isEdit ? '编辑文章' : '写文章' }}</span>
          <div class="header-actions">
            <el-button @click="handleSaveDraft" :loading="saving">
              保存草稿
            </el-button>
            <el-button type="primary" @click="handlePublish" :loading="publishing">
              {{ form.status === 'published' ? '更新' : '发布' }}
            </el-button>
          </div>
        </div>
      </template>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="80px"
        class="post-form"
      >
        <el-form-item label="标题" prop="title">
          <el-input
            v-model="form.title"
            placeholder="请输入文章标题"
            maxlength="200"
            show-word-limit
          />
        </el-form-item>

        <el-form-item label="内容" prop="content">
          <MdEditor
            v-model="form.content"
            :theme="editorTheme"
            :previewTheme="previewTheme"
            :codeTheme="codeTheme"
            :style="{ height: '500px' }"
            @onUploadImg="handleUploadImg"
          />
        </el-form-item>

        <el-form-item label="摘要">
          <el-input
            v-model="form.excerpt"
            type="textarea"
            :rows="3"
            placeholder="请输入文章摘要（可选）"
            maxlength="300"
            show-word-limit
          />
        </el-form-item>

        <el-form-item label="封面图">
          <el-upload
            class="cover-uploader"
            :action="uploadUrl"
            :headers="uploadHeaders"
            :show-file-list="false"
            :on-success="handleCoverSuccess"
            :before-upload="beforeCoverUpload"
          >
            <img v-if="form.cover" :src="form.cover" class="cover-image" />
            <el-icon v-else class="cover-uploader-icon"><Plus /></el-icon>
          </el-upload>
          <el-button v-if="form.cover" type="danger" link @click="form.cover = ''">
            删除封面
          </el-button>
        </el-form-item>

        <el-form-item label="分类">
          <el-select v-model="form.category_id" placeholder="选择分类" clearable>
            <el-option
              v-for="category in categories"
              :key="category.id"
              :label="category.name"
              :value="category.id"
            />
          </el-select>
        </el-form-item>

        <el-form-item label="标签">
          <el-select
            v-model="form.tag_ids"
            multiple
            filterable
            allow-create
            default-first-option
            placeholder="选择或创建标签"
          >
            <el-option
              v-for="tag in tags"
              :key="tag.id"
              :label="tag.name"
              :value="tag.id"
            />
          </el-select>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { FormInstance, FormRules, UploadProps } from 'element-plus'
import { ElMessage } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { MdEditor } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'
import { postApi } from '@/api/post'
import { categoryApi } from '@/api/category'
import { tagApi } from '@/api/tag'
import type { Category, Tag, PostForm } from '@/types'

const route = useRoute()
const router = useRouter()

// 编辑器主题
const editorTheme = ref<'light' | 'dark'>('light')
const previewTheme = ref<'default' | 'github' | 'vuepress' | 'mk-cute' | 'smart-blue'>('github')
const codeTheme = ref<'atom' | 'a11y' | 'github' | 'gradient' | 'kimbie' | 'paraiso' | 'qtcreator' | 'stackoverflow'>('atom')

// 表单引用
const formRef = ref<FormInstance>()

// 加载状态
const loading = ref(false)
const saving = ref(false)
const publishing = ref(false)

// 分类和标签
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])

// 是否为编辑模式
const isEdit = computed(() => !!route.params.id)

// 文章ID
const postId = computed(() => Number(route.params.id))

// 表单数据
const form = reactive<PostForm>({
  title: '',
  content: '',
  excerpt: '',
  cover: '',
  category_id: undefined,
  tag_ids: [],
  status: 'draft'
})

// 表单验证规则
const rules: FormRules = {
  title: [
    { required: true, message: '请输入文章标题', trigger: 'blur' },
    { min: 2, max: 200, message: '标题长度在 2 到 200 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入文章内容', trigger: 'blur' }
  ]
}

// 上传配置
const uploadUrl = computed(() => import.meta.env.VITE_API_BASE_URL + '/api/media/upload')
const uploadHeaders = computed(() => ({
  Authorization: `Bearer ${localStorage.getItem('token')}`
}))

// 获取文章详情
const fetchPost = async () => {
  if (!isEdit.value) return
  
  loading.value = true
  try {
    const post = await postApi.getDetail(postId.value)
    form.title = post.title
    form.content = post.content
    form.excerpt = post.excerpt || ''
    form.cover = post.cover || ''
    form.category_id = post.category_id
    form.tag_ids = post.tags?.map(tag => tag.id) || []
    form.status = post.status as 'draft' | 'published'
  } catch (error) {
    ElMessage.error('获取文章详情失败')
    router.push('/post/list')
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

// 获取标签列表
const fetchTags = async () => {
  try {
    const res = await tagApi.getAll()
    tags.value = res
  } catch (error) {
    // 静默失败
  }
}

// 保存草稿
const handleSaveDraft = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      saving.value = true
      try {
        form.status = 'draft'
        if (isEdit.value) {
          await postApi.update(postId.value, form)
        } else {
          const res = await postApi.create(form)
          router.replace(`/post/edit/${res.id}`)
        }
        ElMessage.success('保存成功')
      } catch (error) {
        ElMessage.error('保存失败')
      } finally {
        saving.value = false
      }
    }
  })
}

// 发布文章
const handlePublish = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      publishing.value = true
      try {
        form.status = 'published'
        if (isEdit.value) {
          await postApi.update(postId.value, form)
        } else {
          await postApi.create(form)
        }
        ElMessage.success(isEdit.value ? '更新成功' : '发布成功')
        router.push('/post/list')
      } catch (error) {
        ElMessage.error(isEdit.value ? '更新失败' : '发布失败')
      } finally {
        publishing.value = false
      }
    }
  })
}

// 上传图片
const handleUploadImg = async (files: File[], callback: (urls: string[]) => void) => {
  const formData = new FormData()
  formData.append('file', files[0])
  
  try {
    const response = await fetch(uploadUrl.value, {
      method: 'POST',
      headers: uploadHeaders.value,
      body: formData
    })
    const result = await response.json()
    if (result.code === 200 || result.code === 0) {
      callback([result.data.url])
    } else {
      ElMessage.error('图片上传失败')
    }
  } catch (error) {
    ElMessage.error('图片上传失败')
  }
}

// 封面上传成功
const handleCoverSuccess: UploadProps['onSuccess'] = (response) => {
  if (response.code === 200 || response.code === 0) {
    form.cover = response.data.url
    ElMessage.success('封面上传成功')
  } else {
    ElMessage.error('封面上传失败')
  }
}

// 封面上传前验证
const beforeCoverUpload: UploadProps['beforeUpload'] = (file) => {
  const isImage = file.type.startsWith('image/')
  const isLt2M = file.size / 1024 / 1024 < 2

  if (!isImage) {
    ElMessage.error('只能上传图片文件!')
    return false
  }
  if (!isLt2M) {
    ElMessage.error('图片大小不能超过 2MB!')
    return false
  }
  return true
}

// 初始化
onMounted(() => {
  fetchCategories()
  fetchTags()
  fetchPost()
})
</script>

<style scoped lang="scss">
.post-edit {
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .header-actions {
      display: flex;
      gap: 10px;
    }
  }

  .post-form {
    .cover-uploader {
      :deep(.el-upload) {
        border: 1px dashed #d9d9d9;
        border-radius: 6px;
        cursor: pointer;
        position: relative;
        overflow: hidden;
        transition: border-color 0.3s;

        &:hover {
          border-color: #409eff;
        }
      }

      .cover-image {
        width: 200px;
        height: 150px;
        display: block;
        object-fit: cover;
      }

      .cover-uploader-icon {
        font-size: 28px;
        color: #8c939d;
        width: 200px;
        height: 150px;
        text-align: center;
        line-height: 150px;
      }
    }
  }
}
</style>
