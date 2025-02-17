<template>
  <More>
    <MoreTrigger>
      <OneFrameIcon class="w-5 h-5 cursor-pointer" />
    </MoreTrigger>
    <MoreContent>
      <MoreItem
        v-for="item in regularItems"
        :key="item"
        class="cursor-pointer"
        @click=""
      >
        {{ item }}
      </MoreItem>
    </MoreContent>
  </More>
</template>

<script setup lang="ts">
import { defineEmits } from "vue";
import { OneFrameIcon } from "@/assets/icons/environment/index";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";

const emit = defineEmits(["close"]);

const regularItems = [
  "刷新指纹",
  "编辑环境",
  "导出Profile",
  "转移",
  "复制环境ID",
  "同步Cookie",
  "运维授权",
  "清除缓存",
  "删除环境",
];
</script>

<style>
/* 遮罩层的渐入渐出效果 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 菜单内容的滑入滑出效果 */
.slide-enter-active,
.slide-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.slide-enter-from,
.slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
  /* 向上滑动 */
}
</style>
