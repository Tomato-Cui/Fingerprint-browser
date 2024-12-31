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
import { ref, watch } from "vue";
import { useRouter, useRoute } from "vue-router";

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
const agents = ref(""); // 输入代理信息的字符串
const proxies = ref<Proxy[]>([]); // 存储解析后的代理对象数组
const batchForm = ref({
  service: "IPRust.io",
  autoRefresh: false,
});

const parseAgents = () => {
  console.log(agents.value);
  proxies.value = (agents.value as any)
    .split("\n") // 按行分割
    .filter((line: any) => line.trim() !== "") // 去除空行
    .map((line: any) => {
      const parts = line.split(":"); // 使用 ":" 分隔
      if (parts.length < 5) {
        console.warn("代理信息格式错误:", line);
        return null;
      }

      const [kind, host, port, username, password] = parts;

      if (!kind || !host || !port || !username || !password) {
        console.warn("代理信息缺少字段:", line);
        return null;
      }

      return {
        kind,
        host,
        port,
        username,
        password,
      } as Proxy;
    })
    .filter((proxy: any, index: any, self: any) => {
      if (!proxy) return false; // 过滤掉解析失败的行

      if (batchForm.value.autoRefresh == true) {
        // 如果 autoRefresh 为 true，则去重
        const isDuplicate =
          self.findIndex(
            (p: any) => p && p.host === proxy.host && p.port === proxy.port
          ) !== index;
        return !isDuplicate; // 仅保留第一个出现的代理
      }

      return true; // 如果 autoRefresh 为 false，不进行去重
    });

  console.log(proxies.value);
};

const addProxies = async () => {
  try {
    const results = await Promise.all(
      proxies.value.map((proxy) => environment_proxies_create(proxy))
    );
    console.log("所有代理添加成功:", results);
  } catch (error) {
    console.warn("批量添加代理失败:", error);
  }
};

// 用户输入的原始代理信息
const rawInput = ref("");
// 存储解析后的代理信息列表
const singleForm = ref([]);

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
// 监听 rawInput 的变化并解析
watch(
  () => rawInput.value,
  (newInput) => {
    const parseAgents = (input: any) => {
      return input
        .split("\n") // 按行分割
        .filter((line: any) => line.trim() !== "") // 过滤空行
        .map((line: any) => {
          const parts = line.split(":"); // 使用 ":" 分隔
          if (parts.length !== 5) {
            console.warn("代理信息格式错误:", line);
            return null; // 跳过格式错误的行
          }

          const [kind, host, port, username, password] = parts;
          return { kind, host, port, username, password };
        })
        .filter((proxy: any) => proxy !== null); // 过滤掉解析失败的行
    };

    // 更新 singleForm
    singleForm.value = parseAgents(newInput);

    // 如果有解析成功的代理信息，取第一条赋值到详细字段
    if (singleForm.value.length > 0) {
      const firstProxy = singleForm.value[0] as any; // 取第一条代理
      host.value = firstProxy.host;
      port.value = firstProxy.port;
      username.value = firstProxy.username;
      password.value = firstProxy.password;
    } else {
      // 没有代理信息时，清空详细字段
      host.value = "";
      port.value = "";
      username.value = "";
      password.value = "";
    }
  }
);

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
  // router.go(-1);
};

// 跳转到新增代理页面
const mydaili = () => {
  addProxies();
};
</script>

<template>
  <div class="p-4 h-main">
    <div class="p-6 bg-white rounded-lg shadow h-main">
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
            v-model="rawInput"
            rows="4"
            class="p-3 w-full text-sm rounded-md border border-gray-300 shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
            placeholder="类型:IP:代理主机端口:用户名:密码"
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
      </div>

      <!-- Batch Add Form -->
      <div v-if="activeTab === 'batch'" class="space-y-6">
        <div class="grid grid-cols-2 gap-6">
          <div class="space-y-4">
            <h3 class="text-sm font-medium text-gray-700">代理服务器</h3>

            <textarea
              v-model="agents"
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
                @click="parseAgents"
              >
                解析
              </button>
            </div>
          </div>

          <div class="p-4 bg-gray-50 rounded-md">
            <h4
              v-if="proxies.length === 0"
              class="mb-2 text-sm font-medium text-gray-700"
            >
              写入规则
            </h4>
            <h4
              v-if="proxies.length"
              class="mb-2 text-sm font-medium text-gray-700"
            >
              解析代理
            </h4>
            <!-- 初始提示信息 -->
            <div
              v-if="proxies.length === 0"
              class="space-y-1 text-xs text-gray-400"
            >
              <p>支持 IPv4, IPv6</p>
              <p>支持 HTTP, HTTPS, SOCKS5 三种类型代理</p>
              <p>每一行一个代理，必须完整添加</p>
              <p>检测IP代理时间，测试自动添加IP，主机端口必须填写对应位置</p>
              <p>以下是例子</p>
              <p>Socks5:36.26.81.57:51800:xmdlyuje:qrh8xi7y</p>
            </div>

            <!-- 代理信息显示区域 -->
            <div v-else class="space-y-1 text-xs text-gray-400">
              <p v-for="(proxy, index) in proxies" :key="index">
                {{ proxy.kind }}:{{ proxy.host }}:{{ proxy.port }}:{{
                  proxy.username
                }}:{{ proxy.password }}
              </p>
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
