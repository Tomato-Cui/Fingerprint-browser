<script setup lang="ts">
import { Button } from "@/components/ui/button";
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
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ref, onMounted, reactive, computed, watch } from "vue";
import TooltipButton from "@/components/tooltip-button.vue";
import {
  LogsIcon,
  PackageOpenIcon,
  PackageIcon,
  ExternalLinkIcon,
  TrashIcon,
  ArrowRightFromLineIcon,
} from "lucide-vue-next";
import SearchInput from "./search-input.vue";
import GroupSelect from "./group-select.vue";
import {
  environment_batch_delete,
  environment_query,
  environment_query_by_group,
} from "@/commands/environment";
import DataTable, { type Payment } from "./data-table.vue";
import { PrimaryButton } from "@/components/button";
import { browser_starts, browser_stops } from "@/commands/browser";
import { useBrowserStatusStore } from "@/stores/browser";
import { toast } from "vue-sonner";
import { convertToCSV, downloadCSV } from "@/util/lib";

const browserStatusStore = useBrowserStatusStore();
const data = ref<Array<Payment>>([]);
const selectData = ref<Number[]>([]);
const columns = ref<any[]>([]);
const groupSelect = ref<string | undefined>();
const searchType = ref<{ title: keyof Payment; value: string }>({
  title: "name",
  value: "名称",
});

const onSyncColumns = (value: any) => (columns.value = value);

const pagination = reactive({
  pageIndex: 1,
  pageSize: 16,
  total: 0,
});

const loadData = (index: number, size: number) => {
  environment_query(index-1, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
  });
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));
const paginationClickHandle = (index: number) => {
  console.log("index:", index);
  
  loadData(index, pagination.pageSize);
  pagination.pageIndex = index;
};
const openGroup = async () => {
  let ids = [...selectData.value].map((item) => item) as any[];
  try {
    let data = await browser_starts(ids);
    data = await data.data;
    let status = Object.values(data).map((item: any) => ({
      id: item.environment_uuid,
      status: item.status,
    }));

    status.forEach((item: any) => {
      browserStatusStore.updateStatus(item.id, item.status);
    });
  } catch (_err) {
    toast.warning("启动失败");
  }
};
const closeGroup = () => {
  let ids = [...selectData.value].map((item) => item) as any[];
  browser_stops(ids).then((res: any) => {
    if (res.message && (res.message as string).includes("关闭成功")) {
      ids.forEach((item) => browserStatusStore.updateStatus(item, false));
    } else {
      toast.warning("启动失败");
    }
  });
};
const batchDelete = () => {
  let ids = [...selectData.value].map((item) => item) as any[];
  environment_batch_delete(ids)
    .then((_: any) => {
      data.value = data.value.filter((item) => !ids.includes(item.uuid));
      toast.success("删除成功");
    })
    .catch((err) => {
      toast.warning(err);
    });
};

const exportData = () => {
  downloadCSV(convertToCSV(data.value));
};

const groupOperationBtns = computed(() => [
  {
    title: "关闭",
    icon: PackageIcon,
    click: closeGroup,
    disabled: selectData.value.length <= 0,
  },
  {
    title: "导入",
    icon: ExternalLinkIcon,
    click: () => {},
    disabled: true,
  },
  {
    title: "导出",
    icon: ArrowRightFromLineIcon,
    click: exportData,
    disabled: selectData.value.length <= 0,
  },
  {
    title: "删除",
    icon: TrashIcon,
    click: batchDelete,
    disabled: selectData.value.length <= 0,
  },
]);
const groupSelectHandle = (value: string) => {
  groupSelect.value = value;
};

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

watch(groupSelect, (newVal) => {
  if (newVal && newVal != "0") {
    environment_query_by_group(
      parseInt(newVal),
      pagination.pageIndex,
      pagination.pageSize
    ).then((res) => {
      let { data: data_, total } = res.data;
      pagination.total = total;
      data.value = data_;
    });
  } else if (newVal == "0") {
    loadData(pagination.pageIndex, pagination.pageSize);
  }
});
</script>

<template>
  <div class="flex flex-col p-3 bg-gray-50 h-main">
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow px-2"
    >
      <div class="flex flex-col pb-2 space-y-4">
        <div class="flex w-full">
          <div class="flex gap-2 items-center py-2 w-3/4">
            <GroupSelect @select="groupSelectHandle" />
            <SearchInput
              :search-current-type="searchType"
              @update:searchType="(value:any) => (searchType = value)"
              @update:searchValue="searchValueHandle"
            />
          </div>
          <div class="flex gap-2 py-2 ju justify-end flex-auto px-2">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <TooltipButton
                  title="筛选"
                  class="p-2.5 hover:bg-gray-100 rounded border-gray-200 border"
                >
                  <LogsIcon class="h-5 w-5 text-gray-600" />
                </TooltipButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuCheckboxItem
                  v-for="column in columns"
                  :key="column.id"
                  class="capitalize"
                  :checked="column.getIsVisible()"
                  @update:checked="(value) => column.toggleVisibility(!!value)"
                >
                  {{ column.id }}
                </DropdownMenuCheckboxItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
        <div class="flex w-full gap-x-2">
          <PrimaryButton class="flex gap-x-2" @click="openGroup">
            <PackageOpenIcon />
            打开
          </PrimaryButton>
          <TooltipButton
            v-for="(item, index) in groupOperationBtns"
            :key="index"
            class="p-2.5 hover:bg-gray-100 rounded border-gray-200 border"
            :title="item.title"
            @click="item.click"
            :disabled="item.disabled"
          >
            <component :is="item.icon" class="h-5 w-5 text-gray-600" />
          </TooltipButton>
        </div>
      </div>

      <div class="flex flex-col h-full">
        <div class="rounded-md flex-auto h-0 overflow-auto">
          <DataTable
            :data="data"
            :pagination="pagination"
            @onSyncColumns="onSyncColumns"
            @onSelect="(v: Number[]) => selectData = v"
          />
        </div>

        <div class="flex items-center justify-end space-x-2 py-1">
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
                class="flex items-center gap-1"
              >
                <PaginationFirst @click="() => paginationClickHandle(1)" />
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
                  <!-- {{ index }} -->
                    <Button
                      class="w-10 h-10 p-0"
                      @click="() => paginationClickHandle(item.value - 1)"
                      :variant="
                        item.value === pagination.pageIndex
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
                        Math.ceil(pagination.total / pagination.pageSize)
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
