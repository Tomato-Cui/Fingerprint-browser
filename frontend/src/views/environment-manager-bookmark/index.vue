<script setup lang="ts">
import { ref } from "vue";
import NewBookmarks from "./new-bookmark.vue";

import {
  NewBookmark,
  SearchIcon,
  ChevronRightIcon,
  ChevronLeftIcon,
} from "@/assets/icons/environment-bookmark-image";

interface Tab {
  id: string;
  name: string;
}

interface Bookmark {
  id: number;
  name: string;
  url: string;
  selected: boolean;
}

const selectAll = ref(false);

const bookmarks = ref<Bookmark[]>([
  {
    id: 1,
    name: "feishu",
    url: "https://gcn1b5cn2pro.feishu.cn/wiki/EcX8w9yw2iCh87kFGyrctxQknQe",
    selected: false,
  },
  {
    id: 2,
    name: "feishu2",
    url: "https://gcn1b5cn2pro.feishu.cn/wiki/EcX8w9yw2iCh87kFGyrctxQknQe",
    selected: false,
  },
  {
    id: 3,
    name: "feishu3",
    url: "https://gcn1b5cn2pro.feishu.cn/wiki/EcX8w9yw2iCh87kFGyrctxQknQe",
    selected: false,
  },
  {
    id: 4,
    name: "feishu4",
    url: "https://gcn1b5cn2pro.feishu.cn/wiki/EcX8w9yw2iCh87kFGyrctxQknQe",
    selected: false,
  },
]);

const toggleSelectAll = () => {
  bookmarks.value.forEach((bookmark) => {
    bookmark.selected = selectAll.value;
  });
};

const editBookmark = (bookmark: Bookmark) => {
  console.log("Edit bookmark:", bookmark);
};

const copyBookmark = (bookmark: Bookmark) => {
  console.log("Copy bookmark:", bookmark);
};

const deleteBookmark = (bookmark: Bookmark) => {
  console.log("Delete bookmark:", bookmark);
};

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

const createBookmark = ref(false);
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
  <div class="flex flex-col h-full">
    <!-- Header -->
    <div class="flex justify-between items-center mx-2 mt-1">
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
            class="pl-8 w-64 border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm border rounded-md px-2 py-1.5 flex items-center outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          />
        </div>
        <button
          class="bg-[#4F46E5] mr-3 rounded-md hover:bg-[#4338CA] text-sm border px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 text-white outline-gray-50 hover:outline-gray-100"
          @click="createBookmark = true"
        >
          <NewBookmark class="w-[14px] h-[14px]" />
          <span class="mx-1"> 新建书签</span>
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="mx-2 mb-3 border-b border-gray-200">
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
    <!-- <div class="flex flex-col justify-center items-center py-16">
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
    </div> -->

    <div class="flex flex-col mx-2 h-full">
      <div class="w-full rounded-lg border border-gray-200">
        <table class="min-w-full">
          <!-- Table Header -->
          <thead class="bg-gray-50">
            <tr>
              <th scope="col" class="py-3.5 pr-3 pl-4 w-12 text-left">
                <input
                  type="checkbox"
                  class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                  v-model="selectAll"
                  @change="toggleSelectAll"
                />
              </th>
              <th
                scope="col"
                class="py-3.5 pr-3 pl-4 text-sm font-semibold text-left text-gray-900"
              >
                <span class="font-thin text-gray-300">|&nbsp;</span>
                书签名称
              </th>
              <th
                scope="col"
                class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
              >
                <span class="font-thin text-gray-300">|&nbsp;</span>
                链接
              </th>
              <th
                scope="col"
                class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
              >
                <span class="font-thin text-gray-300">|&nbsp;</span>
                操作
              </th>
            </tr>
          </thead>

          <!-- Table Body -->
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="bookmark in bookmarks"
              :key="bookmark.id"
              class="hover:bg-gray-50"
            >
              <td class="py-4 pr-3 pl-4 whitespace-nowrap">
                <input
                  type="checkbox"
                  class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                  v-model="bookmark.selected"
                />
              </td>
              <td
                class="py-4 pr-3 pl-4 text-sm text-gray-900 whitespace-nowrap"
              >
                {{ bookmark.name }}
              </td>
              <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                {{ bookmark.url }}
              </td>
              <td class="px-3 py-4 text-sm whitespace-nowrap">
                <div class="flex gap-2 justify-start items-center">
                  <button
                    @click="editBookmark(bookmark)"
                    class="text-[#4F46E5] hover:bg-indigo-50 px-2 py-1 rounded border border-[#5050FA] bg-[#F0F5FF]"
                  >
                    编辑
                  </button>
                  <button
                    @click="copyBookmark(bookmark)"
                    class="text-[#FA8C16] hover:bg-indigo-50 px-2 py-1 rounded border border-[#FA8C16] bg-[#FFF1E8]"
                  >
                    复制到
                  </button>
                  <button
                    @click="deleteBookmark(bookmark)"
                    class="px-2 py-1 text-red-600 rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFE2E2]"
                  >
                    删除
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- pagination -->
    <div
      class="flex justify-end items-center pt-3 mb-2 border-t border-gray-200"
    >
      <span class="text-sm text-gray-500">共 {{ totalItems }} 项</span>
      <div class="flex gap-2 items-center">
        <button
          class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
          :disabled="currentPage === 1"
          @click="prevPage"
        >
          <ChevronLeftIcon class="w-[12px] h-[12px]" />
        </button>
        <span class="px-2 text-center rounded-md border border-gray-300">{{
          currentPage
        }}</span>
        <button
          class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
          :disabled="currentPage === totalPages"
          @click="nextPage"
        >
          <ChevronRightIcon class="w-[12px] h-[12px]" />
        </button>
      </div>
      <select v-model="pageSize" class="px-2 py-1 mx-2 text-sm rounded border">
        <option v-for="size in pageSizes" :key="size" :value="size">
          {{ size }}条/页
        </option>
      </select>
    </div>
  </div>
  <NewBookmarks v-model:createBookmark="createBookmark" />
</template>
