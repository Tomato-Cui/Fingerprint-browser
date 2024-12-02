<template>
  <el-dialog v-model="visible">

  </el-dialog>
</template>

<script setup>
import {computed} from 'vue'
import { useStateStore } from '@/stores/modules/global'; 


const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  }
})

const emits = defineEmits(['update:modelValue'])
const visible = computed({
  get: () => props.modelValue,
  set: (val) => {
    emits('update:modelValue', val)
  }
})
</script>

