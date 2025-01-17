<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Circle, Filter, File } from "@/assets/icons/environment-group-manage";
import {
  SearchIcon,
  ChevronRightIcon,
  ChevronLeftIcon,
} from "@/assets/icons/environment-bookmark-image";

import NewGroup from "./new-group.vue";
import { environment_group_query } from "@/commands/environment-group";

interface Groupmanage {
  id: number;
  name: string;
  top: string;
  environment_count: number;
  grant_user: string;
  selected: boolean;
}

const loadData = () => {
  environment_group_query(1, 1000).then((res) => {
    let { data: data_, total } = res.data;
    totalItems.value = total;
    groupmanage.value = data_;
  });
};

onMounted(() => loadData());

const selectAll = ref(false);

const groupmanage = ref<Groupmanage[]>([]);

const toggleSelectAll = () => {
  groupmanage.value.forEach((Groupmanage) => {
    Groupmanage.selected = selectAll.value;
  });
};

const editGroupmanage = (Groupmanage: Groupmanage) => {
  console.log("Edit Groupmanage:", Groupmanage);
};

const deleteGroupmanage = (Groupmanage: Groupmanage) => {
  console.log("Delete Groupmanage:", Groupmanage);
};

const searchQuery = ref("");

const totalItems = ref(0);
const currentPage = ref(1);
const pageSize = ref(10);
const pageSizes = [10, 20, 50];
const totalPages = ref(1);

const createGroupManage = ref(false);

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
    <div class="flex justify-between items-center px-2 mx-2 mt-1">
      <h4 class="text-2xl font-semibold leading-8">分组管理</h4>

      <div class="flex gap-2 items-center">
        <button
          class="text-white hover:bg-[#4338CA] border font-[500] border-gray-300 text-sm rounded-md px-2 py-1.5 flex items-center outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <Circle class="w-[17.5px] h-[18.3px] text-gray-400" />
        </button>

        <div class="relative">
          <span class="absolute left-2 top-1/2 transform -translate-y-1/2">
            <SearchIcon class="w-[14px] h-[14px] text-left text-gray-400" />
          </span>
          <input
            type="search"
            v-model="searchQuery"
            placeholder="搜索"
            class="pl-8 w-64 border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
          />
        </div>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <Filter
            class="w-[10.21px] h-[10.21px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            筛选
          </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 bg-[#4F46E5] text-white outline-gray-50 hover:outline-gray-100"
          @click="createGroupManage = true"
        >
          <File class="mr-2 w-[11.67px] h-[11.67px]" />
          新建分组
        </button>
      </div>
    </div>

    <!-- Empty State -->
    <!-- <div class="flex flex-col justify-center items-center py-16">
      <div class="relative mb-6 w-48 h-48">
        <IconApple class="w-[239px] h-[191px] border " />
      </div>
      <p
        class="mb-6 max-w-lg text-base font-normal leading-6 text-center text-gray-500"
      >
        在书签管理中添加的书签，会同步给拥有该环境权限的成员；在环境中添加的书签将不会同步
      </p>
      <button
        class="inline-flex items-center px-4 py-2 bg-[#4F46E5] text-white rounded-md hover:bg-[#4338CA] transition-colors"
        @click="createGroupmanage"
      >
        <AddCircle class="mr-2 w-5 h-5" />
        新建书签
      </button>
    </div> -->

    <div class="flex flex-col mx-2 mt-2 h-full">
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
                分组名称
              </th>

              <th
                scope="col"
                class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
              >
                <span class="font-thin text-gray-300">|&nbsp;</span>
                置顶
              </th>

              <th
                scope="col"
                class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
              >
                <span class="font-thin text-gray-300">|&nbsp;</span>
                环境数
              </th>

              <th
                scope="col"
                class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
              >
                <span class="font-thin text-gray-300">|&nbsp;</span>
                授权成员
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
              v-for="Groupmanage in groupmanage"
              :key="Groupmanage.id"
              class="hover:bg-gray-50"
            >
              <td class="py-4 pr-3 pl-4 whitespace-nowrap">
                <input
                  type="checkbox"
                  class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                  v-model="Groupmanage.selected"
                />
              </td>
              <td
                class="py-4 pr-3 pl-4 text-sm text-gray-900 whitespace-nowrap"
              >
                {{ Groupmanage.name }}
              </td>
              <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                {{ Groupmanage.top }}
              </td>

              <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                {{ Groupmanage.environment_count }}
              </td>

              <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                {{ Groupmanage.grant_user }}
              </td>

              <td class="px-3 py-4 text-sm whitespace-nowrap">
                <div class="flex gap-2 justify-start items-center">
                  <button
                    @click="editGroupmanage(Groupmanage)"
                    class="text-[#4F46E5] hover:bg-indigo-50 px-2 py-1 rounded border border-[#5050FA] bg-[#F0F5FF]"
                  >
                    授权
                  </button>

                  <button
                    @click="deleteGroupmanage(Groupmanage)"
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
      <select v-model="pageSize" class="px-2 py-1 mr-2 text-sm rounded border">
        <option v-for="size in pageSizes" :key="size" :value="size">
          {{ size }}条/页
        </option>
      </select>
    </div>
  </div>
  <NewGroup v-model:createGroupManage="createGroupManage" />
</template>
