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
import { useRoute, useRouter } from "vue-router";
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
import {
  environment_batch_delete,
  environment_query_by_group,
} from "@/commands/environment";
import DataTable, { type Payment } from "./data-table.vue";
import { PrimaryButton } from "@/components/button";
import { browser_starts, browser_stops } from "@/commands/browser";
import { useBrowserStatusStore } from "@/stores/browser";
import { toast } from "vue-sonner";
import { convertToCSV, downloadCSV } from "@/util/lib";
import { environment_delete } from "@/commands/environment";
import CreateGroup from "./com/create-group.vue"
import EditAccount from "./com/edit-account.vue"
import EditProxy from "./com/edit-proxy.vue";

const route = useRoute();
const router = useRouter();
const browserStatusStore = useBrowserStatusStore();
const data = ref<Array<Payment>>([]);
const copyData = ref<Array<Payment>>([]);
const selectData = ref<Number[]>([]);
const searchType = ref<{ title: keyof Payment; value: string }>({
  title: "name",
  value: "名称",
});
const groupSelect = ref<string | undefined>();
const columns = ref<any[]>([]);
onMounted(() => {
  console.log("当前分组id：", route.params.id);
})
const createGroupDialog = ref(false)  // 创建分组
const editAccountDialog = ref(false)  // 编辑账号
const editProxyDialog = ref(false)  // 修改代理
const onSyncColumns = (value: any) => (columns.value = value);
const pagination = reactive({
  pageIndex: 1,
  pageSize: 16,
  total: 0,
});

const loadData = (groupId: number, index: number, size: number) => {
  // console.log("加载数据", groupId, index, size);
  environment_query_by_group(groupId, index, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
    copyData.value = data.value
  });
};

onMounted(() => loadData(Number(route.params.id), pagination.pageIndex, pagination.pageSize));
const paginationClickHandle = (index: number) => {
  loadData(Number(route.params.id), index, pagination.pageSize);
  pagination.pageIndex = index;
};
const openGroup = async () => {
  let ids = [...selectData.value].map(
    (item) => ({ environment_id: item } as any)
  );
  try {
    let data = await browser_starts(ids);
    data = await data.data;
    let status = Object.values(data).map((item: any) => ({
      id: item.environment_id,
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
  let ids = [...selectData.value].map((item) => item) as number[];
  browser_stops(ids).then((res: any) => {
    if (res.message && (res.message as string).includes("关闭成功")) {
      ids.forEach((item) => browserStatusStore.updateStatus(item, false));
    } else {
      toast.warning("启动失败");
    }
  });
};
// 批量删除
const batchDelete = () => {
  let ids = [...selectData.value].map((item) => item) as number[];
  environment_batch_delete(ids)
    .then((_: any) => {
      data.value = data.value.filter((item) => !ids.includes(item.id));
      selectData.value = [];
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
    click: () => { },
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

const createGroup = () => {
  createGroupDialog.value = true
};

const searchValueHandle = (value: string) => {
  data.value = copyData.value
  if (value.length != 0) {
    data.value = data.value.filter((item) => {
      let current = item[searchType.value.title as keyof Payment] as string;
      return current ? current.includes(value) : false;
    });
  } else {
    loadData(Number(route.params.id), pagination.pageIndex, pagination.pageSize);
  }
};

// 监听路由中id的变化
watch(() => route.params.id, (value: any) => {
  loadData(Number(value), pagination.pageIndex, pagination.pageSize);
});

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
    loadData(Number(route.params.id), pagination.pageIndex, pagination.pageSize);
  }
});

//下拉菜单中的的按钮
var groupId = 0
// ----编辑环境
const editEnvBtn = (id: number) => {
  console.log("编辑环境:", id);
  groupId = id
  router.push({ path: '/environment-action/create', query: { id: id } })
};
// ----编辑账号
const editAccountBtn = (id: number) => {
  console.log("编辑账号:", id);
  groupId = id
  editAccountDialog.value = true
};
// ----修改代理
const editProxyBtn = (id: number) => {
  console.log("修改代理:", id);
  groupId = id
  editProxyDialog.value = true
};
// ----授权成员
const authMemberBtn = (id: number) => {
  groupId = id
  console.log("授权成员:", id);
};
// ----转移环境
const transferEnvBtn = (id: number) => {
  groupId = id
  console.log("转移环境:", id);
};
// ----设为常用
const setCommonBtn = (id: number) => {
  groupId = id
  console.log("设为常用:", id);
};
// ----删除
const removeEnv = (id: number) => {
  console.log("删除:", id);
  groupId = id
  clickDel.value = true
};

// 删除
const clickDel = ref(false)
const handleSubmitDel = () => {
  environment_delete(groupId).then((res) => {
    if (res.message && (res.message as string).includes("删除成功")) {
      toast.success("删除成功");
      //过滤当前被删除的一行
      data.value = data.value.filter((item) => item.id !== groupId);
    } else {
      toast.warning("删除失败");
    }
  });
  clickDel.value = false
}
</script>

<template>
  <!-- {{ route.params.id }} -->
  <div class="flex flex-col p-3 bg-gray-50 h-main">
    <div class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow px-2">
      <div class="flex flex-col pb-2 space-y-4">
        <div class="flex w-full">
          <div class="flex gap-2 items-center py-2 w-3/4">
            <!-- <GroupSelect @select="groupSelectHandle" /> -->
            <PrimaryButton class="flex gap-x-2" @click="createGroup">
              <!-- <PackageOpenIcon /> -->
              创建分组
            </PrimaryButton>
            <SearchInput :search-current-type="searchType" @update:searchType="(value: any) => (searchType = value)"
              @update:searchValue="searchValueHandle" />
          </div>
          <div class="flex gap-2 py-2 ju justify-end flex-auto px-2">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <TooltipButton title="筛选" class="p-2.5 hover:bg-gray-100 rounded border-gray-200 border">
                  <LogsIcon class="h-5 w-5 text-gray-600" />
                </TooltipButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuCheckboxItem v-for="column in columns" :key="column.id" class="capitalize"
                  :checked="column.getIsVisible()" @update:checked="(value) => column.toggleVisibility(!!value)">
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
          <TooltipButton v-for="(item, index) in groupOperationBtns" :key="index"
            class="p-2.5 hover:bg-gray-100 rounded border-gray-200 border" :title="item.title" @click="item.click"
            :disabled="item.disabled">
            <component :is="item.icon" class="h-5 w-5 text-gray-600" />
          </TooltipButton>
        </div>
      </div>

      <div class="flex flex-col h-full">
        <div class="rounded-md flex-auto h-0 overflow-auto">
          <DataTable :data="data" :pagination="pagination" @editEnvBtn="editEnvBtn" @editAccountBtn="editAccountBtn"
            @editProxyBtn="editProxyBtn" @authMemberBtn="authMemberBtn" @transferEnvBtn="transferEnvBtn"
            @setCommonBtn="setCommonBtn" @removeEnv="removeEnv" @onSyncColumns="onSyncColumns"
            @onSelect="(v: Number[]) => selectData = v" />
        </div>

        <div class="flex items-center justify-end space-x-2 py-1">
          <div class="flex-1 text-sm text-muted-foreground">
            共{{ pagination.total }}条.
          </div>
          <div class="space-x-2">
            <Pagination :total="pagination.total" :itemsPerPage="pagination.pageSize" :default-page="1">
              <PaginationList v-slot="{ items }" class="flex items-center gap-1">
                <PaginationFirst @click="() => paginationClickHandle(0)" />
                <PaginationPrev @click="() => paginationClickHandle(pagination.pageIndex - 1)" />

                <template v-for="(item, index) in items">
                  <PaginationListItem v-if="item.type === 'page'" :key="index" :value="item.value" as-child>
                    <Button class="w-10 h-10 p-0" @click="() => paginationClickHandle(index)" :variant="item.value === pagination.pageIndex + 1 ? 'default' : 'outline'
                      ">
                      {{ item.value }}
                    </Button>
                  </PaginationListItem>
                  <PaginationEllipsis v-else :key="item.type" :index="index" />
                </template>

                <PaginationNext @click="() => paginationClickHandle(pagination.pageIndex + 1)" />
                <PaginationLast @click="() =>
                    paginationClickHandle(
                      Math.ceil(pagination.total / pagination.pageSize) - 1
                    )
                  " />
              </PaginationList>
            </Pagination>
          </div>
        </div>
      </div>
    </div>
  </div>
  <!-- 创建分组 -->
  <CreateGroup v-model:createGroupDialog="createGroupDialog" />
  <!-- 修改账号 -->
  <EditAccount v-model:editAccountDialog="editAccountDialog" :groupId="groupId"/>
  <!-- 修改代理 -->
  <EditProxy v-model:editProxyDialog="editProxyDialog" :groupId="groupId"/>
  <!-- 删除 -->
  <Teleport to="body" class="z-30">
    <div class="fixed inset-0 bg-black/20 flex items-center justify-center z-30" v-if="clickDel">
      <div class="bg-white rounded-lg w-[400px]">
        <!-- Modal Header -->
        <div class="flex justify-between items-center p-4">
          <h2 class="text-lg font-medium">提示</h2>
          <button @click="clickDel = false" class="text-gray-400 hover:text-gray-600">
            <XIcon class="w-5 h-5" />
          </button>
        </div>
        <p class="px-[40px]">确认删除吗？</p>

        <!-- Modal Footer -->
        <div class="flex justify-center space-x-3 p-4">
          <button @click="handleSubmitDel" class="px-8 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
            确定
          </button>
          <button @click="clickDel = false" class="px-8 py-2 border rounded-md hover:bg-gray-50">
            取消
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
