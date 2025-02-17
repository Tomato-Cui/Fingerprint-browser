<script setup lang="ts">
import Model from "@/components/model/model.vue";
import { reactive } from "vue";
import { ref } from "vue";

import { AlertCircleIcon } from "lucide-vue-next";
import { AlertModel } from "@/components/alert-model";
import { environment_proxies_delete } from "@/commands/environment-proxy";

const props = defineProps({
  singleDeleteProxy: {
    type: Boolean,
    default: false,
  },
  proxy: {
    type: Object,
    default: () => ({}),
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

const singlehandleDeleteProxy = (id: number) => {
  environment_proxies_delete(id);

  singleDeleteProxy.value = false;

  privateproxyloadData();
};

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
  <!-- <AlertModel
    class=""
    title="删除该代理吗"
    :open="props.singleDeleteProxy"
    @close="() => emit('update:singleDeleteProxy', false)"
  >
    <div class="relative rounded-xl border border-red-500 bg-red">
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
  </AlertModel> -->

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
            您已选择
            {{ proxy.name }}
            个代理，确定要删除所选代理吗？删除后无法找回，请及时修改已配置的环境。
          </p>
        </div>
      </div>
    </div>
  </AlertModel>
</template>
