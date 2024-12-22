<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
      style="padding: 20px"
    >
      <!-- Header Section -->
      <div class="flex justify-between items-center mb-6">
        <!-- <div class="flex items-center space-x-4"> -->
        <div class="flex items-center mb-4 space-x-2">
          <TooltipButton
            title=""
            @click="openDialog"
            class="flex items-center px-4 py-2 space-x-2 text-white bg-blue-600 rounded-lg"
          >
            <FolderIcon class="w-5 h-5" />
            <span>创建分组</span>
          </TooltipButton>

          <!-- 弹窗蒙层 -->
          <div
            v-if="isVisible"
            class="flex fixed inset-0 z-50 flex-col justify-center items-center bg-gray-800 bg-opacity-50"
          >
            <!-- 弹出框容器 -->
            <div class="p-6 w-96 bg-white rounded-lg shadow-lg">
              <!-- 标题 -->
              <div class="mb-4 text-xl font-semibold">创建分组</div>

              <!-- 输入框区域 -->
              <div class="flex flex-col">
                <label class="block mb-2 text-gray-600">分组名称</label>
                <input
                  v-model="groupName"
                  type="text"
                  placeholder="请编辑新分组"
                  class="p-2 mb-4 w-full rounded border focus:outline-blue-500"
                />

                <label class="block mb-2 text-gray-600">备注</label>
                <input
                  v-model="remark"
                  type="text"
                  placeholder="请输入备注"
                  class="p-2 mb-4 w-full rounded border focus:outline-blue-500"
                />
              </div>

              <!-- 按钮区域 -->
              <div class="flex justify-end space-x-4">
                <button
                  @click="confirmSort"
                  class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
                >
                  确定
                </button>
                <button
                  @click="closePopup"
                  class="px-4 py-2 text-gray-700 bg-gray-300 rounded hover:bg-gray-400"
                >
                  取消
                </button>
              </div>
            </div>
          </div>

          <TooltipButton
            title="授权"
            @click=""
            class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
          >
            <LinkIcon class="w-5 h-5" />
          </TooltipButton>

          <TooltipButton
            title="删除"
            @click=""
            class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
          >
            <TrashIcon class="w-5 h-5" />
          </TooltipButton>

          <div>
            <Select>
              <SelectTrigger style="min-width: 150px">
                <SelectValue
                  placeholder="所有平台"
                  class="p-2 w-full rounded-lg outline-none"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="全部分组"> 全部分组 </SelectItem>
                  <SelectItem value="已分组"> 已分组 </SelectItem>
                  <SelectItem value="未分组"> 未分组 </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>

          <div
            class="flex flex-1 items-center ml-4 max-w-xl rounded-lg border hover:outline-none hover:ring-1 hover:ring-blue-500"
          >
            <SearchIcon class="pl-2 w-8 h-8 text-gray-400" />
            <input
              type="text"
              placeholder="搜索或新建搜索条件"
              class="py-2 pr-4 pl-1 w-full rounded-lg border-gray-200 outline-none"
              style="min-width: 200px"
            />
          </div>
        </div>
      </div>

      <!-- Table -->
      <table class="w-full">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-4 py-3 w-12">
              <input
                type="checkbox"
                class="rounded border-gray-300"
                v-model="selectAll"
                @change="toggleAll"
              />
            </th>
            <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
              分组
            </th>
            <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
              分组环境总数
            </th>
            <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
              创建时间
            </th>
            <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
              创建用户/邮箱
            </th>
            <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
              操作
            </th>
            <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
              更多
            </th>
          </tr>
        </thead>

        <tbody>
          <tr
            v-for="(row, index) in tableData"
            :key="index"
            class="hover:bg-blue-100"
            :class="{ 'bg-blue-50': row.selected }"
            @click="toggleRowSelection(row)"
          >
            <td class="px-4 py-3">
              <input
                type="checkbox"
                class="rounded border-gray-300"
                v-model="row.selected"
              />
            </td>
            <td class="px-4 py-3 text-sm">{{ row.group }}</td>
            <td class="px-4 py-3 text-sm">{{ row.total }}</td>
            <td class="px-4 py-3 text-sm">{{ row.createdAt }}</td>
            <td class="px-4 py-3 text-sm">{{ row.email }}</td>
            <td class="px-4 py-3">
              <button
                class="inline-flex items-center px-4 py-1 text-sm text-blue-600 rounded hover:bg-blue-50"
                style="width: 80px; margin-left: -20px"
              >
                <RefreshCwIcon class="mr-1 w-4 h-4" />
                刷新
              </button>
            </td>
            <td class="px-4 py-3">
              <button class="p-1 rounded hover:bg-gray-100">
                <MoreVerticalIcon class="w-4 h-4" />
              </button>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Pagination -->

      <div class="flex justify-between items-center p-4 mt-auto bg-white">
        <div class="text-sm text-gray-600">共 101 项数据</div>
        <div class="flex items-center space-x-2">
          <button class="p-2 rounded hover:bg-gray-100">
            <ChevronLeftIcon class="w-4 h-4" />
          </button>
          <button class="px-3 py-1 rounded hover:bg-gray-100">1</button>
          <button class="px-3 py-1 rounded hover:bg-gray-100">2</button>
          <button class="px-3 py-1 text-white bg-blue-500 rounded">3</button>
          <button class="px-3 py-1 rounded hover:bg-gray-100">4</button>
          <span>...</span>
          <button class="px-3 py-1 rounded hover:bg-gray-100">20</button>
          <button class="p-2 rounded hover:bg-gray-100">
            <ChevronRightIcon class="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

import {
  FolderIcon,
  LinkIcon,
  TrashIcon,
  SearchIcon,
  ChevronDownIcon,
  RefreshCwIcon,
  MoreVerticalIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
} from "lucide-vue-next";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

import { Model } from "@/components/model/index";

import { PrimaryButton, CancelButton } from "@/components/button";

import { onMounted, onBeforeUnmount } from "vue";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";

import TooltipButton from "@/components/tooltip-button.vue";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

const selectAll = ref(false);
const isVisible = ref(false);
const tableData = ref([
  {
    group: "跨境电商",
    total: 12,
    createdAt: "2024-11-22 14:30:00",
    email: "1236726182@qq.com",
    selected: false,
  },
  {
    group: "广告传媒",
    total: 23,
    createdAt: "2024-11-22 14:30:00",
    email: "18456501928",
    selected: false,
  },
]);

const toggleAll = () => {
  tableData.value.forEach((row) => {
    row.selected = selectAll.value;
  });
};

const openDialog = () => {
  isVisible.value = !isVisible.value;
};

const confirmSort = () => {
  isVisible.value = false;
};

const closePopup = () => {
  isVisible.value = false;
};
</script>
