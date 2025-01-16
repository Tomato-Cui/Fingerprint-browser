<script setup lang="ts">
import Model from "@/components/model/model.vue";
import { reactive } from "vue";
import { PrimaryButton, CancelButton } from "@/components/button";
import { environment_group_create } from "@/commands/environment-group";
import { toast } from "vue-sonner";
import Input from "@/components/input.vue";

const props = defineProps({
  createGroupManage: {
    type: Boolean,
    default: false,
  },
});

const addGroup = reactive({
  groupName: "",
  description: "",
});

const clearForm = () => {
  addGroup.groupName = "";
  addGroup.description = "";
};
//确认
const subMit = () => {
  console.log(" addGroup", addGroup.groupName, addGroup.description);
  environment_group_create(addGroup.groupName, addGroup.description).then(
    (res) => {
      console.log("res", res);
      toast.success("创建成功");
    }
  );

  console.log("创建成功");

  emit("update:createGroupManage", false);

  clearForm();
};

const emit = defineEmits(["update:createGroupManage"]);
</script>

<template>
  <Model
    class=""
    title="新建分组"
    :open="props.createGroupManage"
    @close="() => emit('update:createGroupManage', false)"
  >
    <div class="flex flex-col gap-y-4 mb-2">
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
            v-model="addGroup.description"
            type="text"
            placeholder="请输入备注"
            class="flex-1 px-4 py-1 w-full text-lg rounded-lg border border-gray-300"
          />
        </div>
      </div>

      <div class="flex justify-end border-t border-gray-300">
        <div class="flex gap-x-4 justify-end px-2 pt-3">
          <CancelButton
            class="px-4 border border-gray-300"
            @click="() => emit('update:createGroupManage', false)"
          >
            取消
          </CancelButton>
          <PrimaryButton class="px-4 !bg-[#5050FA]" @click="subMit">
            确定
          </PrimaryButton>
        </div>
      </div>
    </div>
  </Model>
</template>
