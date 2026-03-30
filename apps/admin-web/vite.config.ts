import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  },
  server: {
    port: 3000,
    open: true,
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: 'dist',
    sourcemap: false,
    rollupOptions: {
      output: {
        chunkFileNames: 'js/[name]-[hash].js',
        entryFileNames: 'js/[name]-[hash].js',
        assetFileNames: '[ext]/[name]-[hash].[ext]',
        manualChunks(id) {
          // Element Plus单独打包
          if (id.includes('element-plus')) {
            return 'element-plus'
          }
          // Vue相关库单独打包
          if (id.includes('vue') || id.includes('pinia') || id.includes('vue-router')) {
            return 'vue-vendor'
          }
          // Markdown编辑器单独打包
          if (id.includes('md-editor-v3')) {
            return 'md-editor'
          }
          // 代码高亮相关
          if (id.includes('highlight.js') || id.includes('cod!md-editor-v3')) {
            return 'code-highlight'
          }
        }
      }
    },
    // 提高chunk大小警告阈值
    chunkSizeWarningLimit: 1000
  }
})
