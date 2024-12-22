<template>
  <div
    class="flex flex-col p-4 bg-gray-50 h-main overflow-auto overflow-x-auto min-w-[900px]"
  >
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
    >
      <!-- Header -->
      <div class="bg-white rounded-lg p-4 m-[20px] h-[50px]">
        <div class="flex justify-between items-center mb-[20px] text-center">
          <div class="flex relative flex-1 items-center">
            <div class="relative">
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

            <!-- 搜索框 -->
            <div
              class="ml-4 relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none hover:ring-2 hover:ring-blue-500"
            >
              <input
                v-model="selectVal"
                type="text"
                :placeholder="
                  '请输入' +
                  (selectType === 1
                    ? '名称'
                    : selectType === 2
                    ? '备注'
                    : '手机号或邮箱')
                "
                class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none"
              />
              <SearchIcon
                class="absolute top-2.5 left-3 w-5 h-5 text-gray-400"
              />
              <More>
                <MoreTrigger>
                  <button class="p-2 ml-4 rounded-lg hover:bg-gray-100">
                    <SlidersHorizontalIcon class="w-5 h-5 text-gray-500" />
                  </button>
                </MoreTrigger>
                <MoreContent>
                  <MoreItem class="cursor-pointer" @click="selectT(1)">
                    <Settings2Icon class="w-4 h-4" />名称
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="selectT(2)">
                    <SquarePenIcon class="w-4 h-4" />序号
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />环境ID
                  </MoreItem>

                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />账号平台
                  </MoreItem>

                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />平台账号
                  </MoreItem>

                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />备注
                  </MoreItem>
                </MoreContent>
              </More>
            </div>

            <div class="ml-4">
              <TooltipButton
                @click=""
                class="p-2 w-20 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              >
                <div class="flex justify-center items-center">
                  <Recycle class="w-5 h-5" />
                  <span>恢复</span>
                </div>
              </TooltipButton>
            </div>

            <div class="ml-4">
              <TooltipButton
                @click=""
                class="ml-4 w-20 p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              >
                <div
                  class="flex justify-center items-center"
                  style="text-align: center"
                >
                  <Delete class="w-5 h-5" />
                  <span>删除</span>
                </div>
              </TooltipButton>
            </div>

            <div class="ml-[900px]">
              <TooltipButton
                title="刷新"
                @click=""
                class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              >
                <div
                  class="flex justify-center items-center space-x-2"
                  style="text-align: center"
                >
                  <RefreshCcw class="w-5 h-5" />
                </div>
              </TooltipButton>
            </div>

            <div class="ml-4">
              <More>
                <MoreTrigger>
                  <button
                    class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
                  >
                    <EllipsisVertical class="w-5 h-5 text-gray-500" />
                  </button>
                </MoreTrigger>
                <MoreContent>
                  <MoreItem class="cursor-pointer" @click="selectT(1)">
                    全部恢复
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="deleteAll">
                    全部删除
                  </MoreItem>
                </MoreContent>
              </More>
            </div>

            <!-- 弹出窗遮罩层 -->
            <div
              v-if="visible"
              class="flex fixed inset-0 justify-center items-center bg-gray-800 bg-opacity-50"
            >
              <!-- 弹出窗主体 -->
              <div class="bg-white rounded-lg w-[400px] p-6 shadow-lg">
                <!-- 标题栏 -->
                <div class="flex items-center mb-4">
                  <div
                    class="flex justify-center items-center mr-2 w-8 h-8 text-yellow-500 bg-yellow-100 rounded-full"
                  >
                    <span class="text-lg font-bold">!</span>
                  </div>
                  <h2 class="text-lg font-semibold text-gray-700">删除环境</h2>
                  <button
                    @click="closePopup"
                    class="ml-auto text-gray-400 hover:text-gray-600"
                  >
                    &#x2715;
                  </button>
                </div>

                <!-- 提示文本 -->
                <p class="mb-4 text-gray-600">
                  你确定要彻底删除以下序号的环境吗？
                </p>

                <!-- 环境序号展示 -->
                <div class="flex items-center mb-6">
                  <label class="mr-4 text-gray-600">环境序号</label>
                  <span
                    class="w-12 text-lg font-semibold text-center text-blue-600 bg-gray-100 rounded border"
                  >
                    {{ environmentId }}
                  </span>
                </div>

                <!-- 按钮区域 -->
                <div class="flex justify-end space-x-4">
                  <button
                    @click="confirmDelete"
                    class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
                  >
                    确定
                  </button>
                  <button
                    @click="closePopup"
                    class="px-4 py-2 text-gray-700 bg-gray-200 rounded hover:bg-gray-300"
                  >
                    取消
                  </button>
                </div>
              </div>
            </div>
          </div>
          <!-- <div class="flex gap-4 items-center ml-4"></div> -->
        </div>
      </div>

      <!-- Table -->
      <table class="overflow-auto overflow-x-auto min-w-[900px]">
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
            <th class="px-4 py-3 text-sm font-medium text-left">序号</th>
            <th class="px-4 py-3 text-sm font-medium text-left">名称</th>
            <th class="px-4 py-3 text-sm font-medium text-left">分组</th>
            <th class="px-4 py-3 text-sm font-medium text-left">IP</th>
            <th class="px-4 py-3 text-sm font-medium text-left">账号平台</th>
            <th class="px-4 py-3 text-sm font-medium text-left">备注</th>
            <th class="px-4 py-3 text-sm font-medium text-left">操作者</th>
            <th class="px-4 py-3 text-sm font-medium text-left">删除时间</th>
            <!-- <th class="px-4 py-3 text-sm text-left">操作</th> -->
            <!-- <th class="px-4 py-3 text-sm text-left">更多</th> -->
          </tr>
        </thead>
        <tbody class="">
          <tr
            v-for="(row, index) in tableData"
            :key="index"
            class="hover:bg-blue-100 h-[20px]"
            :class="{ 'bg-blue-50': row.selected }"
            @click="toggleRowSelection(row)"
          >
            <td class="px-4 py-3">
              <input type="checkbox" class="rounded" v-model="row.selected" />
            </td>
            <td class="px-4 py-3 text-sm">{{ row.id }}</td>
            <td class="px-4 py-3 text-sm">{{ row.name }}</td>
            <td class="px-4 py-3 text-sm">{{ row.group }}</td>
            <td class="px-4 py-3 text-sm">
              <div class="flex items-center">
                <img
                  src="../../assets/icons/check.svg"
                  class="mr-1 w-4 h-4"
                  style="width: 20px; height: 20px"
                />

                <img
                  src="../../assets/icons/location.svg"
                  class="mr-1 w-4 h-4"
                />
                {{ row.ip }}
                <span class="ml-1 text-gray-500">{{ row.location }}</span>
              </div>
            </td>
            <td class="px-4 py-3">
              <div class="flex items-center">
                <img src="../../assets/icons/stop.svg" class="mr-1 w-4 h-4" />
                <span
                  v-if="row.platform"
                  class="px-2 py-1 text-xs text-blue-800 bg-blue-100 rounded"
                >
                  {{ row.platform }}
                </span>
              </div>
            </td>
            <!-- <td class="px-4 py-3">
              <div class="flex items-center">
                <button class="">
                  <img src="../../assets/icons/open.svg" class="h-7 w-18" />
                </button>
              </div>
            </td> -->
            <!-- <td class="px-4 py-3">
              <button class="p-1 rounded hover:bg-gray-100">
                <MoreVerticalIcon class="w-4 h-4" />
              </button>
            </td> -->
            <td class="px-4 py-3 text-sm">{{ row.remark }}</td>
            <td class="px-4 py-3 text-sm">{{ row.operator }}</td>
            <td class="px-4 py-3 text-sm">{{ row.deleteTime }}</td>
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
  SearchIcon,
  LayoutGridIcon,
  ListIcon,
  MenuIcon,
  SquareIcon,
  XIcon,
  EditIcon,
  ShareIcon,
  LogInIcon,
  MoreVerticalIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  Trash2,
  Settings2Icon,
  SquarePenIcon,
  Trash2Icon,
  SlidersHorizontalIcon,
  Recycle,
  Delete,
  RefreshCcw,
  EllipsisVertical,
} from "lucide-vue-next";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";

import TooltipButton from "@/components/tooltip-button.vue";

// 表格数据
const tableData = ref([
  {
    id: "01",
    name: "跨境电商",
    group: "小米直营店",
    ip: "112.17.242.11",
    location: "CN 中国-香港",
    platform: "",
    selected: false,
    remark: "备注",
    operator: "张三",
    deleteTime: "2024-01-01 10:00:00",
  },
  {
    id: "02",
    name: "\\",
    group: "\\",
    ip: "112.17.242.11",
    location: "CN 中国-香港",
    platform: "18456502830",
    selected: false,
    remark: "备注",
    operator: "张三",
    deleteTime: "2024-01-01 10:00:00",
  },
]);

// 全选复选框的状态
const selectAll = ref(false);

const visible = ref(false);

// 切换所有行的选择状态
const toggleAll = () => {
  tableData.value.forEach((row) => {
    row.selected = selectAll.value;
  });
};

const deleteAll = () => {
  visible.value = !visible.value;
};

const confirmDelete = () => {
  visible.value = false;
};

const closePopup = () => {
  visible.value = false;
};
</script>
