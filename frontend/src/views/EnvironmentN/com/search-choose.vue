<template>
  <div v-show="props.open" class="w-full max-w-md rounded-lg bg-white p-4 shadow-sm z-50 border">
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-xl font-medium">筛选</h2>
      <button class="text-gray-600">
        <RefreshSquareIcon class="size-6"/>
      </button>
    </div>

    <div class="space-y-3">
      <!-- Filter Sections -->
      <div v-for="(section, index) in sections" :key="index">
        <button @click="toggleSection(index)"
          class="flex w-full items-center justify-between rounded-lg border border-gray-100 p-4 hover:bg-gray-50">
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
</template>

<script setup>
import { ref, defineEmits, defineProps, onMounted } from 'vue'
import { RefreshSquareIcon, AltArrowDownIcon } from '@/assets/icons/environment/index'

const emit = defineEmits(['close'])
const props = defineProps({
  open: false
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

const toggleSection = (index) => {
  sections.value[index].isOpen = !sections.value[index].isOpen
}
</script>

<style scoped>
input[type="datetime-local"]::-webkit-calendar-picker-indicator {
  filter: invert(0.5);
}
</style>