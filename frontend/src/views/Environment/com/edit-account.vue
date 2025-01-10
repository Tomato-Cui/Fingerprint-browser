<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import {
  ref,
  defineEmits,
  defineProps,
  reactive,
  watch,
  onMounted,
  onUnmounted,
} from "vue";
import { UserPlus2Icon, UsersIcon } from "lucide-vue-next";
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";
import { toast } from "vue-sonner";
import { environment_account_create } from "@/commands/environment-account";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import Input from "@/components/input.vue";

const props = defineProps({
  editAccountDialog: Boolean,
  environmentUuid: String,
  environmentId: Number,
  userUuid: String,
});
const emit = defineEmits(["update:editAccountDialog"]);
watch(() => props.editAccountDialog, (v) => {
  clearForm();
});

const editAccount = reactive({
  username: "",
  password: "",
});
const clearForm = () => {
  editAccount.environment_uuid = "";
  platformSelect.value = platforms.value[0];
  editAccount.selectPlatformUrl = "https://www.youtube.com";
  editAccount.username = "";
  editAccount.password = "";
};

const onSubmitHandle = () => {
  environment_account_create({
    platform: platformSelect.value.platform,
    platform_url: platformSelect.value.platform_url,
    platform_account: editAccount.username,
    platform_password: editAccount.password,
    environment_uuid: props.environmentUuid,
  }).then((res) => {
    res.code == 1 ? toast.success("添加成功") : toast.warning("添加失败");
    emit("update:editAccountDialog", false);
  });
};

const platforms = ref([
  {
    platform: "youtube",
    platform_url: "https://www.youtube.com",
  },
  {
    platform: "推特",
    platform_url: "https://twitter.com",
  },
  {
    platform: "脸书",
    platform_url: "https://www.facebook.com",
  },
]);
const platformSelect = ref(platforms.value[0]);
</script>

<template>
  <Model class="min-w-[600px] text-sm" :title="'编辑账号'" :open="props.editAccountDialog"
    @close="() => emit('update:editAccountDialog', false)">
    <div class="px-6 pb-4 text-sm">
      <div class="space-y-4">
        <div class="flex items-center">
          <label class="text-gray-600 w-[100px]">环境序号</label>
          <div class="bg-blue-50 px-4 py-2 rounded-md">
            <span class="text-blue-600">{{ props.environmentUuid }}</span>
          </div>
        </div>

        <div class="flex items-center gap-x-5">
          <label class="text-gray-600 w-[100px]">环境序号</label>
          <Select v-model="platformSelect">
            <SelectTrigger class="w-full">
              <SelectValue :placeholder="选择平台账号" class="p-2 rounded-lg outline-none" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem :value="item" :key="item" v-for="item in platforms">
                  {{ item.platform }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <div class="flex items-center">
          <label class="text-gray-600 w-[100px]">用户名</label>
          <div class="flex-1 relative">
            <Input class="w-full outline-none" placeholder="请输入用户名" v-model="editAccount.username" maxlength="20" />
            <span class="absolute right-10 top-3 text-xs text-gray-400">{{ editAccount.username?.length || 0 }}/20</span>
          </div>
        </div>

        <div class="flex items-center">
          <label class="text-gray-600 w-[100px]">密码</label>
          <div class="flex-1 relative">
            <Input class="w-full outline-none" placeholder="请输入密码" type="password" v-model="editAccount.password"
              maxlength="20" />
            <span class="absolute right-10 top-3 text-xs text-gray-400">{{ editAccount.password?.length || 0
              }}/20</span>
          </div>
        </div>
      </div>
    </div>

    <div class="flex border-t justify-end p-2 px-4 gap-x-4">
      <CancelButton @click="() => emit('update:editAccountDialog', false)">取消
      </CancelButton>
      <PrimaryButton @click="onSubmitHandle">确定</PrimaryButton>
    </div>
  </Model>
</template>
