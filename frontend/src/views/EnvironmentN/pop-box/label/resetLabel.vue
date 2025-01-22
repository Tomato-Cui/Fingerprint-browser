<template>
    <Model :open="props.open" title="重设标签" @close="emit('close')">
        <!-- Content -->
        <div class="p-6 space-y-4">
            <div class="text-sm text-gray-600">已选择项目 {{ selectedCount }}</div>

            <!-- Combobox -->
            <div class="relative flex items-center gap-4">
                <div class="flex items-center gap-2 mb-2">
                    <span class="font-medium">重设标签</span>
                </div>

                <div class="relative flex-1">
                    <div class="w-full border rounded-md px-3 py-2 flex flex-wrap gap-2 min-h-[40px] focus-within:border-blue-500"
                        @click="isOpen = true">
                        <div v-for="tag in selectedTags" :key="tag.id"
                            class="bg-blue-50 text-blue-600 px-2 py-1 rounded text-sm flex items-center gap-1">
                            <span>{{ tag.name }}</span>
                            <XIcon class="w-3 h-3 cursor-pointer" @click.stop="removeTag(tag)" />
                        </div>
                        <input v-model="searchQuery" type="text" placeholder="选择或输入标签"
                            class="flex-1 outline-none min-w-[120px]" @focus="isOpen = true" />
                    </div>

                    <!-- Dropdown -->
                    <transition enter-active-class="transition ease-out duration-200"
                        enter-from-class="opacity-0 transform scale-95" enter-to-class="opacity-100 transform scale-100"
                        leave-active-class="transition ease-in duration-150"
                        leave-from-class="opacity-100 transform scale-100"
                        leave-to-class="opacity-0 transform scale-95">
                        <div v-if="isOpen && filterList.length > 0"
                            class="absolute left-0 right-0 mt-1 bg-white border rounded-md shadow-lg max-h-[200px] overflow-auto z-50">
                            <div v-for="tag in filterList" :key="tag.id" @click="selectTag(tag)"
                                class="flex items-center px-3 py-2 hover:bg-gray-50 cursor-pointer">
                                <div
                                    class="max-w-fit bg-blue-50 border border-blue-400 text-blue-400 rounded-md px-2 py-1 text-sm">
                                    {{ tag.name }}
                                </div>
                                <CheckIcon v-if="isSelected(tag)" class="w-4 h-4 text-blue-600 ml-2" />
                            </div>
                        </div>
                    </transition>
                </div>
            </div>
        </div>

        <!-- Footer -->
        <div class="flex justify-end gap-3 p-4 border-t">
            <button @click="close" class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800">
                取消
            </button>
            <button @click="confirm" class="px-4 py-2 text-sm text-white bg-blue-600 rounded hover:bg-blue-700">
                确认
            </button>
        </div>
    </Model>
</template>

<script setup lang="ts">
import { Model } from '@/components/model'
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { XIcon, CheckIcon } from 'lucide-vue-next'
import { environment_tag_query } from '@/commands/environment-tag'
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/select";

const props = defineProps({
    open: Boolean,
    data: Array,
    selectedCount: {
        type: Number,
        default: 1
    }
})

const emit = defineEmits(['close', 'confirm'])

// Sample tags data
const availableTags = ref<any>([])
//根据searchQueryguol结果
const filterList = computed(() => {
    return availableTags.value.filter((tag: any) => tag.name.includes(searchQuery.value))
})

const isOpen = ref(false)
const searchQuery = ref('')
const selectedTags = ref<Array<any>>([])

const selectTag = (tag: any) => {
    selectedTags.value = []
    // if (!isSelected(tag)) {
    selectedTags.value.push(tag)
    // }
    // searchQuery.value = ''
    isOpen.value = false
}

const removeTag = (tag: { id: number; name: string; color: string }) => {
    selectedTags.value = selectedTags.value.filter(t => t.id !== tag.id)
}

const isSelected = (tag: { id: number; name: string; color: string }) => {
    return selectedTags.value.some(t => t.id === tag.id)
}

const close = () => {
    emit('close')
}

const confirm = () => {
    emit('confirm', selectedTags.value)
    close()
}

// Close dropdown when clicking outside
const handleClickOutside = (event: MouseEvent) => {
    const target = event.target as HTMLElement
    if (!target.closest('.relative')) {
        isOpen.value = false
    }
}

onMounted(() => {
    document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
})

const getList = () => {
    environment_tag_query(1, 100).then(res => {
        availableTags.value = res.data.data
    })
}
watch(() => props.open, (v) => {
    if (v) {
        getList()
    } else {
        selectedTags.value = []
    }
})
</script>