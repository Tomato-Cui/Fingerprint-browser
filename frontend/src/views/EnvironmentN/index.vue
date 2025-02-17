<template>
  <div class="flex flex-col flex-1 px-4 w-full h-full rounded-xl select-none">
    <!-- Header Section -->
    <header class="flex justify-between items-center mb-4">
      <div class="flex items-center space-x-4 w-auto">
        <h3 class="text-xl font-semibold w-[105px]">环境管理</h3>
        <!-- <div class="w-full">
                    <div class="flex items-center justify-between px-6 h-[35px] space-x-14">
                        <div class="px-1 space-x-6 h-11 bg-gray-100 rounded-md md:flex">
                            <button class="px-3 my-1 whitespace-nowrap rounded-md text-primary"
                                :class="'bg-[#FFFFFFFF]' + ' ' + 'shadow-lg'">浏览器环境</button>
                            <button class="px-3 my-1 whitespace-nowrap rounded-md cursor-not-allowed">云手机环境</button>
                        </div>
                    </div>
                </div> -->
      </div>
      <div class="flex items-center space-x-4">
        <div class="flex relative flex-col 0">
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

        <div class="flex relative gap-">
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

    <!-- Tabs -->
    <GroupChoose class="flex flex-col flex-1 select-none" :data="groupList">
      <!-- Action Buttons -->
      <div class="mb-4 flex items-center space-x-2 h-[60px]">
        <!-- <OpenBrowserIcon class="w-[106px] h-[43px]" :class="{'cursor-not-allowed': selectedItems.length==0, 'hover:bg-gray-50 cursor-pointer': selectedItems.length!=0}" @click="selectedItems.length!=0 ? startAll() : void(0)" /> -->
        <div
          class="w-[100px] h-[40px] bg-[#7744ff] flex rounded-lg items-center relative"
          :class="{
            'cursor-not-allowed opacity-50': selectedItems.length == 0,
            'cursor-pointer': selectedItems.length != 0,
          }"
          @click="selectedItems.length != 0 ? startAll() : void 0"
        >
          <RoundArrowRight class="mx-3 size-6" />
          <span class="absolute left-12 text-white">启动</span>
          <div class="flex flex-1 justify-end">
            <HelfGlobalIcon class="size-[51px] opacity-10" />
          </div>
        </div>

        <div
          class="flex overflow-x-auto flex-1 space-x-2 select-none scrollbar-hide cursor-grab"
          @mousedown.stop="startDrag"
          @mousemove="onDrag"
          @mouseup="stopDrag"
          ref="scrollContainer"
        >
          <More
            class="flex min-w-fit"
            v-for="action in visibleActions"
            :key="action.key"
          >
            <MoreTrigger class="min-w-fit">
              <button
                v-show="action.key !== 'start'"
                :disabled="selectedItems.length === 0"
                @click="selectedItems.length !== 0 ? action.action() : void 0"
                :class="{
                  'cursor-not-allowed opacity-50': selectedItems.length == 0,
                  'hover:bg-[#5050FA] hover:text-white':
                    selectedItems.length != 0,
                }"
                class="flex items-center rounded-md bg-[#EDEDFF] px-3 py-[2px] text-sm shadow-sm min-w-fit h-[35px] gap-2"
              >
                <component :is="action.icon" class="mr-1.5 w-4 h-4" />
                <span>{{ action.label }}</span>
                <AltArrowDownIcon v-show="action.children" class="size-5" />
              </button>
            </MoreTrigger>
            <MoreContent
              class="w-[140px]"
              v-if="action.children && selectedItems.length > 0"
            >
              <MoreItem
                v-for="item in action.children"
                @click="
                  action.key === 'group' && item.key !== 'addGroup'
                    ? setGroups(item)
                    : item.active()
                "
                :key="item"
                class="cursor-pointer"
                :class="{ 'hover:bg-white': item.key === 'addGroup' }"
              >
                <div
                  v-if="item.key === 'addGroup'"
                  class="border border-[#5050FA] bg-[#EDEDFF] w-full p-1 rounded-sm flex items-center justify-center"
                >
                  <GroupAddIcon class="size-5" />{{ item.label }}
                </div>
                <div v-else-if="action.key === 'group'">
                  <!-- <Checkbox /> -->
                  {{ item.label }}
                </div>
                <div v-else>{{ item.label }}</div>
              </MoreItem>
            </MoreContent>
          </More>
        </div>

        <div class="relative">
          <button @click="showQuickSettings = !showQuickSettings" class="">
            <MoreOperatorIcon class="size-10" />
          </button>
          <!-- Quick Settings Modal -->
          <Operate
            :open="showQuickSettings"
            @close="showQuickSettings = false"
            @select="operateSelect"
            @edit-proxy="editProxy"
            @edit-env-info="editEnvInfo"
            @export-env="exportEnv"
            @stopAll="stopAll"
            @clean-cache="cleanCache"
            @transfer-env="transferEnv"
            @untransfer-env="untransferEnv"
            @add-label="addLabel"
            @add-group="addGroup"
            @reset-label="resetLabel"
            @clean-label="cleanLabel"
            @edit-start-page="editStartPage"
            @edit-ua="editUA"
            @del-env="delEnv"
            :group-data="groupList.filter((item: any) => item.value !== 'default') as any"
            class="w-[320px]"
          />
        </div>
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
                  class="px-2 py-3 text-left text-sm font-medium text-gray-600 cursor-pointer min-w-[70px]"
                  @click="handleSort('id')"
                >
                  <div class="flex justify-between items-center border-r">
                    <span>序号</span>
                    <div class="mr-2">
                      <CaretDownIcon class="size-3" />
                      <CaretUpIcon class="size-3" />
                    </div>
                  </div>
                </th>
                <th
                  v-if="column.key === 'name'"
                  class="px-2 py-3 text-sm font-medium text-left text-gray-600 cursor-pointer min-w-fit"
                  @click="handleSort('name')"
                >
                  <div class="flex justify-between items-center border-r">
                    <span>环境名称</span>
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
              :class="{ 'bg-[#EDF1F7]': selectedItems.includes(item.uuid) }"
            >
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
                          ? "启动 "
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
                            {{ e.label + "--" + item.id }}
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
                        'bg-green-500': browserStatusStore.getStatus(item.uuid),
                        'bg-red-500': !browserStatusStore.getStatus(item.uuid),
                      }"
                    ></div>
                    {{
                      !browserStatusStore.getStatus(item.uuid)
                        ? "未启动"
                        : "已启动"
                    }}
                  </div>
                  <div v-else class="flex justify-between items-center text-sm">
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
        </table>
        <div
          v-show="filterData.length == 0"
          class="flex flex-col flex-1 justify-center items-center space-y-3"
        >
          <AddCardIcon
            class="size-[200px] text-blue-400 border-gray-300 flex items-center justify-center"
          />
          <p>您可创建具有独立高质量指纹的profile，也可自定义编辑指纹信息</p>
          <button
            @click="
              router.push('/environment/create/environment-simple-create')
            "
            class="p-2 bg-[#5050FA] text-white rounded-lg flex items-center gap-3"
          >
            <AssCircleIcon class="size-5" />
            新建环境
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
  </div>

  <!-- 导出环境 -->
  <ExportEnv
    :open="exportEnvModel"
    @close="exportEnvModel = false"
    :data="messageData"
  />
  <EditProxy
    :open="editProxyModel"
    @close="editProxyModel = false"
    :data="messageData"
    v-model:isChange="flag"
  />
  <EditEnvInfo
    :open="editEnvInfoModel"
    @close="editEnvInfoModel = false"
    :data="messageData"
  />
  <TransferEnv
    :open="transferEnvModel"
    @close="transferEnvModel = false"
    :data="messageData"
    v-model:is-change="flag"
  />
  <UntransferEnv
    :open="UntransferEnvModel"
    @close="UntransferEnvModel = false"
    :data="messageData"
  />
  <CleanCache
    :open="cleanCacheModel"
    @close="cleanCacheModel = false"
    :data="messageData"
  />
  <DelEnv
    :open="delEnvModel"
    @close="delEnvModel = false"
    :data="messageData"
    v-model:isChange="flag"
  />

  <AddLabel
    :open="addLabelModel"
    @close="addLabelModel = false"
    :data="messageData"
  />
  <ResetLabel
    :open="resetLabelModel"
    @close="resetLabelModel = false"
    :data="messageData"
    v-model:isChange="flag"
  />
  <CleanLabel :open="cleanLabelMode" @close="cleanLabelMode = false" />
  <AddGroup
    :open="addGroupModel"
    @close="addGroupModel = false"
    @search-group="searchGroup"
  />
  <EditStartPage
    :open="editStartPageModel"
    @close="editStartPageModel = false"
    :data="messageData"
  />
  <EditUa
    :open="editUAModel"
    @close="editUAModel = false"
    :data="messageData"
  />
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted, onUnmounted } from "vue";
import Operate from "./com/operate.vue";
import TableTheadChoose from "./com/table-thead-choose.vue";
import SelectFilter from "./com/select-filter.vue";
import Page from "./com/page.vue";
import GroupChoose from "./com/group-choose.vue";
import ExportEnv from "./pop-box/export/exportEnv.vue";
import EditProxy from "./pop-box/edit-proxy.vue";
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
  WrapperIcon,
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
import { toast } from "vue-sonner";
import { useBrowserStatusStore } from "@/stores/browser";
import { environment_group_query } from "@/commands/environment-group";
import {
  kernel_location,
  latest_kernel,
  install_location,
} from "@/commands/kernel";

const browserStatusStore = useBrowserStatusStore();
const router = useRouter();
const route = useRoute();
const showSearchChoose = ref(false);
const showQuickSettings = ref(false);
const showColumnsModal = ref(false);
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
const searchQuery = ref("");
const sortColumn = ref("id");
const sortOrder = ref("asc");
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
const allColumns = ref<any>([]); //全部列
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
const theadChoose = (val: any) => {
  allColumns.value = val;
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
const editProxy = () => {
  editProxyModel.value = true;
};
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
const searchGroup = () => {
  environment_group_query(1, 1000).then((res: any) => {
    groupList.value = res.data.data.map((item: any) => {
      return {
        id: item.id,
        title: item.name,
        value: item.name,
        href: "/environment/" + item.id,
      };
    });
    groupList.value = [
      {
        id: 0,
        title: "默认分组",
        value: "default",
        href: "/environment/0",
      },
      ...groupList.value,
    ];
  });
};
onMounted(() => {
  searchGroup();
});
const visibleColumns = computed(() => {
  return allColumns.value.filter((column: any) => column.visible);
});

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

const tableData = ref<any>([]);

const filterData = computed(() => {
  // 根据搜索关键词过滤表格数据
  const filteredTableData = tableData.value.filter((item: any) =>
    item.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );

  console.log("tableData", tableData.value);

  console.log("filteredTableData", filteredTableData);

  // 对过滤后的数据进行排序
  const sortedData = [...filteredTableData].sort((a: any, b: any) => {
    if (sortColumn.value === "id") {
      // 按id排序
      return sortOrder.value === "asc" ? a.ind - b.ind : b.ind - a.ind;
    } else if (sortColumn.value === "name") {
      // 按名称排序
      return sortOrder.value === "asc"
        ? a.name.localeCompare(b.name)
        : b.name.localeCompare(a.name);
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
    // 创建新的对象，只包含uuid和id
    const filteredItem: Record<string, any> = { uuid: item.uuid, id: item.id };
    // 遍历可见列，将对应的值添加到新对象中
    visibleKeys.forEach((key: any) => {
      if (key !== "id" && key in item) {
        filteredItem[key] = item[key];
      }
    });
    return filteredItem;
  });
});

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
const toggleSelectAll = (event: Event) => {
  const target = event.target as HTMLInputElement;
  if (target.checked) {
    selectedItems.value = filterData.value?.map((item) => item.uuid);
  } else {
    selectedItems.value = [];
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

const getList = () => {
  if (route.params.id === "0") {
    environment_query(PageObj.pageNum, PageObj.pageSize).then((res: any) => {
      PageObj.total = res.data.total;
      tableData.value = res.data.data.map((item: any) => {
        return { ...item, created_at: formatDate(item.created_at) };
      });
      dealTableData(tableData.value); //字段处理
      dataCache.value = JSON.parse(JSON.stringify(res.data.data));
    });
  } else {
    environment_query_by_group(
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
      name: item.name,
      action: "启动",
      status: "未启动",
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
