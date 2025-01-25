<!-- <script setup lang="ts">
import Model from "@/components/model/model.vue";
import { reactive } from "vue";
import { ref } from "vue";

import { AlertCircleIcon } from "lucide-vue-next";
import { AlertModel } from "@/components/alert-model";

const props = defineProps({
  singleDeleteProxy: {
    type: Boolean,
    default: false,
  },
});

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

const addGroup = reactive({
  groupName: "",
  description: "",
});

const clearForm = () => {
  addGroup.groupName = "";
  addGroup.description = "";
};
const emit = defineEmits(["update:singleDeleteProxy"]);

const singleDeleteProxy = () => {
  console.log("singleDeleteProxy");
};
</script>

<template>
  <AlertModel
    class=""
    title="删除该代理吗"
    :open="props.singleDeleteProxy"
    @close="() => emit('update:singleDeleteProxy', false)"
  >


  
    <div class="relative bg-white rounded-xl">
      <div class="flex gap-3">
 
        <div class="flex-1">
          <h3 class="mb-2 text-base font-medium text-gray-900">删除该代理吗</h3>
          <p class="text-sm text-gray-500">
            您已选择 {{}}
            个代理，确定要删除所选代理吗？删除后无法找回，请及时修改已配置的环境。
          </p>
        </div>
      </div>
    </div>
  </AlertModel>
</template> -->
