<script setup>
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

const props = defineProps({
  title: "",
  class: "",
  disabled: false,
});

const emit = defineEmits(["click"]);

const clickHandle = (event) => {
  emit("click", event);
};
</script>

<template>
  <Tooltip>
    <TooltipTrigger>
      <button
        @click="clickHandle"
        :class="`${props.class} disabled:cursor-not-allowed`"
        :disabled="props.disabled"
      >
        <slot />
      </button>
    </TooltipTrigger>
    <TooltipContent class="bg-black bg-opacity-50 text-white p-2 rounded">
      <p>{{ props.title }}</p>
    </TooltipContent>
  </Tooltip>
</template>
