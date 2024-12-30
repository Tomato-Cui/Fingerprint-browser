<script setup lang="ts">
import { ref, onMounted, reactive, computed } from "vue";
import { Button } from "@/components/ui/button";
import {
  Trash2Icon,
  Recycle,
  Delete,
  TrashIcon,
  PackageIcon,
  RotateCcw,
} from "lucide-vue-next";

import {} from "@/components/select";
import SearchInput from "./search-input.vue";

import {
  environment_trash_query,
  environment_trash_recover,
  environment_trash_clean,
  environment_trash_recover_all,
  environment_trash_recovers,
  environment_trash_delete_batch,
} from "@/commands/environment-trash";

import TooltipButton from "@/components/tooltip-button.vue";
import { PrimaryButton } from "@/components/button";
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

interface Payment {
  id: number;
  uuid: string;
  name: string;
  group: string;
  location: string;
  description: string;
  delete_from_user_nickname: string;
  domain_name: string;
  remark: string;
  deleted_username: string;
  deleted_at: string;
  selected?: boolean;
}

const searchType = ref<{ title: keyof Payment; value: string }>({
  title: "name",
  value: "名称",
});

const data = ref<Array<Payment>>([]);

const pagination = reactive({
  pageIndex: 0,
  pageSize: 10,
  total: 0,
});

const loadData = (index: number, size: number) => {
  environment_trash_query(index, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
  });
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));

// 全选复选框的状态
const selectAll = ref(false);

// 切换所有行的选择状态
const toggleAll = () => {
  data.value.forEach((row) => {
    row.selected = selectAll.value;
  });
};

const toggleRowSelection = (row: Payment) => {
  row.selected = !row.selected;
};

const deleteAll = () => {
  environment_trash_clean();
  pagination.total = 0;
  data.value = [];
};

const recoverAll = () => {
  environment_trash_recover_all();
  data.value = [];
  // loadData(pagination.pageIndex, pagination.pageSize);
};

const handleDelete = async (uuid: string) => {
  console.log("uuid", uuid);
  await environment_trash_delete_batch([uuid]);
  loadData(pagination.pageIndex, pagination.pageSize);
};

const handleRecover = async (uuid: string) => {
  await environment_trash_recover(uuid).then(() => {
    loadData(pagination.pageIndex, pagination.pageSize);
  });
};

const paginationClickHandle = (index: number) => {
  loadData(index, pagination.pageSize);
  pagination.pageIndex = index;
};

const selectData = computed(() => {
  return data.value.filter((item) => item.selected);
});

const groupOperationBtns = computed(() => [
  {
    title: "多选恢复",
    icon: PackageIcon,
    click: async () => {
      let ids = selectData.value.map((item) => item.uuid);

      await environment_trash_recovers(ids);
      pagination.total = pagination.total - selectData.value.length;
      loadData(pagination.pageIndex, pagination.pageSize);
      selectAll.value = false;
    },

    disabled: selectData.value.length <= 0,
  },

  {
    title: "多选删除",
    icon: TrashIcon,
    click: async () => {
      let ids = selectData.value.map((item) => item.uuid);

      await environment_trash_delete_batch(ids);
      pagination.total = pagination.total - selectData.value.length;
      loadData(pagination.pageIndex, pagination.pageSize);
      selectAll.value = false;
    },
    disabled: selectData.value.length <= 0,
  },
]);

const searchValueHandle = (value: string) => {
  if (value.length != 0) {
    console.log(value);
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
  <div
    class="flex flex-col p-4 bg-gray-50 h-main overflow-auto overflow-x-auto min-w-[900px]"
  >
    <div
      class="flex overflow-hidden flex-col flex-1 py-4 bg-white rounded-lg shadow"
    >
      <!-- Header -->
      <div class="mb-4 bg-white rounded-lg">
        <div class="flex justify-between w-full">
          <!-- 搜索框 -->

          <div class="flex gap-x-4 justify-center">
            <div class="flex gap-2 items-center py-2 ml-4 w-3/4">
              <!-- <GroupSelect @select="groupSelectHandle" /> -->
              <SearchInput
                :search-current-type="searchType"
                @update:searchType="(value:any) => (searchType = value)"
                @update:searchValue="searchValueHandle"
              />
            </div>

            <div class="flex gap-x-4 items-center">
              <TooltipButton
                v-for="(item, index) in groupOperationBtns"
                :key="index"
                class="p-2.5 rounded border border-gray-200 hover:bg-gray-100"
                :title="item.title"
                @click="item.click"
                :disabled="item.disabled"
              >
                <component :is="item.icon" class="w-5 h-5 text-gray-600" />
              </TooltipButton>
            </div>
          </div>

          <div class="flex">
            <div class="ml-4">
              <PrimaryButton @click="recoverAll">
                <div class="flex justify-center items-center">
                  <Recycle class="w-5 h-5" />

                  <span>全部恢复</span>
                </div>
              </PrimaryButton>
            </div>

            <div class="ml-4">
              <PrimaryButton @click="deleteAll">
                <div
                  class="flex justify-center items-center"
                  style="text-align: center"
                >
                  <Delete class="w-5 h-5" />
                  <span>全部删除</span>
                </div>
              </PrimaryButton>
            </div>
          </div>
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
            <th class="px-4 py-3 text-sm font-medium text-left">账号平台</th>
            <th class="px-4 py-3 text-sm font-medium text-left">描述</th>
            <th class="px-4 py-3 text-sm font-medium text-left">操作者</th>
            <th class="px-4 py-3 text-sm font-medium text-left">删除时间</th>
            <th class="px-4 py-3 text-sm font-medium text-left">操作</th>
          </tr>
        </thead>
        <tbody class="">
          <tr
            v-for="(row, index) in data"
            :key="index"
            class="hover:bg-blue-100 h-[20px]"
            :class="{ 'bg-blue-50': row.selected }"
            @click="toggleRowSelection(row)"
          >
            <td class="px-4 py-3">
              <input type="checkbox" class="rounded" v-model="row.selected" />
            </td>
            <td class="px-4 py-3 text-sm">{{ index + 1 }}</td>
            <td class="px-4 py-3 text-sm">{{ row.name }}</td>

            <td class="px-4 py-3">
              <div class="flex items-center">
                <img src="../../assets/icons/stop.svg" class="mr-1 w-4 h-4" />
                <span
                  v-if="row.domain_name"
                  class="px-2 py-1 text-xs text-blue-800 bg-blue-100 rounded"
                >
                  {{ row.domain_name }}
                </span>
              </div>
            </td>

            <td class="px-4 py-3 text-sm">{{ row.description }}</td>
            <td class="px-4 py-3 text-sm">
              {{ row.delete_from_user_nickname }}
            </td>
            <td class="px-4 py-3 text-sm">{{ row.deleted_at }}</td>
            <td class="px-4 py-3 text-sm">
              <div class="flex items-center">
                <button
                  class="flex items-center px-2 py-1 text-sm text-blue-600 rounded-md hover:bg-green-50"
                  @click="handleRecover(row.uuid)"
                >
                  <RotateCcw class="mr-1 w-4 h-4" />
                  恢复
                </button>
                <button
                  class="flex items-center px-2 py-1 text-sm text-blue-600 rounded-md hover:bg-red-50"
                  @click="handleDelete(row.uuid)"
                >
                  <Trash2Icon class="mr-1 w-4 h-4" />
                  删除
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>

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
