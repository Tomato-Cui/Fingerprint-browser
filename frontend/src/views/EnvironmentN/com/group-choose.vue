<script setup lang="ts">
import { onMounted, ref } from "vue";
import { cn } from "@/util/lib";
import { Card, CardHeader } from "@/components/ui/card";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();
const route = useRoute();
// 定义props接收data数组参数
const props = defineProps<{
  data: Array<any>;
}>();

// 鼠标拖动相关状态
const isMouseDown = ref(false); // 是否按下鼠标
const startX = ref(0); // 开始拖动时的X坐标
const scrollLeft = ref(0); // 开始拖动时的滚动位置
const slider = ref<HTMLUListElement | null>(null); // 滑动容器的ref

// 鼠标按下事件处理
const handleMouseDown = (e: MouseEvent) => {
  isMouseDown.value = true;
  startX.value = e.pageX - (slider.value?.offsetLeft || 0);
  scrollLeft.value = slider.value?.scrollLeft || 0;
};

// 鼠标松开事件处理
const handleMouseUp = () => {
  isMouseDown.value = false;
};

// 鼠标移动事件处理
const handleMouseMove = (e: MouseEvent) => {
  if (!isMouseDown.value) return;
  e.preventDefault();
  const x = e.pageX - (slider.value?.offsetLeft || 0);
  const walk = (x - startX.value) * 2; // 滚动距离
  if (slider.value) {
    slider.value.scrollLeft = scrollLeft.value - walk;
  }
};

// 鼠标离开事件处理
const handleMouseLeave = () => {
  isMouseDown.value = false;
};

// 组件挂载时执行
onMounted(() => {
  // getList()
});
</script>

<template>
  <!-- 主卡片容器 -->
  <Card className="flex flex-col p-0 flex-1 overflow-y-auto   ">
    <CardHeader class="p-0">
      <!-- 可滑动的分组列表 -->
      <ul
        ref="slider"
        @mousedown="handleMouseDown"
        @mousemove="handleMouseMove"
        @mouseup="handleMouseUp"
        @mouseleave="handleMouseLeave"
        class="flex overflow-auto justify-start px-0 pb-0 mb-3 space-x-6 w-full text-sm rounded-none border-b scrollbar-hide cursor-grab active:cursor-grabbing"
      >
        <!-- 分组项目 -->
        <li
          :key="item.id"
          v-for="item in props.data"
          @click="router.push(item.href)"
          :class="
            cn(
              'border-transparent border-b-[3px] rounded-none flex items-center pb-1 cursor-pointer min-w-fit',
              route.path == item.href ? 'text-[#5050FA] border-[#5050FA]' : ''
            )
          "
        >
          <div class="flex gap-x-3 justify-center items-center min-w-fit">
            {{ item.title }}
          </div>
        </li>
      </ul>
    </CardHeader>
    <slot />
  </Card>
</template>

<style scoped>
/* 隐藏滚动条 */
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

/* 兼容 Firefox */
.scrollbar-hide {
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE 和 Edge */
}
</style>
