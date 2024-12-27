<script setup lang="ts">
import { environment_proxies_create } from "@/commands/environment-proxy";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
// import { pageId } from "../mydaili/mydaili.vue";

const activeTab = ref("single");
const router = useRouter();
const route = useRoute();
const proxyType = ref("IPV4");
const proxyInfo = ref("");
const host = ref("");
const port = ref("");
const username = ref("");
const password = ref("");
const refreshUrl = ref("");

const singleForm = ref({
  url: "",
  proxyType: "IPv4",
  proxyService: "SOCKS5",
  host: "",
  port: "",
  username: "",
});

interface Proxy {
  id?: number;
  kind: string;
  host: string;
  port: string;
  username?: string;
  password?: string;
  user_uuid?: string;
  environment_group_id?: number;
}

// 跳转到新增代理页面
const SaveOne = () => {
  const proxy: Proxy = {
    id: 0,
    kind: proxyType.value,
    host: host.value,
    port: port.value,
    user_uuid: "1231",
    username: username.value,
    password: password.value,
  };
  environment_proxies_create(proxy);

  // router.push("/mydaili?id=" + proxyId.value);
  router.go(-1);
};

// 跳转到新增代理页面
const mydaili = () => {
  router.push("/mydaili");
};

const batchForm = ref({
  service: "IPRust.io",
  autoRefresh: false,
});

const handleSubmit = () => {
  // Handle form submission
};

const handleCancel = () => {
  // Reset form
  proxyType.value = "IPV4";
  proxyInfo.value = "SOCKS5";
  host.value = "";
  port.value = "";
  username.value = "";
  password.value = "";
  refreshUrl.value = "";
};
</script>

<template>
  <div class="p-4 h-main">
    <div class="p-6 bg-white rounded-lg shadow" style="height: 100%">
      <!-- Tabs -->
      <div class="mb-6 border-b border-gray-200">
        <nav class="flex -mb-px">
          <button
            @click="activeTab = 'single'"
            :class="[
              'py-4 px-6 font-medium text-sm',
              activeTab === 'single'
                ? 'border-b-2 border-blue-500 text-blue-600'
                : 'text-gray-500 hover:text-gray-700',
            ]"
          >
            单个添加
          </button>
          <button
            @click="activeTab = 'batch'"
            :class="[
              'py-4 px-6 font-medium text-sm',
              activeTab === 'batch'
                ? 'border-b-2 border-blue-500 text-blue-600'
                : 'text-gray-500 hover:text-gray-700',
            ]"
          >
            批量添加
          </button>
        </nav>
      </div>

      <!-- Single Add Form -->
      <div
        v-if="activeTab === 'single'"
        class="mx-auto space-y-8"
        style="max-width: 800px"
      >
        <div>
          <label class="block mb-2 text-sm font-medium text-gray-700"
            >智能识别</label
          >
          <textarea
            v-model="singleForm.url"
            rows="4"
            class="p-3 w-full text-sm rounded-md border border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
            placeholder="[代理主机端口][IP][代理类型][用户名][密码][URL]"
          ></textarea>
        </div>

        <!-- Proxy Type -->
        <div class="mb-4">
          <div class="flex items-center" style="min-height: 50px">
            <label class="w-24 text-sm text-gray-600">代理类型</label>
            <div class="flex space-x-4">
              <label
                class="inline-flex items-center p-2 rounded-md border border-gray-300 hover:bg-blue-100"
                style="width: 100px"
              >
                <input
                  type="radio"
                  v-model="proxyType"
                  value="IPV4"
                  class="w-4 h-4 text-blue-600 border-gray-300 focus:ring-blue-500"
                />
                <span class="ml-2 text-sm text-gray-700">IPV4</span>
              </label>

              <label
                class="inline-flex items-center p-2 rounded-md border border-gray-300 hover:bg-blue-100"
                style="width: 100px"
              >
                <input
                  type="radio"
                  v-model="proxyType"
                  value="IPV6"
                  class="w-4 h-4 text-blue-600 border-gray-300 focus:ring-blue-500"
                />
                <span class="ml-2 text-sm text-gray-700">IPV6</span>
              </label>
            </div>
          </div>
        </div>

        <!-- Proxy Information -->
        <div class="mb-4">
          <div class="flex items-center" style="min-height: 50px">
            <label class="w-24 text-sm text-gray-600">代理信息</label>
            <div class="flex flex-1 space-x-2">
              <Select>
                <SelectTrigger>
                  <SelectValue
                    v-model="proxyInfo"
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
                type="text"
                v-model="host"
                placeholder="主机"
                class="flex-1 p-2 rounded-md border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
              />
              <span class="flex items-center text-gray-500">：</span>
              <input
                type="text"
                v-model="port"
                placeholder="端口"
                class="p-2 w-24 rounded-md border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
          </div>
        </div>

        <!-- Username -->
        <div class="mb-4">
          <div class="flex items-center" style="min-height: 50px">
            <label class="w-24 text-sm text-gray-600">用户名</label>
            <input
              type="text"
              v-model="username"
              placeholder="请输入用户名"
              class="flex-1 p-2 rounded-md border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>

        <!-- Password -->
        <div class="mb-4">
          <div class="flex items-center" style="min-height: 50px">
            <label class="w-24 text-sm text-gray-600">账号密码</label>
            <input
              type="password"
              v-model="password"
              placeholder="请输入密码"
              class="flex-1 p-2 rounded-md border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>

        <!-- Refresh URL -->
        <div class="mb-6">
          <div class="flex items-center" style="min-height: 50px">
            <label class="w-24 text-sm text-gray-600">刷新URL</label>
            <div class="flex flex-1 items-center space-x-2">
              <input
                type="url"
                v-model="refreshUrl"
                placeholder="请输入URL"
                class="flex-1 p-2 rounded-md border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
              />
              <button
                @click="refreshUrl = ''"
                class="p-2 text-gray-400 hover:text-gray-600 focus:outline-none"
              >
                <RefreshCw class="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div
          class="flex justify-center pt-6 space-x-4 border-t"
          style="min-height: 50px"
        >
          <button
            @click="SaveOne"
            class="px-8 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700"
          >
            确定
          </button>
          <button
            @click="mydaili"
            class="px-8 py-2 text-gray-600 hover:text-gray-800 focus:outline-none"
          >
            取消
          </button>
        </div>
      </div>

      <!-- Batch Add Form -->
      <div v-if="activeTab === 'batch'" class="space-y-6">
        <div class="grid grid-cols-2 gap-6">
          <div class="space-y-4">
            <h3 class="text-sm font-medium text-gray-700">代理服务器</h3>

            <textarea
              placeholder="请输入代理信息"
              class="flex-1 border-[1px] border-gray-300 rounded-md shadow-sm p-2 text-sm h-96 overflow-y-auto resize-none focus:border-blue-400"
              style="width: 100%; border-width: 1px; outline: none"
            ></textarea>

            <div class="flex justify-between items-center space-x-2">
              <div class="flex items-center space-x-2" style="min-width: 300px">
                <!-- 下拉框 -->
                <Select>
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
                <!-- 自动去重 checkbox -->
                <label class="flex items-center" style="min-width: 100px">
                  <input
                    type="checkbox"
                    v-model="batchForm.autoRefresh"
                    class="w-4 h-4 text-blue-600"
                  />
                  <span class="ml-2 text-sm">自动去重</span>
                </label>
              </div>

              <!-- 按钮右对齐 -->
              <button
                class="px-4 py-2 ml-auto text-sm text-white bg-blue-500 rounded-md hover:bg-blue-600"
                style="height: 40px; width: 80px"
              >
                解析
              </button>
            </div>
          </div>

          <div class="p-4 bg-gray-50 rounded-md">
            <h4 class="mb-2 text-sm font-medium text-gray-700">写入规则</h4>
            <div class="space-y-1 text-xs text-gray-400">
              <p>支持 IPv4, IPv6</p>
              <p>支持 HTTP, HTTPS, SOCKS5 三种类型代理</p>
              <p>每一行一个代理，必须完整添加</p>
              <p>检测IP代理时间，测试自动添加IP，主机端口必须填写对应位置</p>
            </div>
          </div>
        </div>

        <div class="flex justify-between">
          <div class="text-sm text-gray-500">已解析代理</div>
          <div class="space-x-2">
            <button
              class="p-2 text-sm text-gray-500 rounded-md border border-gray-300 hover:text-blue-500 hover:border-blue-500 active:bg-blue-50"
            >
              删除失败
            </button>
            <button
              class="p-2 text-sm text-gray-500 rounded-md border border-gray-300 hover:text-blue-500 hover:border-blue-500 active:bg-blue-50"
            >
              测试全部
            </button>
          </div>
        </div>

        <div @click="mydaili" class="flex">
          <button
            class="px-4 py-2 ml-auto text-sm text-white bg-blue-500 rounded-md hover:bg-blue-600"
          >
            保存
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
