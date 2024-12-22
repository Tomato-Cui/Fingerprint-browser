<template>
  <div class="p-4 bg-gray-50 h-main flex flex-col">


    <!-- Table Container with fixed height and scroll -->
    <div class="flex flex-col flex-1 bg-white rounded-lg shadow overflow-hidden" style="padding: 20px">

      <!-- Header Actions -->
      <div class="flex justify-between mb-6">
        <div class="flex gap-4">
          <button @click="adddaili" class="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700">
            <UserPlusIcon class="w-5 h-5" />
            添加代理
          </button>
          <button class="border border-blue-600 text-blue-600 px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-50">
            <SearchIcon class="w-5 h-5" />
            检查代理
          </button>
        </div>
        <div class="flex gap-4">
          <Popover  v-model:open="filtersisOpen">
            <PopoverTrigger as-child>
              <TooltipButton title="筛选"
                             class="p-2 rounded hover:bg-gray-0   border-[1px] border-gray-300  hover:border-[1px] hover:border-blue-600
              active:bg-blue-50
              hover:text-blue-500">
                <IconFilter class="w-5 h-5"/>
              </TooltipButton>
            </PopoverTrigger>
            <PopoverContent class="min-w-[500px] p-4" align="end">
              <div class="space-y-4">
                <h3 class="text-base font-medium mb-4">筛选</h3>

                <!-- Filter Options -->
                <div v-for="(filter, index) in filters" :key="index" class="space-y-2 flex justify-between">
                  <div class="flex items-center group" style="min-width: 50px">
                    <span class="text-sm text-gray-700 ml-auto">{{ filter.label }}</span>
                  </div>
                  <div class="flex items-center border rounded-md">
                    <Select>
                      <SelectTrigger>
                        <SelectValue
                            :placeholder="filter.placeholder"
                            class="w-full p-2 rounded-lg outline-none"
                            style="min-width: 360px"
                        />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectItem value="扩展屏幕 (1920x1080)">
                            选择1
                          </SelectItem>
                          <SelectItem value="内置屏幕 (1280x720)">
                            选择2
                          </SelectItem>
                        </SelectGroup>
                      </SelectContent>
                    </Select>

                  </div>
                </div>

                <!-- Time Range -->
                <div class="space-y-2 flex ">
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-700">转移时间</span>
                    <input
                        type="datetime-local"
                        class="flex-1 px-3 py-2 text-sm border rounded-md outline-none focus:border-blue-500"
                        v-model="timeRange.start"
                    />
                    <span class="text-gray-400">至</span>
                    <input
                        type="datetime-local"
                        class="flex-1 px-3 py-2 text-sm border rounded-md outline-none focus:border-blue-500"
                        v-model="timeRange.end"
                    />
                  </div>
                </div>

                <!-- Action Buttons -->
                <div class="flex items-center space-x-3 pt-4">
                  <button
                      class="flex-1 px-4 py-2 text-sm text-white bg-blue-600 rounded-md hover:bg-blue-700"
                      @click="applyFilters"
                  >
                    确定
                  </button>
                  <button
                      class="flex-1 px-4 py-2 text-sm text-gray-600 bg-gray-100 rounded-md hover:bg-gray-200"
                      @click="resetFilters"
                  >
                    取消
                  </button>
                </div>
              </div>
            </PopoverContent>
          </Popover>

          <Popover v-model:open="groupisOpen">
            <PopoverTrigger as-child>
              <TooltipButton title="分组管理"
                             class="p-2 rounded hover:bg-gray-0   border-[1px] border-gray-300  hover:border-[1px] hover:border-blue-600
              active:bg-blue-50
              hover:text-blue-500">
                <groupIcons class="w-5 h-5"/>
              </TooltipButton>
            </PopoverTrigger>
            <PopoverContent class="min-w-48 w-48 p-6 text-sm" align="end" style="min-width: 500px">

              <div class="w-full">
                <div class="flex justify-between items-center mb-4">
                  <h3 class="text-lg font-medium">分组管理</h3>
                  <button @click="closePopover" class="text-gray-400 hover:text-gray-600">
                    <XIcon class="w-5 h-5" />
                  </button>
                </div>

                <div class="flex gap-2 mb-4">
                  <input
                      v-model="newGroupName"
                      type="text"
                      placeholder="请输入分组名"
                      class="flex-1 px-3 py-2 border rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                  <button
                      @click="addGroup"
                      class="flex items-center gap-1 px-3 py-2 text-sm text-gray-600 border rounded-md hover:border-blue-500 hover:text-blue-500"
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
                        class="relative group border rounded-md p-2 text-sm hover:border-blue-500"
                        :class="{'border-blue-500': editingIndex === index}"
                    >
                      <input
                          v-if="editingIndex === index"
                          v-model="editingName"
                          @blur="saveEdit"
                          @keyup.enter="saveEdit"
                          ref="editInput"
                          class="w-full outline-none text-sm"
                      />
                      <span v-else>{{ group }}</span>

                      <div
                          class="absolute right-1 top-1/2 -translate-y-1/2 hidden group-hover:flex items-center gap-1"
                          :class="{'!flex': editingIndex === index}"
                      >
                        <button
                            @click="startEdit(index)"
                            class="p-1 text-blue-500 hover:text-blue-600"
                        >
                          <PencilIcon class="w-4 h-4" />
                        </button>
                        <button
                            @click="deleteGroup(index)"
                            class="p-1 text-blue-500 hover:text-blue-600"
                        >
                          <TrashIcon class="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="flex justify-center gap-3 mt-4 pt-4 border-t">
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

          <TooltipButton title="删除" @click="deleteOpenHandle"
                         class="p-2 rounded hover:bg-gray-0   border-[1px] border-gray-300  hover:border-[1px] hover:border-blue-600
              active:bg-blue-50
              hover:text-blue-500">
            <TrashIcon class="w-5 h-5"/>
          </TooltipButton>

          <TooltipButton title="设置"  @click="openSetModal"
                         class="p-2 rounded hover:bg-gray-0   border-[1px] border-gray-300  hover:border-[1px] hover:border-blue-600
              active:bg-blue-50
              hover:text-blue-500">
            <SettingsIcon class="w-5 h-5"/>
          </TooltipButton>

          <TooltipButton title="刷新" @click="() => console.log('abc')"
                         class="p-2 rounded hover:bg-gray-0   border-[1px] border-gray-300  hover:border-[1px] hover:border-blue-600
              active:bg-blue-50
              hover:text-blue-500">
            <RefreshCwIcon class="w-5 h-5"/>
          </TooltipButton>
        </div>
      </div>

      <!-- Table -->
      <div class="overflow-auto flex-1 min-w-[900px]">
        <table class="w-full">

          <thead class="bg-gray-50 sticky top-0 z-10">
          <tr>
            <th class="p-4 text-center">
              <input type="checkbox" class="rounded border-gray-300" v-model="selectAll" @change="toggleSelectAll">
            </th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">代理ID</th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">代理信息</th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">出口IP</th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">关联环境数</th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">备注</th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">IP查询通道</th>
            <th class="p-4 text-center text-sm font-medium text-gray-600">操作</th>
          </tr>
          </thead>

          <tbody>
          <tr v-for="agent in agents" :key="agent.id"
              :class="{
                  'border-t border-gray-100': true,
                  'bg-blue-50 hover:bg-blue-100': agent.selected,
                  'hover:bg-blue-100': !agent.selected,
                }"
          >

            <td class="p-4 text-center">
              <div class="flex justify-center">
                <input type="checkbox" class="rounded border-gray-300" v-model="agent.selected" />
              </div>
            </td>

            <td class="p-4 text-center text-sm">{{ agent.id }}</td>
            <td class="p-4 text-center text-sm">{{ agent.info }}</td>
            <td class="p-4 text-center text-sm">{{ agent.ip }}</td>
            <td class="p-4 text-center text-sm">{{ agent.envCount }}</td>
            <td class="p-4 text-center text-sm">{{ agent.notes }}</td>
            <td class="p-4 text-center text-sm">{{ agent.ipChannel }}</td>

            <td class="p-4 text-center">
              <div class="flex justify-center items-center gap-2 relative"> <!-- 给父容器添加relative定位 -->
                <!-- 刷新按钮 -->
                <button class="text-gray-600 hover:text-blue-600 flex items-center gap-1">
                  <RefreshCwIcon class="w-4 h-4" />
                  刷新
                </button>

                <!-- 更多按钮 -->
                <More>
                  <MoreTrigger>
                    <MoreVerticalIcon
                        @click="toggleDropdown(agent.id)"
                        class="w-4 h-4 cursor-pointer text-gray-600 hover:text-gray-900"
                    />
                  </MoreTrigger>
                  <MoreContent>
                    <MoreItem class="cursor-pointer" @click="openModal">
                      <Settings2Icon class="w-4 h-4" />编辑
                    </MoreItem>
                    <MoreItem class="cursor-pointer" >
                      <SquarePenIcon class="w-4 h-4" />检查
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="deleteOpenHandle">
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

      <!-- Pagination fixed at the bottom -->
      <div class="bg-white  p-4 mt-auto flex items-center justify-between">
        <div class="text-sm text-gray-600">
          共 {{ totalItems }} 项数据
        </div>
        <div class="flex items-center gap-2">
          <button class="p-2 hover:bg-gray-100 rounded" :disabled="currentPage === 1">
            <ChevronLeftIcon class="w-5 h-5" />
          </button>
          <div class="flex gap-1">
            <button
                v-for="page in visiblePages"
                :key="page"
                class="px-3 py-1 rounded"
                :class="page === currentPage ? 'bg-blue-600 text-white' : 'hover:bg-gray-100'"
            >
              {{ page }}
            </button>
          </div>
          <button class="p-2 hover:bg-gray-100 rounded">
            <ChevronRightIcon class="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>

    <!-- Add Agent Modal -->
    <Teleport to="body">
      <div v-if="showSetModal" class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg w-[680px] shadow-xl">
          <!-- Modal Header -->
          <div class="flex justify-between items-center p-4 border-b">
            <h3 class="text-xl font-medium">设置</h3>
            <button @click="showSetModal = false" class="text-gray-400 hover:text-gray-600">
              <XIcon class="w-5 h-5" />
            </button>
          </div>

          <!-- Modal Body -->
          <div class="p-6">
            <div class="space-y-4">
              <h4 class="text-xl font-medium mb-4">随机方式</h4>

              <div class="space-y-3" style="margin-top: 30px">
                <label class="flex items-center gap-3">
                  <input
                      type="radio"
                      name="agentType"
                      value="random"
                      v-model="selectedAgentType"
                      class="w-4 h-4 text-blue-600"
                  >
                  <div  class="flex">
                    <div class="font-medium">随机全部代理</div>
                    <div class="text-sm text-gray-500" style="margin-left: 20px">在已添加的全部代理中随机选择一个代理</div>
                  </div>
                </label>

                <label class="flex items-center gap-3" style="margin-top: 20px">
                  <input
                      type="radio"
                      name="agentType"
                      value="selective"
                      v-model="selectedAgentType"
                      class="w-4 h-4 text-blue-600"
                  >
                  <div class="flex">
                    <div class="font-medium">优选随机未使用的代理</div>
                    <div class="text-sm text-gray-500" style="margin-left: 20px">优选未被使用过的代理中随机选择一个代理</div>
                  </div>
                </label>
              </div>
            </div>
          </div>

          <!-- Modal Footer -->
          <div class="flex justify-end gap-3 p-4 border-t">
            <button
                @click="showSetModal = false"
                class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded"
            >
              取消
            </button>
            <button
                @click="handleConfirm"
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              确定
            </button>
          </div>
        </div>
      </div>
    </Teleport>


    <!-- 编辑窗口-->
    <div v-if="isOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg w-full max-w-md mx-4">
        <!-- Header -->
        <div class="flex justify-between items-center p-4 border-b">
          <h2 class="text-lg font-medium">编辑代理</h2>
          <button
              @click="closeModal"
              class="text-gray-400 hover:text-gray-600"
          >
            <XIcon class="h-5 w-5" />
          </button>
        </div>

        <!-- Form Content -->
        <div class="p-4 space-y-4">
          <!-- Smart Recognition -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">智能识别</label>
            <textarea
                v-model="form.smartRecognition"
                rows="3"
                class="w-full border-[1px] border-gray-300 rounded-md shadow-sm p-2 text-sm overflow-y-auto resize-none focus:border-blue-600"
                style="height: 200px;outline: none"
                placeholder="[代理主机端口][IP][代理类型][用户名][密码][URL]"
            ></textarea>
          </div>

          <!-- Proxy Type -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">代理类型</label>
            <div class="flex space-x-4">
              <label class="flex items-center">
                <input
                    type="radio"
                    v-model="form.proxyType"
                    value="IPv4"
                    class="h-4 w-4 text-blue-600"
                >
                <span class="ml-2 text-sm">IPv4</span>
              </label>
              <label class="flex items-center">
                <input
                    type="radio"
                    v-model="form.proxyType"
                    value="IPv6"
                    class="h-4 w-4 text-blue-600"
                >
                <span class="ml-2 text-sm">IPv6</span>
              </label>
            </div>
          </div>

          <!-- Proxy Service -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">代理服务</label>
            <div class="flex space-x-2">
              <Select >
                <SelectTrigger>
                  <SelectValue
                      placeholder="SOCKS5"
                      class="w-full p-2 rounded-lg outline-none"
                  />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="SOCKS5">
                      SOCKS5
                    </SelectItem>
                    <SelectItem value="SOCKS52">
                      SOCKS52
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
              <input
                  v-model="form.host"
                  type="text"
                  class="flex-1 border border-gray-300 rounded-md shadow-sm p-2 text-sm focus:border-blue-600"
                  placeholder="89.116.77.90"
              >
              <span class="flex items-center text-gray-500">:</span>
              <input
                  v-model="form.port"
                  type="text"
                  class="w-20 border border-gray-300 rounded-md shadow-sm p-2 text-sm focus:border-blue-600"
                  placeholder="8085"
              >
            </div>


          </div>

          <!-- Username -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">用户名</label>
            <input
                v-model="form.username"
                type="text"
                class="w-full border border-gray-300 rounded-md shadow-sm p-2 text-sm focus:border-blue-600"
                placeholder="hmdybckg"
            >
          </div>

          <!-- Password -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">密码</label>
            <input
                v-model="form.password"
                :type="showPassword ? 'text' : 'password'"
                class="w-full border border-gray-300 rounded-md shadow-sm p-2 text-sm focus:border-blue-600"
                placeholder="vx*br*zwq#9z"
            >
          </div>

          <!-- Parse URL -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              解析URL
              <span class="text-gray-400">(选填)</span>
            </label>
            <input
                v-model="form.parseUrl"
                type="text"
                class="w-full border border-gray-300 rounded-md shadow-sm p-2 text-sm "
            >
          </div>

          <!-- Submit Button -->
          <button
              @click="handleSubmit"
              class="w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 transition-colors"
          >
            保存
          </button>
        </div>


      </div>
    </div>

  </div>

  <AlertModel
      title="删除代理"
      :open="deleteModel"
      @close="deleteCloseHandle"
      @cancel="deleteCloseHandle"
      @submit="deleteCloseHandle"
  >
    <div class="text-sm flex flex-col gap-y-4 py-4">
      <!-- Content -->
      <div class="p-4">
        <div class="flex items-center justify-between mb-4">
          <p class="text-gray-700">你确定要删除以下代理吗？</p>
          <button
              @click="isExpanded = !isExpanded"
              class="text-blue-600 text-sm hover:text-blue-700 flex items-center"
          >
            共{{  selectedAgentslen }}个代理
            <ChevronDown
                class="w-4 h-4 ml-1 transition-transform duration-200"
                :class="{ 'transform rotate-180': isExpanded }"
            />
          </button>
        </div>

        <div class="mb-6">
          <p class="text-sm text-gray-600 mb-2">代理名称</p>
          <div
              class="grid gap-2  max-h-[300px] overflow-y-auto"
              :class="isExpanded ? 'grid-cols-3' : 'grid-cols-3'"
          >
            <div
                v-for="(agent, index) in displayedAgents"
                :key="agent.id"
                class="flex items-center justify-between px-3 py-2 bg-blue-50 rounded-md text-sm"
            >
              <span class="text-gray-700">{{ agent.ipChannel }}</span>
              <button
                  @click="removeAgent(agent.id)"
                  class="text-gray-400 hover:text-gray-600"
              >
                <XCircle class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AlertModel>
</template>

<script setup>
import {ref, computed, reactive, nextTick} from 'vue'
import { AlertModel } from "@/components/alert-model";
import {useRouter} from "vue-router";
import {
  UserPlusIcon,
  SearchIcon,
  TrashIcon,
  SettingsIcon,
  RefreshCwIcon,
  MoreVerticalIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  XIcon,
  PlusIcon,
  PencilIcon,
  ChevronDown,
  XCircle,
  ChevronRight
} from 'lucide-vue-next'
import {Popover, PopoverContent, PopoverTrigger} from "@/components/ui/popover";
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
import {IconFilter,groupIcons} from "@/assets/icons/index";

const router = useRouter();
const selectAll = ref(false)
const currentPage = ref(1)
const totalItems = ref(101)
const activeDropdown = ref(null)
const agents = ref([])
const isOpen = ref(false)
const showPassword = ref(false)
const showSetModal = ref(false)  // 控制弹窗显示与隐藏
const selectedAgentType = ref('random')  // 代理选择类型
const form = reactive({
  smartRecognition: '',
  proxyType: 'IPv4',
  proxyService: 'SOCKS5',
  host: '',
  port: '',
  username: '',
  password: '',
  parseUrl: ''
})


// 打开弹窗
const openSetModal = () => {
  showSetModal.value = true
}
// 关闭弹窗并处理确认事件
const handleConfirm = () => {
  showSetModal.value = false  // 关闭弹窗
}


const openModal = () => {
  isOpen.value = true
}
const closeModal = () => {
  isOpen.value = false
}

const handleSubmit = () => {
  // Handle form submission
  console.log('Form submitted:', form)
  closeModal()
}

for (let i = 0; i < 30; i++) {
  const newAgentsData = {
    id: i, info: '\\', ip: '112.17.242.11', envCount: 90, notes: '\\', ipChannel: '十三香代理', selected: false
  }
  agents.value.push(newAgentsData);
}

const visiblePages = computed(() => {
  const pages = []
  for (let i = 1; i <= 20; i++) {
    pages.push(i)
  }
  return pages
})


// 跳转到新增代理页面
const adddaili = () => {
  router.push('/adddaili');
}

// 切换全选
const toggleSelectAll = () => {
  agents.value.forEach(agent => {
    agent.selected = selectAll.value
  })
}

const toggleDropdown = (id) => {
  if (activeDropdown.value === id) {
    activeDropdown.value = null
  } else {
    activeDropdown.value = id
  }
}

const deleteModel = ref(false);

const deleteOpenHandle =() =>{
  deleteModel.value=true;
}

const deleteCloseHandle =() =>{
  deleteModel.value=false;
}


const filters = ref([
  { key: 'platform', label: '账号平台', placeholder: '请选择', value: '' },
  { key: 'browser', label: '浏览器', placeholder: '请选择', value: '' },
  { key: 'environment', label: '环境分组', placeholder: '请选择', value: '' },
  { key: 'proxyType', label: '代理类型', placeholder: '请选择', value: '' },
  { key: 'provider', label: '服务商', placeholder: '请选择', value: '' },
  { key: 'operator', label: '操作者', placeholder: '请选择', value: '' },
  { key: 'transferType', label: '转移类型', placeholder: '含备注与代理', value: '' },
])

const timeRange = ref({
  start: '',
  end: ''
})

const updateFilter = (key, value) => {
  const filter = filters.value.find(f => f.key === key)
  if (filter) {
    filter.value = value
  }
}

const filtersisOpen = ref(false)
const applyFilters = () => {
  const filterValues = {
    ...filters.value.reduce((acc, filter) => ({
      ...acc,
      [filter.key]: filter.value
    }), {}),
    timeRange: timeRange.value
  }
  filtersisOpen.value = false // 关闭弹窗
}

const resetFilters = () => {
  filters.value.forEach(filter => filter.value = '')
  timeRange.value = {
    start: '',
    end: ''
  }
  filtersisOpen.value = false // 关闭弹窗
}



//分组
const groupisOpen = ref(false)
const newGroupName = ref('')
const groups = ref(['分组一', '分组二', '分组三', '分组四', '分组五', '分组六'])
const editingIndex = ref(-1)
const editingName = ref('')

const addGroup = () => {
  if (newGroupName.value.trim()) {
    groups.value.push(newGroupName.value.trim())
    newGroupName.value = ''
  }
}

const startEdit = async (index) => {
  editingIndex.value = index
  editingName.value = groups.value[index]
  await nextTick()
  if (editingIndex.value !== -1) {
    editInput.value?.focus()
  }
}

const saveEdit = () => {
  if (editingIndex.value !== -1 && editingName.value.trim()) {
    groups.value[editingIndex.value] = editingName.value
  }
  editingIndex.value = -1
}

const deleteGroup = (index) => {
  groups.value.splice(index, 1)
}

const confirm = () => {
  // 在这里处理确认逻辑，例如发送更新后的分组到父组件
  emit('update:groups', groups.value)
  closePopover()
}

const closePopover = () => {
  groupisOpen.value = false
  emit('update:open', false)
}

// 定义组件的事件
const emit = defineEmits(['update:open', 'update:groups'])


const isExpanded = ref(false)

let selectedAgentslen = ref();
// 计算显示的代理数据，根据是否展开显示不同数量的代理
const displayedAgents = computed(() => {
  const selectedAgents = agents.value.filter(agent => agent.selected) // 只获取选中的代理
  selectedAgentslen = selectedAgents.length;
  return isExpanded.value ? selectedAgents : selectedAgents.slice(0, 5) // 展开时显示所有选中的代理，不展开时只显示前 5 个
})


const removeAgent = (id) => {
  const index = agents.value.findIndex(agent => agent.id === id)
  if (index !== -1) {
    agents.value.splice(index, 1)  // 从数组中删除指定代理
  }
}




</script>
