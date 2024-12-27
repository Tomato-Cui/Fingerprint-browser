<script setup>
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ShuffleIcon, WaypointsIcon } from "lucide-vue-next";
import ImportProxyModel from "./components/import-proxy-model.vue";
import RightDetail from "./right-detail.vue";
import { useEnvironmentCreateFromStore } from "@/stores/form/environment-create";
import { useEnvironmentCreatesFromStore } from "@/stores/form/environment-creates";
import {
  environment_create,
  environment_batch_create,
} from "@/commands/environment";
import { toast } from "vue-sonner";

const tabs = [
  { id: "single", name: "单个创建", href: "/environment-action/create" },
  { id: "batch", name: "批量创建", href: "/environment-action/creates" },
  { id: "import", name: "窗口导入", href: "/environment-action/imports" },
];

const route = useRoute();
const router = useRouter();

const importProxyOpen = ref(false);
const proxyModelCloseHandle = () => (importProxyOpen.value = false);
const proxyModelOpenHandle = () => (importProxyOpen.value = true);

const environmentCreateFrom = useEnvironmentCreateFromStore();
const environmentCreatesFrom = useEnvironmentCreatesFromStore();
const onSubmit = () => {
  if (route.path == "/environment-action/create") {
    environmentCreateFrom.handleSubmit(async (values) => {
      try {
        let { name } = values;
        let res = await environment_create(name);
        toast.success(res.message);
      } catch (error) {
        toast.warning("创建失败:" + error);
      }
    })();
  } else if (route.path == "/environment-action/creates") {
    environmentCreatesFrom.handleSubmit(async (values) => {
      try {
        let { name, numbers } = values;
        let names = [...new Array(numbers).keys()].map(
          (_, index) => name + "-" + index + 1
        );

        let res = await environment_batch_create(names);
        toast.success(res.message);
      } catch (error) {
        toast.warning("创建失败:" + error);
      }
    })();
  }
};
</script>
<template>
  <div class="h-main bg-white m-4 flex flex-col rounded-md">
    <div class="flex border-b mb-2 justify-between pr-4">
      <nav class="flex">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="() => router.push(tab.href)"
          class="px-6 py-3 -mb-px font-medium"
          :class="
            route.path === tab.href
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-600'
          "
        >
          {{ tab.name }}
        </button>
      </nav>
      <PrimaryButton
        @click="proxyModelOpenHandle"
        v-if="route.path === '/environment-action/creates'"
        class="my-1 text-white px-4 flex justify-center items-center gap-x-2 rounded-lg"
      >
        <WaypointsIcon />
        代理导入
      </PrimaryButton>
    </div>

    <form
      class="flex flex-col overflow-hidden gap-x-2 flex-auto"
      @submit.prevent="onSubmit"
    >
      <div class="flex overflow-hidden gap-x-2 flex-auto">
        <div class="overflow-y-auto flex-auto">
          <router-view />
        </div>

        <RightDetail />
      </div>
      <div class="flex gap-x-4 pt-3 px-6 mb-8 bg-white border-t">
        <PrimaryButton type="submit">确定</PrimaryButton>
        <CancelButton>取消</CancelButton>
      </div>
    </form>
    <ImportProxyModel :open="importProxyOpen" @close="proxyModelCloseHandle" />
  </div>
</template>
