<template>
  <div class="theme-manage">
    <el-card class="toolbar-card">
      <el-button type="primary" @click="handleRefresh">
        <el-icon><Refresh /></el-icon>
        刷新
      </el-button>
    </el-card>

    <el-card class="theme-card">
      <div class="theme-grid">
        <div
          v-for="theme in themeList"
          :key="theme.name"
          class="theme-item"
          :class="{ active: theme.active }"
        >
          <div class="theme-preview">
            <div class="no-preview">
              <el-icon><Picture /></el-icon>
              <span>{{ theme.name }}</span>
            </div>
            <div v-if="theme.active" class="active-badge">
              <el-tag type="success">当前主题</el-tag>
            </div>
          </div>
          
          <div class="theme-info">
            <h3 class="theme-name">{{ theme.name }}</h3>
            <p class="theme-description">{{ theme.description }}</p>
          </div>
          
          <div class="theme-actions">
            <el-button
              v-if="!theme.active"
              type="primary"
              size="small"
              @click="handleActivate(theme)"
            >
              激活
            </el-button>
            <el-button
              v-else
              type="success"
              size="small"
              disabled
            >
              已激活
            </el-button>
          </div>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Refresh, Picture } from '@element-plus/icons-vue'

interface Theme {
  name: string
  description: string
  active: boolean
}

const themeList = ref<Theme[]>([])

const fetchThemes = async () => {
  themeList.value = [
    {
      name: 'zen',
      description: '极简博客主题 - 专注于内容阅读体验',
      active: true
    }
  ]
}

const handleRefresh = () => {
  fetchThemes()
}

const handleActivate = (theme: Theme) => {
  themeList.value.forEach(t => t.active = t.name === theme.name)
}

onMounted(() => {
  fetchThemes()
})
</script>

<style scoped lang="scss">
.theme-manage {
  .toolbar-card,
  .theme-card {
    margin-bottom: 20px;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 20px;

    .theme-item {
      border: 1px solid #dcdfe6;
      border-radius: 8px;
      overflow: hidden;
      transition: all 0.3s;

      &:hover {
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      }

      &.active {
        border-color: #409eff;
        box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
      }

      .theme-preview {
        position: relative;
        width: 100%;
        height: 160px;
        background: linear-gradient(135deg, #fafaf9 0%, #e7e5e4 100%);

        .no-preview {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          height: 100%;
          color: #78716c;

          .el-icon {
            font-size: 32px;
            margin-bottom: 8px;
          }

          span {
            font-family: 'Noto Serif SC', serif;
            font-size: 14px;
          }
        }

        .active-badge {
          position: absolute;
          top: 10px;
          right: 10px;
        }
      }

      .theme-info {
        padding: 16px;

        .theme-name {
          font-family: 'Noto Serif SC', serif;
          font-size: 16px;
          margin: 0 0 8px;
        }

        .theme-description {
          color: #78716c;
          font-size: 13px;
          margin: 0;
          line-height: 1.5;
        }
      }

      .theme-actions {
        padding: 12px 16px;
        border-top: 1px solid #e7e5e4;
      }
    }
  }
}
</style>
