<template>
  <div class="relative" v-if="props.open">
    <div class="size-4 bg-white border-l border-t transform rotate-45 absolute top-[-7px] left-5 z-40"></div>
    <!-- 遮罩层 -->
    <Transition name="fade">
      <div v-if="props.open" class="fixed inset-0 bg-black/0 z-10 cursor-default" @click.stop="emit('close')"></div>
    </Transition>

    <!-- 菜单内容 -->
    <Transition name="slide">
      <div v-if="props.open" class="absolute w-64 rounded-xl bg-white p-2 border z-30 shadow-xl">
        <div class="space-y-1">
          <!-- Regular menu items -->
          <button v-for="item in regularItems" :key="item"
            class="w-full rounded-lg px-4 py-1 text-left text-base hover:bg-gray-50">
            {{ item }}
          </button>

          <!-- Destructive menu items -->
          <button v-for="item in destructiveItems" :key="item"
            class="w-full rounded-lg px-4 py-1 text-left text-base text-red-500 hover:bg-gray-50">
            {{ item }}
          </button>
        </div>
      </div>
    </Transition>
  </div>

</template>

<script setup>
import { defineEmits, defineProps } from "vue";

const props = defineProps({
  open: Boolean, // 控制显示与隐藏
});

const emit = defineEmits(["close"]);

const regularItems = [
  "刷新指纹",
  "编辑环境",
  "导出Profile",
  "转移",
  "复制环境ID",
  "同步Cookie",
  "运维授权",
];

const destructiveItems = ["清除缓存", "删除环境"];
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