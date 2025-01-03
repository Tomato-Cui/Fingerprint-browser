<script setup>
import { onMounted, ref, watch } from "vue";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";

const groups = ref([]);
const emits = defineEmits(["select"]);

const select = ref(undefined);

onMounted(() => {
  group_query(0, 100).then((res) => {
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
        placeholder="请选择环境分组"
        class="w-full p-2 rounded-lg outline-none"
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
