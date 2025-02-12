<script setup lang="ts">
import Model from "@/components/model/model.vue";

import { environment_proxies_modify } from "@/commands/environment-proxy";
import { toast } from "vue-sonner";
import { ip_info } from "@/commands";
import { ref, reactive } from "vue";
import {
  XIcon,
  ChevronDownIcon,
  HelpCircleIcon,
  RefreshCwIcon,
  AlertCircleIcon,
} from "lucide-vue-next";

const activeTab = ref("new");
const error = ref(false);

const formData = ref<FormData>({
  proxyType: "Socks5",
  ipQueryChannel: "IP2Location",
  proxyServer: "",
  proxyAccount: "",
  proxyPassword: "",
  ipMonitoring: false,
  ipChangeAction: "warning",
});

const closeModal = () => {
  // Implement close logic
};

const refreshUrl = () => {
  // Implement refresh URL logic
};

const props = defineProps({
  modifyProxy: {
    type: Boolean,
    default: false,
  },
  proxy: {
    type: Object,
    default: () => ({}),
  },
});

const emit = defineEmits(["update:modifyProxy"]);

interface FormData {
  proxyType: string;
  ipQueryChannel: string;
  proxyServer: string;
  proxyAccount: string;
  proxyPassword: string;
  ipMonitoring: boolean;
  ipChangeAction: "warning" | "block";
}

const payload = {
  id: props.proxy.id,
  kind: formData.value.proxyType,
  host: formData.value.ipQueryChannel,
  port: formData.value.proxyServer,
  username: formData.value.proxyAccount,
  password: formData.value.proxyPassword,
  ipMonitoring: formData.value.ipMonitoring,
  ipChangeAction: formData.value.ipChangeAction,
};

const handleSubmit = () => {
  // Validate IP address format
  const ipPortRegex = /^(\d{1,3}\.){3}\d{1,3}:\d{1,5}$/;
  if (!ipPortRegex.test(formData.value.proxyServer)) {
    alert("请输入有效的IP地址和端口，格式如: 192.168.1.1:7890");
    return;
  }

  // Validate proxy account length
  if (
    !formData.value.proxyAccount ||
    formData.value.proxyAccount.length < 1 ||
    formData.value.proxyAccount.length > 8
  ) {
    alert("代理账号长度必须在1-8个字符之间");
    return;
  }

  // const id = Number(proxy.proxy.id);

  payload.kind = formData.value.proxyType;
  const [host, port] = formData.value.proxyServer.split(":");
  payload.host = host;
  payload.port = port;
  payload.username = formData.value.proxyAccount;
  payload.password = formData.value.proxyPassword;
  payload.ipMonitoring = formData.value.ipMonitoring;
  payload.ipChangeAction = formData.value.ipChangeAction;

  // Pass both id and payload to environment_proxies_modify
  environment_proxies_modify(payload); // TODO: Replace 0 with actual proxy ID
  toast.success("修改成功");
  emit("update:modifyProxy", false);
};

const handleCheckProxy = async () => {
  // Split proxyServer into host and port
  const [host, port] = formData.value.proxyServer.split(":");

  await ip_info(
    formData.value.proxyType,
    host,
    port,
    formData.value.proxyAccount,
    formData.value.proxyPassword
  );

  toast.success("代理检测成功");
};
</script>

<template>
  <Model
    class=""
    title="修改代理"
    :open="props.modifyProxy"
    @close="() => emit('update:modifyProxy', false)"
  >
    <div class="flex flex-col w-full h-full">
      <div class="flex flex-col p-3 w-full">
        <!-- Header -->

        <!-- Tabs -->
        <div class="flex rounded-md mb-2 w-[300px]">
          <button
            :class="[
              'px-4 py-2 rounded-md border border-gray-200',
              activeTab === 'new'
                ? 'bg-blue-50 text-blue-600'
                : 'text-gray-600',
            ]"
            @click="activeTab = 'new'"
          >
            新代理
          </button>
          <button
            :class="[
              'px-4 py-2 rounded-t-md  rounded-r-md rounded-b-md border border-gray-200',
              activeTab === 'existing'
                ? 'bg-blue-50 text-blue-600'
                : 'text-gray-600',
            ]"
            @click="activeTab = 'existing'"
          >
            选择已有代理
          </button>
        </div>

        <!-- Form -->
        <div class="space-y-2">
          <!-- Proxy Type -->
          <div>
            <label class="block mb-2">
              <span class="text-red-500">*</span>
              <span class="text-gray-700">代理类型</span>
            </label>
            <div class="relative">
              <select
                v-model="formData.proxyType"
                class="p-2 pr-10 w-full rounded-md border appearance-none"
              >
                <option value="socks5">Socks5</option>
              </select>
              <ChevronDownIcon
                class="absolute right-2 top-1/2 w-5 h-5 text-gray-400 transform -translate-y-1/2"
              />
            </div>
          </div>

          <!-- IP Lookup -->
          <div>
            <label class="block mb-2">
              <span class="text-red-500">*</span>
              <span class="text-gray-700">IP查询渠道</span>
              <HelpCircleIcon class="inline-block ml-1 w-4 h-4 text-gray-400" />
            </label>
            <div class="relative">
              <select
                v-model="formData.ipLookup"
                class="p-2 pr-10 w-full rounded-md border appearance-none"
              >
                <option value="ip2location">IP2Location</option>
              </select>
              <ChevronDownIcon
                class="absolute right-2 top-1/2 w-5 h-5 text-gray-400 transform -translate-y-1/2"
              />
            </div>
          </div>

          <!-- Proxy Server -->
          <div>
            <label class="block mb-2">
              <span class="text-red-500">*</span>
              <span class="text-gray-700">代理服务器</span>
              <HelpCircleIcon class="inline-block ml-1 w-4 h-4 text-gray-400" />
            </label>
            <div class="flex gap-2">
              <input
                v-model="formData.proxyServer"
                type="text"
                class="flex-1 p-2 rounded-md border"
                placeholder="164.90.143.177:7899"
              />
            </div>
          </div>

          <!-- Account Number -->
          <div>
            <label class="block mb-2 text-gray-700">代理账号</label>
            <input
              v-model="formData.proxyAccount"
              type="text"
              class="p-2 w-full rounded-md border"
              placeholder="gostar9289"
            />
          </div>

          <!-- Password -->
          <div>
            <label class="block mb-2 text-gray-700">代理密码</label>
            <input
              v-model="formData.password"
              type="password"
              class="p-2 w-full rounded-md border"
            />
          </div>

          <!-- Refresh URL -->
          <div>
            <label class="flex gap-2 items-center mb-2">
              <span class="text-gray-700">刷新URL</span>
              <HelpCircleIcon class="w-4 h-4 text-gray-400" />
              <button @click="refreshUrl" class="ml-auto">
                <RefreshCwIcon
                  class="w-4 h-4 text-gray-400 hover:text-gray-600"
                />
              </button>
            </label>
            <input
              v-model="formData.refreshUrl"
              type="text"
              class="p-2 w-full rounded-md border"
              placeholder="请输入刷新URL"
            />
          </div>

          <!-- IP Change Monitor -->
          <div class="flex justify-between items-center">
            <div class="flex gap-2 items-center">
              <span class="text-gray-700">IP变更监控</span>
              <HelpCircleIcon class="w-4 h-4 text-gray-400" />
            </div>
            <label class="inline-flex relative items-center cursor-pointer">
              <input
                type="checkbox"
                v-model="formData.ipMonitoring"
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"
              ></div>
            </label>
          </div>
        </div>

        <!-- Error Message -->
        <div
          v-if="error"
          class="flex gap-2 items-center p-3 mt-6 text-yellow-600 bg-yellow-50 rounded-md"
        >
          <AlertCircleIcon class="w-5 h-5" />
          <span>检测失败！请检查代理信息是否可用</span>
        </div>
      </div>
      <!-- Footer -->
      <div
        class="flex gap-x-2 justify-end items-center pt-2 mb-2 border-t border-gray-300"
      >
        <button
          @click="handleCheckProxy"
          class="gap-x-2 text-blue-600 hover:text-blue-700"
        >
          代理检测
        </button>
        <button
          @click="() => emit('update:modifyProxy', false)"
          class="gap-x-2 px-4 py-2 rounded-md border hover:bg-gray-50"
        >
          取消
        </button>
        <button
          @click="handleSubmit"
          class="px-4 py-2 mx-2 text-white bg-blue-600 rounded-md hover:bg-blue-700"
        >
          确定
        </button>
      </div>
    </div>
  </Model>
</template>
