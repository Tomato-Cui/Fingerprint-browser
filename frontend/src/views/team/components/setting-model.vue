<script setup lang="ts">
import { Model } from "@/components/model/index";
import Input from "@/components/input.vue";
import { Button } from "@/components/ui/button/index";
import { ref } from "vue";
import { team_modify } from "@/commands/team";
import { useProfileStore } from "@/stores/profile";
import { toast } from "vue-sonner";

const props = defineProps<{
  open: Boolean;
}>();
const emits = defineEmits<{
  close: [];
}>();
const profileStore = useProfileStore();
const { id, name, description } = profileStore.getCurrentTeam()!;

const forms = ref({
  teamName: name || "",
  teamDescription: description || "",
});

const submitHandle = () => {
  let name = forms.value.teamName,
    description = forms.value.teamDescription;
  if (!name.length || !description.length) {
    toast.warning("团队信息不能为空");
    return;
  }

  team_modify(id, name, description).then((res) => {
    if (res.code == 1) {
      toast.success("更新成功");
      profileStore.setCurrentTeam({ id, name, description });
      emits("close");
    }
  });
};
</script>

<template>
  <Model :title="'团队设置'" :open="props.open" @close="() => emits('close')">
    <div class="flex flex-col w-5/6 mx-auto gap-y-4">
      <div class="w-full flex items-center">
        <label for="email" class="inline-block w-20 text-sm">团队名称</label>
        <div class="flex grow gap-x-2">
          <Input
            id="email"
            type="text"
            v-model="forms.teamName"
            placeholder="请输入团队名称"
            class="grow text-xs"
          />
        </div>
      </div>
      <div class="w-full flex items-center">
        <label for="description" class="inline-block w-20 text-sm"
          >团队描述</label
        >
        <Input
          id="description"
          placeholder="请输入团队描述"
          v-model="forms.teamDescription"
          class="grow text-xs"
        />
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
        @click="submitHandle"
      >
        确定
      </Button>
    </div>
  </Model>
</template>
