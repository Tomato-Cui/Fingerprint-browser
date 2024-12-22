<template>
  <template v-if="route.path === '/'">
    <Login />
  </template>
  <template v-else>
    <TooltipProvider>
      <Layout>
        <router-view />
      </Layout>
    </TooltipProvider>
  </template>
  <Toaster :rich-colors="true" position="top-center" />
</template>

<script setup>
import Layout from '@/components/layout/index.vue'
import { TooltipProvider } from '@/components/ui/tooltip'
import { useRoute } from 'vue-router';
import { Toaster } from '@/components/ui/sonner'
import { isLogin } from '@/commands/user'
import { onMounted, ref, watch } from 'vue';
import Login from '@/views/Login/index.vue';

const route = useRoute();
</script>

<style style>
/* 全局修改滚动条样式 */
::-webkit-scrollbar {
  width: 4px;
  height: 8px;
  /* 设置滚动条宽度 */
}

/* 滚动条轨道的背景 */
::-webkit-scrollbar-track {
  background: #ffffff;
  /* 轨道背景色 */
  border-radius: 10px;
  /* 可选：圆角效果 */
}

/* 滚动条的滑块部分 */
::-webkit-scrollbar-thumb {
  background: #d2d2d2;
  /* 滑块背景色 */
  border-radius: 10px;
  /* 可选：圆角效果 */
}

/* 鼠标悬停时滑块的背景色 */
::-webkit-scrollbar-thumb:hover {
  background: #a3a1a1;
  /* 鼠标悬停时的滑块背景色 */
}

/* 在容器上悬停时显示滚动条 */
*:hover::-webkit-scrollbar {
  width: 4px;
  /* 滚动条宽度 */
}
</style>