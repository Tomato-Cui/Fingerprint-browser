<script setup lang="ts">
import Model from "@/components/model/model.vue";
import { reactive } from "vue";
import { PrimaryButton, CancelButton } from "@/components/button";

const props = defineProps({
  createGroupManage: {
    type: Boolean,
    default: false,
  },
});

//确认
const subMit = () => {
  emit("update:createGroupManage", false);
};

const emit = defineEmits(["update:createGroupManage"]);

const addGroup = reactive({
  groupName: "",
  description: "",
});
</script>

<template>
  <Model
    class=""
    title="新建分组"
    :open="props.createGroupManage"
    @close="() => emit('update:createGroupManage', false)"
  >
    <div class="flex flex-col gap-y-4 px-2 mb-2">
      <div class="flex gap-4 items-center px-9">
        <div>
          <label
            class="w-[80px] flex justify-end fl font-semibold text-[14px] leading-[22px]"
            >环境名称</label
          >
        </div>
        <div class="w-full">
          <Input
            v-model="addGroup.groupName"
            type="text"
            placeholder="请输入环境名称"
            class="flex-1 px-4 py-1 w-full text-lg rounded-lg border border-gray-300"
          />
        </div>
      </div>

      <div class="flex gap-4 items-center px-9">
        <div>
          <label
            class="w-[80px] flex justify-end fl font-semibold text-[14px] leading-[22px]"
            >备注</label
          >
        </div>
        <div class="w-full">
          <Input
            v-model="addGroup.groupName"
            type="text"
            placeholder="请输入备注"
            class="flex-1 px-4 py-1 w-full text-lg rounded-lg border border-gray-300"
          />
        </div>
      </div>

      <div class="flex justify-end border-t border-gray-300">
        <div class="flex gap-x-4 justify-end pt-3">
          <PrimaryButton class="px-8 bg-blue-500" @click="subMit">
            确定
          </PrimaryButton>

          <CancelButton
            class="px-8"
            @click="() => emit('update:createGroupManage', false)"
          >
            取消
          </CancelButton>
        </div>
      </div>
    </div>
  </Model>
</template>
