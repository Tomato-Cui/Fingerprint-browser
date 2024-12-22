<template>
  <div v-if="props.open" class="fixed inset-0 z-50 overflow-y-auto">
    <!-- Backdrop -->
    <div class="fixed inset-0 bg-black bg-opacity-25 transition-opacity"></div>

    <!-- Modal -->
    <div class="flex min-h-screen items-center justify-center p-4" >
      <div class="relative w-full max-w-md transform overflow-hidden rounded-lg bg-white shadow-xl transition-all" style="min-width: 600px">
        <!-- Header -->
        <div class="flex items-center justify-between px-4 py-3">
          <h3 class="text-lg font-medium">移动</h3>
          <button
              @click="closeModal"
              class="rounded-md p-1 hover:bg-gray-100"
          >
            <XIcon class="h-5 w-5 text-gray-500" />
          </button>
        </div>

        <!-- Content -->
        <div class="px-4 pb-5">
          <!-- Environment Number -->
          <div class="mb-4 flex  items-center gap-4">
            <label class="block text-sm text-gray-700 mb-1 text-right" style="width: 60px;">环境编号</label>
            <p
                style="width: 50px;"
                class="bg-blue-100 text-blue-600  w-12 rounded-md   px-2 py-1 text-center focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            > {{envNumber}}</p>
          </div>

          <!-- Group Selection -->
          <div class="mb-6 flex items-center gap-4">
            <label class="block text-sm text-gray-700 mb-1 text-right" style="width: 60px;">环境分组</label>

            <div class="relative" style="width: 450px;">
              <Select v-model="selectedGroup" class="" >
                <!-- 修改宽度 -->
                <!-- <SelectTrigger class="w-32"> -->
                <SelectTrigger>
                  <SelectValue
                      placeholder="查找分组/创建分组"
                      class="w-full p-2 rounded-lg outline-none"
                  />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem v-for="group in groups" :key="group.id" :value="group.id">
                      {{ group.name }}
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>

            </div>

          </div>

          <!-- Footer -->
          <div class="flex justify-center gap-2">
            <button
                @click="closeModal"
                class="rounded-md bg-blue-600 px-6 py-2 text-sm text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              确定
            </button>
            <button
                @click="closeModal"
                class="rounded-md bg-gray-100 px-6 py-2 text-sm text-gray-600 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
            >
              取消
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { XIcon, ChevronDownIcon } from 'lucide-vue-next'
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/select";

const props = defineProps({
  open: false,
  title: '',
  initialEnvNumber: {
    type: [String, Number],
    default: '7'
  }
})

const emit = defineEmits(['close', 'confirm'])

const envNumber = ref(props.initialEnvNumber)
const selectedGroup = ref('')

// Mock groups data - replace with actual data in your implementation
const groups = ref([
  { id: 1, name: '分组 1' },
  { id: 2, name: '分组 2' },
  { id: 3, name: '分组 3' }
])

const closeModal = () => {
  emit('close')
}

const handleConfirm = () => {
  emit('confirm', {
    envNumber: envNumber.value,
    groupId: selectedGroup.value
  })
}
</script>