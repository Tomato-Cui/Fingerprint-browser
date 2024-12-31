<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineEmits, defineProps, reactive } from "vue";
import { UserPlus2Icon, UsersIcon } from "lucide-vue-next";
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";
import { environment_proxies_modify } from "@/commands/environment-proxy";
import Input from "@/components/input.vue";

const props = defineProps({
  editProxyDialog: Boolean,
  environmentUuid: String,
});
const emit = defineEmits(["update:editProxyDialog"]);

const joinForm = reactive({
  teamCode: "",
  teamLink: "",
  description: "",
});
const createForm = reactive({
  teamName: "",
  remark: "",
});
//确认
const subMit = async () => {
  console.log(proxyForm.value, props.environmentUuid);
  const res = await environment_proxies_modify(
    props.environmentUuid,
    proxyForm.value
  );
  console.log(res);

  //   emit("update:editProxyDialog", false);
};

const environmentNumber = ref(7); // 环境序号
const currentTab = ref("custom"); // 当前选项卡
const activeDropdown = ref(null); // 当前下拉框

const tabs = [
  // 选项卡
  { id: "custom", name: "自定义" },
  { id: "added", name: "已添加代理" },
];

const proxyTypes = ["NO Proxy (本地直连)", "HTTP", "HTTPS", "SOCKS5", "SSH"]; // 代理类型
const selectedProxyType = ref("NO Proxy (本地直连)"); // 默认选择本地直连

const ipChannels = ["IP2Location", "IPInfo", "MaxMind"]; // IP查询通道
const selectedIpChannel = ref("IP2Location"); // 默认选择IP2Location

const toggleDropdown = (dropdown) => {
  // 切换下拉框
  activeDropdown.value = activeDropdown.value === dropdown ? null : dropdown;
};

const selectProxyType = (type) => {
  // 选择代理类型
  selectedProxyType.value = type;
  activeDropdown.value = null;
};

const selectIpChannel = (channel) => {
  // 选择IP查询通道
  selectedIpChannel.value = channel;
  activeDropdown.value = null;
};

// Close dropdown when clicking outside
const handleClickOutside = (event) => {
  // 点击外部关闭下拉框
  if (!event.target.closest(".relative")) {
    activeDropdown.value = null;
  }
};

const proxyForm = ref({
  host: "", // 代理主机地址
  port: "", // 代理端口
  username: "", // 代理用户名
  password: "", // 代理密码
});
</script>

<template>
  <Model
    class="min-w-[700px]"
    :title="'修改代理'"
    :open="props.editProxyDialog"
    @close="() => emit('update:editProxyDialog', false)"
  >
    <div class="flex flex-col p-4 space-x-4">
      <!-- 在这里书写弹出框主题内容代码 -->

      <!-- 主体 -->
      <div class="px-6 pb-8">
        <!-- Environment Number -->
        <div class="pl-4 mb-6">
          <label class="text-gray-600">环境序号</label>
          <div class="inline-block px-4 py-1 ml-4 bg-blue-50 rounded">
            <span class="text-blue-600">{{ props.environmentUuid }}</span>
          </div>
        </div>

        <!-- Custom Configuration Form -->
        <div class="pl-4 space-y-4">
          <!-- Proxy Type Dropdown -->
          <div class="flex relative gap-x-4 items-center">
            <label class="block mb-2 text-gray-600">代理类型</label>
            <div class="flex flex-auto items-center">
              <div class="relative flex-1">
                <button
                  @click="toggleDropdown('proxyType')"
                  class="flex justify-between items-center px-4 py-2 w-full text-left rounded-lg border hover:border-gray-400"
                >
                  <span>{{ selectedProxyType }}</span>
                  <svg
                    class="w-5 h-5 text-gray-400"
                    :class="{
                      'transform rotate-180': activeDropdown === 'proxyType',
                    }"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                </button>

                <div
                  v-if="activeDropdown === 'proxyType'"
                  class="absolute z-10 mt-1 w-full bg-white rounded-lg border shadow-lg"
                >
                  <div
                    v-for="type in proxyTypes"
                    :key="type"
                    @click="selectProxyType(type)"
                    class="px-4 py-2 cursor-pointer hover:bg-gray-100"
                  >
                    {{ type }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div
          v-if="selectedProxyType != 'NO Proxy (本地直连)'"
          class="pt-4 space-y-4"
        >
          <div class="flex relative gap-x-4 items-center">
            <label
              class="flex gap-x-2 items-center mb-2 text-gray-600 whitespace-nowrap"
              ><span class="text-red-500">*</span>代理主机</label
            >
            <Input v-model="proxyForm.host" class="flex-auto" />
          </div>
          <div class="flex relative gap-x-4 items-center">
            <label
              class="flex gap-x-2 items-center mb-2 text-gray-600 whitespace-nowrap"
              ><span class="text-red-500">*</span>代理端口</label
            >
            <Input v-model="proxyForm.port" class="flex-auto" />
          </div>

          <div class="flex relative gap-x-4 items-center">
            <label
              class="flex gap-x-2 items-center mb-2 text-gray-600 whitespace-nowrap"
              ><span class="text-red-500">&nbsp;</span>代理账号</label
            >
            <Input v-model="proxyForm.username" class="flex-auto" />
          </div>

          <div class="flex relative gap-x-4 items-center">
            <label
              class="flex gap-x-2 items-center mb-2 text-gray-600 whitespace-nowrap"
              ><span class="text-red-500">&nbsp;</span>代理密码</label
            >
            <Input v-model="proxyForm.password" class="flex-auto" />
          </div>
        </div>
      </div>

      <div class="flex justify-center border-t">
        <div class="flex gap-x-4 justify-start py-8">
          <PrimaryButton class="px-8" @click="subMit">确定 </PrimaryButton>
          <CancelButton
            class="px-8"
            @click="() => emit('update:editProxyDialog', false)"
            >取消
          </CancelButton>
        </div>
      </div>
    </div>
  </Model>
</template>
