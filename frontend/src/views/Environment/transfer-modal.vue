<script setup>
import { ref } from "vue";
import { Model } from "@/components/model/index";
import Input from "@/components/input.vue";
import { PrimaryButton, CancelButton } from "@/components/button";
import { user_query_search_by_email } from "@/commands/user";
import { environment_transfer_history_batch_create } from "@/commands/environment-transfer-history";
import { SearchX } from "lucide-vue-next";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Button from "@/components/ui/button/Button.vue";
import { toast } from "vue-sonner";

const searchOn = ref(false);
const searchList = ref([]);
const currentUserEmail = ref("");
const props = defineProps({
  open: false,
  title: "",
  name: "",
  uuid: "",
});

const emit = defineEmits(["close", "submit", "load"]);
const onSubmit = () => {
  environment_transfer_history_batch_create(
    [props.uuid],
    currentUserEmail.value
  ).then((res) => {
    if (res.code == 1) {
      toast.success("转移成功");
    } else {
      toast.error("转移失败");
    }
    emit("close");
  });
};
const inputChangeHandle = (v) => {
  user_query_search_by_email(v).then((res) => {
    searchOn.value = true;
    searchList.value = res.data;
    currentUserEmail.value = res.data[0];
  });
};
</script>

<template>
  <Model
    class="max-w-[725px]"
    :title="props.title"
    :open="props.open"
    @close="() => emit('close')"
  >
    <div
      class="flex flex-col gap-y-4 items-center px-4 py-2 mx-auto w-3/4 text-sm bg-orange-200 rounded-md border"
    >
      !
      环境内的账号、Cookie等信息也将转移，扩展、书签等不转移；该操作不可撤销，请谨慎操作。
    </div>
    <div class="flex flex-col gap-y-4 items-center w-full">
      <div class="flex gap-x-2 items-center px-2 w-3/4">
        <span class="whitespace-nowrap">目标环境:</span>
        <span>{{ props.name }}</span>
      </div>
      <div class="flex gap-x-2 items-center px-2 w-3/4">
        <span class="whitespace-nowrap">转移团队:</span>
        <div class="flex gap-x-2 w-full">
          <Input
            v-if="!searchOn"
            type="text"
            class="w-full"
            @change="(e) => inputChangeHandle(e.target.value)"
            placeholder="请输入用户邮箱"
            autocomplete="off"
          />
          <div v-else class="flex gap-x-2 w-full">
            <Select v-model="currentUserEmail">
              <SelectTrigger class="w-full">
                <SelectValue
                  :placeholder="
                    (searchList.length > 0 && searchList[0]?.name) ||
                    '请输入用户邮箱'
                  "
                  class="p-2 rounded-lg outline-none"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem
                    :value="item"
                    :key="item"
                    v-for="item in searchList"
                  >
                    {{ item }}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <Button
              class="w-11 h-full whitespace-nowrap"
              @click="() => (searchOn = false)"
              variant="outline"
              size="icon"
            >
              <SearchX title="清除" class="w-4 h-4 text-gray-300" />
            </Button>
          </div>
        </div>
      </div>
    </div>
    <div class="flex gap-x-2 justify-end pr-10 my-2 w-full">
      <CancelButton @click="() => emit('close')">取消</CancelButton>
      <PrimaryButton @click="onSubmit">转移</PrimaryButton>
    </div>
  </Model>
</template>
