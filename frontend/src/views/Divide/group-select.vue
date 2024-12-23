<script setup>
import { ref } from "vue";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
const groupSwitchSelect = ref("");
</script>
<template>
  <Select v-model="groupSwitchSelect">
    <SelectTrigger class="w-44 bg-[#f9f9f9]">
      <SelectValue
        placeholder="请选择环境分组"
        class="w-full p-2 rounded-lg outline-none"
      />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectItem value="1"> 未分组 </SelectItem>
        <SelectItem value="2"> 分组1 </SelectItem>
        <SelectItem value="3"> 分组2 </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
