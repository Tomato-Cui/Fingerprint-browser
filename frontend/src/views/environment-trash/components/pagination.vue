<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  Pagination,
  PaginationEllipsis,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from "@/components/ui/pagination";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";
import { ref, watch } from "vue";
import { cn } from "@/util/lib";

const pageNum = ref<number>(0);
const props = defineProps<{
  total: number;
  pageSize: number;
}>();
const allowPageSize = ref<"17" | "32" | "42">("17");

const emits = defineEmits<{
  "update:page": [number];
  "update:pageSize": [number];
}>();

watch(allowPageSize, (_) => {
  emits("update:pageSize", parseInt(allowPageSize.value as any as string));
});
watch(pageNum, (_) => {
  emits("update:page", pageNum.value);
});
</script>
<template>
  <div class="border-t p-2 flex justify-end select-none">
    <Pagination
      v-slot="{ page }"
      :total="props.total"
      :items-per-page="props.pageSize"
      :sibling-count="1"
      :default-page="1"
      class="flex items-center justify-center gap-x-1"
    >
      <p class="text-xs whitespace-nowrap">共{{ props.total }}项</p>
      <PaginationList v-slot="{ items }" class="flex items-center gap-x-1">
        <PaginationPrev
          class="w-8 h-8 border-none cursor-pointer shadow-none"
          @click="--pageNum"
        />
        <template v-for="(item, index) in items">
          <PaginationListItem
            v-if="item.type === 'page'"
            :key="index"
            :value="item.value"
            as-child
          >
            <Button
              :class="
                cn(
                  'w-8 h-8 p-0 text-black hover:bg-[#F7F9FC]',
                  item.value === page ? 'bg-[#D8DCF3] ' : 'bg-white'
                )
              "
              @click="pageNum = item.value"
            >
              {{ item.value }}
            </Button>
          </PaginationListItem>
          <PaginationEllipsis v-else :key="item.type" :index="index" />
        </template>
        <PaginationNext
          class="w-8 h-8 border-none cursor-pointer shadow-none"
          @click="++pageNum"
        />
      </PaginationList>
      <Select
        :default-value="allowPageSize"
        v-model:model-value="allowPageSize"
      >
        <SelectTrigger class="h-8 select-none">
          <SelectValue class="px-0 w-full rounded-lg outline-none" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem
              :key="item"
              :value="item"
              v-for="item in ['17', '32', '42']"
              >{{ item }}条/页</SelectItem
            >
          </SelectGroup>
        </SelectContent>
      </Select>
    </Pagination>
  </div>
</template>
