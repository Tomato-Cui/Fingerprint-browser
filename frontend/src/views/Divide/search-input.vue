<script setup>
import { h, ref } from "vue";
import { SearchIcon, SlidersHorizontalIcon } from "lucide-vue-next";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";

const searchTypes = [
  { title: "name", value: "名称" },
  { title: "description", value: "描述" },
];
const searchValue = ref("");
const props = defineProps({
  searchCurrentType: {},
});
const emits = defineEmits(["update:searchValue", "update:searchType"]);
</script>

<template>
  <div
    class="relative max-w-xl flex items-center border rounded-lg bg-[#f9f9f9] hover:outline-none flex-auto hover:border-primary outline-none transform ease-in-out delay-150 duration-100"
  >
    <input
      class="w-full pl-10 pr-4 py-2 rounded-lg border-gray-200 bg-[#f9f9f9] outline-none hover:ring-0 hover:outline-none"
      :placeholder="'请输入' + props.searchCurrentType.value + '搜索'"
      @change="(v) => emits('update:searchValue', v.target.value)"
    />
    <SearchIcon class="w-5 h-5 text-gray-400 absolute left-3 top-2.5" />
    <More>
      <MoreTrigger>
        <button class="ml-4 p-2 hover:bg-gray-100 rounded-lg">
          <SlidersHorizontalIcon class="w-5 h-5 text-gray-500" />
        </button>
      </MoreTrigger>
      <MoreContent>
        <MoreItem
          class="cursor-pointer"
          v-for="(item, index) in searchTypes"
          :key="index"
          @click="() => emits('update:searchType', item)"
        >
          {{ item.value }}
        </MoreItem>
      </MoreContent>
    </More>
  </div>
</template>
