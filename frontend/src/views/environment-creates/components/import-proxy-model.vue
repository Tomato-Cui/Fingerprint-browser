<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
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

const props = defineProps({
  open: false,
});
const emits = defineEmits(["close"]);

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
const proxies = ref([]); // 存储解析后的代理对象数组
const batchForm = ref({
  service: "IPRust.io",
  autoRefresh: false,
});

const parseAgents = () => {
  console.log(agents.value);
  proxies.value = agents.value
    .split("\n") // 按行分割
    .filter((line) => line.trim() !== "") // 去除空行
    .map((line) => {
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
      };
    })
    .filter((proxy, index, self) => {
      if (!proxy) return false; // 过滤掉解析失败的行

      if (batchForm.value.autoRefresh == true) {
        // 如果 autoRefresh 为 true，则去重
        const isDuplicate =
          self.findIndex(
            (p) => p && p.host === proxy.host && p.port === proxy.port
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

// 监听 rawInput 的变化并解析
watch(
  () => rawInput.value,
  (newInput) => {
    const parseAgents = (input) => {
      return input
        .split("\n") // 按行分割
        .filter((line) => line.trim() !== "") // 过滤空行
        .map((line) => {
          const parts = line.split(":"); // 使用 ":" 分隔
          if (parts.length !== 5) {
            console.warn("代理信息格式错误:", line);
            return null; // 跳过格式错误的行
          }

          const [kind, host, port, username, password] = parts;
          return { kind, host, port, username, password };
        })
        .filter((proxy) => proxy !== null); // 过滤掉解析失败的行
    };

    // 更新 singleForm
    singleForm.value = parseAgents(newInput);

    // 如果有解析成功的代理信息，取第一条赋值到详细字段
    if (singleForm.value.length > 0) {
      const firstProxy = singleForm.value[0];
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
  const proxy = {
    id: 0,
    kind: proxyType.value,
    host: host.value,
    port: port.value,
    user_uuid: "1231",
    username: username.value,
    password: password.value,
  };
  environment_proxies_create(proxy);
};

// 跳转到新增代理页面
const mydaili = () => {
  addProxies();
};
</script>
<template>
  <Model
    class="min-w-[1250px]"
    title="导入代理"
    :open="props.open"
    @close="() => emits('close')"
  >
    <div class="space-y-6 mx-2 ">
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
    </div>

    <div class="flex items-center gap-4 border-t py-2 px-2 justify-end">
      <CancelButton @click="() => emits('close')"> 取消 </CancelButton>
      <PrimaryButton @click="() => emits('close')"> 保存 </PrimaryButton>
    </div>
  </Model>
</template>
