<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <div class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow" style="padding: 20px">
      <!-- Header -->
      <div lass="bg-white rounded-lg p-4 m-[20px] h-[50px]">
        <div class="flex justify-between items-center mb-[20px]">
          <div class="relative flex-1 flex items-center gap-4 min-w-[500px] max-w-[1000px]">
            <!-- 搜索框 -->
            <div
              class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none hover:ring-2 hover:ring-blue-500">
              <input v-model="selectVal" type="text" :placeholder="'请输入' +
                (selectType === 1
                  ? '名称'
                  : selectType === 2
                    ? '备注'
                    : '手机号或邮箱')
                " class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none" />
              <SearchIcon class="absolute top-2.5 left-3 w-5 h-5 text-gray-400" />
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
                    <SquarePenIcon class="w-4 h-4" />备注
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />手机号/邮箱
                  </MoreItem>
                </MoreContent>
              </More>
            </div>

            <TooltipButton title="搜索条件" @click="openDialog"
              class="p-2 rounded border-[1px] ml-[-20px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              style="margin-left: 5px">
              <MenuIcon class="w-5 h-5" />
            </TooltipButton>

            <!-- 弹窗蒙层 -->
            <div v-if="showDialog" class="flex fixed inset-0 justify-center items-center bg-gray-800 bg-opacity-50">
              <!-- 弹窗内容 -->
              <div class="w-full max-w-md bg-white rounded-lg shadow-lg">
                <!-- 弹窗标题 -->
                <div class="p-4 text-xl font-semibold border-b">筛选</div>

                <!-- 筛选表单 -->
                <div class="p-4 space-y-4">
                  <!-- 循环生成选择框 -->
                  <div v-for="(item, index) in filters" :key="index" class="flex items-center">
                    <label class="w-24 text-gray-700">{{ item.label }}</label>
                    <select v-model="item.value"
                      class="flex-1 px-3 py-2 rounded-md border focus:outline-none focus:ring-2 focus:ring-blue-500">
                      <option value="" disabled>请选择</option>
                      <option v-for="option in item.options" :key="option" :value="option">
                        {{ option }}
                      </option>
                    </select>
                  </div>

                  <!-- 转移类型 -->
                  <div class="flex items-center">
                    <label class="w-24 text-gray-700">转移类型</label>
                    <select v-model="transferType"
                      class="flex-1 px-3 py-2 rounded-md border focus:outline-none focus:ring-2 focus:ring-blue-500">
                      <option value="含备注与代理">含备注与代理</option>
                    </select>
                  </div>

                  <!-- 转移时间 -->
                  <div class="flex items-center space-x-2">
                    <label class="w-24 text-gray-700">转移时间</label>
                    <input type="date" v-model="startTime"
                      class="flex-1 px-3 py-2 rounded-md border focus:ring-2 focus:ring-blue-500"
                      style="width: 150px" />
                    <span>→</span>
                    <input type="date" v-model="endTime"
                      class="flex-1 px-3 py-2 rounded-md border focus:ring-2 focus:ring-blue-500"
                      style="width: 150px" />
                  </div>
                </div>

                <!-- 底部按钮 -->
                <div class="flex justify-center p-4 space-x-4 border-t">
                  <button @click="handleConfirm" class="px-6 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600">
                    确定
                  </button>
                  <button @click="handleCancel"
                    class="px-6 py-2 text-gray-800 bg-gray-200 rounded-md hover:bg-gray-300">
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
      <div class="overflow-auto flex-1">
        <table class="w-full">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48">
                序号
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48">
                名称
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48">
                分组
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48">
                IP
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48">
                账号平台
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">备注</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">浏览器</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">转移时间</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">转移类型</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                转移团队名称
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">转移团队ID</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">代理类型</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">API服务商</th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">操作者</th>
              <!-- <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
            操作
          </th> -->
              <!-- <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
            更多
          </th> -->
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, index) in tableData" :key="index" class="hover:bg-blue-100"
              :class="{ 'bg-blue-50': row.selected }" @click="toggleRowSelection(row)">
              <!-- <td class="px-4 py-3">
            <input
              type="checkbox"
              class="rounded border-gray-300"
              v-model="row.selected"
            />
          </td> -->
              <td class="px-4 py-3 text-sm min-w-48">{{ row.id }}</td>
              <td class="px-4 py-3 text-sm min-w-48">
                <div class="flex items-center">
                  <span class="text-gray-500">{{ row.name }}</span>
                  <!-- <button class="p-1 rounded hover:bg-gray-100">
                <img src="../../assets/icons/modify.svg" class="w-4 h-4" />
              </button> -->
                </div>
              </td>
              <td class="px-4 py-3 text-sm min-w-48">
                <div class="flex items-center">
                  <span class="text-gray-500">{{ row.group }}</span>
                </div>
              </td>
              <td class="px-4 py-3 text-sm min-w-48">
                <div class="flex items-center">
                  <img src="../../assets/icons/location.svg" class="mr-1 w-4 h-4" />
                  {{ row.ip }}
                  <span class="ml-1 text-gray-500">{{ row.location }}</span>
                </div>
              </td>
              <td class="px-4 py-3 min-w-48">
                <div class="flex items-center space-x-2">
                  <span v-if="row.platform" class="flex items-center">
                    <span
                      class="flex justify-center items-center mr-1 w-6 h-6 text-xs text-blue-800 bg-blue-100 rounded-full">b</span>
                    {{ row.platform }}
                  </span>
                  <span v-else class="text-gray-400">\</span>
                </div>
              </td>

              <td class="px-4 py-3 text-sm min-w-48">{{ row.remark }}</td>

              <td class="px-4 py-3 text-sm min-w-48">{{ row.browser }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.transferTime }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.transferType }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.transferTeamName }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.transferTeamId }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.agentType }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.apiService }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.operator }}</td>
            </tr>
          </tbody>
        </table>
      </div>
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

      <!-- <div class="flex justify-between items-center mt-auto">
      <div class="text-sm text-gray-600">共 101 项数据</div>
      <div class="flex items-center space-x-1">
        <button class="p-2 rounded hover:bg-gray-100">
          <ChevronLeftIcon class="w-4 h-4" />
        </button>
        <button class="px-3 py-1 text-sm rounded hover:bg-gray-100">1</button>
        <button class="px-3 py-1 text-sm rounded hover:bg-gray-100">2</button>
        <button class="px-3 py-1 text-sm text-white bg-blue-600 rounded">
          3
        </button>
        <button class="px-3 py-1 text-sm rounded hover:bg-gray-100">4</button>
        <span class="text-sm">...</span>
        <button class="px-3 py-1 text-sm rounded hover:bg-gray-100">20</button>
        <button class="p-2 rounded hover:bg-gray-100">
          <ChevronRightIcon class="w-4 h-4" />
        </button>
      </div>
    </div> -->
    </div>
  </div>
</template>

<script setup>
import { ref, defineProps, defineEmits } from "vue";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  SearchIcon,
  LayoutGridIcon,
  MenuIcon,
  PencilIcon,
  BoxIcon,
  MoreVerticalIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  SlidersHorizontalIcon,
  Settings2Icon,
  SquarePenIcon,
  Trash2Icon,
} from "lucide-vue-next";
const selectAll = ref(false);

const tableData = ref([]);
for (let i = 0; i < 130; i++) {
  const newAgentsData = {
    id: "02",
    name: "\\",
    group: "\\",
    ip: "112.17.242.11",
    location: "CN 中国-香港",
    platform: "18456502830",
    selected: false,
    remark: "备注",
    browser: "Chrome",
    transferTime: "2024-01-01 10:00:00",
    transferType: "手动转移",
    transferTeamName: "团队名称",
    transferTeamId: "1234567890",
    agentType: "代理类型",
    apiService: "API服务商",
    operator: "操作者",
  };
  tableData.value.push(newAgentsData);
}

const toggleAll = () => {
  tableData.value.forEach((row) => {
    row.selected = selectAll.value;
  });
};

// 控制弹窗显示/隐藏
const showDialog = ref(false);
const openDialog = () => {
  showDialog.value = !showDialog.value;
};

const handleConfirm = () => {
  showDialog.value = false;
};

const handleCancel = () => {
  showDialog.value = false;
};

const props = defineProps({
  isOpen: Boolean, // 控制弹窗显示
});

const emit = defineEmits(["close", "confirm"]);

// 筛选项数据
const filters = ref([
  { label: "账号平台", value: "", options: ["平台1", "平台2", "平台3"] },
  { label: "浏览器", value: "", options: ["Chrome", "Firefox", "Edge"] },
  { label: "环境分组", value: "", options: ["组1", "组2"] },
  { label: "代理类型", value: "", options: ["类型1", "类型2"] },
  { label: "服务商", value: "", options: ["服务商1", "服务商2"] },
  { label: "操作者", value: "", options: ["操作员1", "操作员2"] },
]);

// 其他筛选字段
const transferType = ref("含备注与代理");
const startTime = ref("");
const endTime = ref("");

// 按钮事件
// const confirm = () => {
//   console.log(
//     "确认筛选条件:",
//     filters.value,
//     transferType.value,
//     startTime.value,
//     endTime.value
//   );
//   emit("confirm", {
//     filters: filters.value,
//     transferType: transferType.value,
//     startTime: startTime.value,
//     endTime: endTime.value,
//   });
// };

// const closeModal = () => {
//   emit("close");
// };
</script>
