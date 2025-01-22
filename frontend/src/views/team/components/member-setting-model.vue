<script setup lang="ts">
import { Model } from "@/components/model/index";
import Input from "@/components/input.vue";
import { Button } from "@/components/ui/button/index";
import { ref, watch } from "vue";
import { useProfileStore } from "@/stores/profile";
import { toast } from "vue-sonner";
import { team_group_query_all } from "@/commands/team-group";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { team_modify_team_user_info } from "@/commands/team";

const props = defineProps<{
  open: Boolean;
  current_user_info: any;
}>();
const emits = defineEmits<{
  close: [];
  reload: [];
}>();

const store = useProfileStore();
const envPermissions = ref<any[]>([]);
const selectEnvPermission = ref();
const teamInfo = store.getCurrentTeam();

watch([() => props.open], () => {
  if (!props.open) return;
  if (!teamInfo) return;
  team_group_query_all(teamInfo.id).then((res) => {
    if (res.code != 1) return;
    selectEnvPermission.value =
      (res.data as any[]).find(
        (item) => item.name == props.current_user_info.permission
      ).id + "";
    envPermissions.value = res.data;
  });
});

const submitHanlde = () => {
  team_modify_team_user_info(
    teamInfo!.id,
    parseInt(selectEnvPermission.value),
    props.current_user_info.uuid,
    ""
  ).then((res) => {
    let { code } = res;
    if (code == 1) {
      toast.success("更新成功");
      emits("close");
      emits("reload");
    }
  });
};
</script>

<template>
  <Model :title="'团队设置'" :open="props.open" @close="() => emits('close')">
    <div class="flex flex-col w-5/6 mx-auto gap-y-4">
      <div class="w-full flex items-center">
        <label for="email" class="inline-block w-20 text-sm">成员邮箱</label>
        <div class="flex grow gap-x-2">
          <Input
            id="email"
            type="text"
            v-model="current_user_info.email"
            placeholder="请输入成员邮箱"
            class="grow text-xs"
            disabled
          />
        </div>
      </div>
      <div class="w-full flex items-center">
        <label for="nickname" class="inline-block w-20 text-sm">成员昵称</label>
        <Input
          id="nickname"
          placeholder="请输入成员昵称"
          v-model="current_user_info.nickname"
          class="grow text-xs"
          disabled
        />
      </div>
      <div class="w-full flex items-center">
        <label for="nickname" class="inline-block w-20 text-sm">成员角色</label>
        <div class="grow">
          <Select
            :default-value="selectEnvPermission"
            @update:model-value="(v:any) => (selectEnvPermission = v)"
          >
            <SelectTrigger class="h-8 select-none">
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
    <div class="flex justify-end border-t p-2 px-4 gap-x-4">
      <Button
        @click="() => emits('close')"
        variant="outline"
        class="h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]"
      >
        取消
      </Button>
      <Button
        class="h-8 gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98] bg-[#5050FA] hover:bg-[#5558E3] hover:text-white text-white"
        @click="submitHanlde"
      >
        确定
      </Button>
    </div>
  </Model>
</template>
