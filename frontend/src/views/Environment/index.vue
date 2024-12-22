<template>
  <div class="flex flex-col p-4 bg-gray-50 h-main">
    <div class="flex overflow-hidden flex-col flex-1 bg-white rounded-lg shadow" style="padding: 20px">
      <!-- Header Section -->
      <div class="flex justify-between items-center mb-4">
        <div class="flex items-center space-x-2">
          <div lass="bg-white rounded-lg p-4 m-[20px] h-[50px]">
            <div class="flex justify-between items-center mb-[20px]">
              <div class="relative" style="align-items: center">
                <div
                  class="appearance-none bg-[#f5f6fa] border border-gray-300 rounded-lg px-4 py-2 w-[200px] cursor-pointer hover:ring-2 hover:ring-blue-500"
                  @click="openOptions">
                  {{ selectedLabel || "请选择" }}
                  <ChevronDownIcon class="absolute right-2 top-2.5 w-5 h-5 text-gray-500" />
                </div>
                <!-- 下拉菜单 -->
                <ul v-if="isOpen"
                  class="absolute bg-white border border-gray-300 rounded-lg w-[200px] mt-1 shadow-lg z-20 p-5">
                  <li v-for="status in status" :key="status.label" @click="selectStatus(status.label)"
                    class="px-4 py-2 hover:bg-[#e8edff] hover:text-blue-600 rounded-md cursor-pointer">
                    {{ status.label }}
                  </li>
                </ul>
                <ChevronDownIcon class="absolute right-2 top-2.5 w-5 h-5 text-gray-400 pointer-events-none" />
              </div>

              <div class="relative flex-1 flex items-center align-items: center gap-4 min-w-[500px] max-w-[1000px]">
                <!-- 搜索框 -->
                <div
                  class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none hover:ring-2 hover:ring-blue-500"
                  style="width: 50vw; margin-left: 20px">
                  <input v-model="selectVal" type="text" :placeholder="'请输入' +
                    (selectType === 1
                      ? '名称'
                      : selectType === 2
                        ? '备注'
                        : '手机号或邮箱')
                    " class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none" />
                  <SearchIcon class="absolute top-2.5 left-3 w-5 h-5 text-gray-400" />
                  <More>
                    <MoreTrigger>
                      <button class="p-2 ml-4 rounded-lg hover:bg-gray-100">
                        <SlidersHorizontalIcon class="w-5 h-5 text-gray-500" />
                      </button>
                    </MoreTrigger>
                    <MoreContent>
                      <MoreItem class="cursor-pointer" @click="selectT(1)">
                        <!-- <Settings2Icon class="w-4 h-4" /> -->
                        名称
                      </MoreItem>
                      <MoreItem class="cursor-pointer" @click="selectT(2)">
                        <!-- <SquarePenIcon class="w-4 h-4" /> -->
                        序号
                      </MoreItem>
                      <MoreItem class="cursor-pointer" @click="selectT(3)">
                        <!-- <Trash2Icon class="w-4 h-4" /> -->
                        环境ID
                      </MoreItem>
                      <MoreItem class="cursor-pointer" @click="selectT(3)">
                        <!-- <Trash2Icon class="w-4 h-4" /> -->
                        账号平台
                      </MoreItem>
                      <MoreItem class="cursor-pointer" @click="selectT(3)">
                        <!-- <Trash2Icon class="w-4 h-4" /> -->
                        平台账号
                      </MoreItem>
                      <MoreItem class="cursor-pointer" @click="selectT(3)">
                        <!-- <Trash2Icon class="w-4 h-4" /> -->
                        备注
                      </MoreItem>
                    </MoreContent>
                  </More>
                </div>
              </div>

              <div class="relative space-x-4">
                <div v-if="showSortWindow"
                  class="absolute top-2 right-36 z-50 p-4 w-32 text-sm text-center bg-white rounded-lg border shadow-lg"
                  :style="{
                    right: '-10px',
                  }">
                  <ul>
                    <li v-for="field in fields" :key="field.id" @click="selectField(field)" :class="[
                      'py-2 px-4 cursor-pointer rounded hover:bg-blue-50',
                      selectedField === field
                        ? 'bg-blue-100 text-blue-500'
                        : '',
                    ]">
                      {{ field.name }}
                    </li>
                  </ul>
                </div>
              </div>

              <div class="flex justify-start items-center px-4 space-x-4">
                <Popover :open="sortOpen">
                  <PopoverTrigger as-child>
                    <TooltipButton title="排序方式" @click="openSortWindow"
                      class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
                      <ListIcon class="w-5 h-5" />
                    </TooltipButton>
                  </PopoverTrigger>
                  <PopoverContent class="p-4 text-sm" align="end">
                    <div class="bg-white rounded">
                      <div class="flex justify-between items-center mb-4 space-x-2">
                        <label class="block font-medium text-gray-700 whitespace-nowrap">排序标准</label>
                        <select v-model="sortCriteria" class="p-2 w-72 rounded border border-gray-300">
                          <option value="creationTime">创建时间</option>
                          <option value="updateTime">更新时间</option>
                        </select>
                      </div>

                      <div class="flex justify-between items-center mb-4 space-x-2">
                        <label class="block font-medium text-gray-700 whitespace-nowrap">排序方式</label>
                        <select v-model="sortOrder" class="p-2 w-72 rounded border border-gray-300">
                          <option value="asc">正序</option>
                          <option value="desc">倒序</option>
                        </select>
                      </div>

                      <div class="flex justify-center space-x-4">
                        <button @click="resetSort" class="px-4 py-2 text-gray-700 bg-gray-100 rounded">
                          重置
                        </button>
                        <button @click="confirmSort" class="px-4 py-2 text-white bg-blue-500 rounded">
                          确定
                        </button>
                      </div>
                    </div>
                  </PopoverContent>
                </Popover>

                <More>
                  <MoreTrigger
                    class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
                    <MenuIcon class="w-5 h-5" />
                  </MoreTrigger>
                  <MoreContent>
                    <div style="width: 100px; margin: 10px">
                      <div class="flex flex-col space-y-3">
                        <div class="flex justify-between items-center">
                          <span>窗口消息</span>
                        </div>
                        <div class="flex justify-between items-center">
                          <span>操作</span>
                        </div>
                        <div class="flex justify-between items-center">
                          <span>备注</span>
                          <input type="checkbox" class="toggle-checkbox" />
                        </div>
                        <div class="flex justify-between items-center">
                          <span>项目</span>
                          <input type="checkbox" class="toggle-checkbox" />
                        </div>
                        <div class="flex justify-between items-center">
                          <span>创建时间</span>
                          <input type="checkbox" class="toggle-checkbox" checked />
                        </div>
                        <div class="flex justify-between items-center">
                          <span>更新时间</span>
                          <input type="checkbox" class="toggle-checkbox" checked />
                        </div>
                        <div class="flex justify-between items-center">
                          <span>最后打开</span>
                          <input type="checkbox" class="toggle-checkbox" checked />
                        </div>
                        <div class="flex justify-between items-center">
                          <span>其他</span>
                          <input type="checkbox" class="toggle-checkbox" />
                        </div>
                      </div>
                    </div>
                  </MoreContent>
                </More>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center mb-4 space-x-2">
        <button class="flex items-center px-4 py-2 text-sm text-white bg-blue-500 rounded-lg">
          <SquareIcon class="mr-1 w-4 h-4" />
          打开
        </button>

        <TooltipButton title="关闭" @click="() => console.log('abc')"
          class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
          <XIcon class="w-5 h-5" />
        </TooltipButton>

        <TooltipButton title="导入" @click="opendaoruModel"
          class="p-2 rounded border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
          <Import class="w-5 h-5" />
        </TooltipButton>

        <TooltipButton title="导出" @click="opendaochuModel"
          class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
          <ShareIcon class="w-5 h-5" />
        </TooltipButton>

        <TooltipButton title="移动" @click="movedaochuModel"
          class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
          <LogInIcon class="w-5 h-5" />
        </TooltipButton>

        <TooltipButton title="删除" @click="deleteOpenHandle"
          class="p-2 rounded hover:bg-gray-0 border-[1px] border-gray-300 hover:border-[1px] hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
          <Trash2 class="w-5 h-5" />
        </TooltipButton>
      </div>

      <!-- Table -->
      <div class="overflow-auto overflow-x-auto flex-1 min-w-[900px]">
        <table class="w-full" :tableData="tableData" :total="total" :page="page" :pageSize="pageSize"
          @update:page="updatePage" @update:pageSize="updatePageSize">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 w-12 text-left">
                <!-- 表头复选框，控制全选 -->
                <input type="checkbox" class="rounded border-gray-300" v-model="selectAll" @change="toggleAll" />
              </th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">序号</th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">名称</th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">分组</th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">IP</th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">
                账号平台
              </th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">备注</th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">
                创建时间
              </th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">操作</th>
              <th class="px-4 py-3 w-32 text-sm font-medium text-left">更多</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, index) in tableData" :key="index" class="hover:bg-blue-100"
              :class="{ 'bg-blue-50': row.selected }">
              <td class="px-4 py-3 w-12" style="text-align: left">
                <!-- 每行的复选框，绑定当前行的 `selected` 属性 -->
                <input type="checkbox" class="rounded" v-model="row.selected" />
              </td>

              <td class="px-4 py-3 w-32 text-sm" style="text-align: left">
                <div>
                  <span class="text-gray-500">{{ row.id }}</span>
                  <!-- <Tooltip>
                    <TooltipTrigger>
                      <button
                        class="p-1 rounded opacity-0 transition-opacity duration-200 hover:bg-gray-100 hover:opacity-100"
                      >
                        <img
                          src="../../assets/icons/modify.svg"
                          class="w-4 h-4"
                        />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>修改</p>
                    </TooltipContent>
                  </Tooltip> -->
                </div>
              </td>

              <td class="px-4 py-3 w-32 text-sm group" style="text-align: left">
                <div>
                  <span class="text-gray-500">{{ row.name }}</span>
                  <Tooltip>
                    <TooltipTrigger>
                      <button @click="openEditNameModal"
                        class="p-1 rounded opacity-0 transition-opacity duration-200 hover:bg-gray-100 group-hover:opacity-100">
                        <img src="../../assets/icons/modify.svg" class="w-4 h-4" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>修改</p>
                    </TooltipContent>
                  </Tooltip>
                </div>
              </td>

              <td class="px-4 py-3 w-32 text-sm group" style="text-align: left">
                <div>
                  <span class="text-gray-500">{{ row.group }}</span>
                  <Tooltip>
                    <TooltipTrigger>
                      <button @click="openEditGroupModal"
                        class="p-1 rounded opacity-0 transition-opacity duration-200 hover:bg-gray-100 group-hover:opacity-100">
                        <img src="../../assets/icons/modify.svg" class="w-4 h-4" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>修改</p>
                    </TooltipContent>
                  </Tooltip>
                </div>
              </td>

              <td class="px-4 py-3 w-32 text-sm group" style="text-align: left">
                <div class="flex items-center">
                  <img src="../../assets/icons/check.svg" class="mr-1 w-4 h-4" style="width: 20px; height: 20px" />

                  <img src="../../assets/icons/location.svg" class="mr-1 w-4 h-4" />
                  <div class="flex flex-col">
                    <div>
                      {{ row.ip }}
                    </div>
                    <div>
                      {{ row.location }}
                    </div>
                  </div>
                  <Tooltip>
                    <TooltipTrigger>
                      <button @click="openEditProxyModal"
                        class="p-1 rounded opacity-0 transition-opacity duration-200 hover:bg-gray-100 group-hover:opacity-100">
                        <img src="../../assets/icons/modify.svg" class="w-4 h-4" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>修改</p>
                    </TooltipContent>
                  </Tooltip>
                </div>
              </td>

              <td class="px-4 py-3 w-32 group" style="text-align: left">
                <div class="flex">
                  <button class="p-2 rounded hover:bg-gray-100">
                    <Ellipsis class="w-3 h-3" />
                  </button>

                  <span v-if="row.platform" class="px-2 py-1 text-xs">
                    <div>
                      {{ row.platform }}
                      <TooltipButton title="复制账号" @click=""
                        class="hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
                        <Copy style="width: 10px; height: 10px" />
                      </TooltipButton>
                    </div>

                    <div>
                      {{ row.password }}
                      <TooltipButton title="复制密码" @click=""
                        class="hover:border-blue-600 active:bg-blue-50 hover:text-blue-500">
                        <Copy style="width: 10px; height: 10px" />
                      </TooltipButton>
                    </div>
                  </span>

                  <Tooltip>
                    <TooltipTrigger>
                      <button @click="openEditAccountModal"
                        class="p-1 rounded opacity-0 transition-opacity duration-200 hover:bg-gray-100 group-hover:opacity-100">
                        <img src="../../assets/icons/modify.svg" class="w-4 h-4" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>修改</p>
                    </TooltipContent>
                  </Tooltip>
                </div>
              </td>

              <td class="px-4 py-3 w-32 text-sm group" style="text-align: left">
                <div>
                  <span class="text-gray-500">{{ row.remark }}</span>
                  <Tooltip>
                    <TooltipTrigger>
                      <button @click="openEditRemarksModal"
                        class="p-1 rounded opacity-0 transition-opacity duration-200 hover:bg-gray-100 group-hover:opacity-100">
                        <img src="../../assets/icons/modify.svg" class="w-4 h-4" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>修改</p>
                    </TooltipContent>
                  </Tooltip>
                </div>
              </td>

              <td class="px-4 py-3 w-32 text-sm" style="text-align: left">
                <div>
                  <span class="text-gray-500">{{ row.createTime }}</span>
                </div>
              </td>

              <td class="px-4 py-3 w-32" style="text-align: left">
                <div>
                  <button class="">
                    <img src="../../assets/icons/open.svg" class="h-7 w-18" />
                  </button>
                </div>
              </td>

              <td class="px-4 py-3 w-32" style="text-align: left">
                <div>
                  <button v-if="activeTab === 'recommended'" class="p-1 rounded hover:bg-gray-100">
                    <MoreVerticalIcon class="w-4 h-4" />
                  </button>
                </div>

                <More>
                  <MoreTrigger>
                    <Ellipsis v-if="activeTab !== 'recommended'" class="w-6 h-6 cursor-pointer" />
                  </MoreTrigger>
                  <MoreContent>
                    <MoreItem class="cursor-pointer" @click="settingOpenHandle">
                      <Settings2Icon class="w-4 h-4" />详情
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="updateOpenHandle">
                      <SquarePenIcon class="w-4 h-4" />编辑环境
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Wrench class="w-4 h-4" />编辑账号
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Activity class="w-4 h-4" />修改代理
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Cookie class="w-4 h-4" />编辑Cookie
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <User class="w-4 h-4" />授权成员
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Bot class="w-4 h-4" />CookieRobot
                    </MoreItem>

                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Dock class="w-4 h-4" />转移环境
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Paintbrush class="w-4 h-4" />清除缓存
                    </MoreItem>
                    <MoreItem class="cursor-pointer" @click="() => removeSoftwareHandle(app)">
                      <Trash2Icon class="w-4 h-4" />删除环境
                    </MoreItem>
                  </MoreContent>
                </More>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <!-- Pagination -->
      <div class="flex justify-between items-center p-4 mt-auto bg-white">
        <div class="text-sm text-gray-600">共 101 项数据</div>
        <div class="flex items-center space-x-2">
          <button class="p-2 rounded hover:bg-gray-100">
            <ChevronLeftIcon class="w-4 h-4" />
          </button>
          <button class="px-3 py-1 rounded hover:bg-gray-100">1</button>
          <button class="px-3 py-1 rounded hover:bg-gray-100">2</button>
          <button class="px-3 py-1 text-white bg-blue-500 rounded">3</button>
          <button class="px-3 py-1 rounded hover:bg-gray-100">4</button>
          <span>...</span>
          <button class="px-3 py-1 rounded hover:bg-gray-100">20</button>
          <button class="p-2 rounded hover:bg-gray-100">
            <ChevronRightIcon class="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  </div>

  <AlertModel title="删除代理" :open="deleteModel" @close="deleteCloseHandle" @cancel="deleteCloseHandle"
    @submit="deleteCloseHandle">
    <div class="flex flex-col gap-y-4 py-4 text-sm">
      <!-- Content -->
      <div class="p-4">
        <div class="flex justify-between items-center mb-4">
          <p class="text-gray-700">你确定要删除以下环境吗？</p>
          <button @click="isExpanded = !isExpanded" class="flex items-center text-sm text-blue-600 hover:text-blue-700">
            共{{ selectedAgentslen }}个环境
            <ChevronDown class="ml-1 w-4 h-4 transition-transform duration-200"
              :class="{ 'transform rotate-180': isExpanded }" />
          </button>
        </div>

        <div class="mb-6">
          <p class="mb-2 text-sm text-gray-600">环境名称</p>
          <div class="grid gap-2 max-h-[300px] overflow-y-auto" :class="isExpanded ? 'grid-cols-3' : 'grid-cols-3'">
            <div v-for="(agent, index) in displayedAgents" :key="agent.id"
              class="flex justify-between items-center px-3 py-2 text-sm bg-blue-50 rounded-md">
              <span class="text-gray-700">{{ agent.name }}</span>
              <button @click="removeAgent(agent.id)" class="text-gray-400 hover:text-gray-600">
                <XCircle class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AlertModel>

  <daoruModel :open="daoruModelProps.open" :title="daoruModelProps.title"
    @close="() => (daoruModelProps.open = false)" />
  <daochuModel :open="daochuModelProps.open" :title="daochuModelProps.title"
    @close="() => (daochuModelProps.open = false)" />
  <moveModel :open="moveModelProps.open" :title="moveModelProps.title" @close="() => (moveModelProps.open = false)" />
  <EditNameModal :open="editNameModalProps.open" :title="editNameModalProps.title"
    @close="() => (editNameModalProps.open = false)" />
  <EditRemarksModal :open="editRemarksModalProps.open" :title="editRemarksModalProps.title"
    @close="() => (editRemarksModalProps.open = false)" />
  <EditProxyModal :open="editProxyModalProps.open" :title="editProxyModalProps.title"
    @close="() => (editProxyModalProps.open = false)" />
  <EditGroupModal :open="editGroupModalProps.open" :title="editGroupModalProps.title"
    @close="() => (editGroupModalProps.open = false)" />
  <EditAccountModal :open="editAccountModalProps.open" :title="editAccountModalProps.title"
    @close="() => (editAccountModalProps.open = false)" />
</template>

<script setup>
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";

import { computed, reactive, ref, watch } from "vue";
import { onMounted, onBeforeUnmount } from "vue";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  Ellipsis,
  ListIcon,
  LogInIcon,
  MenuIcon,
  MoreVerticalIcon,
  SearchIcon,
  ShareIcon,
  SquareIcon,
  Trash2,
  XIcon,
  Import,
  Settings2Icon,
  SquarePenIcon,
  Trash2Icon,
  Paintbrush,
  Dock,
  Bot,
  User,
  Cookie,
  Activity,
  Wrench,
  SlidersHorizontalIcon,
  Copy,
  ChevronDown,
  XCircle,
} from "lucide-vue-next";
import TooltipButton from "@/components/tooltip-button.vue";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import daoruModel from "./daoruModel.vue";
import daochuModel from "./daochuModel.vue";
import moveModel from "./moveModel.vue";
import EditNameModal from "./EditNameModal.vue";
import EditRemarksModal from "./EditRemarksModal.vue";
import { AlertModel } from "@/components/alert-model";
import EditProxyModal from "./EditProxyModal.vue";
import EditGroupModal from "./EditGroupModal.vue";
import EditAccountModal from "./EditAccountModal.vue";


// 表格数据
const tableData = ref([]);


for (let i = 0; i < 30; i++) {
  const newAgentsData = {
    id: "01",
    name: "跨境电商",
    group: "小米直营店",
    ip: "112.17.242.11",
    location: "CN 中国-香港",
    platform: "",
    remark: "测试",
    createTime: "2024-01-01 10:00:00",
    selected: false,
    password: "123456",
  };
  tableData.value.push(newAgentsData);
}

// 全选复选框的状态
const selectAll = ref(false);

const sortOpen = ref(false);

const openSortWindow = () => {
  sortOpen.value = !sortOpen.value;
};

const resetSort = () => {
  sortOpen.value = false;
};

const confirmSort = () => {
  sortOpen.value = false;
};

// 监听表格数据的 `selected` 属性，更新 `selectAll` 的状态
watch(
  () => tableData.value.map((row) => row.selected),
  (newSelectedValues) => {
    selectAll.value = newSelectedValues.every(Boolean); // 如果所有选项都选中，则全选复选框选中
  },
  { deep: true }
);

// 切换所有行的选择状态
const toggleAll = () => {
  tableData.value.forEach((row) => {
    row.selected = selectAll.value;
  });
};

// 控制悬浮窗显示/隐藏
const showSortWindow = ref(false);

// 排序字段列表
const fields = ref([
  { id: 1, name: "名称" },
  { id: 2, name: "序号" },
  { id: 3, name: "环境ID" },
  { id: 4, name: "账号平台" },
  { id: 5, name: "平台账号" },
  { id: 6, name: "备注" },
]);

// 当前选中的排序字段
const selectedField = ref(null);

// 切换悬浮窗显示状态
const toggleSortWindow = () => {
  showSortWindow.value = !showSortWindow.value;
};

// 选择排序字段
const selectField = (field) => {
  selectedField.value = field;
  console.log(`当前选择的排序字段: ${field.name}`);
  showSortWindow.value = false; // 选择后关闭悬浮窗
};

// 点击其他区域关闭悬浮窗
document.addEventListener("click", (e) => {
  if (!e.target.closest(".relative")) {
    showSortWindow.value = false;
  }
});

// 控制弹窗显示/隐藏
const showDialog = ref(false);

// 排序相关状态
const sortCriteria = ref("creationTime"); // 默认排序标准
const sortOrder = ref("desc"); // 默认排序方式

// 方法
const toggleDialog = () => {
  showDialog.value = !showDialog.value;
};

const reset = () => {
  sortCriteria.value = "creationTime";
  sortOrder.value = "desc";
};

const confirm = () => {
  console.log("排序标准:", sortCriteria.value);
  console.log("排序方式:", sortOrder.value);
  toggleDialog();
};

// 控制悬浮窗显示与隐藏
const isDropdownOpen = ref(false);

// 配置选项的状态
const options = ref({
  windowMessage: false,
  action: false,
  note: false,
  project: false,
  createTime: true,
  updateTime: true,
  lastOpen: true,
  other: false,
});

// 切换悬浮窗显示状态
const toggleDropdown = () => {
  isDropdownOpen.value = !isDropdownOpen.value;
};

// 审核筛选区--------------------------------
const isOpen = ref(false); //审核筛选
const x = ref(0);
const y = ref(0);
const selectFlag = ref(0);
const selectedLabel = ref("待审核");

const status = ref([
  { id: 1, label: "全部" },
  { id: 2, label: "已分组" },
  { id: 3, label: "未分组" },
]);

const openOptions = () => {
  //打开选项
  // isOpen.value = !isOpen.value;
  selectFlag.value = 1; //开启下拉框标志
};

const selectStatus = (label) => {
  selectedLabel.value = label;
};
onMounted(() => {
  document.addEventListener("click", handleDocumentClick);
});
onBeforeUnmount(() => {
  document.removeEventListener("click", handleDocumentClick);
});

function handleDocumentClick(event) {
  x.value = event.clientX;
  y.value = event.clientY;
  if (!isOpen.value && selectFlag.value === 1) {
    isOpen.value = true;
  } else {
    selectFlag.value = 0;
    isOpen.value = false;
  }
}

const deleteModel = ref(false);

const deleteOpenHandle = () => {
  deleteModel.value = true;
};

const deleteCloseHandle = () => {
  deleteModel.value = false;
};

const isExpanded = ref(false);

let selectedAgentslen = ref();
// 计算显示的代理数据，根据是否展开显示不同数量的代理
const displayedAgents = computed(() => {
  const selectedAgents = tableData.value.filter((agent) => agent.selected); // 只获取选中的代理
  selectedAgentslen = selectedAgents.length;
  return isExpanded.value ? selectedAgents : selectedAgents.slice(0, 5); // 展开时显示所有选中的代理，不展开时只显示前 5 个
});

const removeAgent = (id) => {
  const index = tableData.value.findIndex((agent) => agent.id === id);
  if (index !== -1) {
    tableData.value.splice(index, 1); // 从数组中删除指定代理
  }
};

const daoruModelProps = reactive({
  open: false,
  title: "",
});

const daochuModelProps = reactive({
  open: false,
  title: "",
});

const moveModelProps = reactive({
  open: false,
  title: "",
});

const editNameModalProps = reactive({
  open: false,
  title: "",
});

const editRemarksModalProps = reactive({
  open: false,
  title: "",
});

const editProxyModalProps = reactive({
  open: false,
  title: "",
});

const editGroupModalProps = reactive({
  open: false,
  title: "",
});

const editAccountModalProps = reactive({
  open: false,
  title: "",
});

const movedaochuModel = () => {
  moveModelProps.open = true;
};

const opendaochuModel = () => {
  daochuModelProps.open = true;
};

const opendaoruModel = () => {
  daoruModelProps.open = true;
};

const openEditNameModal = () => {
  editNameModalProps.open = true;
  console.log(editNameModalProps.open);
};

const openEditRemarksModal = () => {
  editRemarksModalProps.open = true;
};

const openEditProxyModal = () => {
  editProxyModalProps.open = true;
};

const openEditGroupModal = () => {
  editGroupModalProps.open = true;
};

const openEditAccountModal = () => {
  editAccountModalProps.open = true;
};
</script>

<style scoped>
/* 自定义开关样式 */
.toggle-checkbox {
  appearance: none;
  width: 24px;
  height: 14px;
  background-color: #ccc;
  border-radius: 7px;
  position: relative;
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;
}

.toggle-checkbox:checked {
  background-color: #3b82f6;
}

.toggle-checkbox:before {
  content: "";
  position: absolute;
  top: 1px;
  left: 1px;
  width: 12px;
  height: 12px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.2s ease-in-out;
}

.toggle-checkbox:checked:before {
  transform: translateX(10px);
}
</style>
