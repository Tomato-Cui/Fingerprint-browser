<template>
  <div class="common-layout">
    <el-container class="container">
      <el-header>
        <HomeHeader />
      </el-header>
      <el-container class="main-content">
        <el-aside width="200px">
          <HomeAside />
        </el-aside>
        <el-main>
          <slot></slot>
          <!-- <HomeMain /> -->
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import HomeHeader from "@/layouts/Header/index.vue";
import HomeAside from "@/layouts/Aside/index.vue";
import HomeMain from "@/layouts/Main/index.vue";
</script>

<style scoped>
.container {
  /* display: flex; */
  flex-direction: column;
  height: 100vh;
  width: 100%;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden; /* 防止溢出，确保右侧内容显示 */
  padding: 0px 3.33% 0px 3.33%;
  background-color: #f5fafe;
  /* background-color: #fd7272; */
}

.el-header {
  padding: 0;
  height: 90px;
}
.el-main {
  padding: 0;
  overflow-y: auto;
}
</style>
