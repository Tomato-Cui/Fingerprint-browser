<template>
  <div class="h-main">
   
      <!-- <div>我的账户</div>
      <div>偏好设置</div>
      <div>本地设置</div> -->
      <!-- <Account /> -->
       <Preference />
    <!-- <Account
      v-if="router.currentRoute.value.path === '/personalCenter/account'"
    />
    <Preference
      v-if="router.currentRoute.value.path === '/personalCenter/preference'"
    />
    <Local v-if="router.currentRoute.value.path === '/personalCenter/local'" /> -->
  </div>
</template>

<script setup lang="ts">
import Account from "./Account/index.vue";
import Preference from "./Preference/index.vue";
import Local from "./Local/index.vue";

import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
</script>

<style scoped lang="less">

</style>
