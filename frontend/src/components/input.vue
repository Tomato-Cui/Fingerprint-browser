<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/util/lib";
import { useVModel } from "@vueuse/core";

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
  class?: HTMLAttributes["class"];
}>();

const emits = defineEmits<{
  (e: "update:modelValue", payload: string | number): void;
}>();

const modelValue = useVModel(props, "modelValue", emits, {
  passive: true,
  defaultValue: props.defaultValue,
});
</script>

<template>
  <input
    readonly
    onfocus="this.removeAttribute('readonly')"
    v-model="modelValue"
    :class="
      cn(
        'px-4 py-2 border rounded-lg focus:outline-none focus:ring-1 focus:ring-blue-600 transform ease-in-out delay-150 duration-100',
        props.class
      )
    "
  />
</template>
