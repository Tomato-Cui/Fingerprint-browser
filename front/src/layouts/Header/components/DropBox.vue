<template>
  <el-menu
    class="el-menu-popper-demo"
    mode="horizontal"
    :popper-offset="1"
    style="max-width: 600px"
  >
    <el-menu-item index="login" @click="router.push('/')"> 登录 </el-menu-item>
  </el-menu>
</template>

<script setup>
import { useRouter } from "vue-router";
const router = useRouter();
</script>

<style scoped>
.el-menu-popper-demo {
  padding: 0px;
  border: 0px;
  margin: 0px;
  width: 100%;
  height: 100%;
  justify-content: center; /* 水平居中 */
  align-items: center;
}
.el-sub-menu {
  padding: 0px;
  border: 0px;
  margin: 0px;
  height: 100%;
}
</style>
