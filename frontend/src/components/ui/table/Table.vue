<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/util/lib'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div class="relative w-full">
    <table :class="cn('w-full caption-bottom text-sm overflow-auto', props.class)">
      <slot />
    </table>
  </div>
</template>
