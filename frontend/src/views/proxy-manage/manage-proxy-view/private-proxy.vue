<script setup lang="ts">
import Layout from "@/views/proxy-manage/manage-proxyLayout.vue";

import {
  ChevronRightIcon,
  ChevronLeftIcon,
} from "@/assets/icons/environment-bookmark-image";
import { RefreshCw, GripVertical } from "lucide-vue-next";
import { PrimaryButton, CancelButton } from "@/components/button";
import EditProxy from "./edit-proxy.vue";
import ModifyProxy from "./modify-proxy.vue";
import {
  Round,
  FileText,
  PenModify,
  Setting,
  Plate,
} from "@/assets/icons/proxy-manage-image";

import {
  environment_proxies_delete,
  environment_proxies_query,
} from "@/commands/environment-proxy";
import { environment_proxies_batch_delete } from "@/commands/environment-proxy";
import { toast } from "vue-sonner";

import { AlertModel } from "@/components/alert-model";
import { ip_info } from "@/commands";
import TableTheadChoose from "../com/table-thead-choose.vue";
import { WrapperIcon } from "@/assets/icons/environment/index";

import { ref, reactive, computed, watch, onMounted, onUnmounted } from "vue";
import Operate from "./com/operate.vue";

import SelectFilter from "./com/select-filter.vue";
import Page from "./com/page.vue";
import GroupChoose from "./com/group-choose.vue";
import ExportEnv from "./pop-box/export/exportEnv.vue";

import EditEnvInfo from "./pop-box/edit-env-info.vue";
import TransferEnv from "./pop-box/transfer-env.vue";
import UntransferEnv from "./pop-box/untransfer-env.vue";
import CleanCache from "./pop-box/clean-cache.vue";
import DelEnv from "./pop-box/del-env.vue";
import AddLabel from "./pop-box/label/addLabel.vue";
import ResetLabel from "./pop-box/label/resetLabel.vue";
import CleanLabel from "./pop-box/label/cleanLabel.vue";
import AddGroup from "./pop-box/group/addGroup.vue";
import EditStartPage from "./pop-box/edit-start-page.vue";
import EditUa from "./pop-box/edit-ua.vue";
// import Auth from './synchronizer/auth.vue'
import SynchronizerIndex from "./synchronizer/index.vue";
import {
  AssCircleIcon,
  AddCardIcon,
  SearchIcon,
  GroupIcon,
  BookmarkIcon,
  ApiIcon,
  SynchronizerIcon,
  GroupAddIcon,
  MoreOperatorIcon,
  AltArrowDownIcon,
  CaretDownIcon,
  CaretUpIcon,
} from "@/assets/icons/environment/index";
import {
  HelfGlobalIcon,
  RoundArrowRight,
} from "@/assets/icons/environment/index";
import {
  environment_query,
  environment_query_by_group,
  environment_batch_move_to_group,
} from "@/commands/environment";
import { useRouter, useRoute } from "vue-router";
import { OneFrameIcon } from "@/assets/icons/environment/index";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import {
  browser_starts,
  browser_stops,
  browser_start,
} from "@/commands/browser";

import { useBrowserStatusStore } from "@/stores/browser";
import { environment_group_query } from "@/commands/environment-group";
import {
  kernel_location,
  latest_kernel,
  install_location,
} from "@/commands/kernel";
import { table } from "console";
import NewGroup from "./proxy-new-group.vue";

const browserStatusStore = useBrowserStatusStore();
const router = useRouter();
const route = useRoute();
const showSearchChoose = ref(false);
const showQuickSettings = ref(false);

const synchronizerDialog = ref(false);
const exportEnvModel = ref(false);
const editProxyModel = ref(false);
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

const flag = ref(false); //修改后是否需要重新查询表格数据的标志

watch(
  () => flag.value,
  (_) => {
    //需要做数据的重新查询
    getList();
  }
);
watch(
  () => [delEnvModel.value, cleanCacheModel.value], // 监听多个值
  ([newDelEnvModel, newCleanCacheModel]) => {
    if (!newDelEnvModel && !newCleanCacheModel) {
      console.log("关闭弹窗:", newDelEnvModel, newCleanCacheModel);
      getM(); // 如果任意一个值变为 false，调用 getM
    }
  }
);
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
// 清理事件监听器
onMounted(() => {
  if (scrollContainer.value) {
    scrollContainer.value.addEventListener("mousedown", startDrag);
    scrollContainer.value.addEventListener("mousemove", onDrag);
    scrollContainer.value.addEventListener("mouseup", stopDrag);
    scrollContainer.value.addEventListener("mouseleave", stopDrag);
  }
});
onUnmounted(() => {
  if (scrollContainer.value) {
    scrollContainer.value.removeEventListener("mousedown", startDrag);
    scrollContainer.value.removeEventListener("mousemove", onDrag);
    scrollContainer.value.removeEventListener("mouseup", stopDrag);
    scrollContainer.value.removeEventListener("mouseleave", stopDrag);
  }
});
//导出
const exportEnv = () => {
  exportEnvModel.value = true;
};
const allActions = ref<any>([]); //全部操作

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
    // 查找对应的 uuid
    const item = originData.value.find((item: any) => item.id === id);
    if (!item) {
      console.error("未找到对应的数据");
      return;
    }
    const uuid = item.uuid;
    // 使用 Clipboard API 复制到剪贴板
    await navigator.clipboard.writeText(uuid);
    // 提示用户复制成功
    toast.success("UUID 已复制到剪贴板");
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
const operateSelect = (val: any) => {
  allActions.value = val;
};

const visibleActions = computed(() => {
  // console.log("??:", allActions.value);
  return allActions.value.filter((action: any) => action.visible);
});

const kerneleocationExist = async () => {
  let kernelLocationE = localStorage.getItem("kernel_location") ? true : false;
  if (kernelLocationE) return true;

  let versions = await latest_kernel();
  let version = versions.data.version;
  let url;
  try {
    url = versions.data.platforms["windows-x86_64"].url;
  } catch {}

  let locationResp = await kernel_location(version);
  let { location, exist } = locationResp.data;
  if (exist) {
    localStorage.setItem("kernel_location", location);
    return true;
  }

  await install_location(url, location, ({ progress, total }) => {
    console.log(`Downloaded ${progress} of ${total} bytes`);
  });

  return kernelLocationE;
};

//启动单个环境
const startEnv = async (uuid: string) => {
  await kerneleocationExist();

  browser_start(uuid)
    .then((res: any) => {
      if (res.code == 1) {
        browserStatusStore.updateStatus(uuid, true);
        toast.success(res.message);
      } else {
        toast.error("启动失败，可能环境已被转移");
      }
    })
    .catch((res) => {
      toast.error("启动失败：", res);
    });
};
// 单个关闭
const stopEnv = (uuid: string) => {
  browser_stops([uuid]).then((res: any) => {
    toast.success(res.message);
    browserStatusStore.updateStatus(uuid, false);
  });
};
// 批量启动
const startAll = async () => {
  await kerneleocationExist();

  browser_starts(selectedItems.value).then((res: any) => {
    if (res.code == 1) {
      toast.success(res.message);
      selectedItems.value.forEach((item: string) => {
        browserStatusStore.updateStatus(item, true);
      });
    } else {
      toast.error("启动失败，可能环境已被转移");
    }
  });
};
// 批量关闭
const stopAll = () => {
  browser_stops(selectedItems.value).then((res: any) => {
    toast.success(res.message);
    selectedItems.value.forEach((item: string) => {
      browserStatusStore.updateStatus(item, false);
    });
  });
};

//编辑代理
// const editProxy = () => {
//   editProxyModel.value = true;
// };
//编辑环境信息
const editEnvInfo = () => {
  editEnvInfoModel.value = true;
};
//转移环境
const transferEnv = () => {
  transferEnvModel.value = true;
};
//取消转移
const untransferEnv = () => {
  UntransferEnvModel.value = true;
};
//清除缓存
const cleanCache = () => {
  cleanCacheModel.value = true;
};
//删除环境
const delEnv = () => {
  delEnvModel.value = true;
};
//添加标签
const addLabel = () => {
  addLabelModel.value = true;
};
//重设标签
const resetLabel = () => {
  resetLabelModel.value = true;
};
//清空标签
const cleanLabel = () => {
  cleanLabelMode.value = true;
};
//添加分组
const addGroup = () => {
  addGroupModel.value = true;
};
//编辑启动页
const editStartPage = () => {
  editStartPageModel.value = true;
};
//编辑UA
const editUA = () => {
  editUAModel.value = true;
};
//修改分组
const setGroups = (item: any) => {
  console.log("修改分组：", selectedItems.value, item);
  environment_batch_move_to_group(selectedItems.value, item.key).then(
    (res: any) => {
      toast.success(res.message);
      flag.value = !flag.value;
    }
  );
};
// 添加分组后重新做分组查询
const groupList = ref<any>([]);

// const searchGroup = () => {
//   environment_group_query(1, 1000).then((res: any) => {
//     groupList.value = res.data.data.map((item: any) => {
//       return {
//         id: item.id,
//         title: item.name,
//         value: item.name,
//         href: "/environment/" + item.id,
//       };
//     });
//     groupList.value = [
//       {
//         id: 0,
//         title: "默认分组",
//         value: "default",
//         href: "/environment/0",
//       },
//       ...groupList.value,
//     ];
//   });
// };

const getList = () => {
  if (route.params.id === "0") {
    environment_proxies_query(1, 10).then((res: any) => {
      PageObj.total = res.data.total;
      console.log("res.data.data", res.data.data);
      console.log("getList", getList);
      tableData.value = res.data.data.map((item: any) => {
        return { ...item, created_at: formatDate(item.created_at) };
      });
      dealTableData(tableData.value); //字段处理
      dataCache.value = JSON.parse(JSON.stringify(res.data.data));
    });
  } else {
    // environment_query_by_group(
    //   +route.params.id,
    //   PageObj.pageNum,
    //   PageObj.pageSize
    // ).then((res: any) => {
    //   PageObj.total = res.data.total;
    //   tableData.value = res.data.data.map((item: any) => {
    //     return { ...item, created_at: formatDate(item.created_at) };
    //   });
    //   dealTableData(tableData.value);
    //   dataCache.value = JSON.parse(JSON.stringify(res.data.data));
    // });
  }

  selectedItems.value.length = 0;
};

// onMounted(() => {
//   searchGroup();
// });

const filterOpt = () => {
  setTimeout(() => {
    showSearchChoose.value = !showSearchChoose.value;
  }, 10);
};
// 打开同步器
const openSynchronizer = () => {
  synchronizerDialog.value = !synchronizerDialog.value;
};
const toBookMark = () => {
  router.push("/environment-manager-bookmark");
};
const toGroupManager = () => {
  router.push("/environment-group-manager");
};
const headOperate = ref([
  {
    label: "分组管理",
    icon: GroupIcon,
    go: toGroupManager, //前往分组管理
  },
  {
    label: "书签",
    icon: BookmarkIcon,
    go: toBookMark,
  },
  {
    label: "api",
    icon: ApiIcon,
    go: filterOpt,
  },
  {
    label: "同步器",
    icon: SynchronizerIcon,
    go: openSynchronizer,
  },
]);

//分页
const PageObj = reactive({
  total: 0,
  pageNum: 1,
  pageSize: 17,
});
watch(
  () => PageObj.pageSize,
  (_) => {
    console.log("??:", PageObj);
    getList();
  }
);
const getPageNum = (val: number) => {
  PageObj.pageNum = val;
  getList();
};
const getPageSize = (val: number) => {
  PageObj.pageNum = 1;
  PageObj.pageSize = val;
  getList();
};

// 多选
const selectedItems = ref<string[]>([]);
const isAllSelected = computed(() => {
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

defineExpose({
  selectedItems,
});

//赋值被选中的内容
const getM = () => {
  messageData.value = [];
  originData.value.forEach((item: any) => {
    if (selectedItems.value.includes(item.uuid)) {
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

//监听理由变化
watch(
  () => route.params.id,
  () => {
    getList();
  }
);
//筛选部分
const selectFilterRef = reactive<any>({
  proxy: false,
  account: "",
  tag: "",
  group: "",
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
const resetSelectFilterHandle = () => {
  selectFilterRef.proxy = false;
  selectFilterRef.account = "";
  selectFilterRef.tag = "";
  selectFilterRef.group = "";
};

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

const originData = ref<any>([]);
const messageData = ref<any>([]); //用于传入操作组件的数据
const dataCache = ref<any>([]); //用于筛选

/**
 * 处理表格数据
 * @param data 待处理数据
 */
const dealTableData = (data: any) => {
  let ind = 1; // 序号
  originData.value = JSON.parse(JSON.stringify(tableData.value));

  // 将字段与表头对上
  tableData.value = data.map((item: any) => {
    return {
      ind: ind++, // 序号递增
      id: item.id,
      name: item.username,

      // action: "启动",
      // status: "未启动",
      account:
        item.accounts?.platform + item.accounts?.platform_account || "--", // 使用可选链避免报错
      proxy: item.proxy_host ? `${item.proxy_host}:${item.proxy_port}` : "--",
      description: item.description || "--",
      tab: item.tag_name || "--",
      groupName: item.group_name || "--",
      create_at: item.created_at,
      lastOpen: "最后启动",
      uuid: item.uuid,
    };
  });
};

onMounted(() => {
  getList();
});
const handleSort = (column: string) => {
  if (sortColumn.value === column) {
    sortOrder.value = sortOrder.value === "asc" ? "desc" : "asc";
  } else {
    sortColumn.value = column;
    sortOrder.value = "asc";
  }
};

// proxy: Proxymanage
const copyProxyId = () => {
  toast("复制成功");
};

const showColumnsModal = ref(false);

const singlehandleDeleteProxy = (id: number) => {
  // 遍历删除每个选中的代理

  environment_proxies_delete(id);

  singleDeleteProxy.value = false;

  privateproxyloadData();
};

const selectAll = ref(false);

const proxyList = ref<Proxymanage[]>([]);

const toggleSelectAll = () => {
  proxyList.value.forEach((proxyList) => {
    proxyList.selected = selectAll.value;
  });
};

const sortColumn = ref("id");
const searchQuery = ref("");

const sortOrder = ref("asc");

const tableData = ref<any>([]);

const filterData = computed(() => {
  // Filter the tableData based on the search query
  const filteredTableData = tableData.value.filter((item: any) =>
    item.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );

  console.log("tableData", tableData.value);

  console.log("filteredTableData", filteredTableData);

  // Sort the filtered data
  const sortedData = [...filteredTableData].sort((a: any, b: any) => {
    if (sortColumn.value === "id") {
      return sortOrder.value === "asc" ? a.ind - b.ind : b.ind - a.ind;
    } else if (sortColumn.value === "name") {
      return sortOrder.value === "asc"
        ? a.name.localeCompare(b.name)
        : b.name.localeCompare(a.name);
    }
    return 0;
  });

  // Map the sorted and filtered data to include only visible columns
  return sortedData.map((item: any) => {
    const visibleKeys = [
      "ind",
      ...visibleColumns.value.map((col: any) => col.key),
    ];
    const filteredItem: Record<string, any> = { uuid: item.uuid, id: item.id };
    visibleKeys.forEach((key: any) => {
      if (key !== "id" && key in item) {
        filteredItem[key] = item[key];
      }
    });
    return filteredItem;
  });
});

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

const totalItems = ref(0);
const currentPage = ref(1);
const pageSize = ref(10);
const pageSizes = [10, 20, 50];
const totalPages = ref(1);

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
    privateproxyloadData();
  }
};

const proxy = ref();

const privateproxyloadData = () => {
  environment_proxies_query(currentPage.value, pageSize.value).then((res) => {
    let { data: data_, total } = res.data;
    totalItems.value = total;
    proxyList.value = data_;
    totalPages.value = Math.ceil(total / pageSize.value);
  });
};

// 监听页码和每页条数变化
watch([currentPage, pageSize], () => {
  privateproxyloadData();
});

const batchDeleteProxy = ref(false);

onMounted(() => privateproxyloadData());

const checkProxyStatus = (proxy: Proxymanage) => {
  ip_info(
    proxy.kind,
    proxy.host,
    proxy.port,
    proxy.name, // username
    proxy.attribution // password
  ).then((res) => {
    if (res.code == 1) {
      toast.success("代理检测成功");
    } else {
      toast.warning(res.message);
    }
  });
};

const handleDeleteProxy = async () => {
  // Get selected proxy IDs
  const selectedProxyIds = proxyList.value
    .filter((proxy) => proxy.selected)
    .map((proxy) => proxy.id);

  if (selectedProxyIds.length === 0) {
    toast.error("请选择要删除的代理");
    return;
  }

  try {
    await environment_proxies_batch_delete(selectedProxyIds);
    toast.success("删除成功");
    selectAll.value = false;
    await privateproxyloadData();
  } catch (error) {
    console.error("Failed to delete proxies:", error);
    toast.error("删除失败");
  }
  batchDeleteProxy.value = false;
};

interface Field {
  id: string;
  label: string;
  enabled: boolean;
}

const singleDeleteProxy = ref(false);

const editProxy = ref(false);

const editProxyModify = (Proxymanage: Proxymanage) => {
  editProxy.value = true;
  proxy.value = Proxymanage;
};

const modifyProxy = ref(false);
const modifyProxyNew = () => {
  modifyProxy.value = true;
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
    privateproxyloadData();
  }
};

const toggleSelectItem = (envUuid: string) => {
  const index = selectedItems.value.indexOf(envUuid);
  if (index === -1) {
    selectedItems.value.push(envUuid);
  } else {
    selectedItems.value.splice(index, 1);
  }
};

const createGroupManage = ref(false);

const theadChoose = (val: any) => {
  allColumns.value = val;
};

const visibleColumns = computed(() => {
  return allColumns.value.filter((column: any) => column.visible);
});

const allColumns = ref<any>([]); //全部列
</script>

<template>
  <Layout>
    <template v-slot:manage-proxy-content>
      <div class="flex flex-col w-full h-full">
        <!-- 设置分组那一栏 -->
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
            :disabled="!proxyList.some((p) => p.selected)"
          >
            <Round
              class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
            />
            <span class="font-sans font-semibold text-center text-black">
              删除代理
            </span>
          </button>

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
                    {{ proxyList.filter((p) => p.selected).length }}
                    个代理，确定要删除所选代理吗？删除后无法找回，请及时修改已配置的环境。
                  </p>
                </div>
              </div>
            </div>
          </AlertModel>
        </div>

        <!-- 选择多少个 -->

        <div>
          <h1
            class="ml-2 font-thin text-[14px] mb-1 leading-[18px] text-gray-300"
          >
            已选择{{ proxyList.filter((p) => p.selected).length }}项
          </h1>
        </div>

        <div
          class="flex overflow-x-auto flex-col mx-2 h-full rounded-lg border border-red-500 grow"
        >
          <table class="overflow-auto min-w-full">
            <!-- Table Header -->
            <thead class="sticky top-0 z-10 bg-gray-50 border border-gray-200">
              <tr class="overflow-auto border-b bg-popover">
                <th class="py-3.5 pr-3 pl-4 w-12 text-left">
                  <input
                    type="checkbox"
                    class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                    v-model="selectAll"
                    @change="toggleSelectAll"
                  />
                </th>

                <template v-for="column in visibleColumns" :key="column.key">
                  <th
                    v-if="column.key === 'id'"
                    class="py-3.5 pr-3 pl-4 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    代理序号
                  </th>

                  <th
                    v-if="column.key === 'name'"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    代理名称
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
                <!-- <th
                  scope="col"
                  class="flex justify-end items-center px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[60px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>

                  <button @click="showColumnsModal = !showColumnsModal">
                    <WrapperIcon class="size-5" />
                  </button>

                  <TableTheadChoose
                    :open="showColumnsModal"
                    @close="showColumnsModal = false"
                    @select="theadChoose"
                    class="absolute right-0 top-0 w-[320px]"
                  />
                </th> -->

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

            <!-- Table Body -->

            <tbody>
              <tr
                v-for="(item, index) in filterData"
                :key="index"
                class="border border-green-500"
                :class="{ 'bg-[#EDF1F7]': selectedItems.includes(item.uuid) }"
              >
                {{
                  filterData
                }}
                <td class="py-3 pl-4">
                  <input
                    type="checkbox"
                    class="rounded border-gray-300 data-[state=checked]:bg-[#5050FA] size-3.5"
                    :checked="selectedItems.includes(item.uuid)"
                    @click="toggleSelectItem(item.uuid)"
                  />
                </td>
                <template
                  v-for="(value, key) in item as Record<string, any>"
                  :key="key"
                >
                  {{ item }}
                  <td
                    class="px-2 py-3 text-sm"
                    v-if="key != 'uuid' && key != 'id'"
                  >
                    <div
                      v-if="key === 'action'"
                      class="flex justify-between items-center"
                    >
                      <span
                        @click="
                          !browserStatusStore.getStatus(item.uuid)
                            ? startEnv(item.uuid)
                            : stopEnv(item.uuid)
                        "
                        :class="{
                          'border-[#5050fa] bg-[#EDEDFF] text-blue-500 hover:bg-blue-100':
                            !browserStatusStore.getStatus(item.uuid),
                          'border-red-300 bg-red-50 text-red-500 hover:bg-red-100':
                            browserStatusStore.getStatus(item.uuid),
                        }"
                        class="border w-[70px] flex justify-center py-1 rounded-lg border-[#5050fa] bg-[#EDEDFF] cursor-pointer"
                      >
                        {{
                          !browserStatusStore.getStatus(item.uuid)
                            ? "启动"
                            : "关闭"
                        }}</span
                      >
                      <span class="">
                        <More class="relative top-8">
                          <MoreTrigger>
                            <OneFrameIcon class="w-5 h-5 cursor-pointer" />
                          </MoreTrigger>
                          <MoreContent class="min-w-[200px]">
                            <MoreItem
                              v-for="e in regularItems"
                              :key="e.id"
                              class="cursor-pointer"
                              @click="e.active?.(item.id)"
                            >
                              {{ e.label }}
                            </MoreItem>
                          </MoreContent>
                        </More>
                      </span>
                    </div>
                    <div
                      v-else-if="key === 'status'"
                      class="flex gap-x-2 items-center"
                    >
                      <div
                        class="rounded-full size-1.5"
                        :class="{
                          'bg-green-500': browserStatusStore.getStatus(
                            item.uuid
                          ),
                          'bg-red-500': !browserStatusStore.getStatus(
                            item.uuid
                          ),
                        }"
                      ></div>
                      {{
                        !browserStatusStore.getStatus(item.uuid)
                          ? "未启动"
                          : "已启动"
                      }}
                    </div>

                    <div
                      v-else
                      class="flex justify-between items-center text-sm"
                    >
                      <span>{{ value || "--" }}</span>
                    </div>
                  </td>
                </template>
                <!-- 固定在右侧的列 -->
                <td class="sticky right-0 px-4 py-3 text-right bg-background">
                  <!-- 这里可以放置操作按钮或其他内容 -->
                </td>
              </tr>
            </tbody>

            <tbody class="overflow-hidden bg-white divide-y divide-gray-200">
              <tr
                v-for="Proxymanage in proxyList"
                :key="Proxymanage.id"
                :class="{
                  'hover:bg-gray-50': !Proxymanage.selected, // 鼠标悬停时背景色
                  'bg-blue-100': Proxymanage.selected, // 选中时背景色
                }"
              >
                <td class="py-4 pr-3 pl-4 whitespace-nowrap">
                  <input
                    type="checkbox"
                    class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                    v-model="Proxymanage.selected"
                  />
                </td>
                <td
                  class="py-4 pr-3 pl-4 text-sm text-gray-900 whitespace-nowrap"
                >
                  {{ Proxymanage.id }}
                </td>
                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.name || "未设置" }}
                </td>

                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.attribution }}
                </td>

                <td class="px-3 py-4 text-sm whitespace-nowrap">
                  <div class="flex gap-2 justify-start items-center">
                    <button
                      @click="editProxyModify(Proxymanage)"
                      class="text-[#4F46E5] hover:bg-indigo-50 px-2 rounded border border-[#5050FA] bg-[#F0F5FF]"
                    >
                      编辑
                    </button>

                    <button
                      @click="copyProxyId"
                      class="px-2 text-[#FA8C16] rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFF7E6]"
                    >
                      复制代理ID
                    </button>

                    <button
                      @click="singleDeleteProxy = true"
                      class="px-2 text-red-600 rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFE2E2]"
                    >
                      删除
                    </button>
                    <AlertModel
                      class=""
                      title="删除该代理吗"
                      :open="singleDeleteProxy"
                      @close="() => (singleDeleteProxy = false)"
                      @submit="singlehandleDeleteProxy(Proxymanage.id)"
                      @cancel="() => (singleDeleteProxy = false)"
                    >
                      <div class="relative bg-white rounded-xl">
                        <div class="flex gap-3">
                          <div class="flex-1">
                            <h3
                              class="mb-2 text-base font-medium text-gray-900"
                            >
                              删除该代理吗
                            </h3>
                            <p class="text-sm text-gray-500">
                              确定要删除所选代理吗？删除后无法找回，请及时修改已配置的环境。
                            </p>
                          </div>
                        </div>
                      </div>
                    </AlertModel>
                  </div>
                </td>

                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  <div class="flex gap-2 items-center">
                    <span>{{
                      Proxymanage.state === "active" ? "异常" : "正常"
                    }}</span>
                    <button
                      @click="checkProxyStatus(Proxymanage)"
                      class="px-2 py-1 text-xs text-blue-600 rounded border border-blue-200 hover:bg-blue-50"
                    >
                      检查状态
                    </button>
                  </div>
                </td>
                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.channel }}
                </td>
                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.group }}
                </td>
                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.browser_environment }}
                </td>
                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.phone_environment }}
                </td>
                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.create_message }}
                </td>
                <td
                  class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap"
                ></td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- pagination -->
        <div
          class="flex justify-end items-center px-2 pt-3 mt-2 mb-2 border-t border-gray-200"
        >
          <span class="text-sm text-gray-500">共 {{ totalItems }} 项</span>
          <div class="flex gap-2 items-center">
            <button
              class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
              :disabled="currentPage === 1"
              @click="prevPage"
            >
              <ChevronLeftIcon class="w-[12px] h-[12px]" />
            </button>
            <span
              class="min-w-[2rem] text-center border border-gray-200 rounded-sm"
              >{{ currentPage }}</span
            >
            <button
              class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
              :disabled="currentPage === totalPages"
              @click="nextPage"
            >
              <ChevronRightIcon class="w-[12px] h-[12px]" />
            </button>
          </div>
          <select v-model="pageSize" class="px-2 py-1 text-sm rounded border">
            <option v-for="size in pageSizes" :key="size" :value="size">
              {{ size }}条/页
            </option>
          </select>
        </div>
      </div>

      <EditProxy v-model:editProxy="editProxy" :proxy="proxy" />
      <!-- <SingleDeleteProxy v-model:singleDeleteProxy="singleDeleteProxy" /> -->
      <ModifyProxy v-model:modifyProxy="modifyProxy" :proxy="proxy" />
    </template>
  </Layout>
  <NewGroup v-model:createGroupManage="createGroupManage" />
</template>
