<template>
  <div class="error-page">
    <div class="error-content">
      <h1>404</h1>
      <p>页面不存在</p>
      <el-button type="primary" @click="$router.push('/')">返回首页</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped lang="scss">
.error-page {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f0f2f5;

  .error-content {
    text-align: center;

    h1 {
      font-size: 80px;
      color: #409eff;
      margin: 0;
    }

    p {
      font-size: 20px;
      color: #666;
      margin: 20px 0 30px;
    }
  }
}
</style>
