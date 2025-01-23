<script setup lang="ts">
import Model from "@/components/model/model.vue";
import { reactive } from "vue";
import { PlusIcon, ListIcon, EyeIcon, EyeOffIcon } from "lucide-vue-next";
import router from "@/router";
import { environment_proxies_create } from "@/commands/environment-proxy";
import { ref } from "vue";

// interface Props {
//   modelValue?: "warning" | "block";
// }

// const props = withDefaults(defineProps<Props>(), {
//   modelValue: "warning",
// });

const props = defineProps({
  editProxy: {
    type: Boolean,
    default: false,
  },
});

// const emit = defineEmits<{
//   (e: "update:modelValue", value: "warning" | "block"): void;
// }>();

const emit = defineEmits(["update:editProxy"]);

// const selected = ref<"warning" | "block">(props.modelValue);

// watch(selected, (newValue) => {
//   emit("update:modelValue", newValue);
// });

interface FormData {
  proxyType: string;
  ipQueryChannel: string;
  proxyServer: string;
  proxyAccount: string;
  proxyPassword: string;
  ipMonitoring: boolean;
  ipChangeAction: "warning" | "block";
}

const showPassword = ref(false);

const formData = ref<FormData>({
  proxyType: "Socks5",
  ipQueryChannel: "IP2Location",
  proxyServer: "",
  proxyAccount: "",
  proxyPassword: "",
  ipMonitoring: false,
  ipChangeAction: "warning",
});

const payload = {
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
  const ipRegex = /^(\d{1,3}\.){3}\d{1,3}$/;
  if (!ipRegex.test(formData.value.proxyServer)) {
    alert("请输入有效的IP地址，格式如: 192.168.1.1");
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

  environment_proxies_create(payload);

  // Add your form submission logic here
  emit("update:editProxy", false);
};

const addGroup = reactive({
  groupName: "",
  description: "",
});

const clearForm = () => {
  addGroup.groupName = "";
  addGroup.description = "";
};
//确认
</script>

<template>
  <Model
    class=""
    title="编辑代理"
    :open="props.editProxy"
    @close="() => emit('update:editProxy', false)"
  >
    <div class="flex flex-col gap-y-4 mb-2">
      <!-- Form -->
      <div
        @submit.prevent="handleSubmit"
        class="flex flex-col justify-start pt-3 space-y-6 border border-yellow-500"
      >
        <div
          class="flex flex-col justify-start px-12 pt-3 space-y-6 w-full h-full"
        >
          <!-- Proxy Type -->
          <div class="flex gap-4 items-start w-[400px]">
            <label
              class="flex gap-1 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
            >
              代理类型
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
          <div class="flex gap-4 items-start w-[400px]">
            <label
              class="flex gap-1 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
            >
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
          <div class="flex gap-4 items-start w-[400px]">
            <label
              class="flex gap-1 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
            >
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
          <div class="flex gap-4 items-start w-[400px]">
            <label
              class="flex gap-1 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
            >
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
          <div class="flex gap-4 items-start w-[400px]">
            <label
              class="flex gap-1 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
            >
              代理密码</label
            >
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
          <div class="flex gap-4 w-[400px]">
            <div class="flex flex-row items-end">
              <label
                class="flex gap-1 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
              >
                IP变更监控</label
              >
            </div>
            <div class="flex items-end">
              <label class="inline-flex relative cursor-pointer">
                <div class="flex items-end">
                  <input
                    type="checkbox"
                    v-model="formData.ipMonitoring"
                    class="flex items-end sr-only peer"
                  />
                  <div
                    class="w-11 h-6 bg-gray-200 rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[8px] after:left-[2px] after:bg-white after:border after:rounded-full after:h-5 after:w-5 after:transition-all"
                  ></div>
                </div>
                <span
                  class="flex gap-1 mx-2 pt-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
                  >{{ formData.ipMonitoring ? "开启" : "关闭" }}</span
                >
              </label>
            </div>
          </div>

          <!-- Additional IP Monitoring Settings -->
          <div
            v-if="formData.ipMonitoring"
            class="flex flex-row mt-4 space-y-4 w-[400px]"
          >
            <div class="flex flex-row">
              <div class="flex items-start">
                <label
                  class="flex gap-1 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
                >
                  发生IP变化时</label
                >
              </div>

              <!-- <div class="flex flex-col gap-2 ml-8">
                <label class="flex gap-2 items-center">
                  <input
                    type="radio"
                    v-model="formData.ipChangeAction"
                    value="warning"
                    class="w-4 h-4 rounded-full border-2 border-gray-400 appearance-none cursor-pointer checked:bg-blue-500 checked:ring-0 focus:outline-none"
                  />
                  <span
                    class="font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
                    >警告</span
                  >
                </label>

                <label class="flex gap-2 items-center">
                  <input
                    type="radio"
                    v-model="formData.ipChangeAction"
                    value="block"
                    class="w-4 h-4 rounded-full border-2 border-gray-400 appearance-none cursor-pointer checked:bg-blue-500 checked:ring-0 focus:outline-none"
                  />
                  <span
                    class="font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
                    >禁止访问</span
                  >
                </label>
              </div> -->

              <div class="flex flex-col ml-4 gap-y-3 justify-start w-[95px]">
                <label class="flex items-start cursor-pointer">
                  <div
                    class="flex relative justify-center items-center w-6 h-6"
                  >
                    <input
                      type="radio"
                      name="action"
                      value="warning"
                      v-model="formData.ipChangeAction"
                      class="sr-only peer"
                    />

                    <div
                      class="absolute w-4 h-4 rounded-full border-2 transition-colors duration-200"
                      :class="
                        formData.ipChangeAction === 'warning'
                          ? 'border-blue-600'
                          : 'border-gray-200'
                      "
                    ></div>

                    <div
                      class="absolute w-2 h-2 bg-blue-600 rounded-full transition-all duration-200"
                      :class="
                        formData.ipChangeAction === 'warning'
                          ? 'opacity-100 scale-100'
                          : 'opacity-0 scale-0'
                      "
                    ></div>
                  </div>
                  <span
                    class="flex gap-1 w-24 ml-2 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
                    >警告</span
                  >
                </label>

                <label class="flex items-start cursor-pointer">
                  <div
                    class="flex relative justify-center items-center w-6 h-6"
                  >
                    <input
                      type="radio"
                      name="action"
                      value="block"
                      v-model="formData.ipChangeAction"
                      class="sr-only peer"
                    />

                    <div
                      class="absolute w-4 h-4 rounded-full border-2 transition-colors duration-200"
                      :class="
                        formData.ipChangeAction === 'block'
                          ? 'border-blue-600'
                          : 'border-gray-200'
                      "
                    ></div>

                    <div
                      class="absolute w-2 h-2 bg-blue-600 rounded-full transition-all duration-200"
                      :class="
                        formData.ipChangeAction === 'block'
                          ? 'opacity-100 scale-100'
                          : 'opacity-0 scale-0'
                      "
                    ></div>
                  </div>
                  <span
                    class="flex gap-1 ml-2 w-24 font-pingfang text-sm font-semibold text-[14px] leading-[22px]"
                    >禁止访问</span
                  >
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Form Actions -->
        <div
          class="flex gap-x-4 justify-end items-center pt-3 pb-3 border-t border-gray-300"
        >
          <button
            type="button"
            class="py-2 text-blue-500 rounded-lg"
            @click="() => emit('update:editProxy', false)"
          >
            代理检测
          </button>

          <button
            type="button"
            @click="() => emit('update:editProxy', false)"
            class="px-6 py-2 rounded-lg border hover:bg-gray-50"
          >
            取消
          </button>

          <button
            type="submit"
            class="px-6 py-2 mr-3 text-white bg-blue-600 rounded-lg hover:bg-blue-700"
            @click="handleSubmit"
          >
            确认
          </button>
        </div>
      </div>
    </div>
  </Model>
</template>
