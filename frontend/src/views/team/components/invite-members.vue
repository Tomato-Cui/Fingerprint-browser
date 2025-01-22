<script setup lang="ts">
import { Model } from "@/components/model/index";
import Input from "@/components/input.vue";
import { Search } from "lucide-vue-next";
import Loading from "@/components/loading.vue";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { Button } from "@/components/ui/button/index";
import { ref, watch } from "vue";
import { user_query_search_by_email } from "@/commands/user";
import { team_group_query_all } from "@/commands/team-group";
import { team_send } from "@/commands/user-team-temp";
import { useProfileStore } from "@/stores/profile";
import { toast } from "vue-sonner";

const store = useProfileStore();

const props = defineProps<{
  open: Boolean;
}>();
const emits = defineEmits<{
  close: [];
}>();

const selectEnvPermission = ref();
const envPermissions = ref<any[]>([]);
const searchLoading = ref(false);
const userInfo = ref<any>({});
const searchEmailModel = ref("");
const searchResult = ref();

const searchUser = async () => {
  let fromUser = localStorage.getItem("nickname");
  if (fromUser && fromUser == searchEmailModel.value) {
    toast.warning("不能自己邀请自己");
    return;
  }
  let res: any = await user_query_search_by_email(searchEmailModel.value);
  let { code, data } = res;
  searchLoading.value = false;
  if (code != 1) {
    searchResult.value = "搜索的用户不存在.";
    return;
  }

  userInfo.value = data;
};

const inviteHandle = async () => {
  let fromUser = localStorage.getItem("nickname");
  if (fromUser && fromUser == searchEmailModel.value) {
    toast.warning("不能自己邀请自己");
    return;
  }
  let teamInfo = store.getCurrentTeam();
  if (!teamInfo) return;

  let res: any = await team_send(teamInfo.id, userInfo.value.email, "");
  let { code } = res;
  if (code != 1) return;
  toast.success("邀请成功");
  emits("close");
};

watch([() => props.open], () => {
  userInfo.value = {};
  if (!props.open) return;
  let teamInfo = store.getCurrentTeam();
  if (!teamInfo) return;
  team_group_query_all(teamInfo.id).then((res) => {
    if (res.code != 1) return;
    selectEnvPermission.value = (res.data as any[]).find(
      (item) => item.name == "默认组"
    ).id;
    envPermissions.value = res.data;
  });
});
</script>

<template>
  <Model :title="'邀请成员'" :open="props.open" @close="() => emits('close')">
    <div class="flex flex-col w-5/6 mx-auto gap-y-4">
      <span class="text-xs text-gray-500"
        >团队成员可使用邮箱或登录密码登录</span
      >
      <div class="w-full flex items-center">
        <label for="email" class="inline-block w-20 text-sm">登陆邮箱</label>
        <div class="flex grow gap-x-2">
          <Input
            id="email"
            v-model="searchEmailModel"
            placeholder="请输入邮箱"
            class="grow text-xs"
          />
          <Button
            variant="outline"
            @click="searchUser"
            class="h-8 w-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]"
          >
            <Search v-if="!searchLoading" class="text-gray-400 h-4 w-4 z-10" />
            <Loading v-else />
          </Button>
        </div>
      </div>
      <div class="w-full flex items-center">
        <label for="email" class="inline-block w-20 text-sm">成员昵称</label>
        <Input
          id="email"
          placeholder="请输入成员昵称"
          :value="userInfo.nickname || searchResult"
          class="grow text-xs"
          disabled
        />
      </div>
      <div class="w-full">
        <div class="flex text-xs items-center">
          <label for="select" class="inline-block w-20 text-sm">权限</label>
          <div class="grow">
            <Select
              :default-value="selectEnvPermission"
              @update:model-value="(v) => (selectEnvPermission = v)"
              class="grow"
            >
              <SelectTrigger disabled class="h-8 select-none">
                <SelectValue
                  placeholder="选择权限组"
                  class="px-0 w-full rounded-lg outline-none text-xs"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem
                    v-for="item in envPermissions"
                    :key="item.id"
                    :value="item.id"
                    >{{ item.name }}</SelectItem
                  >
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-end border-t p-2 px-4 gap-x-4">
      <Button
        @click="() => emits('close')"
        variant="outline"
        class="h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]"
      >
        取消
      </Button>
      <Button
        :disabled="!userInfo.email"
        class="h-8 gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98] bg-[#5050FA] hover:bg-[#5558E3] hover:text-white text-white"
        @click="inviteHandle"
      >
        确定
      </Button>
    </div>
  </Model>
</template>
