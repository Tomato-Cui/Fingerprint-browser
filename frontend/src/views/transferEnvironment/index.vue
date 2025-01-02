<script setup lang="ts">
import { ref, onMounted, reactive } from "vue";

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

import { environment_transfer_history_query } from "@/commands/environment-transfer-history";

interface Payment {
  id: number;
  name: string;
  group: string;
  location: string;
  description: string;
  browser: string;
  remark: string;
  created_at: string;

  domain_name: string;
  transferTime: string;
  transferType: string;
  selected?: boolean;
}

const data = ref<Array<Payment>>([]);

const searchType = ref<{ title: keyof Payment; value: string }>({
  title: "name",
  value: "名称",
});

const pagination = reactive({
  pageIndex: 0,
  pageSize: 10,
  total: 0,
});

const loadData = (index: number, size: number) => {
  environment_transfer_history_query(index, size).then((res) => {
    let { data: data_, total } = res.data;

    pagination.total = total;
    data.value = data_;
  });
};

const paginationClickHandle = (index: number) => {
  loadData(index, pagination.pageSize);
  pagination.pageIndex = index;
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));

const searchValueHandle = (value: string) => {
  if (value.length != 0) {
    data.value = data.value.filter((item) => {
      let current = item[searchType.value.title as keyof Payment] as string;
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
                @update:searchType="(value:any) => (searchType = value)"
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
                      class="p-0 w-10 h-10"
                      @click="() => paginationClickHandle(index)"
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
