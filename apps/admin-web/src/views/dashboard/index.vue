<template>
  <div class="dashboard">
    <div class="page-header">
      <h1 class="page-title">📊 仪表盘</h1>
    </div>
    
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon" style="background: #eff6ff;">📄</div>
        <div class="stat-value">{{ stats.posts }}</div>
        <div class="stat-label">文章总数</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #f0fdf4;">💬</div>
        <div class="stat-value">{{ stats.comments }}</div>
        <div class="stat-label">评论总数</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #fefce8;">👥</div>
        <div class="stat-value">{{ stats.users }}</div>
        <div class="stat-label">用户总数</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #fdf2f8;">👁️</div>
        <div class="stat-value">{{ stats.views }}</div>
        <div class="stat-label">总浏览量</div>
      </div>
    </div>
    
    <div class="cards-row">
      <div class="card">
        <div class="card-title">📝 最新文章</div>
        <div v-if="recentPosts.length === 0" class="empty-state">暂无文章</div>
        <table v-else class="data-table">
          <thead>
            <tr>
              <th>标题</th>
              <th>分类</th>
              <th>状态</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="post in recentPosts" :key="post.id">
              <td>{{ post.title }}</td>
              <td>{{ post.category_name || '-' }}</td>
              <td>
                <span :class="['tag', post.hide === 'n' ? 'tag-success' : 'tag-info']">
                  {{ post.hide === 'n' ? '已发布' : '草稿' }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <div class="card">
        <div class="card-title">💬 最新评论</div>
        <div v-if="recentComments.length === 0" class="empty-state">暂无评论</div>
        <table v-else class="data-table">
          <thead>
            <tr>
              <th>作者</th>
              <th>内容</th>
              <th>状态</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="comment in recentComments" :key="comment.id">
              <td>{{ comment.author }}</td>
              <td class="comment-content">{{ comment.content }}</td>
              <td>
                <span :class="['tag', getCommentStatusClass(comment.hide)]">
                  {{ getCommentStatusText(comment.hide) }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { postApi } from '@/api/post'
import { commentApi } from '@/api/comment'
import { userApi } from '@/api/user'

const stats = reactive({
  posts: 0,
  comments: 0,
  users: 0,
  views: '0'
})

const recentPosts = ref<any[]>([])
const recentComments = ref<any[]>([])

const getCommentStatusClass = (status: string) => {
  const map: Record<string, string> = {
    n: 'tag-success',
    y: 'tag-warning',
    spam: 'tag-danger'
  }
  return map[status] || 'tag-info'
}

const getCommentStatusText = (status: string) => {
  const map: Record<string, string> = {
    n: '通过',
    y: '待审',
    spam: '垃圾'
  }
  return map[status] || status
}

const loadDashboardData = async () => {
  try {
    const [postsRes, commentsRes, usersRes] = await Promise.all([
      postApi.getList({ page: 1, per_page: 5 }),
      commentApi.getList({ page: 1, per_page: 5 }),
      userApi.getList({ page: 1, per_page: 1 })
    ])
    
    if (postsRes.data) {
      stats.posts = postsRes.pagination?.total || 0
      recentPosts.value = postsRes.data.slice(0, 5)
    }
    
    if (commentsRes.data) {
      stats.comments = commentsRes.pagination?.total || 0
      recentComments.value = commentsRes.data.slice(0, 5)
    }
    
    if (usersRes.data) {
      stats.users = usersRes.pagination?.total || 0
    }
  } catch (error) {
    console.error('加载仪表盘数据失败:', error)
  }
}

onMounted(() => {
  loadDashboardData()
})
</script>

<style scoped>
.dashboard {
  max-width: 1300px;
}

.page-header {
  background: var(--card-bg);
  padding: 20px;
  border-radius: var(--radius);
  margin-bottom: 24px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--primary);
  margin: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: var(--card-bg);
  border-radius: var(--radius);
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
}

.stat-icon {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  margin-bottom: 12px;
}

.stat-value {
  font-size: 30px;
  font-weight: 700;
  letter-spacing: -0.5px;
  color: var(--text);
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
  margin-top: 4px;
}

.cards-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

.card {
  background: var(--card-bg);
  border-radius: var(--radius);
  padding: 24px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
}

.card-title {
  font-size: 17px;
  font-weight: 700;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
}

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

.comment-content {
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tag {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
}

.tag-success {
  background: #d1fae5;
  color: #065f46;
}

.tag-warning {
  background: #fef3c7;
  color: #92400e;
}

.tag-danger {
  background: #fee2e2;
  color: #991b1b;
}

.tag-info {
  background: #dbeafe;
  color: #1e40af;
}

@media (max-width: 768px) {
  .cards-row {
    grid-template-columns: 1fr;
  }
  
  .stats-grid {
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  }
}
</style>