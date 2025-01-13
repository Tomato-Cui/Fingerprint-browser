<script setup lang="ts">
import { ref } from "vue";

import {
  AddCircle,
  IconApple,
  NewBookmark,
  SearchIcon,
  ChevronRightIcon,
  ChevronLeftIcon,
} from "@/assets/icons/environment-image";
interface Tab {
  id: string;
  name: string;
}

const tabs: Tab[] = [
  { id: "all", name: "所有环境" },
  { id: "group", name: "分组环境" },
];

const searchQuery = ref("");
const activeTab = ref("all");
const totalItems = ref(0);
const currentPage = ref(1);
const pageSize = ref(10);
const pageSizes = [10, 20, 50];
const totalPages = ref(1);

const createBookmark = () => {
  // Implement bookmark creation logic
  console.log("Creating new bookmark");
};

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
};
</script>

<template>
  <div class="p-6 min-h-screen bg-white">
    <!-- Header -->
    <div class="flex justify-between items-center mb-6">
      <h4 class="text-2xl font-semibold leading-8">书签</h4>
      <div class="flex gap-4 items-center">
        <div class="relative">
          <span class="absolute left-2 top-1/2 transform -translate-y-1/2">
            <SearchIcon class="w-[14px] h-[14px] text-left text-gray-400" />
          </span>
          <input
            type="search"
            v-model="searchQuery"
            placeholder="搜索"
            class="pl-8 w-64 h-10 rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          />
        </div>
        <button
          class="inline-flex items-center px-4 py-2 bg-[#4F46E5] text-white rounded-md hover:bg-[#4338CA] transition-colors"
          @click="createBookmark"
        >
          <NewBookmark class="mr-2 w-5 h-5" />
          新建书签
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="mb-8 border-b border-gray-200">
      <nav class="flex -mb-px space-x-8">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="activeTab = tab.id"
          :class="[
            activeTab === tab.id
              ? 'border-indigo-500 text-indigo-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
            'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm',
          ]"
        >
          {{ tab.name }}
        </button>
      </nav>
    </div>

    <!-- Empty State -->
    <div class="flex flex-col justify-center items-center py-16">
      <div class="relative mb-6 w-48 h-48">
        <IconApple class="w-[239px] h-[191px]" />
      </div>
      <p
        class="mb-6 max-w-lg text-base font-normal leading-6 text-center text-gray-500"
      >
        在书签管理中添加的书签，会同步给拥有该环境权限的成员；在环境中添加的书签将不会同步
      </p>
      <button
        class="inline-flex items-center px-4 py-2 bg-[#4F46E5] text-white rounded-md hover:bg-[#4338CA] transition-colors"
        @click="createBookmark"
      >
        <AddCircle class="mr-2 w-5 h-5" />
        新建书签
      </button>
    </div>

    <div class="flex flex-col min-h-screen">
      <!-- 其他页面内容 -->
      <div class="flex gap-4 justify-end items-center mt-8">
        <span class="text-sm text-gray-500">共 {{ totalItems }} 项</span>
        <div class="flex gap-2 items-center">
          <button
            class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
            :disabled="currentPage === 1"
            @click="prevPage"
          >
            <ChevronLeftIcon class="w-[12px] h-[12px]" />
          </button>
          <span class="min-w-[2rem] text-center">{{ currentPage }}</span>
          <button
            class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
            :disabled="currentPage === totalPages"
            @click="nextPage"
          >
            <ChevronRightIcon class="w-[12px] h-[12px]" />
          </button>
        </div>
        <select v-model="pageSize" class="px-2 py-1 text-sm rounded border">
          <option v-for="size in pageSizes" :key="size" :value="size">
            {{ size }}条/页
          </option>
        </select>
      </div>
    </div>
  </div>
</template>
