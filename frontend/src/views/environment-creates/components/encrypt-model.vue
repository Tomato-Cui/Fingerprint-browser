<script setup>
import { ref } from "vue";
import { CloudUpload as CloudUploadIcon } from "lucide-vue-next";
import { Model, ModelClose } from "@/components/model/index";
import Input from "@/components/input.vue";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useExtensionCenterFromStore } from "@/stores/form/extension-center";
import { toast } from "vue-sonner";

const props = defineProps({
  open: false,
  title: "",
});

const emit = defineEmits(["close"]);
</script>

<template>
  <Model
    class="min-w-[725px]"
    :title="props.title"
    :open="props.open"
    @close="() => emit('close')"
  >
    <slot />
  </Model>
</template>
