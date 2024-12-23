<script setup>
import { h, ref } from "vue";
import { SearchIcon, SlidersHorizontalIcon } from "lucide-vue-next";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
</script>

<template>
  <div
    class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none flex-auto hover:border-primary outline-none transform ease-in-out delay-150 duration-100"
  >
    <input
      class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none hover:ring-0 hover:outline-none"
      :placeholder="'请输入' + searchFilterValue + '搜索'"
    />
    <SearchIcon class="w-5 h-5 text-gray-400 absolute left-3 top-2.5" />
    <More>
      <MoreTrigger>
        <button class="ml-4 p-2 hover:bg-gray-100 rounded-lg">
          <SlidersHorizontalIcon class="w-5 h-5 text-gray-500" />
        </button>
      </MoreTrigger>
      <MoreContent>
        <MoreItem class="cursor-pointer">
          <Settings2Icon class="w-4 h-4" />名称
        </MoreItem>
        <MoreItem class="cursor-pointer">
          <SquarePenIcon class="w-4 h-4" />备注
        </MoreItem>
        <MoreItem class="cursor-pointer">
          <Trash2Icon class="w-4 h-4" />手机号/邮箱
        </MoreItem>
      </MoreContent>
    </More>
  </div>
</template>
