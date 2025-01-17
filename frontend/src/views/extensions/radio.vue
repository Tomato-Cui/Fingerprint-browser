<script setup lang="ts">
import { cn } from "@/util/lib";

export interface RadioInterface {
  icon?: any;
  value: string;
}

interface PropsInterface {
  check: RadioInterface;
  data: Array<RadioInterface>;
}
interface Emits {
  (event: "update:check", newCheck: RadioInterface): void;
}

const props = defineProps<PropsInterface>();
const emits = defineEmits<Emits>();
</script>

<template>
  <div class="flex gap-x-2 border-[1.4px] border-[#C5CEE0] p-1 rounded-lg">
    <div
      v-for="item in props.data"
      :class="
        cn(
          'flex items-center space-x-1 py-1.5 px-2 rounded-lg cursor-pointer',
          props.check.value == item.value
            ? item.icon
              ? 'bg-[#5050FA] text-white fill-white'
              // : 'bg-[#7171FF] text-white fill-white'
              : 'bg-[#5050FA] text-white fill-white'
            : ''
        )
      "
      @click="() => emits('update:check', item)"
    >
      <component
        v-if="item.icon"
        :is="item.icon"
        :class="
          cn(
            'size-5 fill-[#8F9BB3]',
            props.check.value == item.value ? 'fill-white' : ''
          )
        "
      />
      <span>{{ item.value }}</span>
    </div>
  </div>
</template>
