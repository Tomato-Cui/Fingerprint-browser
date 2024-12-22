<template>
  <el-container class="container">
    <Layout>
      <!-- 头部导航栏 -->
      <HomeHeader />

      <el-container class="main-content">
        <!-- 左侧菜单栏 -->

        <HomeMenu />

        <!-- 右侧内容区域 -->
        <HomeComtent />
      </el-container>
    </Layout>
  </el-container>
</template>

<script setup lang="ts">
import HomeHeader from "@/views/Home/HomeHeader.vue"; // 引入主内容组件头部导航栏组件引入左侧菜单栏组件
// import MainView from '../src/components/MainView.vue'; // 引入右侧内容组件
import HomeMenu from "@/views/Home/HomeMenu.vue"; // 引入左侧菜单栏组件
import HomeComtent from "@/views/Home/HomeComtent.vue"; // 引入右侧内容组件
import Layout from "@/layouts/index.vue";
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden; /* 防止溢出，确保右侧内容显示 */
  padding: 0px 0px 3.33% 3.33%;
  /* background-color: #f5fafe; */
  background-color: #fd7272;
}
</style>
