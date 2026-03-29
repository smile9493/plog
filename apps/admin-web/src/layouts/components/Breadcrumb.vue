<template>
  <el-breadcrumb class="breadcrumb" separator="/">
    <el-breadcrumb-item v-for="(item, index) in breadcrumbs" :key="item.path">
      <span v-if="index === breadcrumbs.length - 1" class="no-redirect">
        {{ item.meta?.title }}
      </span>
      <a v-else @click.prevent="handleLink(item)">
        {{ item.meta?.title }}
      </a>
    </el-breadcrumb-item>
  </el-breadcrumb>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()

const breadcrumbs = ref<any[]>([])

// 生成面包屑
const generateBreadcrumbs = () => {
  const matched = route.matched.filter(item => item.meta && item.meta.title)
  breadcrumbs.value = matched.filter(item => {
    return item.meta && item.meta.title && !item.meta.hidden
  })
}

// 点击链接
const handleLink = (item: any) => {
  const { path, redirect } = item
  if (redirect) {
    router.push(redirect)
    return
  }
  router.push(path)
}

// 监听路由变化
watch(
  () => route.path,
  () => {
    generateBreadcrumbs()
  },
  { immediate: true }
)
</script>

<style scoped lang="scss">
.breadcrumb {
  display: inline-block;
  font-size: 14px;
  line-height: 60px;

  .no-redirect {
    color: #97a8be;
    cursor: text;
  }

  a {
    color: #606266;
    cursor: pointer;

    &:hover {
      color: #409eff;
    }
  }
}
</style>
