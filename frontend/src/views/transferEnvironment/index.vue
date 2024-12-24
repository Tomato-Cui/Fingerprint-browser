<script setup lang="ts">
import { ref, defineProps, defineEmits, onMounted, reactive } from "vue";
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
import { environment_trash_query } from "@/commands/environment-trash";

export interface Payment {
  id: number;
  name: string;
  group: string;
  location: string;
  remark: string;
  browser: string;
  transferTime: string;
  transferType: string;
}

// const selectAll = ref(false);

// const tableData = ref([]);

// const browserStatusStore = useBrowserStatusStore();
// const data = ref<Array<Payment>>([]);
const selectData = ref<Number[]>([]);
const columns = ref<any[]>([]);
const groupSelect = ref<string | undefined>();

const data = ref<Array<Payment>>([]);
// const searchType = ref<{ title: keyof Payment; value: string }>({
//   title: "name",
//   value: "名称",
// });

const onSyncColumns = (value: any) => (columns.value = value);

const pagination = reactive({
  pageIndex: 0,
  pageSize: 16,
  total: 0,
});

// const loadingData = (index: number, size: number) => {
//   environment_trash_query(index, size).then((res) => {
//     let { data: data_, total } = res.data;
//     pagination.total = total;
//     data_.value = data_;
//   });
// };

const loadData = (index: number, size: number) => {
  environment_trash_query(index, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
  });
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));

// const paginationClickHandle = (index) => {
//   loadData(index, pagination.pageSize);
//   pagination.pageIndex = index;
// };

// const openGroup = async () => {
//   let ids = [...selectData.value].map(
//     (item) => ({ environment_id: item } as any)
//   );
//   try {
//     let data = await browser_starts(ids);
//     data = await data.data;
//     let status = Object.values(data).map((item: any) => ({
//       id: item.environment_id,
//       status: item.status,
//     }));

//     status.forEach((item: any) => {
//       browserStatusStore.updateStatus(item.id, item.status);
//     });
//   } catch (_err) {
//     toast.warning("启动失败");
//   }
// };
</script>

<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
      style="padding: 20px"
    >
      <!-- Header -->
      <div lass="bg-white rounded-lg p-4 m-[20px] h-[50px]">
        <div class="flex justify-between items-center mb-[20px]">
          <div
            class="relative flex-1 flex items-center gap-4 min-w-[500px] max-w-[1000px]"
          >
            <!-- 搜索框 -->
            <div
              class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none hover:ring-2 hover:ring-blue-500"
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
                    <SquarePenIcon class="w-4 h-4" />备注
                  </MoreItem>
                  <MoreItem class="cursor-pointer" @click="selectT(3)">
                    <Trash2Icon class="w-4 h-4" />手机号/邮箱
                  </MoreItem>
                </MoreContent>
              </More>
            </div>

            <TooltipButton
              title="搜索条件"
              @click="openDialog"
              class="p-2 rounded border-[1px] ml-[-20px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              style="margin-left: 5px"
            >
              <MenuIcon class="w-5 h-5" />
            </TooltipButton>

            <!-- 弹窗蒙层 -->
            <div
              v-if="showDialog"
              class="flex fixed inset-0 justify-center items-center bg-gray-800 bg-opacity-50"
            >
              <!-- 弹窗内容 -->
              <div class="w-full max-w-md bg-white rounded-lg shadow-lg">
                <!-- 弹窗标题 -->
                <div class="p-4 text-xl font-semibold border-b">筛选</div>

                <!-- 筛选表单 -->
                <div class="p-4 space-y-4">
                  <!-- 循环生成选择框 -->
                  <div
                    v-for="(item, index) in filters"
                    :key="index"
                    class="flex items-center"
                  >
                    <label class="w-24 text-gray-700">{{ item.label }}</label>
                    <select
                      v-model="item.value"
                      class="flex-1 px-3 py-2 rounded-md border focus:outline-none focus:ring-2 focus:ring-blue-500"
                    >
                      <option value="" disabled>请选择</option>
                      <option
                        v-for="option in item.options"
                        :key="option"
                        :value="option"
                      >
                        {{ option }}
                      </option>
                    </select>
                  </div>

                  <!-- 转移类型 -->
                  <div class="flex items-center">
                    <label class="w-24 text-gray-700">转移类型</label>
                    <select
                      v-model="transferType"
                      class="flex-1 px-3 py-2 rounded-md border focus:outline-none focus:ring-2 focus:ring-blue-500"
                    >
                      <option value="含备注与代理">含备注与代理</option>
                    </select>
                  </div>

                  <!-- 转移时间 -->
                  <div class="flex items-center space-x-2">
                    <label class="w-24 text-gray-700">转移时间</label>
                    <input
                      type="date"
                      v-model="startTime"
                      class="flex-1 px-3 py-2 rounded-md border focus:ring-2 focus:ring-blue-500"
                      style="width: 150px"
                    />
                    <span>→</span>
                    <input
                      type="date"
                      v-model="endTime"
                      class="flex-1 px-3 py-2 rounded-md border focus:ring-2 focus:ring-blue-500"
                      style="width: 150px"
                    />
                  </div>
                </div>

                <!-- 底部按钮 -->
                <div class="flex justify-center p-4 space-x-4 border-t">
                  <button
                    @click="handleConfirm"
                    class="px-6 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600"
                  >
                    确定
                  </button>
                  <button
                    @click="handleCancel"
                    class="px-6 py-2 text-gray-800 bg-gray-200 rounded-md hover:bg-gray-300"
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
      <div class="overflow-auto flex-1">
        <table class="w-full">
          <thead class="bg-gray-50">
            <tr>
              <th
                class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48"
              >
                序号
              </th>
              <th
                class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48"
              >
                名称
              </th>
              <th
                class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48"
              >
                分组
              </th>
              <th
                class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48"
              >
                IP
              </th>
              <th
                class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48"
              >
                账号平台
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                备注
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                浏览器
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                转移时间
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                转移类型
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                转移团队名称
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                转移团队ID
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                代理类型
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                API服务商
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                操作者
              </th>
              <!-- <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
            操作
          </th> -->
              <!-- <th class="px-4 py-3 text-sm font-medium text-left text-gray-600">
            更多
          </th> -->
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
                  <img
                    src="../../assets/icons/location.svg"
                    class="mr-1 w-4 h-4"
                  />
                  {{ row.ip }}
                  <span class="ml-1 text-gray-500">{{ row.location }}</span>
                </div>
              </td>
              <td class="px-4 py-3 min-w-48">
                <div class="flex items-center space-x-2">
                  <span v-if="row.platform" class="flex items-center">
                    <span
                      class="flex justify-center items-center mr-1 w-6 h-6 text-xs text-blue-800 bg-blue-100 rounded-full"
                      >b</span
                    >
                    {{ row.platform }}
                  </span>
                  <span v-else class="text-gray-400">\</span>
                </div>
              </td>

              <td class="px-4 py-3 text-sm min-w-48">{{ row.remark }}</td>

              <td class="px-4 py-3 text-sm min-w-48">{{ row.browser }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.transferTime }}</td>
              <td class="px-4 py-3 text-sm min-w-48">{{ row.transferType }}</td>
              <td class="px-4 py-3 text-sm min-w-48">
                {{ row.transferTeamName }}
              </td>
              <td class="px-4 py-3 text-sm min-w-48">
                {{ row.transferTeamId }}
              </td>
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
    </div>
  </div>
</template>
