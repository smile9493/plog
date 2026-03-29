<template>
  <div v-if="!item.meta?.hidden" class="sidebar-item">
    <!-- 单个菜单项 -->
    <el-menu-item v-if="!hasChildren" :index="resolvePath(item.path)">
      <el-icon v-if="item.meta?.icon">
        <component :is="item.meta.icon" />
      </el-icon>
      <template #title>
        <span>{{ item.meta?.title }}</span>
      </template>
    </el-menu-item>

    <!-- 有子菜单 -->
    <el-sub-menu v-else :index="resolvePath(item.path)">
      <template #title>
        <el-icon v-if="item.meta?.icon">
          <component :is="item.meta.icon" />
        </el-icon>
        <span>{{ item.meta?.title }}</span>
      </template>
      <sidebar-item
        v-for="child in item.children"
        :key="child.path"
        :item="child"
        :base-path="resolvePath(item.path)"
      />
    </el-sub-menu>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  item: any
  basePath: string
}

const props = defineProps<Props>()

// 判断是否有子菜单
const hasChildren = computed(() => {
  const children = props.item.children || []
  const showingChildren = children.filter((child: any) => !child.meta?.hidden)
  return showingChildren.length > 0
})

// 解析路径
const resolvePath = (path: string) => {
  // 如果是绝对路径,直接返回
  if (path.startsWith('/')) {
    return path
  }
  
  // 如果basePath是根路径,直接返回path
  if (props.basePath === '/') {
    return '/' + path
  }
  
  // 否则拼接路径
  if (props.basePath.endsWith('/')) {
    return props.basePath + path
  }
  return props.basePath + '/' + path
}
</script>

<style scoped lang="scss">
.sidebar-item {
  display: contents;
}
</style>
