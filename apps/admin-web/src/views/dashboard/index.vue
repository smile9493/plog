<template>
  <div class="dashboard">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <el-icon class="stat-icon" style="color: #409eff;"><Document /></el-icon>
            <div class="stat-info">
              <div class="stat-value">{{ stats.posts }}</div>
              <div class="stat-label">文章总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <el-icon class="stat-icon" style="color: #67c23a;"><ChatDotSquare /></el-icon>
            <div class="stat-info">
              <div class="stat-value">{{ stats.comments }}</div>
              <div class="stat-label">评论总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <el-icon class="stat-icon" style="color: #e6a23c;"><User /></el-icon>
            <div class="stat-info">
              <div class="stat-value">{{ stats.users }}</div>
              <div class="stat-label">用户总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-content">
            <el-icon class="stat-icon" style="color: #f56c6c;"><View /></el-icon>
            <div class="stat-info">
              <div class="stat-value">{{ stats.views }}</div>
              <div class="stat-label">访问总量</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="16">
        <el-card>
          <template #header>
            <span>最近文章</span>
          </template>
          <el-table :data="recentPosts" style="width: 100%">
            <el-table-column prop="title" label="标题" />
            <el-table-column prop="status" label="状态" width="100">
              <template #default="scope">
                <el-tag v-if="scope?.row" :type="scope.row.status === 'published' ? 'success' : 'info'">
                  {{ scope.row.status === 'published' ? '已发布' : '草稿' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="created_at" label="发布时间" width="180" />
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card>
          <template #header>
            <span>快捷操作</span>
          </template>
          <div class="quick-actions">
            <el-button type="primary" @click="$router.push('/post/create')">
              <el-icon><Edit /></el-icon>
              写文章
            </el-button>
            <el-button @click="$router.push('/media')">
              <el-icon><Upload /></el-icon>
              上传文件
            </el-button>
            <el-button @click="$router.push('/category')">
              <el-icon><FolderAdd /></el-icon>
              添加分类
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Document, ChatDotSquare, User, View, Edit, Upload, FolderAdd } from '@element-plus/icons-vue'

const stats = ref({
  posts: 0,
  comments: 0,
  users: 0,
  views: 0
})

const recentPosts = ref([
  {
    title: '欢迎使用 Plog CMS',
    status: 'published',
    created_at: '2026-03-28 10:00:00'
  }
])

onMounted(() => {
  // TODO: 从 API 获取统计数据
  stats.value = {
    posts: 1,
    comments: 0,
    users: 1,
    views: 100
  }
})
</script>

<style scoped lang="scss">
.dashboard {
  .stat-card {
    .stat-content {
      display: flex;
      align-items: center;

      .stat-icon {
        font-size: 48px;
        margin-right: 20px;
      }

      .stat-info {
        .stat-value {
          font-size: 24px;
          font-weight: bold;
          color: #333;
        }

        .stat-label {
          font-size: 14px;
          color: #999;
          margin-top: 5px;
        }
      }
    }
  }

  .quick-actions {
    display: flex;
    flex-direction: column;
    gap: 10px;

    .el-button {
      width: 100%;
      justify-content: flex-start;
    }
  }
}
</style>
