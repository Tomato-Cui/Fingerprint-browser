<script setup lang="ts">
import { ref, watch } from "vue";
import { SidebarTrigger } from "@/components/ui/sidebar";
import {
  IconRestart,
  IconHeadphonesRound,
  IconMailbox,
  IconVector,
  IconSettings,
} from "@/assets/icons/sidebar/index";
import { logout as useLogout } from "@/commands/user";
import Update from "@/views/system-operation/update.vue";
import Message from "@/views/system-operation/message.vue";
import PersonCen from "@/views/system-operation/personality/index.vue";
import Connect from "@/views/system-operation/connect.vue";
import { toast } from "vue-sonner";
import { useRouter, useRoute } from "vue-router";
import { AlertModel } from "@/components/alert-model";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { useUserStore } from "@/stores/user";

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

const updateDialog = ref(false);
const messageDialog = ref(false);
const personDialog = ref(false);
const logoutDialog = ref(false);
const connectModel = ref(false);

const skipV = () => {
  updateDialog.value = false;
};
const updateV = () => {
  updateDialog.value = false;
};
const logout = () => {
  logoutDialog.value = true;
};
const subLogout = () => {
  useLogout().then((res: any) => {
    toast.success(res.message);
    localStorage.clear();
    router.push("/");
  });
};
watch(
  () => updateDialog.value,
  (v) => {
    if (v) {
      messageDialog.value = false;
      personDialog.value = false;
    }
  }
);
watch(
  () => messageDialog.value,
  (v) => {
    if (v) {
      updateDialog.value = false;
      personDialog.value = false;
    }
  }
);
watch(
  () => personDialog.value,
  (v) => {
    if (v) {
      messageDialog.value = false;
      updateDialog.value = false;
    }
  }
);
watch(
  () => route.path,
  (_) => {
    messageDialog.value = false;
    updateDialog.value = false;
    personDialog.value = false;
  }
);

const funMes = () => {
  setTimeout(() => {
    messageDialog.value = !messageDialog.value;
  }, 10);
};
const funUp = () => {
  setTimeout(() => {
    updateDialog.value = !updateDialog.value;
  }, 10);
};
const funPer = () => {
  if (personDialog.value) return;
  setTimeout(() => {
    personDialog.value = !personDialog.value;
  }, 10);
};
</script>

<template>
  <header class="flex-none relative">
    <div class="flex justify-between border-b px-2 py-1">
      <div class="flex gap-x-2 items-center select-none">
        <SidebarTrigger class="h-10 w-10" />
        <div class="h-2/3"></div>
        <!-- <img
          src="https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000"
          class="h-10 w-auto rounded-full" /> -->
        <div
          class="flex items-center gap-x-3 rounded-lg outline outline-offset-0 hover:outline-offset-[.5px] text-ring-foreground outline-gray-100 dark:outline-slate-900 p-1 px-2 cursor-pointer"
          @click="funPer"
        >
          <div
            class="bg-gradient-to-l to-[#7D41FF] from-[#5E51FF] w-8 h-8 rounded-full flex items-center justify-center text-white"
          >
            {{
              userStore.userInfo.account
                ? userStore.userInfo.account.substring(0, 2)
                : ""
            }}
          </div>
          <div className="flex-1 min-w-0 group-data-[state=collapsed]:hidden">
            <h3
              className="font-semibold truncate text-sm min-w-10 max-w-28"
            >
              {{ userStore.userInfo.account }}
            </h3>
          </div>
          <IconVector
            :class="[
              'size-3 transition-all ease-in-out delay-100 duration-300',
              personDialog ? 'rotate-180' : '',
            ]"
          />
        </div>
      </div>
      <PersonCen
        :open="personDialog"
        @close="personDialog = false"
        @logout="logout"
      />
      <div class="flex gap-x-4 items-center relative">
        <button
          @click="funUp"
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white"
        >
          <IconRestart class="text-white" /> 更新
        </button>
        <Update
          :open="updateDialog"
          @close="updateDialog = false"
          @skip="skipV"
          @update="updateV"
          class="absolute top-[45px] right-[309px]"
        />
        <button
          @click="funMes"
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white"
        >
          <IconMailbox />信息
        </button>
        <!-- <div ref="targetDiv"> -->
        <Message
          :open="messageDialog"
          @close="messageDialog = false"
          class="absolute top-[45px] right-[220px]"
        />
        <!-- </div> -->
        <More class="flex w-5">
          <MoreTrigger class="min-w-fit">
            <button
              class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white"
            >
              <IconHeadphonesRound />联系我们
            </button>
          </MoreTrigger>
          <MoreContent class="w-5">
            <MoreItem @click="connectModel = true"> 联系方式 </MoreItem>
            <MoreItem @click="router.push('/quickStart')"> 帮助中心 </MoreItem>
          </MoreContent>
        </More>

        <button
          @click="router.push('/accountSet')"
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white"
        >
          <IconSettings />系统设置
        </button>
      </div>
    </div>
  </header>
  <AlertModel
    title="退出登录"
    :open="logoutDialog"
    @close="() => (logoutDialog = false)"
    @cancel="() => (logoutDialog = false)"
    @submit="subLogout"
  >
    <div class="text-sm flex flex-col gap-y-4 py-4">
      <p>确定退出登录吗？</p>
    </div>
  </AlertModel>
  <Connect :open="connectModel" @close="connectModel = false" />
</template>
