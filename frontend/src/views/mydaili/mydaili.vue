<script setup lang="ts">
import { ref, computed, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import {
  UserPlusIcon,
  SearchIcon,
  TrashIcon,
  SettingsIcon,
  MoreVerticalIcon,
  PlusIcon,
  PencilIcon,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import TooltipButton from "@/components/tooltip-button.vue";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { groupIcons } from "@/assets/icons/index";
import {
  environment_proxies_delete,
  environment_proxies_query,
  environment_proxies_modify,
  environment_proxies_batch_delete,
} from "@/commands/environment-proxy";

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
  environment_proxy_group_create,
  environment_proxy_group_delete,
  environment_proxy_group_modify,
  environment_proxy_group_query,
} from "@/commands/environment-proxy-group";
import { toast } from "vue-sonner";

interface Payment {
  id: number;
  info: string;
  ip: string;
  kind: string;
  port: number;
  envCount: number;
  notes: string;
  ipChannel: string;
  name: string;
  group: string;
  location: string;
  domain_name: string;
  remark: string;
  deleted_username: string;
  deleted_at: string;
  selected?: boolean;
}

const data = ref<Array<Payment>>([]);

const pagination = reactive({
  pageIndex: 0,
  pageSize: 10,
  total: 0,
});

const loadData = (index: number, size: number) => {
  console.log("调用loadData", index, size);
  environment_proxies_query(index, size).then((res) => {
    let { data: data_, total } = res.data;
    pagination.total = total;
    data.value = data_;
  });
};

onMounted(() => loadData(pagination.pageIndex, pagination.pageSize));

const router = useRouter();
const selectAll = ref(false);

const activeDropdown = ref(null);
const isOpen = ref(false);
const showPassword = ref(false);
const showSetModal = ref(false); // 控制弹窗显示与隐藏
const selectedAgentType = ref("random"); // 代理选择类型
const form = reactive({
  smartRecognition: "",
  proxyType: "IPv4",
  proxyService: "SOCKS5",
  host: "",
  port: "",
  username: "",
  password: "",
  parseUrl: "",
});

const selectData = computed(() => {
  return data.value.filter((item) => item.selected);
});

const deleteAll = async () => {
  let ids = selectData.value.map((item) => item.id);
  await environment_proxies_batch_delete(ids);

  loadData(pagination.pageIndex, pagination.pageSize);
  selectAll.value = false;
  disabled: selectData.value.length <= 0;
};

// 打开弹窗
const openSetModal = () => {
  showSetModal.value = true;
};
// 关闭弹窗并处理确认事件
const handleConfirm = () => {
  showSetModal.value = false; // 关闭弹窗
};

const currentId = ref();
const openModal = (row: any) => {
  currentId.value = row;
  isOpen.value = true;
};
const closeModal = () => {
  isOpen.value = false;
};

const saveModal = async () => {
  const payload = {
    kind: form.proxyService,
    host: form.host,
    port: form.port,
    username: form.username,
    password: form.password,
    user_uuid: currentId.value.user_uuid,
  };
  console.log("currentId.value", currentId.value);
  console.log("payload", payload);

  await environment_proxies_modify(currentId.value.id, payload);
  isOpen.value = false;
  loadData(pagination.pageIndex, pagination.pageSize);
};

// 跳转到新增代理页面
const adddaili = () => {
  router.push("/adddaili");
};

const toggleDropdown = (id) => {
  if (activeDropdown.value === id) {
    activeDropdown.value = null;
  } else {
    activeDropdown.value = id;
  }
};

const deleteModel = ref(false);

const deleteOpenHandle = async (id: number) => {
  await environment_proxies_delete(id);
  loadData(pagination.pageIndex, pagination.pageSize);
};

const filters = ref([
  { key: "platform", label: "账号平台", placeholder: "请选择", value: "" },
  { key: "browser", label: "浏览器", placeholder: "请选择", value: "" },
  { key: "environment", label: "环境分组", placeholder: "请选择", value: "" },
  { key: "proxyType", label: "代理类型", placeholder: "请选择", value: "" },
  { key: "provider", label: "服务商", placeholder: "请选择", value: "" },
  { key: "operator", label: "操作者", placeholder: "请选择", value: "" },
  {
    key: "transferType",
    label: "转移类型",
    placeholder: "含备注与代理",
    value: "",
  },
]);

const toggleRowSelection = (row: Payment) => {
  row.selected = !row.selected;
};

//分组
const groupisOpen = ref(false);
const newGroupName = ref("");

const groups = ref<any[]>([]);

const groupData = async () => {
  await environment_proxy_group_query(1, 1000000).then((res) => {
    groups.value = res.data.data;
  });
};

onMounted(() => {
  groupData();
});

const toggleAll = () => {
  data.value.forEach((row) => {
    row.selected = selectAll.value;
  });
};
const editingIndex = ref(-1);
const editingName = ref();

const addGroup = async () => {
  const payload = {
    name: newGroupName.value.trim(),
  };
  if (newGroupName.value.trim() === "") {
    toast.warning("分组名不能为空");
    return;
  }
  await environment_proxy_group_create(payload);
  groupData();
};

const startEdit = async (groups: any, index: number) => {
  editingName.value = groups[index].name;
  editingIndex.value = index;
};

const saveEdit = async (groups: any, index: number) => {
  console.log("editingIndex", editingIndex.value);
  console.log("groups", groups);
  console.log("index", index);
  await environment_proxy_group_modify(groups[editingIndex.value].id, {
    name: editingName.value,
  });
  toast.success("修改成功");

  groupData();
};

const deleteGroup = async (index: number) => {
  await environment_proxy_group_delete(groups.value[index].id);
  groupData();
  toast.success("删除成功");
};

const confirm = () => {
  // 在这里处理确认逻辑，例如发送更新后的分组到父组件
  emit("update:groups", groups.value);
  closePopover();
};

const closePopover = () => {
  groupisOpen.value = false;
  emit("update:open", false);
};

// 定义组件的事件
const emit = defineEmits(["update:open", "update:groups"]);

const paginationClickHandle = (index: number) => {
  console.log(index);
  loadData(index, pagination.pageSize);
  pagination.pageIndex = index;
};
</script>

<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <!-- Table Container with fixed height and scroll -->
    <div
      class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow"
      style="padding: 20px"
    >
      <!-- Header Actions -->
      <div class="flex justify-between mb-6">
        <div class="flex gap-4">
          <button
            @click="adddaili"
            class="flex gap-2 items-center px-4 py-2 text-white bg-blue-600 rounded-lg hover:bg-blue-700"
          >
            <UserPlusIcon class="w-5 h-5" />
            添加代理
          </button>
          <button
            class="flex gap-2 items-center px-4 py-2 text-blue-600 rounded-lg border border-blue-600 hover:bg-blue-50"
          >
            <SearchIcon class="w-5 h-5" />
            检查代理
          </button>
        </div>
        <div class="flex gap-4">
          <Popover v-model:open="groupisOpen">
            <PopoverTrigger as-child>
              <TooltipButton
                title="分组管理"
                class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
              >
                <groupIcons class="w-5 h-5" />
              </TooltipButton>
            </PopoverTrigger>
            <PopoverContent
              class="p-6 w-48 text-sm min-w-48"
              align="end"
              style="min-width: 500px"
            >
              <div class="w-full">
                <div class="flex justify-between items-center mb-4">
                  <h3 class="text-lg font-medium">分组管理</h3>
                  <button
                    @click="closePopover"
                    class="text-gray-400 hover:text-gray-600"
                  >
                    <XIcon class="w-5 h-5" />
                  </button>
                </div>

                <div class="flex gap-2 mb-4">
                  <input
                    v-model="newGroupName"
                    type="text"
                    placeholder="请输入分组名"
                    class="flex-1 px-3 py-2 text-sm rounded-md border focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                  <button
                    @click="addGroup"
                    class="flex gap-1 items-center px-3 py-2 text-sm text-gray-600 rounded-md border hover:border-blue-500 hover:text-blue-500"
                  >
                    <PlusIcon class="w-4 h-4" />
                    添加
                  </button>
                </div>

                <div class="max-h-[300px] overflow-y-auto">
                  <div class="grid grid-cols-3 gap-3">
                    <div
                      v-for="(group, index) in groups"
                      :key="index"
                      class="relative p-2 text-sm rounded-md border group hover:border-blue-500"
                      :class="{ 'border-blue-500': editingIndex === index }"
                    >
                      <input
                        v-if="editingIndex === index"
                        v-model="editingName"
                        @blur="saveEdit(groups, index)"
                        @keyup.enter="saveEdit(groups, index)"
                        ref="editInput"
                        class="w-full text-sm outline-none"
                      />
                      <span v-else>{{ group.name }}</span>

                      <div
                        class="hidden absolute right-1 top-1/2 gap-1 items-center -translate-y-1/2 group-hover:flex"
                        :class="{ '!flex': editingIndex === index }"
                      >
                        <button
                          title="修改"
                          @click="startEdit(groups, index)"
                          class="p-1 text-blue-500 hover:text-blue-600"
                        >
                          <PencilIcon class="w-4 h-4" />
                        </button>
                        <button
                          title="删除"
                          @click="deleteGroup(index)"
                          class="p-1 text-blue-500 hover:text-blue-600"
                        >
                          <TrashIcon class="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="flex gap-3 justify-center pt-4 mt-4 border-t">
                  <button
                    @click="confirm"
                    class="px-4 py-2 text-sm text-white bg-blue-600 rounded-md hover:bg-blue-700"
                  >
                    确定
                  </button>
                  <button
                    @click="closePopover"
                    class="px-4 py-2 text-sm text-gray-600 bg-gray-100 rounded-md hover:bg-gray-200"
                  >
                    取消
                  </button>
                </div>
              </div>
            </PopoverContent>
          </Popover>

          <TooltipButton
            title="多选删除"
            @click="deleteAll()"
            class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
          >
            <TrashIcon class="w-5 h-5" />
          </TooltipButton>

          <TooltipButton
            title="设置"
            @click="openSetModal"
            class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500"
          >
            <SettingsIcon class="w-5 h-5" />
          </TooltipButton>
        </div>
      </div>

      <!-- Table -->
      <div class="overflow-auto flex-1 min-w-[900px]">
        <table class="w-full">
          <thead class="sticky top-0 z-10 bg-gray-50">
            <tr>
              <th class="p-4 text-left">
                <input
                  type="checkbox"
                  class="rounded border-gray-300"
                  v-model="selectAll"
                  @change="toggleAll"
                />
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                序号
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                代理信息
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                出口IP
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                关联环境数
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                备注
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                IP查询通道
              </th>
              <th class="p-4 text-sm font-medium text-left text-gray-600">
                操作
              </th>
            </tr>
          </thead>

          <tbody>
            <tr
              v-for="(row, index) in data"
              :key="index"
              class="hover:bg-blue-100 h-[20px]"
              :class="{ 'bg-blue-50': row.selected }"
              @click="toggleRowSelection(row)"
            >
              <td class="p-4 text-left">
                <input
                  type="checkbox"
                  class="rounded border-gray-300"
                  v-model="row.selected"
                />
              </td>

              <td class="p-4 text-sm text-left">
                {{ pagination.pageIndex * pagination.pageSize + index + 1 }}
              </td>
              <td class="p-4 text-sm text-left">{{ row.kind || "\\" }}</td>
              <td class="p-4 text-sm text-left">{{ row.port || "\\" }}</td>
              <td class="p-4 text-sm text-left">{{ row.envCount || "\\" }}</td>
              <td class="p-4 text-sm text-left">{{ row.notes || "\\" }}</td>
              <td class="p-4 text-sm text-left">{{ row.ipChannel || "\\" }}</td>

              <td class="p-4 text-left">
                <div class="flex relative gap-2 justify-left">
                  <!-- 更多按钮 -->
                  <More>
                    <MoreTrigger>
                      <MoreVerticalIcon
                        @click="toggleDropdown(row.id)"
                        class="w-4 h-4 text-gray-600 cursor-pointer hover:text-gray-900"
                      />
                    </MoreTrigger>
                    <MoreContent>
                      <MoreItem class="cursor-pointer" @click="openModal(row)">
                        <Settings2Icon class="w-4 h-4" />编辑
                      </MoreItem>
                      <MoreItem class="cursor-pointer">
                        <SquarePenIcon class="w-4 h-4" />检查
                      </MoreItem>
                      <MoreItem
                        class="cursor-pointer"
                        @click="deleteOpenHandle(row.id)"
                      >
                        <Trash2Icon class="w-4 h-4" />删除
                      </MoreItem>
                    </MoreContent>
                  </More>
                </div>
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

    <!-- Add Agent Modal -->
    <Teleport to="body">
      <div
        v-if="showSetModal"
        class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-30"
      >
        <div class="bg-white rounded-lg w-[680px] shadow-xl">
          <!-- Modal Header -->
          <div class="flex justify-between items-center p-4 border-b">
            <h3 class="text-xl font-medium">设置</h3>
            <button
              @click="showSetModal = false"
              class="text-gray-400 hover:text-gray-600"
            >
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Modal Body -->
          <div class="p-6">
            <div class="space-y-4">
              <h4 class="mb-4 text-xl font-medium">随机方式</h4>

              <div class="space-y-3" style="margin-top: 30px">
                <label class="flex gap-3 items-center">
                  <input
                    type="radio"
                    name="agentType"
                    value="random"
                    v-model="selectedAgentType"
                    class="w-4 h-4 text-blue-600"
                  />
                  <div class="flex">
                    <div class="font-medium">随机全部代理</div>
                    <div
                      class="text-sm text-gray-500"
                      style="margin-left: 20px"
                    >
                      在已添加的全部代理中随机选择一个代理
                    </div>
                  </div>
                </label>

                <label class="flex gap-3 items-center" style="margin-top: 20px">
                  <input
                    type="radio"
                    name="agentType"
                    value="selective"
                    v-model="selectedAgentType"
                    class="w-4 h-4 text-blue-600"
                  />
                  <div class="flex">
                    <div class="font-medium">优选随机未使用的代理</div>
                    <div
                      class="text-sm text-gray-500"
                      style="margin-left: 20px"
                    >
                      优选未被使用过的代理中随机选择一个代理
                    </div>
                  </div>
                </label>
              </div>
            </div>
          </div>

          <!-- Modal Footer -->
          <div class="flex gap-3 justify-end p-4 border-t">
            <button
              @click="showSetModal = false"
              class="px-4 py-2 text-gray-600 rounded hover:bg-gray-100"
            >
              取消
            </button>
            <button
              @click="handleConfirm"
              class="px-4 py-2 text-white bg-blue-600 rounded hover:bg-blue-700"
            >
              确定
            </button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- 编辑窗口-->
    <div
      v-if="isOpen"
      class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50"
    >
      <div class="mx-4 w-full max-w-md bg-white rounded-lg">
        <!-- Header -->
        <div class="flex justify-between items-center p-4 border-b">
          <h2 class="text-lg font-medium">编辑代理</h2>
          <button
            @click="closeModal()"
            class="text-gray-400 hover:text-gray-600"
          >
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <!-- Form Content -->
        <div class="p-4 space-y-4">
          <!-- Smart Recognition -->
          <div>
            <label class="block mb-1 text-sm font-medium text-gray-700"
              >智能识别</label
            >
            <textarea
              v-model="form.smartRecognition"
              rows="3"
              class="w-full border-[1px] border-gray-300 rounded-md shadow-sm p-2 text-sm overflow-y-auto resize-none focus:border-blue-600"
              style="height: 200px; outline: none"
              placeholder="[代理主机端口][IP][代理类型][用户名][密码][URL]"
            ></textarea>
          </div>

          <!-- Proxy Type -->
          <div>
            <label class="block mb-2 text-sm font-medium text-gray-700"
              >代理类型</label
            >
            <div class="flex space-x-4">
              <label class="flex items-center">
                <input
                  type="radio"
                  v-model="form.proxyType"
                  value="IPv4"
                  class="w-4 h-4 text-blue-600"
                />
                <span class="ml-2 text-sm">IPv4</span>
              </label>
              <label class="flex items-center">
                <input
                  type="radio"
                  v-model="form.proxyType"
                  value="IPv6"
                  class="w-4 h-4 text-blue-600"
                />
                <span class="ml-2 text-sm">IPv6</span>
              </label>
            </div>
          </div>

          <!-- Proxy Service -->
          <div>
            <label class="block mb-2 text-sm font-medium text-gray-700"
              >代理服务</label
            >
            <div class="flex space-x-2">
              <Select v-model="form.proxyService">
                <SelectTrigger>
                  <SelectValue
                    placeholder="SOCKS5"
                    class="p-2 w-full rounded-lg outline-none"
                  />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="SOCKS5"> SOCKS5 </SelectItem>
                    <SelectItem value="SOCKS52"> SOCKS52 </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
              <input
                v-model="form.host"
                type="text"
                class="flex-1 p-2 text-sm rounded-md border border-gray-300 shadow-sm focus:border-blue-600"
                placeholder="89.116.77.90"
              />
              <span class="flex items-center text-gray-500">:</span>
              <input
                v-model="form.port"
                type="text"
                class="p-2 w-20 text-sm rounded-md border border-gray-300 shadow-sm focus:border-blue-600"
                placeholder="8085"
              />
            </div>
          </div>

          <!-- Username -->
          <div>
            <label class="block mb-1 text-sm font-medium text-gray-700"
              >用户名</label
            >
            <input
              v-model="form.username"
              type="text"
              class="p-2 w-full text-sm rounded-md border border-gray-300 shadow-sm focus:border-blue-600"
              placeholder="hmdybckg"
            />
          </div>

          <!-- Password -->
          <div>
            <label class="block mb-1 text-sm font-medium text-gray-700"
              >密码</label
            >
            <input
              v-model="form.password"
              :type="showPassword ? 'text' : 'password'"
              class="p-2 w-full text-sm rounded-md border border-gray-300 shadow-sm focus:border-blue-600"
              placeholder="vx*br*zwq#9z"
            />
          </div>

          <!-- Parse URL -->
          <div>
            <label class="block mb-1 text-sm font-medium text-gray-700">
              解析URL
              <span class="text-gray-400">(选填)</span>
            </label>
            <input
              v-model="form.parseUrl"
              type="text"
              class="p-2 w-full text-sm rounded-md border border-gray-300 shadow-sm"
            />
          </div>

          <!-- Submit Button -->
          <button
            @click="saveModal()"
            class="px-4 py-2 w-full text-white bg-blue-500 rounded-md transition-colors hover:bg-blue-600"
          >
            保存
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
