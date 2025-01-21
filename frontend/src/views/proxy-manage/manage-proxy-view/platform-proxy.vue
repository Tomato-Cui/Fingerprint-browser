<script setup lang="ts">
import { ref } from "vue";
import Layout from "@/views/proxy-manage/manage-proxyLayout.vue";
import { AddCheckWhite } from "@/assets/icons/proxy-manage-image/index";

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
</script>

<template>
  <Layout>
    <template v-slot:manage-proxy-content>
      <div class="flex-auto p-6 w-full bg-white rounded-lg grow">
        <div class="flex gap-6">
          <!-- Header Controls -->

          <div class="flex gap-4 justify-start flex-col w-[60%] min-w-[700px]">
            <div class="flex flex-row gap-4">
              <div class="flex flex-row gap-4 w-[50%]">
                <div
                  class="flex font-semibold text-[14px] w-[20%] min-w-[100px] justify-start leading-[22px] items-center"
                >
                  <label for="protocolType">协议类型</label>
                </div>
                <div class="min-w-[200px] flex justify-start">
                  <select
                    v-model="protocolType"
                    class="px-3 w-full h-10 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="auto">自动识别</option>
                    <option value="http">HTTP</option>
                    <option value="socks5">SOCKS5</option>
                  </select>
                </div>
              </div>

              <div class="flex flex-row gap-4 w-[50%]">
                <div
                  class="flex font-semibold text-[14px] w-[20%] min-w-[100px] leading-[22px] justify-start items-center"
                >
                  <label for="protocolType">IP查询渠道</label>
                </div>

                <div class="min-w-[200px] flex justify-start">
                  <select
                    v-model="ipQueryChannel"
                    class="px-3 w-full h-10 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="IP2Location">IP2Location</option>
                  </select>
                  <span class="absolute right-0 -top-1.5 text-sm text-red-500"
                    >*</span
                  >
                </div>
              </div>
            </div>

            <!-- Example and Info Section -->

            <!-- Example Section -->
            <div class="flex-1">
              <div class="w-full">
                <Textarea
                  v-model="text"
                  placeholder="示例:&#10;192.168.0.1:8000&#10;192.168.0.1:8000:代理账号:代理密码{分组}socks5://192.168.0.1:8000[刷新URL]{分组}HTTP://192.168.0.1:8000:代理账号:代理密码[刷新URL]{分组}HTTP://[2001:db8:2de:0:0:0:0:e13]:8000:代理账号:代理密码[刷新URL]{分组HTTP://[2001:db8:2de:0:0:0:0:e13]:8000[刷新URL]{分组}"
                  class="font-sans font-normal text-[14px] placeholder:text-[12px] leading-[22px] flex-1 px-4 py-1 w-full text-lg rounded-lg border border-gray-300 h-[150px]"
                />
              </div>
            </div>
          </div>

          <!-- Info Box -->
          <div
            class="p-4 bg-blue-50 rounded-lg w-[40%] text-xs h-[200px] overflow-auto"
          >
            <div class="font-xs">[IPv4格式]</div>
            <div
              class="space-y-1 text-sm text-gray-600 text-[12px] leading-[16px] font-normal"
            >
              <div>
                代理类型://代理host:代理端口:代理账号:代理密码[刷新URL][分组]
              </div>
              <div>
                代理类型://代理账号:代理密码@代理host:代理端口[刷新URL][分组]
              </div>
            </div>

            <div class="font-xs">[IPv6格式]</div>
            <div class="text-sm text-gray-600">
              <div>
                代理类型://[代理host]:代理端口:代理账号:代理密码[刷新URL][分组]
              </div>
              <div>
                代理类型://代理账号:代理密码@[代理host]:代理端口[刷新URL][分组]
              </div>
            </div>

            <div class="text-sm text-gray-600">
              <div>1. 「代理Host」、「代理端口」要求必填</div>
              <div>2. 不填写代理类型时，将默认为SOCKS5类型</div>
              <div>3. 仅支持HTTP、HTTPS、SSH、SOCKS5等协议代理类型</div>
              <div>4. 每一行一个代理，一次最多添加100个代理</div>
              <div>
                5.
                代理信息中的字段不允许有"@"或":"符号，若符致解析异常，将存在这类数据，请使用【添加代理-单个添加】功能进行添加
              </div>
              <div>6. 添加时重复的代理将自动忽略</div>
            </div>
          </div>
        </div>

        <!-- Added Proxies Section -->
        <div class="flex items-center mb-4">
          <div class="text-sm text-gray-500">已添加代理 1 项</div>

          <div class="flex justify-start ml-3 text-white">
            <button
              class="flex justify-start bg-[#4F46E5] mr-3 rounded-md hover:bg-[#4338CA] text-sm border px-2 py-1.5 items-center font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 text-white outline-gray-50 hover:outline-gray-100"
            >
              <AddCheckWhite
                class="w-[14px] h-[14px] m-1 fill-current text-white"
              />
              <span class="text-white"> 批量检测代理</span>
            </button>
          </div>
        </div>

        <!-- Table -->
        <div class="w-full rounded-lg border">
          <table class="w-full">
            <thead class="bg-gray-50 border-b">
              <tr>
                <th class="p-4 w-10">
                  <input
                    type="checkbox"
                    class="text-blue-600 rounded border-gray-300 focus:ring-blue-500"
                  />
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  代理类型
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  代理Host
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  代理端口
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  代理账号
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  代理密码
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  刷新URL
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  分组
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  代理名称
                </th>
                <th class="p-4 text-sm font-medium text-left min-w-[90px]">
                  检测结果
                </th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-if="proxies.length === 0"
                v-for="proxy in proxyItems"
                :key="proxy.id"
              >
                <td class="py-4 pr-3 pl-4 whitespace-nowrap">
                  <input
                    type="checkbox"
                    class="w-4 h-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-600"
                    v-model="proxy.selected"
                  />
                </td>
                <td class="py-4 pr-3 pl-4 whitespace-nowrap">
                  {{ proxy.name }}
                </td>
                <td class="py-4 pr-3 pl-4 whitespace-nowrap">
                  {{ proxy.url }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Footer Actions -->
      <div class="flex gap-3 justify-end pt-3 mb-3 border-t border-gray-300">
        <button
          class="px-6 py-2 rounded-lg border transition-colors hover:bg-gray-50"
        >
          取消
        </button>
        <button
          class="px-6 py-2 text-white bg-blue-600 rounded-lg transition-colors hover:bg-blue-700"
        >
          确认
        </button>
      </div>
    </template>
  </Layout>
</template>
