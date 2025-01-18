<script setup lang="ts">
import { IconEmptyFolder } from "@/assets/icons/environment-trash/index.ts";
const props = defineProps<{ cols: any[]; data: any[] }>();
</script>
<template>
  <tr v-if="props.data.length == 0" class="h-full">
    <td :colspan="props.cols.length + 1" class="text-center py-4">
      <IconEmptyFolder class="mx-auto" />
      <p class="mt-2 text-gray-500 text-sm">暂无数据</p>
    </td>
  </tr>
</template>
