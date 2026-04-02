<template>
  <div class="plugin-manage">
    <el-card class="empty-card">
      <div class="empty-state">
        <el-icon class="empty-icon"><Box /></el-icon>
        <p class="empty-text">暂无插件</p>
        <p class="empty-desc">插件功能开发中，敬请期待</p>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { Box } from '@element-plus/icons-vue'
</script>

<style scoped lang="scss">
.plugin-manage {
  .empty-card {
    margin-bottom: 20px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: #909399;

    .empty-icon {
      font-size: 48px;
      margin-bottom: 16px;
    }

    .empty-text {
      font-size: 16px;
      margin: 0 0 8px;
    }

    .empty-desc {
      font-size: 14px;
      margin: 0;
    }
  }
}
</style>
