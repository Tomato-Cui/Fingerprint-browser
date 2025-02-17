<script setup lang="ts">
// 导入工具函数和图标组件

import { useRouter, useRoute } from "vue-router";

import SelectFilter from "./select-filter.vue";

import { computed, onMounted, reactive, ref, watch } from "vue";
import { SearchIcon } from "@/assets/icons/environment-bookmark-image";

import DelEnv from "./pop-box/del-env.vue";
import {
  environment_trash_delete_batch,
  environment_trash_query,
  environment_trash_recovers,
} from "@/commands/environment-trash";
import { IconRecover, IconReTrash } from "@/assets/icons/environment-trash";

import { toast } from "vue-sonner";
import {
  environment_proxies_batch_delete,
  environment_proxies_delete,
  environment_proxies_query,
  environment_proxies_query_by_group,
} from "@/commands/environment-proxy";
import {
  kernel_location,
  latest_kernel,
  install_location,
} from "@/commands/kernel";
import { useBrowserStatusStore } from "@/stores/browser";

import NewGroup from "./manage-proxy-view/proxy-new-group.vue";

import SingleDeleteProxy from "./manage-proxy-view/single-delete-proxy.vue";

import ExportEnv from "./pop-box/export/exportEnv.vue";
import { environment_proxy_group_query } from "@/commands/environment-proxy-group";
import GroupChoose from "./com/group-choose.vue";
import TableTheadChoose from "./com/table-thead-choose.vue";
import { AlertModel } from "@/components/alert-model";
import {
  AssCircleIcon,
  AddCardIcon,
  GroupIcon,
  BookmarkIcon,
  ApiIcon,
  WrapperIcon,
  CaretDownIcon,
  CaretUpIcon,
  MoreOperatorIcon,
} from "@/assets/icons/environment/index";

import Page from "@/views/proxy-manage/com/page.vue";
import EditProxy from "@/views/proxy-manage/manage-proxy-view/edit-proxy.vue";
import ModifyProxy from "@/views/proxy-manage/manage-proxy-view/modify-proxy.vue";

import {
  Round,
  FileText,
  PenModify,
  Plate,
} from "@/assets/icons/proxy-manage-image";
import { useProxyStatusStore } from "@/stores/proxy-status";
import { itemFromKind } from "@tauri-apps/api/menu";

// 多选
const selectedItems = ref<string[]>([]);

// 多选
const route = useRoute();

const PageObj = reactive({
  total: 0,
  pageNum: 1,
  pageSize: 17,
});

const regularItems = [
  { id: 1, label: "编辑环境", active: (id: number) => simpleEditEnv(id) },
  {
    id: 2,
    label: "导出Profile",
    active: (id: number) => simpleOp(exportEnvModel, id),
  },
  {
    id: 3,
    label: "转移",
    active: (id: number) => simpleOp(transferEnvModel, id),
  },
  { id: 4, label: "复制环境ID", active: (id: number) => copyEnvId(id) },
  { id: 5, label: "同步Cookie", active: (id: number) => syncCookie(id) },
  { id: 6, label: "运维授权", active: (id: number) => auth(id) },
  {
    id: 7,
    label: "清除缓存",
    active: (id: number) => simpleOp(cleanCacheModel, id),
  },
  {
    id: 8,
    label: "删除环境",
    active: (id: number) => simpleOp(delEnvModel, id),
  },
];

//单个操作
const simpleOp = (model: any, id: number) => {
  messageData.value = originData.value.find((item: any) => item.id === id);
  messageData.value = [messageData.value];
  model.value = true;
};

interface Proxymanage {
  id: number;
  kind: string;
  host: string;
  port: string;
  name: string;
  attribution: string;
  state: string;
  channel: string;
  group: string;
  browser_environment: string;
  phone_environment: string;
  create_message: string;
  selected: boolean;
}

const dataCache = ref<any[]>([]); // 缓存的原始数据

const tableData = ref<any>([]);

const id = ref(0);

function formatDate(inputDate: string): string {
  // 将输入字符串转换为 Date 对象
  const date = new Date(inputDate);
  // 提取年月日时分秒
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0"); // 月份从 0 开始，需要加 1
  const day = String(date.getDate()).padStart(2, "0");
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");
  // 拼接成目标格式
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

const toggleSelectItem = (id: string) => {
  const index = selectedItems.value.indexOf(id);
  if (index === -1) {
    selectedItems.value.push(id);
  } else {
    selectedItems.value.splice(index, 1);
  }
};

const originData = ref<any>([]);

const dealTableData = (data: any) => {
  let ind = 1; // 序号
  originData.value = JSON.parse(JSON.stringify(tableData.value));

  // 将字段与表头对上
  tableData.value = data.map((item: any) => {
    return {
      ind: ind++, // 序号递增
      id: item.id,
      name: item.username || "--",
      attribution: item.host + ":" + item.port || "192.1.1.1:4585",
      kind: item.kind || "--",
      action: "启动",
      state: item.deleted_at ? "已删除" : "准备好",
      channel: item.channel || "--",
      group: item.group_name || "--",
      browser_environment: item.browser_environment || "--",
      phone_environment: item.phone_environment || "--",
      create_message: item.create_message || "--",
      port: item.port || "--",
      proxy: item.host ? `${item.host}:${item.port}` : "--",
      password: item.password || "--",
      groupName: item.environment_group_id || "--",
      create_at: item.created_at,
      update_at: item.updated_at,
      delete_at: item.deleted_at,
    };
  });
};

const getList = () => {
  if (route.params.id === "0") {
    environment_proxies_query(PageObj.pageNum, PageObj.pageSize).then(
      (res: any) => {
        PageObj.total = res.data.total;
        tableData.value = res.data.data.map((item: any) => {
          return { ...item, created_at: formatDate(item.created_at) };
        });
        dealTableData(tableData.value); //字段处理
        dataCache.value = JSON.parse(JSON.stringify(res.data.data));
      }
    );
  } else {
    environment_proxies_query_by_group(
      +route.params.id,
      PageObj.pageNum,
      PageObj.pageSize
    ).then((res: any) => {
      PageObj.total = res.data.total;
      tableData.value = res.data.data.map((item: any) => {
        return { ...item, created_at: formatDate(item.created_at) };
      });
      dealTableData(tableData.value);
      dataCache.value = JSON.parse(JSON.stringify(res.data.data));
    });
  }

  selectedItems.value.length = 0;
};

const isAllSelected = computed(() => {
  console.log("selectedItems.value:", selectedItems.value);
  console.log("tableData.value:", tableData.value);
  return (
    tableData.value.length > 0 &&
    selectedItems.value.length === tableData.value.length
  );
});
const isIndeterminate = computed(() => {
  return (
    selectedItems.value.length > 0 &&
    selectedItems.value.length < tableData.value.length
  );
});

//删除环境
const delEnv = () => {
  delEnvModel.value = true;
};

const messageData = ref<any>([]); //用于传入操作组件的数据

const sortOrder = ref("asc");

const searchQuery = ref("");

// 同步Cookie
const syncCookie = (id: number) => {
  console.log("同步Cookie:", id);
  toast.warning("同步Cookie中");
  setTimeout(() => {
    toast.success("同步Cookie成功");
  }, 1000);
};

//运维授权
const auth = (id: number) => {
  toast.warning("功能开发中:" + id);
};

// 复制
const copyEnvId = async (id: number) => {
  try {
    // 查找对应的 id
    const item = originData.value.find((item: any) => item.id === id);
    if (!item) {
      console.error("未找到对应的数据");
      return;
    }
    const id = item.id;
    // 使用 Clipboard API 复制到剪贴板
    await navigator.clipboard.writeText(id);
    // 提示用户复制成功
    toast.success("id 已复制到剪贴板");
  } catch (error) {
    toast.error("复制失败，请重试"); // 提示用户复制失败
  }
};

const simpleEditEnv = (id: number) => {
  router.push({
    path: "/environment/create/environment-advanced-create",
    query: {
      env: JSON.stringify(originData.value.find((item: any) => item.id === id)),
    },
  });
};

const totalItems = ref(0);
const currentPage = ref(1);
const pageSize = ref(10);
const pageSizes = [10, 20, 50];
const totalPages = ref(1);

const privateproxyloadData = () => {
  environment_proxies_query(currentPage.value, pageSize.value).then((res) => {
    let { data: data_, total } = res.data;
    totalItems.value = total;
    proxyList.value = data_;
    totalPages.value = Math.ceil(total / pageSize.value);
  });
};

onMounted(() => {
  privateproxyloadData();
});

const selectAll = ref(false);

watch(
  () => PageObj.pageSize,
  (_) => {
    console.log("??:", PageObj);
    getList();
  }
);

//监听理由变化
watch(
  () => route.params.id,
  () => {
    getList();
  }
);

//赋值被选中的内容
const getM = () => {
  messageData.value = [];
  originData.value.forEach((item: any) => {
    if (selectedItems.value.includes(item.id)) {
      messageData.value.push(item);
    }
  });
};

//监听多选框的变化
watch(
  () => selectedItems.value,
  (_) => {
    getM();
  },
  { deep: true } // 深度监听
);

interface Proxy {
  id: number;
  proxyType: string;
  kind: string;
  host: string;
  port: string;
  ipQueryChannel: string;
  proxyServer: string;
  proxyAccount: string;
  proxyPassword: string;
  username: string;
  password: string;
  ipMonitoring: boolean;
  ipChangeAction: string;
}

const proxy = ref<Proxy>({
  id: 0,
  proxyType: "",
  kind: "",
  host: "",
  port: "",
  ipQueryChannel: "",
  proxyServer: "",
  proxyAccount: "",
  proxyPassword: "",
  username: "",
  password: "",
  ipMonitoring: false,
  ipChangeAction: "warning",
});

const handleDeleteProxy = async () => {
  const selectedProxyIds = selectedItems.value.map((id) => parseInt(id));

  environment_proxies_batch_delete(selectedProxyIds);

  batchDeleteProxy.value = false;
  tableData.value = tableData.value
    .filter((item: any) => !selectedProxyIds.includes(item.id))
    .map((item: any, index: number) => ({ ...item, ind: index + 1 }));
};

const modifyProxy = ref(false);

const proxyList = ref<Proxymanage[]>([]);

const batchDeleteProxy = ref(false);
const modifyProxyNew = () => {
  console.log("modifyProxyNew");
  modifyProxy.value = true;
};

const sortColumn = ref("id");

const filterData = computed(() => {
  // 根据搜索关键词过滤表格数据
  const filteredTableData = tableData.value.filter((item: any) =>
    item.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );

  // 对过滤后的数据进行排序
  const sortedData = [...filteredTableData].sort((a: any, b: any) => {
    if (sortColumn.value === "id") {
      // 按id排序
      return sortOrder.value === "asc" ? a.ind - b.ind : b.ind - a.ind;
    } else if (sortColumn.value === "username") {
      // 按名称排序
      return sortOrder.value === "asc"
        ? a.username.localeCompare(b.username)
        : b.username.localeCompare(a.username);
    }
    return 0;
  });

  // 映射排序和过滤后的数据，只包含可见列
  return sortedData.map((item: any) => {
    // 获取可见列的键名
    const visibleKeys = [
      "ind",
      ...visibleColumns.value.map((col: any) => col.key),
    ];
    // 创建新的对象，只包含id和id
    const filteredItem: Record<string, any> = {
      id: item.id,
    };
    // 遍历可见列，将对应的值添加到新对象中
    visibleKeys.forEach((key: any) => {
      if (key !== "id" && key in item) {
        filteredItem[key] = item[key];
      }
    });
    return filteredItem;
  });
});

const allColumns = ref<any>([]); //全部列

const theadChoose = (val: any) => {
  allColumns.value = val;
};

const showColumnsModal = ref(false);

const handleSort = (column: string) => {
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === "asc" ? "desc" : "asc";
  } else {
    sortColumn.value = column;
    sortOrder.value = "asc";
  }
};

const visibleActions = computed(() => {
  // console.log("??:", allActions.value);
  return allActions.value.filter((action: any) => action.visible);
});

const flag = ref(false); //修改后是否需要重新查询表格数据的标志

const visibleColumns = computed(() => {
  return allColumns.value.filter((column: any) => column.visible);
});

const toggleSelectAll = (event: Event) => {
  const target = event.target as HTMLInputElement;
  if (target.checked) {
    selectedItems.value = filterData.value?.map((item) => item.id);
  } else {
    selectedItems.value = [];
  }

  proxyList.value.forEach((proxyList) => {
    proxyList.selected = selectAll.value;
  });
};

const router = useRouter();

const showSearchChoose = ref(false);
const showQuickSettings = ref(false);

const synchronizerDialog = ref(false);
const exportEnvModel = ref(false);

const editEnvInfoModel = ref(false);
const transferEnvModel = ref(false);
const UntransferEnvModel = ref(false);
const cleanCacheModel = ref(false);
const delEnvModel = ref(false);
const addLabelModel = ref(false);
const resetLabelModel = ref(false);
const cleanLabelMode = ref(false);
const addGroupModel = ref(false);
const editStartPageModel = ref(false);
const editUAModel = ref(false);

const getPageSize = (val: number) => {
  PageObj.pageNum = 1;
  PageObj.pageSize = val;

  console.log("PageObj:", PageObj);
  getList();
};
const getPageNum = (val: number) => {
  PageObj.pageNum = val;
  getList();
};
const groupList = ref<any>([]);

const searchGroup = () => {
  environment_proxy_group_query(1, 1000).then((res: any) => {
    groupList.value = res.data.data.map((item: any) => {
      return {
        id: item.id,
        title: item.name,
        value: item.name,
        href: "/proxy-manage/" + item.id,
      };
    });
    groupList.value = [
      {
        id: 0,
        title: "自有代理",
        value: "default",
        href: "/proxy-manage/0",
      },
      ...groupList.value,
    ];
  });
};

// proxy: Proxymanage
const copyProxyId = () => {
  toast("复制成功");
};

let selectedId = ref<number | null>(null);
const singlehandleDeleteProxy = () => {
  if (!selectedId.value) return;
  environment_proxies_delete(selectedId.value);
  singleDeleteProxy.value = false;
  tableData.value = tableData.value
    .filter((item: any) => item.id !== selectedId.value)
    .map((item: any, index: number) => ({ ...item, ind: index + 1 }));
};

const editProxyModel = ref(false);
// const singleDeleteProxy = ref(false);

const singleDeleteProxy = ref(false);

const singleDeleteProxyModel = (id: number) => {
  singleDeleteProxy.value = true;
  proxy.value = originData.value.find((item: any) => item.id === id);
};

onMounted(() => {
  searchGroup();
});

const editProxyModify = (item: any) => {
  proxy.value = item;
  editProxy.value = true;
  id.value = item;

  console.log("id.value:", id.value);
};

// 数据相关的响应式变量
const data = ref<any[]>([]); // 当前显示的数据
const selected = ref<any[]>([]); // 选中的项目
// 分页配置
const pagination = reactive({
  total: 100,
  pageNum: 1,
  pageSize: 17,
});
// 筛选条件
const selectFilterRef = reactive<any>({
  proxy: false,
  account: "",
  tag: "",
  group: "",
});

watch(
  () => flag.value,
  (_) => {
    //需要做数据的重新查询
    getList();
  }
);

const proxyStatusStore = useProxyStatusStore();

//编辑代理
const editProxy = ref(false);

const allActions = ref<any>([]); //全部操作

// 拖拽状态
const isDragging = ref(false);
const startX = ref(0); // 拖拽起始位置
const scrollLeft = ref(0); // 容器初始滚动位置
const scrollContainer = ref<HTMLElement | null>(null); // 容器 DOM 引用
// 开始拖拽
const startDrag = (event: MouseEvent) => {
  isDragging.value = true;
  startX.value = event.pageX; // 记录鼠标按下时的位置
  scrollLeft.value = scrollContainer.value?.scrollLeft || 0; // 记录容器当前的滚动位置
  scrollContainer.value?.classList.add("cursor-grabbing"); // 添加抓取样式
  scrollContainer.value?.classList.remove("cursor-grab");
};

const filterOpt = () => {
  setTimeout(() => {
    showSearchChoose.value = !showSearchChoose.value;
  }, 10);
};

const singleNewProxy = () => {
  router.push("/single-new-proxy");
  console.log("singleNewProxy:", router.currentRoute.value);
};

const toGroupManager = () => {
  router.push("/environment-group-manager");
};

const headOperate = ref([
  {
    label: "批量检测代理",
    icon: GroupIcon,
    go: toGroupManager,
  },
  {
    label: "添加代理",
    icon: BookmarkIcon,
    go: singleNewProxy,
  },
  {
    label: "购买代理",
    icon: ApiIcon,
    go: filterOpt,
  },
]);
// 拖拽中
const onDrag = (event: MouseEvent) => {
  if (!isDragging.value || !scrollContainer.value) return;
  const x = event.pageX - startX.value; // 计算鼠标移动的距离
  scrollContainer.value.scrollLeft = scrollLeft.value - x; // 调整容器的滚动位置
};
// 停止拖拽
const stopDrag = () => {
  isDragging.value = false;
  scrollContainer.value?.classList.remove("cursor-grabbing"); // 移除抓取样式
  scrollContainer.value?.classList.add("cursor-grab");
};

const createGroupManage = ref(false);

// 重置筛选
const resetSelectFilterHandle = () => (data.value = dataCache.value);

onMounted(() => {
  getList();
});

watch(selectFilterRef, (_) => {
  console.log("---:", dataCache.value);

  let tData = dataCache.value
    .filter((_: any) => (selectFilterRef.proxy ? true : true))
    .filter((item: any) =>
      selectFilterRef.account
        ? selectFilterRef.account == item.accounts.platform
        : true
    )
    .filter((item: any) =>
      selectFilterRef.tag ? selectFilterRef.tag == item.tag_name : true
    )
    .filter((item: any) =>
      selectFilterRef.group ? selectFilterRef.group == item.group_name : true
    );

  dealTableData(tData);
});
watch(
  () => [delEnvModel.value, cleanCacheModel.value], // 监听多个值
  ([newDelEnvModel, newCleanCacheModel]) => {
    if (!newDelEnvModel && !newCleanCacheModel) {
      console.log("关闭弹窗:", newDelEnvModel, newCleanCacheModel);
      getM(); // 如果任意一个值变为 false，调用 getM
    }
  }
);
</script>

<style scoped>
/* 隐藏滚动条 */
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

/* 兼容 Firefox */
.scrollbar-hide {
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE 和 Edge */
}
</style>

<template>
  <div class="flex flex-col flex-1 px-4 w-full h-full rounded-xl select-none">
    <!-- 顶部工具栏 -->
    <header class="flex justify-between items-center mb-4">
      <div class="flex items-center space-x-4 w-auto">
        <h3 class="text-xl font-semibold w-[105px]">代理管理</h3>
      </div>
      <div class="flex items-center space-x-4">
        <div class="flex relative flex-col">
          <div class="flex gap-2 items-center">
            <div class="flex items-center pl-2 rounded-lg border">
              <SearchIcon class="size-5" />
              <input
                type="search"
                placeholder="搜索"
                v-model="searchQuery"
                class="rounded-md px-3 py-1.5 text-sm outline-none w-[260px] h-[35px]"
              />
            </div>
            <SelectFilter
              :data="dataCache"
              :selects="selectFilterRef"
              @update:on-proxy-select="(v) => (selectFilterRef.proxy = v)"
              @update:on-account-select="(v) => (selectFilterRef.account = v)"
              @update:on-tag-select="(v) => (selectFilterRef.tag = v)"
              @update:on-group-select="(v) => (selectFilterRef.group = v)"
              @update:reset="resetSelectFilterHandle"
            />
          </div>
        </div>

        <div class="flex relative gap-2">
          <button
            v-for="item in headOperate"
            @click="item.go"
            class="flex border text-[14px] border-gray-300 px-2 py-1 gap-2 rounded-lg h-[35px] items-center hover:bg-blue-50"
          >
            <component :is="item.icon" class="size-5" />
            {{ item.label }}
          </button>
          <SynchronizerIndex
            :open="synchronizerDialog"
            @close="synchronizerDialog = false"
            class="absolute top-10"
          />
        </div>
      </div>
    </header>

    <GroupChoose class="flex flex-col flex-1 select-none" :data="groupList">
      <div class="flex flex-row gap-x-2 m-2">
        <button
          @click="createGroupManage = true"
          class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <FileText
            class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            设置分组
          </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <PenModify
            class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            修改IP查询渠道
          </span>
        </button>

        <button
          @click="modifyProxyNew()"
          class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <Plate
            class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            修改代理
          </span>
        </button>

        <button
          class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          @click="batchDeleteProxy = true"
          :disabled="!selectedItems.length"
        >
          <Round
            class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
          />
          <span class="font-sans font-semibold text-center text-black">
            删除代理
          </span>
        </button>
      </div>

      <!-- Table -->
      <div
        class="flex overflow-auto flex-col flex-1 rounded-lg border border-gray-300"
      >
        <table class="overflow-auto min-w-full">
          <thead class="overflow-auto sticky top-0 z-10 w-full">
            <tr class="overflow-auto border-b bg-popover">
              <th class="py-3 pl-4 w-12 text-left">
                <input
                  type="checkbox"
                  class="rounded border-gray-300 data-[state=checked]:bg-[#5050FA] size-3.5"
                  :checked="isAllSelected"
                  @change="toggleSelectAll"
                  :indeterminate="isIndeterminate"
                />
              </th>
              <template v-for="column in visibleColumns" :key="column.key">
                <th
                  v-if="column.key === 'id'"
                  class="px-2 py-3 text-left text-sm font-medium text-gray-600 cursor-pointer min-w-[100px]"
                  @click="handleSort('id')"
                >
                  <div class="flex justify-between items-center border-r">
                    <span>代理序号</span>
                    <div class="mr-2">
                      <CaretDownIcon class="size-3" />
                      <CaretUpIcon class="size-3" />
                    </div>
                  </div>
                </th>
                <th
                  v-if="column.key === 'name'"
                  class="px-2 py-3 text-sm font-medium text-left text-gray-600 cursor-pointer min-w-[100px]"
                  @click="handleSort('name')"
                >
                  <div class="flex justify-between items-center border-r">
                    <span>代理名称</span>
                    <div class="mr-2">
                      <CaretDownIcon class="size-3" />
                      <CaretUpIcon class="size-3" />
                    </div>
                  </div>
                </th>

                <th
                  v-if="column.key !== 'id' && column.key !== 'name'"
                  class="px-2 py-3 text-left text-sm font-medium text-gray-600 min-w-[150px]"
                >
                  <div class="flex justify-between items-center border-r">
                    <span>{{ column.label }}</span>
                  </div>
                </th>
              </template>
              <!-- 固定在右侧的列 -->
              <th class="sticky right-0 px-4 py-3 text-right bg-background">
                <button @click="showColumnsModal = !showColumnsModal">
                  <WrapperIcon class="size-5" />
                </button>
                <!-- Columns Modal -->
                <TableTheadChoose
                  :open="showColumnsModal"
                  @close="showColumnsModal = false"
                  @select="theadChoose"
                  class="absolute right-0 top-0 w-[320px]"
                />
              </th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(item, index) in filterData"
              :key="index"
              class="border-b"
              :class="{ 'bg-[#EDF1F7]': selectedItems.includes(item.id) }"
            >
              <td class="py-3 pl-4">
                <input
                  type="checkbox"
                  class="rounded border-gray-300 data-[state=checked]:bg-[#5050FA] size-3.5"
                  :checked="selectedItems.includes(item.id)"
                  @click="toggleSelectItem(item.id)"
                />
              </td>

              <template
                v-for="(value, key) in item as Record<string, any>"
                :key="key"
              >
                <td class="px-2 py-3 text-sm" v-if="key != 'id'">
                  <div
                    v-if="key === 'action'"
                    class="flex justify-between items-center"
                  >
                    <div class="flex gap-2 justify-start items-center">
                      <button
                        @click="editProxyModify(item.id)"
                        class="text-[#4F46E5] hover:bg-indigo-50 px-2 rounded border border-[#5050FA] bg-[#F0F5FF]"
                      >
                        编辑
                      </button>

                      <!-- <button
                        @click="copyProxyId"
                        class="px-2 text-[#FA8C16] rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFF7E6]"
                      >
                        复制代理ID
                      </button> -->

                      <button
                        @click="
                          () => {
                            singleDeleteProxy = true;
                            selectedId = item.id;
                          }
                        "
                        class="px-2 text-red-600 rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFE2E2]"
                      >
                        删除
                      </button>
                    </div>
                  </div>
                  <div
                    v-else-if="key === 'state'"
                    class="flex gap-x-2 items-center"
                  >
                    <div
                      class="rounded-full size-1.5"
                      :class="{
                        'bg-blue-500': proxyStatusStore.getStatus(item.id),
                        'bg-yellow-500': !proxyStatusStore.getStatus(item.id),
                      }"
                    ></div>
                    {{
                      !proxyStatusStore.getStatus(item.id)
                        ? "检测中..."
                        : "准备好"
                    }}
                  </div>
                  <div v-else class="flex justify-between items-center text-sm">
                    <span>{{ value || "-asda-" }}</span>
                  </div>
                </td>
              </template>
              <!-- 固定在右侧的列 -->
              <td class="sticky right-0 px-4 py-3 text-right bg-background">
                <!-- 这里可以放置操作按钮或其他内容 -->
              </td>
            </tr>
          </tbody>
        </table>

        <AlertModel
          class=""
          title="删除该代理吗"
          :open="singleDeleteProxy"
          @close="() => (singleDeleteProxy = false)"
          @submit="() => singlehandleDeleteProxy()"
          @cancel="() => (singleDeleteProxy = false)"
        >
          <div class="relative bg-white rounded-xl">
            <div class="flex gap-3">
              <div class="flex-1">
                <h3 class="mb-2 text-base font-medium text-gray-900">
                  删除该代理吗
                </h3>
                <p class="text-sm text-gray-500">
                  确定要删除所选代理吗？删除后无法找回，请及时修改已配置的环境。
                </p>
              </div>
            </div>
          </div>
        </AlertModel>

        <AlertModel
          class=""
          title="删除该代理吗"
          :open="batchDeleteProxy"
          @close="() => (batchDeleteProxy = false)"
          @submit="handleDeleteProxy"
          @cancel="() => (batchDeleteProxy = false)"
        >
          <div class="relative bg-white rounded-xl">
            <div class="flex gap-3">
              <div class="flex-1">
                <h3 class="mb-2 text-base font-medium text-gray-900">
                  删除该代理吗
                </h3>
                <p class="text-sm text-gray-500">
                  您已选择
                  {{ selectedItems.length }}
                  个代理，确定要删除所选代理吗？删除后无法找回，请及时修改已配置的环境。
                </p>
              </div>
            </div>
          </div>
        </AlertModel>
        <div
          v-show="filterData.length == 0"
          class="flex flex-col flex-1 justify-center items-center space-y-3"
        >
          <AddCardIcon
            class="size-[200px] text-blue-400 border-gray-300 flex items-center justify-center"
          />
          <p>您可创建具有独立高质量指纹的profile，也可自定义编辑指纹信息</p>
          <button
            @click="router.push('/single-new-proxy')"
            class="p-2 bg-[#5050FA] text-white rounded-lg flex items-center gap-3"
          >
            <AssCircleIcon class="size-5" />
            新建代理
          </button>
        </div>
      </div>

      <!-- page -->
      <Page
        :total="PageObj.total"
        :modelValue="PageObj.pageNum"
        @update:model-value="getPageNum"
        @update:page-size="getPageSize"
      />
    </GroupChoose>

    <!-- <div class="flex overflow-auto flex-col grow">
      <slot name="manage-proxy-content"></slot>
    </div> -->
  </div>

  <NewGroup v-model:createGroupManage="createGroupManage" />
  <EditProxy v-model:editProxy="editProxy" :proxy="proxy" :id="id" />
  <!-- <SingleDeleteProxy
    v-model:singleDeleteProxy="singleDeleteProxy"
    :selectedItems="selectedItems"
  /> -->
  <ModifyProxy v-model:modifyProxy="modifyProxy" :proxy="proxy" />

  <ExportEnv
    :open="exportEnvModel"
    @close="exportEnvModel = false"
    :data="messageData"
  />

  <DelEnv
    :open="delEnvModel"
    @close="delEnvModel = false"
    :data="messageData"
    v-model:isChange="flag"
  />
</template>
