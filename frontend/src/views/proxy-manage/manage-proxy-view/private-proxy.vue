<script setup lang="ts">
import { onMounted, ref } from "vue";
import Layout from "@/views/proxy-manage/manage-proxyLayout.vue";


import {
  ChevronLeftIcon,
} from "@/assets/icons/environment-bookmark-image";

import { RefreshCw, GripVertical } from "lucide-vue-next";
import { PrimaryButton, CancelButton } from "@/components/button";


import {
  Setting,
} from "@/assets/icons/proxy-manage-image";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";






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

const loadData = () => {
  // environment_group_query(1, 1000).then((res) => {
  //   let { data: data_, total } = res.data;
  //   totalItems.value = total;
  //   Proxymanage.value = data_;
  // });
};

onMounted(() => loadData());

const selectAll = ref(false);

const Proxymanage = ref<Proxymanage[]>([]);

const toggleSelectAll = () => {
  Proxymanage.value.forEach((Proxymanage) => {
    Proxymanage.selected = selectAll.value;
  });
};

const editProxymanage = (Proxymanage: Proxymanage) => {
  console.log("Edit Proxymanage:", Proxymanage);
};

const deleteProxymanage = (Proxymanage: Proxymanage) => {
  console.log("Delete Proxymanage:", Proxymanage);
};


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
      <div
        class="flex overflow-x-auto flex-col mt-2 max-w-full h-full bg-yellow-400 border border-blue-500"
      >
        <div class="min-w-max rounded-lg border border-gray-200">
          <table class="min-w-full">
            <!-- Table Header -->
            <thead class="bg-gray-50">
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
                  class="py-3.5 pr-3 pl-4 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理序号
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理名称
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理归属
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  操作
                </th>

                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  代理状态
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  IP查询渠道
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  分组
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  浏览器环境
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  云手机环境
                </th>
                <th
                  scope="col"
                  class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  创建信息
                </th>
                <th
                  scope="col"
                  class="flex justify-end px-3 py-3.5 text-sm font-semibold text-left text-gray-900"
                >
                  <span class="font-thin text-gray-300">|&nbsp;</span>
                  <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                      <Setting
                        class="w-[16px] h-[16px] text-gray-400 items-center justify-center mx-1 cursor-pointer"
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

                      <div class="p-4 max-h-[70vh] overflow-y-auto">
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
                v-for="Proxymanage in Proxymanage"
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
      </div>

      <!-- pagination -->
      <div class="flex justify-end items-center pt-3 m-2 border border-red-500">
        <span class="text-sm text-gray-500">共 {{ totalItems }} 项</span>
        <div class="flex gap-2 items-center">
          <button
            class="justify-center items-center p-1 rounded border-gray-300 disabled:opacity-50"
            :disabled="currentPage === 1"
            @click="prevPage"
          >
            <ChevronLeftIcon class="w-[12px] h-[12px]" />
          </button>
          <span class="min-w-[2rem] text-center">{{ currentPage }}</span>
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
    </template>
  </Layout>
</template>
