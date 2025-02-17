<script setup lang="ts">
import { onMounted, ref } from "vue";
import { cn } from "@/util/lib";
import { Card, CardHeader } from "@/components/ui/card";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();
const route = useRoute();
const props = defineProps<{
  data: Array<any>;
}>();

const isMouseDown = ref(false);
const startX = ref(0);
const scrollLeft = ref(0);
const slider = ref<HTMLUListElement | null>(null);

const handleMouseDown = (e: MouseEvent) => {
  isMouseDown.value = true;
  startX.value = e.pageX - (slider.value?.offsetLeft || 0);
  scrollLeft.value = slider.value?.scrollLeft || 0;
};

const handleMouseUp = () => {
  isMouseDown.value = false;
};

const handleMouseMove = (e: MouseEvent) => {
  if (!isMouseDown.value) return;
  e.preventDefault();
  const x = e.pageX - (slider.value?.offsetLeft || 0);
  const walk = (x - startX.value) * 2;
  if (slider.value) {
    slider.value.scrollLeft = scrollLeft.value - walk;
  }
};

const handleMouseLeave = () => {
  isMouseDown.value = false;
};
onMounted(() => {
  // getList()
});
</script>

<template>
  <Card className="flex flex-col p-0 flex-1 overflow-y-auto">
    <CardHeader class="p-0">
      <ul
        ref="slider"
        @mousedown="handleMouseDown"
        @mousemove="handleMouseMove"
        @mouseup="handleMouseUp"
        @mouseleave="handleMouseLeave"
        class="flex overflow-auto justify-start px-0 pb-0 mb-3 space-x-6 w-full text-sm rounded-none border-b scrollbar-hide cursor-grab active:cursor-grabbing"
      >
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
