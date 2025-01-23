<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import Layout from "@/views/proxy-manage/manage-proxyLayout.vue";
import { AddCheckWhite } from "@/assets/icons/proxy-manage-image/index";

import SetField from "./setting.vue";
import { environment_group_query } from "@/commands/environment-group";

import { Circle, Filter } from "@/assets/icons/environment-group-manage";
import {
  SearchIcon,
  ChevronRightIcon,
  ChevronLeftIcon,
} from "@/assets/icons/environment-bookmark-image";

import { RefreshCw, GripVertical } from "lucide-vue-next";
import { PrimaryButton, CancelButton } from "@/components/button";

import BuyProxy from "./manage-proxy-view/buy-proxy.vue";

import {
  FileText,
  PenModify,
  Plate,
  Round,
} from "@/assets/icons/proxy-manage-image";

import {
  AddCheck,
  AddProxy,
  Buy,
  Setting,
} from "@/assets/icons/proxy-manage-image";

import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { environment_proxies_query } from "@/commands/environment-proxy";
import { setCheckRow, getCheckRow } from "../proxy-operation-store";
import { checkRows } from "../proxy-operation-store";
import { environment_proxies_batch_delete } from "@/commands/environment-proxy";

interface Proxy {
  id: string;
  type: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
  refreshUrl?: string;
  group?: string;
  name?: string;
  status?: string;
}

const protocolType = ref("auto");
const ipQueryChannel = ref("IP2Location");
const proxies = ref<Proxy[]>([]);

const text = ref("");

interface ProxyItem {
  id: number;
  name: string;
  url: string;
  selected: boolean;
}

const proxyItems = ref<ProxyItem[]>([
  {
    id: 1,
    name: "feishu",
    url: "https://gcn1b5cn2pro",
    selected: false,
  },
  {
    id: 2,
    name: "feishu2",
    url: "https://gcn1b5cn2pro",
    selected: false,
  },
  {
    id: 3,
    name: "feishu3",
    url: "https://gcn1b5cn2pro",
    selected: false,
  },
  {
    id: 4,
    name: "feishu4",
    url: "https://gcn1b5cn2pro",
    selected: false,
  },
]);

const setField = ref(false);

const createProxymanage = ref(false);

const selectAll = ref(false);

const proxyList = ref<Proxymanage[]>([]);

watch(proxyList, (newVal) => {
  setCheckRow(newVal.map((item) => item.id));
});

const toggleSelectAll = () => {
  proxyList.value.forEach((proxyList) => {
    proxyList.selected = selectAll.value;
  });
};

const editProxymanage = (Proxymanage: Proxymanage) => {
  console.log("Edit Proxymanage:", Proxymanage);
};

const deleteProxymanage = (Proxymanage: Proxymanage) => {
  console.log("Delete Proxymanage:", Proxymanage);
};

const searchQuery = ref("");

interface Proxymanage {
  id: number;
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
  }
};

const platformproxyloadData = () => {
  environment_proxies_query(1, 1000).then((res) => {
    let { data: data_, total } = res.data;
    totalItems.value = total;
    proxyList.value = data_;
  });
};

onMounted(() => platformproxyloadData());

const handleDeleteProxy = async (rows: any[]) => {
  console.log("rows", [...Object.values(rows)]);
  console.log("checkRows", checkRows.value);

  // Ensure checkRows.value exists and is an array
  if (!checkRows.value || !Array.isArray(checkRows.value)) {
    console.error("No valid rows selected for deletion");
    return;
  }

  try {
    await environment_proxies_batch_delete(checkRows.value);
  } catch (error) {
    console.error("Failed to delete proxies:", error);
  }
  selectAll.value = false;
  platformproxyloadData();
};

interface Field {
  id: string;
  label: string;
  enabled: boolean;
}
const fields = ref<Field[]>([
  { id: "proxy_number", label: "代理序号", enabled: true },
  { id: "proxy_name", label: "代理名称", enabled: true },
  { id: "proxy_affiliation", label: "代理归属", enabled: true },
  { id: "operation", label: "操作", enabled: true },
  { id: "proxy_status", label: "代理状态", enabled: true },
  { id: "ip_query", label: "IP查询渠道", enabled: true },
  { id: "group", label: "分组", enabled: false },
  { id: "proxy_type", label: "代理类型", enabled: false },
  { id: "proxy_host", label: "代理主机", enabled: false },
  { id: "browser_env", label: "浏览器环境", enabled: false },
  { id: "cloud_phone_env", label: "云手机环境", enabled: false },
  { id: "creation_info", label: "创建信息", enabled: true },
]);

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
};
</script>

<template>
  <Layout>
    <template v-slot:manage-proxy-content>
      <div class="flex flex-col w-full h-full">
        <div class="flex flex-row gap-x-2 m-2">
          <button
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
            class="text-sm border rounded-md px-2 py-1.5 bg-[#F5F5FF] flex items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
            @click="handleDeleteProxy(getCheckRow())"
          >
            <Round
              class="w-[20px] h-[20px] text-gray-400 items-center justify-center mx-1"
            />
            <span class="font-sans font-semibold text-center text-black">
              删除代理
            </span>
          </button>
        </div>

        <div>
          <h1
            class="ml-2 font-thin text-[14px] mb-1 leading-[18px] text-gray-300"
          >
            已选择1项
          </h1>
        </div>

        <div
          class="flex overflow-x-auto flex-col mx-2 h-full rounded-lg border border-gray-200 grow"
        >
          <table class="min-w-full border border-gray-200">
            <!-- Table Header -->
            <thead class="bg-gray-50 border border-gray-200">
              <tr>
                <th scope="col" class="py-3.5 pr-3 pl-4 w-12 text-left">
                  <input
                    type="checkbox"
                    class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                    v-model="selectAll"
                    @change="toggleSelectAll"
                  />
                </th>
                <th
                  scope="col"
                  class="py-3.5 pr-3 pl-4 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理序号
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理名称
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理归属
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[205px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  操作
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理状态
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[110px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  IP查询渠道
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  分组
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[110px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  浏览器环境
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[110px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  云手机环境
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  创建信息
                </th>
                <th
                  scope="col"
                  class="flex justify-end items-center px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[60px]"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>

                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Setting
                        class="w-[16px] h-[16px] text-gray-400 mx-1 cursor-pointer"
                      />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" class="w-72">
                      <div
                        class="flex justify-between items-center p-4 border-b"
                      >
                        <h2 class="text-lg font-medium">自定义表格字段</h2>
                        <button
                          class="p-2 rounded-full transition-colors hover:bg-gray-100"
                        >
                          <RefreshCw class="w-5 h-5 text-gray-600" />
                        </button>
                      </div>

                      <div class="p-4 max-h-[70vh]">
                        <div class="space-y-3">
                          <div
                            v-for="field in fields"
                            :key="field.id"
                            class="flex justify-between items-center p-3 bg-gray-50 rounded-lg"
                          >
                            <div class="flex gap-3 items-center">
                              <GripVertical class="w-4 h-4 text-gray-400" />
                              <span class="text-gray-700">{{
                                field.label
                              }}</span>
                            </div>
                            <label
                              class="inline-flex relative items-center cursor-pointer"
                            >
                              <input
                                type="checkbox"
                                v-model="field.enabled"
                                class="sr-only peer"
                              />
                              <div
                                class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"
                              ></div>
                            </label>
                          </div>
                        </div>
                      </div>

                      <div class="flex justify-end border-t border-gray-300">
                        <div class="flex gap-x-4 justify-end px-2 pt-3 mb-2">
                          <CancelButton
                            class="px-4 border border-gray-300"
                            @click=""
                          >
                            取消
                          </CancelButton>
                          <PrimaryButton class="px-4 !bg-[#5050FA]" @click="">
                            确定
                          </PrimaryButton>
                        </div>
                      </div>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </th>
              </tr>
            </thead>

            <!-- Table Body -->
            <tbody class="bg-white divide-y divide-gray-200">
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
                  {{ Proxymanage.name }}
                </td>

                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.attribution }}
                </td>

                <td class="px-3 py-4 text-sm whitespace-nowrap">
                  <div class="flex gap-2 justify-start items-center">
                    <button
                      @click="editProxymanage(Proxymanage)"
                      class="text-[#4F46E5] hover:bg-indigo-50 px-2 rounded border border-[#5050FA] bg-[#F0F5FF]"
                    >
                      编辑
                    </button>

                    <button
                      @click="deleteProxymanage(Proxymanage)"
                      class="px-2 text-[#FA8C16] rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFF7E6]"
                    >
                      复制代理
                    </button>

                    <button
                      @click="deleteProxymanage(Proxymanage)"
                      class="px-2 text-red-600 rounded hover:bg-red-50 border border-[#ED003F] bg-[#FFE2E2]"
                    >
                      删除
                    </button>
                  </div>
                </td>

                <td class="px-3 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {{ Proxymanage.state }}
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
    </template>
  </Layout>
</template>
