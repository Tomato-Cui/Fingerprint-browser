<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <!-- Table and Pagination Section -->
    <div
        class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
        style="padding: 20px"
    >
      <!-- Search and Filter Header -->
      <div class="flex justify-between items-center mb-6">
        <div class="flex gap-4 m-w-96">
          <div
              class="flex relative flex-1 items-center ml-4 max-w-xl rounded-lg border hover:outline-none hover:ring-1 hover:ring-blue-500"
          >
            <input
                class="py-2 pr-4 pl-10 w-full rounded-lg border-gray-200 outline-none"
                placeholder="搜索或新建搜索条件"
                style="min-width: 300px"
                type="text"
            />
            <SearchIcon class="absolute top-2.5 left-3 w-5 h-5 text-gray-400"/>

            <Popover v-model:open="shaixuan">
              <PopoverTrigger as-child>
                <button class="p-2 ml-4 rounded-lg hover:bg-blue-50">
                  <SlidersHorizontalIcon class="w-5 h-5 text-gray-500"/>
                </button>
              </PopoverTrigger>
              <PopoverContent
                  align="center"
                  class="p-2 w-48 text-sm"
                  style="min-width: 150px"
              >
                <div class="py-1">
                  <button
                      v-for="(item, index) in menuItems"
                      :key="index"
                      class="px-3 py-2 w-full text-sm text-left text-gray-700 rounded-md transition-colors duration-150 hover:bg-blue-50"
                      @click="selectItem(item)"
                  >
                    {{ item.label }}
                  </button>
                </div>
              </PopoverContent>
            </Popover>
          </div>

          <div class="relative">
            <Select>
              <SelectTrigger style="min-width: 150px">
                <SelectValue
                    class="p-2 w-full rounded-lg outline-none"
                    placeholder="所有平台"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="所有平台"> 所有平台</SelectItem>
                  <SelectItem value="谷歌"> 谷歌</SelectItem>
                  <SelectItem value="火狐"> 火狐</SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </div>

        <div class="flex gap-4">
          <TooltipButton
              class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              title="删除"
              @click="deleteOpenHandle"
          >
            <TrashIcon class="w-5 h-5"/>
          </TooltipButton>

          <TooltipButton
              class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              title="刷新"
              @click="() => console.log('abc')"
          >
            <RefreshCwIcon class="w-5 h-5"/>
          </TooltipButton>
        </div>
      </div>

      <!-- Table -->
      <div class="overflow-auto overflow-x-auto flex-1">
        <table class="w-full min-w-max">
          <thead class="sticky top-0 z-10 bg-gray-50">
          <tr>
            <!-- Table headers -->
            <th class="p-4 w-12 text-center">
              <input
                  v-model="selectAll"
                  class="rounded border-gray-300"
                  type="checkbox"
                  @change="toggleAll"
              />
            </th>
            <th class="p-4 w-32 text-sm font-medium text-center text-gray-600">
              所属平台
            </th>
            <th class="p-4 w-32 text-sm font-medium text-center text-gray-600">
              所属环境
            </th>
            <th class="p-4 w-32 text-sm font-medium text-center text-gray-600">
              创建时间
            </th>
            <th class="p-4 w-32 text-sm font-medium text-center text-gray-600">
              添加人
            </th>
            <th class="p-4 w-32 text-sm font-medium text-center text-gray-600">
              操作
            </th>
          </tr>
          </thead>
          <tbody>
          <!-- Render Data for Current Page -->
          <tr
              v-for="item in tableData"
              :key="item.id"
              :class="{
          'border-t border-gray-100': true,
          'bg-blue-50 hover:bg-blue-100': item.selected,
          'hover:bg-blue-100': !item.selected,
        }"
          >
            <td class="p-4 w-12 text-center">
              <div class="flex justify-center">
                <input
                    v-model="item.selected"
                    class="rounded border-gray-300"
                    type="checkbox"
                />
              </div>
            </td>
            <td class="p-4 w-32 text-sm text-center">
              <div class="flex gap-2 justify-center items-center">
                <img :alt="item.platform" :src="item.icon" class="w-5 h-5"/>
                {{ item.platform }}
              </div>
            </td>
            <td class="p-4 w-32 text-sm text-center">
              {{ item.environment }}
            </td>
            <td class="p-4 w-32 text-sm text-center">{{ item.createdAt }}</td>
            <td class="p-4 w-32 text-sm text-center">{{ item.addedBy }}</td>
            <td class="p-4 w-32 text-sm text-center">
              <div class="flex gap-4 justify-center items-center">
                <button class="text-blue-600 hover:text-blue-700" @click="openupdateModal">
                  编辑
                </button>
                <More>
                  <MoreTrigger>
                    <EllipsisVerticalIcon
                        class="w-5 h-5 text-gray-400 cursor-pointer hover:text-gray-600"
                        @click="toggleDropdown(item.id)"
                    />
                  </MoreTrigger>
                  <MoreContent>
                    <MoreItem class="cursor-pointer" @click="showDetails(item)">
                      <Settings2Icon class="w-4 h-4"/>
                      查看详情
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="deleteOpenHandle(item)">
                      <Trash2Icon class="w-4 h-4"/>
                      删除
                    </MoreItem>
                  </MoreContent>
                </More>
              </div>
            </td>
          </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div class="flex justify-between items-center p-4 mt-auto bg-white">
        <div class="text-sm text-gray-600">共 {{ totalItems }} 项数据</div>
        <div class="flex gap-2 items-center">
          <!-- Previous Page Button -->
          <button
              :disabled="currentPage === 1"
              class="p-2 rounded hover:bg-gray-100"
              @click="goToPage(currentPage - 1)"
          >
            <ChevronLeftIcon class="w-5 h-5"/>
          </button>

          <!-- Page Number Buttons -->
          <div class="flex gap-1">
            <button
                v-for="page in visiblePages"
                :key="page"
                :class="
          page === currentPage
            ? 'bg-blue-600 text-white'
            : 'hover:bg-gray-100'
        "
                class="px-3 py-1 rounded"
                @click="goToPage(page)"
            >
              {{ page }}
            </button>
          </div>

          <!-- Next Page Button -->
          <button
              :disabled="currentPage === totalPages"
              class="p-2 rounded hover:bg-gray-100"
              @click="goToPage(currentPage + 1)"
          >
            <ChevronRightIcon class="w-5 h-5"/>
          </button>
        </div>
      </div>
    </div>


    <!-- showModelPropn -->
    <ShowModel
        :open="showModelProps.open"
        :title="showModelProps.title"
        @close="() => (showModelProps.open = false)"
    />
    <updateModel
        :open="updateModelProps.open"
        :title="updateModelProps.title"
        @close="() => (updateModelProps.open = false)"
    />

    <AlertModel
        :open="deleteModel"
        title="删除账号"
        @cancel="deleteCloseHandle"
        @close="deleteCloseHandle"
        @submit="deleteCloseHandle"
    >
      <div class="flex flex-col gap-y-4 py-4 text-sm">
        <!-- Content -->
        <div class="p-4">
          <div class="flex justify-between items-center mb-4">
            <p class="text-gray-700">你确定要删除以下账号吗？</p>
            <button
                class="flex items-center text-sm text-blue-600 hover:text-blue-700"
                @click="isExpanded = !isExpanded"
            >
              共{{ selectedAgentslen }}个账号
              <ChevronDown
                  :class="{ 'transform rotate-180': isExpanded }"
                  class="ml-1 w-4 h-4 transition-transform duration-200"
              />
            </button>
          </div>

          <div class="mb-6">
            <p class="mb-2 text-sm text-gray-600">账号名称</p>
            <div
                :class="isExpanded ? 'grid-cols-3' : 'grid-cols-3'"
                class="grid gap-2 max-h-[300px] overflow-y-auto"
            >
              <div
                  v-for="(agent, index) in displayedAgents"
                  :key="agent.id"
                  class="flex justify-between items-center px-3 py-2 text-sm bg-blue-50 rounded-md"
              >
                <span class="text-gray-700">{{ agent.platform }}</span>
                <button
                    class="text-gray-400 hover:text-gray-600"
                    @click="removeAgent(agent.id)"
                >
                  <XCircle class="w-4 h-4"/>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </AlertModel>
  </div>
</template>

<script setup>
import {computed, reactive, ref, watch} from "vue";
import ShowModel from "./showModel.vue";
import updateModel from "./updateModel.vue";
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,} from "@/components/select";
import {
  ChevronDown,
  ChevronLeftIcon,
  ChevronRightIcon,
  EllipsisVerticalIcon,
  RefreshCwIcon,
  SearchIcon,
  Settings2Icon,
  SlidersHorizontalIcon,
  Trash2Icon,
  TrashIcon,
  XCircle,
} from "lucide-vue-next";
import {More, MoreContent, MoreItem, MoreTrigger} from "@/components/more";
import {AlertModel} from "@/components/alert-model";
import {Popover, PopoverContent, PopoverTrigger,} from "@/components/ui/popover";
import TooltipButton from "@/components/tooltip-button.vue";
import { environment_account_query } from "@/commands/environment-account.ts";

const selectAll = ref(false);
const activeDropdown = ref(null);
const selectedItem = ref(null);

const currentPage = ref(0);  // 当前页

const totalItems = ref(110);   // 数据总数
const itemsPerPage = ref(1); // 每页显示的数量
const tableData = ref([]);
// 用于分页的总页数
const totalPages = computed(() =>
    Math.ceil(totalItems.value / itemsPerPage.value)
);

// 计算显示的页面范围
const visiblePages = computed(() => {
  const pages = [];
  for (let i = 1; i <= totalPages.value; i++) {
    if (
        i === 1 ||
        i === totalPages.value ||
        (i >= currentPage.value - 1 && i <= currentPage.value + 1)
    ) {
      pages.push(i);
    }
  }
  return pages;
});

// 调用 API 获取数据
const fetchData = async (pageNum, pageSize) => {
  try {
    console.log(pageNum,pageSize)
    const data = await environment_account_query(pageNum, pageSize);
    console.log(data)
    tableData.value = data.items;  // 假设 API 返回的数据是 { items: [...], total: number }
    // totalItems.value = data.total; // 总项数
  } catch (error) {
    console.error("Error fetching data", error);
  }
};

// 初始加载数据
fetchData(currentPage.value, itemsPerPage.value);

// 在页码变化时重新获取数据
watch(currentPage, (newPage) => {
  fetchData(newPage, itemsPerPage.value);
});

//修改分页按钮的点击事件
const goToPage = (page) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page;
  }
};

//更新 totalPages 计算

const toggleAll = () => {
  tableData.value.forEach((item) => {
    item.selected = selectAll.value;
  });
};

const toggleDropdown = (id) => {
  if (activeDropdown.value === id) {
    activeDropdown.value = null;
  } else {
    activeDropdown.value = id;
  }
};

const showDetails = (item) => {
  selectedItem.value = item;
  showModelProps.open = true;
  activeDropdown.value = null;
};

const showModelProps = reactive({
  open: false,
  title: "",
});

const updateModelProps = reactive({
  open: false,
  title: "",
});

const openupdateModal = () => {
  updateModelProps.open = true;
};

const deleteModel = ref(false);

const deleteOpenHandle = (item) => {
  deleteModel.value = true;
};

const deleteCloseHandle = () => {
  deleteModel.value = false;
};

const menuItems = ref([
  {id: "environment", label: "所属环境"},
  {id: "platform", label: "账号平台"},
]);

const selectItem = (item) => {
  console.log("Selected:", item.label);
  closeshaixuan();
  // 这里可以添加选择后的处理逻辑
};
const shaixuan = ref(false);
const closeshaixuan = () => {
  shaixuan.value = false;
};

const isExpanded = ref(false);

let selectedAgentslen = ref();

const displayedAgents = computed(() => {
  const selectedAgents = tableData.value.filter((agent) => agent.selected); // 只获取选中的代理
  selectedAgentslen = selectedAgents.length;
  return isExpanded.value ? selectedAgents : selectedAgents.slice(0, 5); // 展开时显示所有选中的代理，不展开时只显示前 5 个
});

const removeAgent = (id) => {
  const index = tableData.value.findIndex((agent) => agent.id === id);
  if (index !== -1) {
    tableData.value.splice(index, 1); // 从数组中删除指定代理
  }
};
</script>

<style scoped>
/* Optional: Add fade animation */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
