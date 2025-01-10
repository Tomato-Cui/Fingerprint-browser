<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import  { environment_proxy_group_query } from '@/commands/environment-proxy-group';

const groups = ref<any[]>([]);
const emits = defineEmits(["select"]);

const select = ref<string | undefined>(undefined);

onMounted(() => {
  environment_proxy_group_query(1, 1000).then((res) => {
    groups.value = res.data.data;
  });
});

watch(select, (newSelect) => {
  emits("select", newSelect);
});
</script>
<template>
  <Select v-model="select">
    <SelectTrigger class="w-44 bg-[#f9f9f9]">
      <SelectValue
        placeholder="请选择代理分组"
        class="p-2 w-full rounded-lg outline-none"
      />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectItem value="0">未分组</SelectItem>
        <SelectItem :value="item.id" v-for="item in groups">
          {{ item.name }}
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
