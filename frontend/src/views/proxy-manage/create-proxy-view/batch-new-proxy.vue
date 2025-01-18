<script setup lang="ts">
import { ref } from "vue";
import { PlusIcon, ListIcon, EyeIcon, EyeOffIcon } from "lucide-vue-next";
import Layout from "@/views/proxy-manage/new-proxyLayout.vue";

interface FormData {
  proxyType: string;
  ipQueryChannel: string;
  proxyServer: string;
  proxyAccount: string;
  proxyPassword: string;
  ipMonitoring: boolean;
  ipChangeAction: "warn" | "block";
}

const activeTab = ref("single");
const showPassword = ref(false);

const formData = ref<FormData>({
  proxyType: "Socks5",
  ipQueryChannel: "IP2Location",
  proxyServer: "",
  proxyAccount: "",
  proxyPassword: "",
  ipMonitoring: false,
  ipChangeAction: "warn",
});

const handleSubmit = () => {
  console.log("Form submitted:", formData.value);
  // Add your form submission logic here
};
</script>

<template>
  <Layout>
    <div class="p-6 mx-auto max-w-2xl bg-white rounded-xl shadow-sm">
      <!-- Tabs -->
      <div class="flex gap-6 mb-8 border-b"></div>

      <!-- Form -->
      <form @submit.prevent="handleSubmit" class="space-y-6">
        <!-- Proxy Type -->
        <div class="flex gap-4 items-start">
          <label class="flex gap-1 pt-2 w-24 text-sm">
            代理类型++++
            <span class="text-red-500">*</span>
          </label>
          <select
            v-model="formData.proxyType"
            class="flex-1 p-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="Socks5">Socks5</option>
            <option value="HTTP">HTTP</option>
          </select>
        </div>

        <!-- IP Query Channel -->
        <div class="flex gap-4 items-start">
          <label class="flex gap-1 pt-2 w-24 text-sm">
            IP查询渠道
            <span class="text-red-500">*</span>
          </label>
          <select
            v-model="formData.ipQueryChannel"
            class="flex-1 p-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="IP2Location">IP2Location</option>
            <option value="Other">Other</option>
          </select>
        </div>

        <!-- Proxy Server -->
        <div class="flex gap-4 items-start">
          <label class="flex gap-1 pt-2 w-24 text-sm">
            代理服务器
            <span class="text-red-500">*</span>
          </label>
          <input
            v-model="formData.proxyServer"
            type="text"
            placeholder="请输入代理服务器地址"
            class="flex-1 p-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <!-- Proxy Account -->
        <div class="flex gap-4 items-start">
          <label class="flex gap-1 pt-2 w-24 text-sm">
            代理账号
            <span class="text-red-500">*</span>
          </label>
          <input
            v-model="formData.proxyAccount"
            type="text"
            placeholder="请输入代理账号"
            class="flex-1 p-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <!-- Proxy Password -->
        <div class="flex gap-4 items-start">
          <label class="pt-2 w-24 text-sm">代理密码</label>
          <div class="relative flex-1">
            <input
              v-model="formData.proxyPassword"
              :type="showPassword ? 'text' : 'password'"
              placeholder="请输入代理密码"
              class="p-2 w-full rounded-lg border focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button
              type="button"
              @click="showPassword = !showPassword"
              class="absolute top-2.5 right-3 text-gray-400"
            >
              <EyeIcon v-if="showPassword" class="w-5 h-5" />
              <EyeOffIcon v-else class="w-5 h-5" />
            </button>
          </div>
        </div>

        <!-- IP Change Monitoring -->
        <div class="flex gap-4 items-start">
          <label class="pt-2 w-24 text-sm">IP变更监控</label>
          <div class="flex-1">
            <label class="inline-flex relative items-center cursor-pointer">
              <input
                type="checkbox"
                v-model="formData.ipMonitoring"
                class="sr-only peer"
              />
              <div
                class="w-11 h-6 bg-gray-200 rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border after:rounded-full after:h-5 after:w-5 after:transition-all"
              ></div>
              <span class="ml-2 text-sm text-gray-500">{{
                formData.ipMonitoring ? "开启" : "关闭"
              }}</span>
            </label>

            <!-- Additional IP Monitoring Settings -->
            <div v-if="formData.ipMonitoring" class="mt-4 space-y-4">
              <label class="block text-sm text-gray-500">当IP变更时：</label>
              <div class="space-y-2">
                <label class="flex gap-2 items-center">
                  <input
                    type="radio"
                    v-model="formData.ipChangeAction"
                    value="warn"
                    class="text-blue-600"
                  />
                  <span class="text-sm">警告</span>
                </label>
                <label class="flex gap-2 items-center">
                  <input
                    type="radio"
                    v-model="formData.ipChangeAction"
                    value="block"
                    class="text-blue-600"
                  />
                  <span class="text-sm">禁止访问</span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Form Actions -->
        <div class="flex gap-4 justify-end pt-4">
          <button
            type="button"
            class="px-6 py-2 rounded-lg border hover:bg-gray-50"
          >
            代理检测
          </button>
          <button
            type="submit"
            class="px-6 py-2 text-white bg-blue-600 rounded-lg hover:bg-blue-700"
          >
            确认添加
          </button>
        </div>
      </form>
    </div>
  </Layout>
</template>
