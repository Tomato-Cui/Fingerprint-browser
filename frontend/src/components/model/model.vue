<script setup>
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogClose,
} from "@/components/ui/dialog";
import { X } from "lucide-vue-next";

const props = defineProps({
  class: "",
  title: "",
  description: "",
  open: false,
});

const emit = defineEmits(["close"]);
</script>

<template>
  <Dialog :open="props.open">
    <DialogContent
      :class="`p-0 shadow-[0_20px_50px_rgba(8,_112,_184,_0.1)] ${props.class} `"
    >
      <DialogHeader class="border-b p-2 flex flex-row justify-between">
        <DialogTitle class="text-md font-[600]">
          {{ props.title || "&nbsp;" }}
          <span class="text-xs text-gray-500 font-normal">
            {{ props.description || "&nbsp;" }}
          </span>
        </DialogTitle>
        <DialogClose @click="() => emit('close')" class="cursor-pointer">
          <X class="w-5 h-5 text-gray-600" />
        </DialogClose>
      </DialogHeader>
      <slot></slot>
    </DialogContent>
  </Dialog>
</template>
