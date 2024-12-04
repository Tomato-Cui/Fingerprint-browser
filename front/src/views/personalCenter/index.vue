<template>
  <div class="personal-center">
    <el-menu
      ellipsis
      class="el-menu-popper-demo"
      mode="horizontal"
      :default-active="router.currentRoute.value.path"
      :popper-offset="16"
      style=""
      @select="handleSelect"
    >
      <el-menu-item index="/personalCenter/account">我的账户</el-menu-item>
      <el-menu-item index="/personalCenter/preference">偏好设置</el-menu-item>
      <el-menu-item index="/personalCenter/local">本地设置</el-menu-item>
    </el-menu>
    <Account
      v-if="router.currentRoute.value.path === '/personalCenter/account'"
    />
    <Preference
      v-if="router.currentRoute.value.path === '/personalCenter/preference'"
    />
    <Local v-if="router.currentRoute.value.path === '/personalCenter/local'" />
  </div>
</template>

<script setup lang="ts">
import Account from "./Account/index.vue";
import Preference from "./Preference/index.vue";
import Local from "./Local/index.vue";

import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const activeName = ref("first");

const handleSelect = (index: string) => {
  //   console.log("index--:", index);
  router.push(index);
};
</script>

<style scoped lang="less">
.personal-center {
  height: 100%;
  width: 100%;
  background-color: #fff;
  .el-menu-popper-demo {
    // height: 100px;
    :deep(.el-menu-item) {
      font-size: 18px;
    }
  }
}
</style>
