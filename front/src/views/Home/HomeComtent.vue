<template>
    <!-- 右侧主要内容区域容器 -->
    <div class="aside">
        <!-- 内容模板容器 -->
        <div class="template">
            <!-- 路由视图容器，用于显示当前路由对应的组件内容 -->
            <main class="main-content">
                <router-view />
            </main>
        </div>
    </div>
   
</template>
  
<script setup lang="ts">
// 使用 setup 语法糖的 TypeScript 组件
// 该组件作为主页面的右侧内容区域，负责展示路由对应的具体内容
</script>
  
<style scoped>
/* 右侧区域样式 */
.aside {
    flex: 1; /* 使用弹性布局，占据剩余所有空间 */
    background-color: rgb(125, 188, 250); /* 设置浅蓝色背景 */
    overflow-y: auto; /* 内容超出时显示垂直滚动条 */
    position: relative; /* 设置相对定位，作为子元素定位的参考 */
}

/* 内容模板容器样式 */
.template {
    position: absolute; /* 绝对定位 */
    width: 1130px; /* 固定宽度 */
    height: 649px; /* 固定高度 */
    border-radius: 16px; /* 圆角边框 */
    opacity: 1; /* 完全不透明 */
    margin-left: 14px; /* 左侧外边距 */
    background: #ffffff; /* 白色背景 */
}
</style>