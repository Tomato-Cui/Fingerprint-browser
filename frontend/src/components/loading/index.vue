<script setup>
import { CircleAlert as CircleAlertIcon } from "lucide-vue-next";
import {
  AlertDialog,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { cn } from "@/util/lib";

import {
  AlertDialogOverlay,
  AlertDialogPortal,
  AlertDialogContent,
} from "radix-vue";
import { X } from "lucide-vue-next";
import { PrimaryButton, CancelButton } from "@/components/button";
import { computed } from "vue";

const props = defineProps({
  open: false,
});
</script>

<template>
  <AlertDialog :open="props.open">
    <AlertDialogPortal>
      <AlertDialogOverlay
        class="fixed inset-0 z-50 bg-black/10 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0"
      />
      <AlertDialogContent
        v-bind="forwarded"
        :class="
          cn(
            'fixed left-1/2 top-1/4 z-50 flex flex-col items-center justify-center w-48 -translate-x-1/2 -translate-y-1/2 gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in  data-[state=closed]:animate-out  data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0  data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95  data-[state=closed]:slide-out-to-left-1/2  data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg',
            props.class
          )
        "
      >
        <slot />
      </AlertDialogContent>
    </AlertDialogPortal>
  </AlertDialog>
</template>
