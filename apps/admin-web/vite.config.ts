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
    // 禁用代码分割，打包成单个文件
    rollupOptions: {
      output: {
        manualChunks: undefined,
      }
    },
    chunkSizeWarningLimit: 2000,
    // 禁用模块预加载
    modulePreload: false
  }
})
