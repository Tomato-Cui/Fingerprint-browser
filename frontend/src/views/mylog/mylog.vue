<script setup>
import { ref, computed, onMounted } from "vue";
import datatable from "./datatable.vue";
import {
  NetworkIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  InfoIcon,
  ChevronDownIcon,
} from "lucide-vue-next";
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
import { IconTransform } from "@/assets/icons";
import { opeartion_query } from "@/commands/operation_log";
import { toast } from "vue-sonner";

const selectAll = ref(false);
const currentPage = ref(3);
const totalItems = ref(101);
const itemsPerPage = ref(10);

const totalPages = computed(() =>
  Math.ceil(totalItems.value / itemsPerPage.value)
);

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

const tableData = ref([]);
onMounted(() => {
  opeartion_query(0, 10).then((res) => {
    if (res.code == 1) {
      tableData.value = res.data.data.map((item) => ({
        id: item.id,
        selected: false,
        member: item.user_uuid,
        operation: "修改窗口",
        content: "/",
        time: item.created_at,
        result: "失败",
        ip: "MEIGO.CN",
        logisopen: false,
      }));
    } else {
      toast.warning("数据获取失败");
    }
  });
});

const menuItems = [
  // { id: 1, label: "窗口" },
  // { id: 2, label: "代理" },
  // { id: 3, label: "账号" },
  // { id: 4, label: "项目" },
  // { id: 5, label: "扩展" },
  { id: 6, label: "团队" },
];
const selected = ref([]);

const selectedLabels = computed(() => {
  return menuItems
    .filter((item) => selected.value.includes(item.id))
    .map((item) => item.label);
});

const toggleItem = (id) => {
  const index = selected.value.indexOf(id);
  if (index === -1) {
    selected.value.push(id);
  } else {
    selected.value.splice(index, 1);
  }
};

const toggleAll = () => {
  tableData.value.forEach((item) => {
    item.selected = selectAll.value;
  });
};
</script>

<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <!-- Table and Pagination Section -->
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
      style="padding: 20px"
    >
      <!-- Set fixed height here -->
      <!-- Filters -->
      <div class="flex gap-4 mb-6">
        <div class="relative">
          <Select>
            <SelectTrigger style="min-width: 250px">
              <SelectValue
                placeholder="所有成员"
                class="p-2 w-full rounded-lg outline-none"
              />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem value="所有成员"> 所有成员 </SelectItem>
                <SelectItem value="选择2"> 选择2 </SelectItem>
                <SelectItem value="选择3"> 选择3 </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <div class="relative" style="min-width: 250px">
          <Popover>
            <PopoverTrigger as-child>
              <button
                class="flex justify-between p-2 px-4 w-full text-sm text-gray-500 rounded-md border border-gray-300 outline-none"
              >
                <span>{{
                  selectedLabels.length ? selectedLabels.join(", ") : "全部操作"
                }}</span>
                <ChevronDownIcon class="w-4 h-4" />
              </button>
            </PopoverTrigger>
            <PopoverContent
              class="p-2 w-48 text-sm min-w-48"
              align="end"
              style="min-width: 250px"
            >
              <div class="bg-white rounded-md">
                <div class="py-2">
                  <label
                    v-for="item in menuItems"
                    :key="item.id"
                    class="flex items-center px-4 py-2 w-full text-sm text-gray-700 cursor-pointer hover:bg-gray-50"
                  >
                    <input
                      type="checkbox"
                      :checked="selected.includes(item.id)"
                      @change="toggleItem(item.id)"
                      class="mr-3 w-4 h-4 text-blue-600 rounded border-gray-300 focus:ring-blue-500"
                    />
                    <component
                      class="h-5 text-gray-500"
                      :class="{ 'text-blue-600': selected.includes(item.id) }"
                    />
                    <span
                      class="text-sm"
                      :class="{
                        'text-blue-600 font-medium': selected.includes(item.id),
                      }"
                    >
                      {{ item.label }}
                    </span>
                  </label>
                </div>
              </div>
            </PopoverContent>
          </Popover>
        </div>

        <div class="relative">
          <datatable></datatable>
        </div>
      </div>

      <!-- Table -->
      <div class="overflow-auto flex-1">
        <table class="w-full">
          <!-- Table Header -->
          <thead class="bg-gray-50 sticky top-0 z-10">
            <tr>
              <th class="p-4 text-sm text-left font-medium text-gray-600">
                成员
              </th>
              <th class="p-4 text-sm text-left font-medium text-gray-600">
                操作
              </th>
              <th class="p-4 text-sm text-left font-medium text-gray-600">
                具体内容
              </th>
              <th class="p-4 text-sm text-left font-medium text-gray-600">
                时间
              </th>
              <th class="p-4 text-sm text-left font-medium text-gray-600">
                结果
              </th>
              <th class="p-4 text-sm text-left font-medium text-gray-600">
                操作者IP
              </th>
            </tr>
          </thead>

          <tbody>
            <tr
              v-for="item in tableData"
              :key="item.id"
              :class="{
                'border-t border-gray-100': true,
                'bg-blue-50 hover:bg-blue-100': item.selected,
                'hover:bg-blue-100': !item.selected,
              }"
            >
              <td class="p-4 text-sm whitespace-nowrap text-left">
                {{ item.member }}
              </td>
              <td class="p-4 text-sm whitespace-nowrap text-left">
                {{ item.operation }}
              </td>
              <td class="p-4 text-sm whitespace-nowrap text-left">
                {{ item.content }}
              </td>
              <td class="p-4 text-sm whitespace-nowrap text-left">
                {{ item.time }}
              </td>
              <td class="p-4 text-sm whitespace-nowrap text-left">
                <div>
                  <span
                    class="px-2 py-1 rounded text-xs text-left"
                    :class="
                      item.result === '成功'
                        ? 'bg-green-100 text-green-600'
                        : 'bg-red-100 text-red-600'
                    "
                  >
                    {{ item.result }}
                  </span>
                </div>
              </td>

              <td class="p-4 text-sm text-left">
                <div class="flex items-center justify-start gap-2">
                  <span>{{ item.ip }}</span>
                  <Popover v-model:open="item.logisopen">
                    <PopoverTrigger as-child>
                      <NetworkIcon
                        class="h-4 w-4 text-gray-400"
                        @mouseenter="item.logisopen = true"
                        @mouseleave="item.logisopen = false"
                      />
                    </PopoverTrigger>
                    <PopoverContent
                      class="min-w-24 w-24 p-2 text-sm space-y-2"
                      align="center"
                    >
                      <p>{{ item.ip }}</p>
                      <p>{{ item.operation }}</p>
                    </PopoverContent>
                  </Popover>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div class="bg-white p-4 mt-auto flex items-center justify-between">
        <div class="text-sm text-gray-600">共 {{ totalItems }} 项数据</div>
        <div class="flex items-center gap-2">
          <button
            class="p-2 hover:bg-gray-100 rounded"
            :disabled="currentPage === 1"
          >
            <ChevronLeftIcon class="w-5 h-5" />
          </button>
          <div class="flex gap-1">
            <button
              v-for="page in visiblePages"
              :key="page"
              class="px-3 py-1 rounded"
              :class="
                page === currentPage
                  ? 'bg-blue-600 text-white'
                  : 'hover:bg-gray-100'
              "
            >
              {{ page }}
            </button>
          </div>
          <button
            class="p-2 hover:bg-gray-100 rounded"
            :disabled="currentPage === totalPages"
          >
            <ChevronRightIcon class="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
