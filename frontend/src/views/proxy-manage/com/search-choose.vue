<template>
  <Transition name="modal-fade">

    <div v-show="props.open" ref="targetDiv" class="w-full max-w-md rounded-lg bg-white p-4 shadow-sm z-30 border">
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-xl font-medium">筛选</h2>
        <button class="text-gray-600">
          <RefreshSquareIcon class="size-6" />
        </button>
      </div>

      <div class="space-y-3">
        <!-- Filter Sections -->
        <div v-for="(section, index) in sections" :key="index">
          <button @click="toggleSection(index)"
            class="flex w-full items-center justify-between rounded-lg border border-gray-100 p-2 hover:bg-gray-50">
            <div class="flex items-center">
              {{ section.title }}
              <div v-if="section.hasNotification" class="ml-2 h-1.5 w-1.5 rounded-full bg-red-500"></div>
            </div>
            <AltArrowDownIcon class="h-5 w-5 text-gray-400 transition-transform"
              :class="{ 'rotate-180': section.isOpen }" />
          </button>

          <!-- Section Content -->
          <div v-show="section.isOpen" class="mt-2 px-4">
            <div v-if="section.type === 'timeRange'" class="flex items-center space-x-4">
              <div class="flex-1">
                <input type="datetime-local" class="w-full rounded-lg border border-gray-200 p-2 text-sm"
                  v-model="section.startTime" />
              </div>
              <div class="text-gray-400">至</div>
              <div class="flex-1">
                <input type="datetime-local" class="w-full rounded-lg border border-gray-200 p-2 text-sm"
                  v-model="section.endTime" />
              </div>
            </div>
            <div v-else class="py-2 text-sm text-gray-500">
              {{ section.content }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, defineEmits, defineProps, onMounted, onUnmounted } from 'vue'
import { RefreshSquareIcon, AltArrowDownIcon } from '@/assets/icons/environment/index'

const emit = defineEmits(['close'])
const props = defineProps({
  open: Boolean
})
const sections = ref([
  {
    title: '绑定代理情况',
    content: '暂无内容',
    isOpen: false,
    hasNotification: false
  },
  {
    title: '账号平台',
    content: '暂无内容',
    isOpen: false,
    hasNotification: false
  },
  {
    title: '代理信息',
    content: '暂无内容',
    isOpen: false,
    hasNotification: true
  },
  {
    title: '标签',
    content: '暂无内容',
    isOpen: false,
    hasNotification: false
  },
  {
    title: '账号平台',
    content: '暂无内容',
    isOpen: false,
    hasNotification: false
  },
  {
    title: '开始时间',
    type: 'timeRange',
    startTime: '',
    endTime: '',
    isOpen: false,
    hasNotification: false
  }
])

const toggleSection = (index: number) => {
  sections.value[index].isOpen = !sections.value[index].isOpen
}

// 获取指定的 div 元素
const targetDiv = ref<any>(null);

// 监听点击事件
const handleClick = (event: MouseEvent) => {

    // 判断是否点击到了指定的 div
    if (targetDiv.value && !targetDiv.value.contains(event.target)) {
        if (props.open === true) emit('close');  //未点击到则关闭弹窗
    }
};

// 在组件挂载时添加事件监听
onMounted(() => {
    document.addEventListener('click', handleClick);
});

// 在组件卸载时移除事件监听
onUnmounted(() => {
    document.removeEventListener('click', handleClick);
});
</script>

<style scoped>
input[type="datetime-local"]::-webkit-calendar-picker-indicator {
  filter: invert(0.5);
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>