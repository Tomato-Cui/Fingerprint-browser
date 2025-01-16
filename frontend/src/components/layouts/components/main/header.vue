<script setup lang="ts">
import { SidebarTrigger } from "@/components/ui/sidebar";
import {
  IconRestart,
  IconHeadphonesRound,
  IconMailbox,
  IconVector,
} from "@/assets/icons/sidebar/index";
</script>

<template>
  <header class="flex-none relative">
    <div class="flex justify-between border-b px-2 py-1">
      <div
        className="flex items-center gap-3 h-12 group-data-[state=collapsed]:flex-col"
      >
        <SidebarTrigger class="" />
        <img
          src="https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000"
          class="h-10 w-auto rounded-full"
        />
        <div className="flex-1 min-w-0 group-data-[state=collapsed]:hidden">
          <h3
            className="font-semibold truncate text-sm max-w-28 text-[#8F9BB3]"
          >
            JoJolin Team
          </h3>
        </div>
        <IconVector class="w-4 h-4" />
      </div>
      <div class="flex gap-x-4 items-center">
        <button
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <IconRestart /> 更新
        </button>
        <button
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <IconMailbox />信息
        </button>
        <button
          class="text-sm border rounded-md px-2 py-1.5 flex gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 outline-gray-50 hover:outline-gray-100"
        >
          <IconHeadphonesRound />联系我们
        </button>
      </div>
    </div>
  </header>
</template>
