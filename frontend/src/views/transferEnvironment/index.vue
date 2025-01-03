<script setup>
import { cn } from "@/util/lib";
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
  TrashIcon,
  PackageIcon,
  RotateCcw,
} from "lucide-vue-next";
import { environment_trash_query } from "@/commands/environment-trash";
import { environment_transfer_query } from "@/commands/environment_transfer";

import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from "@/components/ui/pagination";
import { Button } from "@/components/ui/button";
import SearchInput from "./search-input.vue";
import TooltipButton from "@/components/tooltip-button.vue";
import { PrimaryButton } from "@/components/button";
import { environment_transfer_history_query } from "@/commands/environment-transfer-history";


const selectData = ref([]);
const columns = ref([]);
const groupSelect = ref();

const data = ref([]);

const searchType = ref({
  title: "name",
  value: "名称",
});
const onSyncColumns = (value) => (columns.value = value);

const pagination = reactive({
  pageIndex: 0,
  pageSize: 10,
  total: 0,
});

const loadData = (index, size) => {
  environment_transfer_history_query(index, size).then((res) => {
    let { data: data_, total } = res.data;

    pagination.total = total;
    data.value = data_;
  });
};

const paginationClickHandle = (index) => {
  loadData(index, pagination.pageSize);
  pagination.pageIndex = index;
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));

const searchValueHandle = (value) => {
  if (value.length != 0) {
    data.value = data.value.filter((item) => {
      let current = item[searchType.value.title ] ;
      return current ? current.includes(value) : false;
    });
  } else {
    loadData(pagination.pageIndex, pagination.pageSize);
  }
};
</script>

<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
      style="padding: 20px"
    >
      <!-- Header -->
      <div class="bg-white rounded-lg">
        <div class="flex justify-between w-full">
          <!-- 搜索框 -->

          <div class="flex gap-x-4 justify-center">
            <div class="flex gap-2 items-center py-2 mb-4">
              <!-- <GroupSelect @select="groupSelectHandle" /> -->
              <SearchInput
                :search-current-type="searchType"
                @update:searchType="(value) => (searchType = value)"
                @update:searchValue="searchValueHandle"
              />
            </div>
          </div>
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
                IP
              </th>
              <th
                class="px-4 py-3 text-sm font-medium text-left text-gray-600 min-w-48"
              >
                账号平台
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                描述
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                浏览器
              </th>
              <th class="px-4 py-3 text-sm font-medium text-left min-w-48">
                转移时间
              </th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(row, index) in data"
              :key="index"
              class="hover:bg-blue-100"
              :class="{ 'bg-blue-50': row.selected }"
              @click=""
            >
              <td class="px-4 py-3 text-sm min-w-48">
                {{ pagination.pageIndex * pagination.pageSize + index + 1 }}
              </td>
              <td class="px-4 py-3 text-sm min-w-48">
                <div class="flex items-center">
                  <span class="text-gray-500">{{ row.name }}</span>
                </div>
              </td>

              <td class="px-4 py-3 text-sm min-w-48">
                <div class="flex items-center">
                  <img
                    src="../../assets/icons/location.svg"
                    class="mr-1 w-4 h-4"
                  />
                  {{ row.ip }}
                </div>
              </td>
              <td class="px-4 py-3 min-w-48">
                <div class="flex items-center space-x-2">
                  <span class="flex items-center">
                    <span
                      class="flex justify-center items-center mr-1 w-6 h-6 text-xs text-blue-800 bg-blue-100 rounded-full"
                      >b</span
                    >
                    {{ row.domain_name || "\\" }}
                  </span>
                </div>
              </td>

              <td class="px-4 py-3 text-sm min-w-48">
                {{ row.description || "\\" }}
              </td>

              <td class="px-4 py-3 text-sm min-w-48">
                {{ row.browser || "\\" }}
              </td>
              <td class="px-4 py-3 text-sm min-w-48">
                {{ row.created_at || "\\" }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div class="flex justify-between items-center p-4 mt-auto bg-white">
        <div class="flex justify-end items-center py-1 space-x-2 w-full">
          <div class="flex-1 text-sm text-muted-foreground">
            共{{ pagination.total }}条.
          </div>
          <div class="space-x-2">
            <Pagination
              :total="pagination.total"
              :itemsPerPage="pagination.pageSize"
              :default-page="1"
            >
              <PaginationList
                v-slot="{ items }"
                class="flex gap-1 items-center"
              >
                <PaginationFirst @click="() => paginationClickHandle(0)" />
                <PaginationPrev
                  @click="() => paginationClickHandle(pagination.pageIndex - 1)"
                />

                <template v-for="(item, index) in items">
                  <PaginationListItem
                    v-if="item.type === 'page'"
                    :key="index"
                    :value="item.value"
                    as-child
                  >
                    <Button
                      :class="
                        cn(
                          'w-10 h-10 p-0',
                          item.value === pagination.pageIndex + 1
                            ? 'hover:bg-blue-700'
                            : 'hover:bg-slate-100'
                        )
                      "
                      @click="
                        () => {
                          paginationClickHandle(item.value - 1);
                        }
                      "
                      :variant="
                        item.value === pagination.pageIndex + 1
                          ? 'default'
                          : 'outline'
                      "
                    >
                      {{ item.value }}
                    </Button>
                  </PaginationListItem>
                  <PaginationEllipsis v-else :key="item.type" :index="index" />
                </template>

                <PaginationNext
                  @click="() => paginationClickHandle(pagination.pageIndex + 1)"
                />
                <PaginationLast
                  @click="
                    () =>
                      paginationClickHandle(
                        Math.ceil(pagination.total / pagination.pageSize) - 1
                      )
                  "
                />
              </PaginationList>
            </Pagination>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
