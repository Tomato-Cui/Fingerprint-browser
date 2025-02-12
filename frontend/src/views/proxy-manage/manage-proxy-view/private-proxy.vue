<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
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
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  environment_proxies_delete,
  environment_proxies_query,
} from "@/commands/environment-proxy";
import { environment_proxies_batch_delete } from "@/commands/environment-proxy";
import { toast } from "vue-sonner";

import { AlertModel } from "@/components/alert-model";
import { ip_info } from "@/commands";
import TableTheadChoose from "@/views/proxy-manage/table-thead-choose.vue";
import { WrapperIcon } from "@/assets/icons/environment/index";

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
  console.log("modifyProxyNew");
  modifyProxy.value = true;
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
    privateproxyloadData();
  }
};

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

        <div>
          <h1
            class="ml-2 font-thin text-[14px] mb-1 leading-[18px] text-gray-300"
          >
            已选择{{ proxyList.filter((p) => p.selected).length }}项
          </h1>
        </div>

        <div
          class="flex overflow-x-auto flex-col mx-2 h-full rounded-lg border border-gray-200 grow"
        >
          <table class="min-w-full border border-gray-200">
            <!-- Table Header -->
            <thead class="sticky top-0 z-10 bg-gray-50 border border-gray-200">
              <tr>
                <th scope="col" class="py-3.5 pr-3 pl-4 w-12 text-left">
                  <input
                    type="checkbox"
                    class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                    v-model="selectAll"
                    @change="toggleSelectAll"
                  />
                </th>

                <template v-for="column in visibleColumns" :key="column.key">
                  <th
                    v-if="column.key === 'proxy_number'"
                    scope="col"
                    class="py-3.5 pr-3 pl-4 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    代理序号
                  </th>

                  <th
                    v-if="column.key === 'name'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    代理名称
                  </th>

                  <th
                    v-if="column.key === 'proxy_affiliation'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    代理归属
                  </th>

                  <th
                    v-if="column.key === 'operation'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[205px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    操作
                  </th>

                  <th
                    v-if="column.key === 'proxy_status'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    代理状态
                  </th>
                  <th
                    v-if="column.key === 'ip_query'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[110px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    IP查询渠道
                  </th>
                  <th
                    v-if="column.key === 'group'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    分组
                  </th>
                  <th
                    v-if="column.key === 'proxy_type'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[110px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    浏览器环境
                  </th>
                  <th
                    v-if="column.key === 'cloud_phone_env'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[110px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    云手机环境
                  </th>
                  <th
                    v-if="column.key === 'creation_info'"
                    scope="col"
                    class="px-3 py-3.5 text-sm font-semibold text-left text-gray-900 min-w-[100px]"
                  >
                    <span class="font-thin text-gray-300">|&nbsp;</span>
                    创建信息
                  </th>
                </template>
                <th
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
                </th>
              </tr>
            </thead>

            <!-- Table Body -->

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

                    <!-- @click="copyProxyId(Proxymanage)" -->
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
</template>
