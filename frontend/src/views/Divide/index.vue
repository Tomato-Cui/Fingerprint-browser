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
import { cn } from "@/util/lib";
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
  environment_delete,
} from "@/commands/environment";
import DataTable, { type Payment } from "./data-table.vue";
import { PrimaryButton } from "@/components/button";
import { browser_starts, browser_stops } from "@/commands/browser";
import { useBrowserStatusStore } from "@/stores/browser";
import { toast } from "vue-sonner";
import { convertToCSV, downloadCSV } from "@/util/lib";
import CreateGroup from "./com/create-group.vue";
import EditAccount from "./com/edit-account.vue";
import EditProxy from "./com/edit-proxy.vue";
import { AlertModel } from "@/components/alert-model";

const route = useRoute();
const router = useRouter();
const browserStatusStore = useBrowserStatusStore();
const data = ref<Array<Payment>>([]);
const copyData = ref<Array<Payment>>([]);
const selectData = ref<string[]>([]);
const searchType = ref<{ title: keyof Payment; value: string }>({
  title: "name",
  value: "名称",
});
const groupSelect = ref<string | undefined>();
const columns = ref<any[]>([]);
const createGroupDialog = ref(false); // 创建分组
const editAccountDialog = ref(false); // 编辑账号
const editProxyDialog = ref(false); // 修改代理
const removeDialog = ref(false); // 删除环境
const hiddenColumns = {
  id: false,
  uuid: false,
  user_uuid: false,
  proxy_host: false,
  proxy_port: false,
};

const onSyncColumns = (value: any) => (columns.value = value);
const pagination = reactive({
  pageIndex: 0,
  pageSize: 16,
  total: 0,
});

const loadData = (groupId: number, index: number, size: number) => {
  // console.log("加载数据", environmentUuid, index, size);
  environment_query_by_group(groupId, index, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
    copyData.value = data.value;
  });
};

onMounted(() =>
  loadData(Number(route.params.id), pagination.pageIndex, pagination.pageSize)
);
const paginationClickHandle = (index: number) => {
  loadData(Number(route.params.id), index, pagination.pageSize);
  pagination.pageIndex = index;
};
const openGroup = async () => {
  let ids = [...selectData.value].map((item) => item) as string[];

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
  let ids = [...selectData.value].map((item) => item) as string[];
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
  let ids = [...selectData.value].map((item) => item) as string[];

  environment_batch_delete(ids)
    .then((_: any) => {
      data.value = data.value.filter((item) => !ids.includes(item.uuid));
      selectData.value = [];
    })
    .catch((err) => {
      toast.warning(err);
    });

  selectData.value = [];
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

const searchValueHandle = (value: string) => {
  data.value = copyData.value;
  if (value.length != 0) {
    data.value = data.value.filter((item) => {
      let current = item[searchType.value.title as keyof Payment] as string;
      return current ? current.includes(value) : false;
    });
  } else {
    loadData(
      Number(route.params.id),
      pagination.pageIndex,
      pagination.pageSize
    );
  }
};

// 监听路由中id的变化
watch(
  () => route.params.id,
  (value: any) => {
    loadData(Number(value), pagination.pageIndex, pagination.pageSize);
  }
);

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
    loadData(
      Number(route.params.id),
      pagination.pageIndex,
      pagination.pageSize
    );
  }
});

//下拉菜单中的的按钮
var environmentUuid = "";
var environmentId = 0;
var userUuid = "";

const editEnvBtn = (uuid: string) => {
  environmentUuid = uuid;
  const environment = data.value.find((item) => item.uuid === environmentUuid);
  router.push({
    path: "/environment-action/create",
    query: {
      id: uuid,
      action: "edit",
      environment: JSON.stringify(environment),
    },
  });
};

const editAccountBtn = (uuid: string, id: number, user_uuid: string) => {
  environmentUuid = uuid;
  environmentId = id;
  userUuid = user_uuid;
  editAccountDialog.value = true;
};

const editProxyBtn = (uuid: string) => {
  environmentUuid = uuid;
  editProxyDialog.value = true;
};

const authMemberBtn = (uuid: string) => {
  environmentUuid = uuid;
};

const transferEnvBtn = (uuid: string) => {
  environmentUuid = uuid;
};

const setCommonBtn = (uuid: string) => {
  environmentUuid = uuid;
};

const clickDel = ref(false);
const handleSubmitDel = () => {
  console.log("uuid:", environmentUuid);

  environment_delete(environmentUuid).then((res) => {
    if (res.message && (res.message as string).includes("删除成功")) {
      toast.success("删除成功");
      //过滤当前被删除的一行
      data.value = data.value.filter((item) => item.uuid !== environmentUuid);
    } else {
      toast.warning("删除失败");
    }
  });
  clickDel.value = false;
};

const removeHandle = (uuid: string) => {
  environmentUuid = uuid;
  removeDialog.value = true;
};
const removeSubmitHandle = () => {
  environment_delete(environmentUuid).then((res) => {
    res.code == 0 ? toast.warning(res.message) : toast.success(res.message);
    loadData(
      Number(route.params.id),
      pagination.pageIndex,
      pagination.pageSize
    );
  });
  removeDialog.value = false;
};
</script>

<template>
  <!-- {{ route.params.id }} -->
  <div class="flex flex-col p-3 bg-gray-50 h-main">
    <div
      class="flex overflow-hidden flex-col flex-1 px-2 bg-white rounded-lg shadow"
    >
      <div class="flex flex-col pb-2 space-y-4">
        <div class="flex w-full">
          <div class="flex gap-2 items-center py-2 w-3/4">
            <!-- 创建分组 -->
            <!-- <PrimaryButton class="flex gap-x-2" @click="createGroup">
              创建分组
            </PrimaryButton>
            <SearchInput
              :search-current-type="searchType"
              @update:searchType="(value: any) => (searchType = value)"
              @update:searchValue="searchValueHandle"
            />
            </PrimaryButton> -->
            <SearchInput
              :search-current-type="searchType"
              @update:searchType="(value: any) => (searchType = value)"
              @update:searchValue="searchValueHandle"
            />
          </div>
          <div class="flex flex-auto gap-2 justify-end px-2 py-2 ju">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <TooltipButton
                  title="筛选"
                  class="p-2.5 rounded border border-gray-200 hover:bg-gray-100"
                >
                  <LogsIcon class="w-5 h-5 text-gray-600" />
                </TooltipButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <template v-for="column in columns">
                  <DropdownMenuCheckboxItem
                    v-if="
                      !Object.keys(hiddenColumns).some((item) => {
                        return item.toLowerCase() === column.id.toLowerCase();
                      })
                    "
                    :key="column.id"
                    :checked="column.getIsVisible()"
                    class="capitalize"
                    @update:checked="
                      (value) => column.toggleVisibility(!!value)
                    "
                  >
                    {{ column.id }}
                  </DropdownMenuCheckboxItem>
                </template>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
        <div class="flex gap-x-2 w-full">
          <PrimaryButton class="flex gap-x-2" @click="openGroup">
            <PackageOpenIcon />
            打开
          </PrimaryButton>
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

      <div class="flex flex-col h-full">
        <div class="overflow-auto flex-auto h-0 rounded-md">
          <DataTable
            :data="data"
            :hiddenColumns="hiddenColumns"
            :pagination="pagination"
            @editEnvBtn="editEnvBtn"
            @editAccountBtn="editAccountBtn"
            @editProxyBtn="editProxyBtn"
            @authMemberBtn="authMemberBtn"
            @transferEnvBtn="transferEnvBtn"
            @setCommonBtn="setCommonBtn"
            @removeEnv="removeHandle"
            @onSyncColumns="onSyncColumns"
            @onSelect="(v: string[]) => selectData = v"
          />
        </div>

        <div class="flex justify-end items-center py-1 space-x-2">
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
  <!-- 创建分组 -->
  <CreateGroup v-model:createGroupDialog="createGroupDialog" />
  <!-- 修改账号 -->
  <EditAccount
    v-model:editAccountDialog="editAccountDialog"
    :environmentId="environmentId"
    :environmentUuid="environmentUuid"
    :userUuid="userUuid"
  />
  <!-- 修改代理 -->
  <EditProxy
    v-model:editProxyDialog="editProxyDialog"
    :environmentUuid="environmentUuid"
  />
  <!-- 删除 -->
  <Teleport to="body" class="z-30">
    <div
      class="flex fixed inset-0 z-30 justify-center items-center bg-black/20"
      v-if="clickDel"
    >
      <div class="bg-white rounded-lg w-[400px]">
        <!-- Modal Header -->
        <div class="flex justify-between items-center p-4">
          <h2 class="text-lg font-medium">提示</h2>
          <button
            @click="clickDel = false"
            class="text-gray-400 hover:text-gray-600"
          >
            <XIcon class="w-5 h-5" />
          </button>
        </div>
        <p class="px-[40px]">确认删除吗？</p>

        <!-- Modal Footer -->
        <div class="flex justify-center p-4 space-x-3">
          <button
            @click="handleSubmitDel"
            class="px-8 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700"
          >
            确定
          </button>
          <button
            @click="clickDel = false"
            class="px-8 py-2 rounded-md border hover:bg-gray-50"
          >
            取消
          </button>
        </div>
      </div>
    </div>
  </Teleport>
  <AlertModel
    title="删除环境"
    :open="removeDialog"
    @close="() => (removeDialog = false)"
    @cancel="() => (removeDialog = false)"
    @submit="removeSubmitHandle"
  >
    <div
      class="text-orange-400 border-[1px] p-2 px-4 border-orange-400 rounded-md bg-orange-100 flex items-center gap-x-4 text-sm"
    >
      删除成功后，可以前往回收站恢复
    </div>
    <div class="flex flex-col gap-y-4 py-4 text-sm">
      <p>
        环境UUID
        <span class="p-2 ml-4 text-blue-600 bg-blue-200 rounded-md">{{
          environmentUuid
        }}</span>
      </p>
    </div>
  </AlertModel>
</template>
