<script setup lang="ts">
import { onMounted } from "vue";
import { useProfileStore } from "@/stores/profile";
import { useUserStore } from "@/stores/user";
import { query_current_team_info } from "@/commands/team";
import { user_query_search_by_email } from "@/commands/user";

const { setCurrentTeam } = useProfileStore();

const emits = defineEmits(["loading"]);
const userStore = useUserStore();

onMounted(async () => {
  {
    let loadTeamInfo = await query_current_team_info();
    let { code, data } = loadTeamInfo;
    if (code != 1) return;

    setCurrentTeam({
      id: data.id,
      name: data.name,
      description: data.description,
    });
  }

  {
    let loadUserInfo = (await user_query_search_by_email(
      userStore.getUserInfo().account
    )) as any;
    let { code, data } = loadUserInfo;
    if (code != 1) return;

    userStore.setUserInfo({
      id: data.id,
      account: data.nickname,
      email: data.email,
    });
  }

  emits("loading");
});
</script>

<template></template>
