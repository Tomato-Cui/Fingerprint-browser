<script setup lang="ts">
import { cn } from "@/util/lib";
import { IconFilter } from "@/assets/icons/environment-trash/index.ts";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Button } from "@/components/ui/button";

const props = defineProps<{
  data: any;
  checks: any;
}>();

const emits = defineEmits<{
  "update:onCheck": [string, boolean];
}>();
</script>
<template>
  <Popover>
    <PopoverTrigger>
      <Button
        variant="outline"
        class="h-8 flex gap-x-1 px-3 outline-gray-50 outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 active:scale-[.98]"
      >
        <component :is="IconFilter" />
        筛选
      </Button>
    </PopoverTrigger>
    <PopoverContent align="end" class="p-1 w-32">
      <div
        v-for="item in props.checks"
        class="flex gap-x-2 items-center text-sm p-1 cursor-pointer hover:bg-blue-100 transition-all duration-100 delay-100 ease-in-out rounded-md"
        :key="item.id"
      >
        <Checkbox
          :checked="item.check"
          :id="item.id"
          @update:checked="(v) => emits('update:onCheck', item.id, v)"
          class="data-[state=checked]:bg-[#5050FA] data-[state=checked]:border-gray-600 border-gray-300"
        />
        <lable :class="cn(item.check ? 'text-[#5050FA]' : '')" :for="item.id">{{
          item.title
        }}</lable>
      </div>
    </PopoverContent>
  </Popover>
</template>
