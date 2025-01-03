<script setup>
import { useRoute, useRouter } from "vue-router";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ShuffleIcon, WaypointsIcon } from "lucide-vue-next";
import RightDetail from "./right-detail.vue";
import { useEnvironmentCreateFromStore } from "@/stores/form/environment-create";
import { useEnvironmentCreatesFromStore } from "@/stores/form/environment-creates";
import {
  environment_create,
  environment_batch_create,
  environment_modify_basic_info,
} from "@/commands/environment";
import { toast } from "vue-sonner";

const tabs = [
  { id: "single", name: "单个创建", href: "/environment-action/create" },
  { id: "batch", name: "批量创建", href: "/environment-action/creates" },
  { id: "import", name: "窗口导入", href: "/environment-action/imports" },
];

const route = useRoute();
const router = useRouter();

const environmentCreateFrom = useEnvironmentCreateFromStore();
const environmentCreatesFrom = useEnvironmentCreatesFromStore();
const onSubmit = () => {
  if (route.path === "/environment-action/create") {
    environmentCreateFrom.handleSubmit(async (values) => {
      try {
        let { name, description } = values;
        const action = route.query.action;
        console.log("这是action:  " + action);
        if (action === "edit") {
          const environment = route.query.environment
            ? JSON.parse(route.query.environment)
            : null;
          console.log("这是环境" + environment.uuid);
          // 修改逻辑
          environment.name = name;
          environment.description = description;
          let res = await environment_modify_basic_info(
            environment.uuid,
            environment
          );
          toast.success("环境信息更新成功：" + res.message);
        } else {
          // 创建逻辑
          let res = await environment_create(name);
          toast.success("环境创建成功：" + res.message);
        }
      } catch (error) {
        toast.warning("创建失败:" + error);
      }
    })();
  } else if (route.path === "/environment-action/creates") {
    environmentCreatesFrom.handleSubmit(async (values) => {
      try {
        let { name, numbers } = values;
        let names = [...new Array(numbers)].map(
          (_, index) => name + "-" + (index + 1)
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
        <CancelButton @click="router.go(-1)">取消</CancelButton>
        <PrimaryButton type="submit">确定</PrimaryButton>
      </div>
    </form>
  </div>
</template>
