<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
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
import PersonCen from '@/views/system-operation/personality/index.vue'
import Connect from "@/views/system-operation/connect.vue";
import { toast } from "vue-sonner";
import { useRouter, useRoute } from "vue-router";
import { AlertModel } from "@/components/alert-model";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { Model } from "@/components/model/index";

const router = useRouter()
const route = useRoute()
const updateDialog = ref(false)
const messageDialog = ref(false)
const personDialog = ref(false)
const logoutDialog = ref(false)
const connectModel = ref(false)

const skipV = () => {
  updateDialog.value = false
}
const updateV = () => {
  updateDialog.value = false
}
const logout = () => {
  logoutDialog.value = true
}
const subLogout = () => {
  useLogout().then((res: any) => {
    toast.success(res.message)
    router.push("/")
  })
}
watch(() => updateDialog.value, (v) => {
  if (v) {
    messageDialog.value = false
    personDialog.value = false
  }
})
watch(() => messageDialog.value, (v) => {
  if (v) {
    updateDialog.value = false
    personDialog.value = false
  }
})
watch(() => personDialog.value, (v) => {
  if (v) {
    messageDialog.value = false
    updateDialog.value = false
  }
})
watch(() => route.path, (_) => {
  messageDialog.value = false
  updateDialog.value = false
  personDialog.value = false
})

const funMes = () => {
  setTimeout(() => {
    messageDialog.value = !messageDialog.value
  }, 10)
}
const funUp = () => {
  setTimeout(() => {
    updateDialog.value = !updateDialog.value
  }, 10)
}
const funPer = () => {
  setTimeout(() => {
    personDialog.value = !personDialog.value
  }, 10)
}
</script>

<template>
  <header class="flex-none relative">
    <div class="flex justify-between border-b px-2 py-1">
      <div @click="funPer"
        className="flex items-center gap-3 h-12 group-data-[state=collapsed]:flex-col cursor-pointer">
        <SidebarTrigger class="" />
        <img
          src="https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000"
          class="h-10 w-auto rounded-full" />
        <div className="flex-1 min-w-0 group-data-[state=collapsed]:hidden">
          <h3 className="font-semibold truncate text-sm max-w-28 text-[#8F9BB3]">
            JoJolin Team
          </h3>
        </div>
        <IconVector class="w-4 h-4" />
      </div>
      <PersonCen :open="personDialog" @close="personDialog = false" @logout="logout" />
      <div class="flex gap-x-4 items-center relative">
        <button @click="funUp"
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white">
          <IconRestart class="text-white" /> 更新
        </button>
        <Update :open="updateDialog" @close="updateDialog = false" @skip="skipV" @update="updateV"
          class="absolute top-[45px] right-[309px]" />
        <button @click="funMes"
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white">
          <IconMailbox />信息
        </button>
        <!-- <div ref="targetDiv"> -->
        <Message :open="messageDialog" @close="messageDialog = false" class="absolute top-[45px] right-[220px]" />
        <!-- </div> -->
        <More class="flex w-5">
          <MoreTrigger class="min-w-fit">
            <button
              class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white">
              <IconHeadphonesRound />联系我们
            </button>
          </MoreTrigger>
          <MoreContent class="w-5">
            <MoreItem @click="connectModel = true">
              联系方式
            </MoreItem>
            <MoreItem @click="router.push('/quickStart')">
              帮助中心
            </MoreItem>
          </MoreContent>
        </More>

        <button @click="router.push('/accountSet')"
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100 hover:bg-[#5050FA] hover:text-white">
          <IconSettings />系统设置
        </button>
      </div>
    </div>
  </header>
  <AlertModel title="退出登录" :open="logoutDialog" @close="() => (logoutDialog = false)"
    @cancel="() => (logoutDialog = false)" @submit="subLogout">
    <div class="text-sm flex flex-col gap-y-4 py-4">
      <p>确定退出登录吗？</p>
    </div>
  </AlertModel>
  <Connect :open="connectModel" @close="connectModel = false"/>
</template>
