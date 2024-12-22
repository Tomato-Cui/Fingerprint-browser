<script setup>
import { ref } from "vue";
import TooltipButton from "@/components/tooltip-button.vue";
import { ArrowUpWideNarrow, ArrowDownWideNarrow } from "lucide-vue-next";

const isSort = ref(false);
</script>

<template>
  <TooltipButton
    title="排列"
    @click="isSort = !isSort"
    class="p-2.5 hover:bg-gray-100 rounded border-gray-200 border"
  >
    <ArrowDownWideNarrow v-if="isSort" class="h-5 w-5 text-gray-600" />
    <ArrowUpWideNarrow v-else class="h-5 w-5 text-gray-600" />
  </TooltipButton>
</template>
