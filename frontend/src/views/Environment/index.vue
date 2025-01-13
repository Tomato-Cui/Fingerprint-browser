<script lang="ts" setup>
import { cn } from "@/util/lib";
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
import { computed, onMounted, reactive, ref, watch } from "vue";
import TooltipButton from "@/components/tooltip-button.vue";
import {
  ArrowRightFromLineIcon,
  ExternalLinkIcon,
  LogsIcon,
  PackageIcon,
  PackageOpenIcon,
  TrashIcon,
  Boxes,
  SendToBack,
} from "lucide-vue-next";
import SearchInput from "./search-input.vue";
import GroupSelect from "./group-select.vue";
import {
  environment_batch_delete,
  environment_query,
  environment_query_by_group,
  environment_delete,
  environment_query_id,
} from "@/commands/environment";
import DataTable, { type Payment } from "./data-table.vue";
import { PrimaryButton } from "@/components/button";
import { browser_starts, browser_stops } from "@/commands/browser";
import { useBrowserStatusStore } from "@/stores/browser";
import { toast } from "vue-sonner";
import { convertToCSV, downloadCSV } from "@/util/lib";
import TransferModal from "./transfer-modal.vue";
import MoveToGroup from "./com/move-to-group.vue";
import EditProxy from "@/views/Environment/com/edit-proxy.vue";
import EditAccount from "@/views/Environment/com/edit-account.vue";
import { useRouter } from "vue-router";
import { AlertModel } from "@/components/alert-model";
import CreateGroup from "@/views/Divide/com/create-group.vue";

const router = useRouter();
const browserStatusStore = useBrowserStatusStore();
const data = ref<Array<Payment>>([]);
const selectData = ref<string[]>([]);
const columns = ref<any[]>([]);
const hiddenColumns = {
  id: false,
  uuid: false,
  user_uuid: false,
  proxy_host: false,
  proxy_port: false,
};
const groupSelect = ref<string | undefined>();
const createGroupDialog = ref(false); //添加分组弹出框
const searchType = ref<{ title: keyof Payment; value: string }>({
  title: "name",
  value: "名称",
});
const transferModal = ref({
  open: false,
  title: "转移环境",
  name: "",
  uuid: "",
});

const moveToGroupModal = ref({
  open: false,
  title: "设置分组",
  name: [] as string[],
  uuid: [] as string[],
});

const onSyncColumns = (value: any) => (columns.value = value);

const pagination = reactive({
  pageIndex: 1,
  pageSize: 16,
  total: 0,
});

const loadData = (index: number, size: number) => {
  environment_query(index - 1, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
  });
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));
const paginationClickHandle = (index: number) => {
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
      toast.warning("关闭失败");
    }
  });
};
const batchDelete = () => {
  let ids = [...selectData.value].map((item) => item) as any[];
  environment_batch_delete(ids)
    .then((_: any) => {
      loadData(pagination.pageIndex, pagination.pageSize);
      // data.value = data.value.filter((item) => !ids.includes(item.uuid));
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
  {
    title: "移入分组",
    icon: SendToBack,
    click: moveToGroup,
    disabled: selectData.value.length <= 0,
  },
]);
const groupSelectHandle = (value: string) => {
  groupSelect.value = value;
};

const environmentName = ref<string[]>([]);
/**
 * 移入分组
 */
const moveToGroup = () => {
  let ids = [...selectData.value].map((item) => item) as any[];
  ids.forEach((item) => {
    environment_query_id(item).then((res) => {
      environmentName.value.push(res.data.name);
    });
  });
  moveToGroupModal.value.open = true;
  moveToGroupModal.value.uuid = ids as string[];
  moveToGroupModal.value.name = environmentName.value;
  console.log("1234567890:", environmentName.value);
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

const transferEnv = (uuid: string, name: string) => {
  transferModal.value.open = true;
  transferModal.value.uuid = uuid;
  transferModal.value.name = name;
};
const editAccountDialog = ref(false); // 编辑账号
const editProxyDialog = ref(false); // 修改代理
const removeDialog = ref(false); // 删除环境
let environmentUuid = "";
let environmentId = 0;
let userUuid = "";

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

const removeHandle = (uuid: string) => {
  environmentUuid = uuid;
  removeDialog.value = true;
};
const removeSubmitHandle = () => {
  environment_delete(environmentUuid).then((res) => {
    res.code == 0 ? toast.warning(res.message) : toast.success(res.message);
    loadData(pagination.pageIndex, pagination.pageSize);
  });
  removeDialog.value = false;
};
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex overflow-hidden flex-col flex-1">
      <div class="flex flex-col pb-2 space-y-4">
        <div class="flex w-full">
          <div class="flex gap-2 items-center py-2 w-3/4">
            <GroupSelect @select="groupSelectHandle" />
            <SearchInput
              :search-current-type="searchType"
              @update:searchType="(value: any) => (searchType = value)"
              @update:searchValue="searchValueHandle"
            />
          </div>
          <div class="flex flex-auto gap-2 justify-end px-2 py-2 ju">
            <TooltipButton
              class="p-2.5 rounded border border-gray-200 hover:bg-gray-100"
              title="添加分组"
              @click="createGroupDialog = true"
            >
              <component :is="Boxes" class="w-5 h-5 text-gray-600" />
            </TooltipButton>
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <TooltipButton
                  class="p-2.5 rounded border border-gray-200 hover:bg-gray-100"
                  title="筛选"
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
            :disabled="item.disabled"
            :title="item.title"
            class="p-2.5 rounded border border-gray-200 hover:bg-gray-100"
            @click="item.click"
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
            @onSyncColumns="onSyncColumns"
            @onSelect="(v: string[]) => selectData = v"
            @onTransferEnv="transferEnv"
            @editAccountBtn="editAccountBtn"
            @editEnvBtn="editEnvBtn"
            @editProxyBtn="editProxyBtn"
            @removeEnv="removeHandle"
          />
        </div>

        <div class="flex justify-end items-center py-1 space-x-2">
          <div class="flex-1 text-sm text-muted-foreground">
            共{{ pagination.total }}条.
          </div>
          <div class="space-x-2">
            <Pagination
              :default-page="1"
              :itemsPerPage="pagination.pageSize"
              :total="pagination.total"
            >
              <PaginationList
                v-slot="{ items }"
                class="flex gap-1 items-center"
              >
                <GroupSelect @select="groupSelectHandle" />
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
    <TransferModal
      :open="transferModal.open"
      :title="transferModal.title"
      :uuid="transferModal.uuid"
      :name="transferModal.name"
      @close="transferModal.open = false"
    />

    <MoveToGroup
      :open="moveToGroupModal.open"
      :title="moveToGroupModal.title"
      :uuid="moveToGroupModal.uuid"
      :name="moveToGroupModal.name"
      @close="
        () => {
          (moveToGroupModal.open = false), (environmentName = []);
        }
      "
    />
  </div>

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
    <div class="text-sm flex flex-col gap-y-4 py-4">
      <p>
        环境UUID
        <span class="bg-blue-200 p-2 rounded-md text-blue-600 ml-4">{{
          environmentUuid
        }}</span>
      </p>
    </div>
  </AlertModel>
  <!-- 添加分组 -->
  <CreateGroup v-model:createGroupDialog="createGroupDialog" />
</template>
