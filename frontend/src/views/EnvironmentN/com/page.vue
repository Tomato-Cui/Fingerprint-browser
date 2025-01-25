<script setup lang="ts">
import { ref, computed, defineProps, defineEmits } from 'vue'
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { AltArrowDownIcon } from "@/assets/icons/environment/index.ts"
const props = defineProps({
    total: {
        type: Number,
        default: 0
    },
    modelValue: {
        type: Number,
        default: 1
    }
})
const emit = defineEmits(['update:modelValue', 'update:pageSize'])
const currentPage = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
})
const pageSize = ref(17)
const pageSizeOptions = [3, 17, 20, 50, 100]
const isLastPage = computed(() => {
    return currentPage.value * pageSize.value >= props.total
})
const prevPage = () => {
    if (currentPage.value > 1) {
        currentPage.value--
    }
}
const nextPage = () => {
    if (!isLastPage.value) {
        currentPage.value++
    }
}
const togglePageSizeDropdown = () => {
    // showPageSizeDropdown.value = !showPageSizeDropdown.value
}
const selectPageSize = (size: number) => {
    pageSize.value = size
    emit('update:pageSize', size)
    // emit("update:modelValue", 1)
    // showPageSizeDropdown.value = false
    console.log();

}
</script>

<template>
    <div class="flex items-center space-x-4 text-sm h-12 justify-end border-t">
        <!-- Total count -->
        <span class="text-gray-500">共 {{ total }} 项</span>

        <!-- Navigation -->
        <div class="flex items-center space-x-2">
            <button @click="prevPage" :disabled="currentPage === 1" :class="{ 'cursor-not-allowed': currentPage == 1 }"
                class="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 hover:bg-gray-100 disabled:opacity-50">
                <AltArrowDownIcon class="h-4 w-4 rotate-90" />
            </button>

            <div class="flex h-8 min-w-[32px] items-center justify-center rounded-lg bg-gray-100 px-3">
                {{ currentPage }}
            </div>

            <button @click="nextPage" :disabled="isLastPage" :class="{ 'cursor-not-allowed': isLastPage }"
                class="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 hover:bg-gray-100 disabled:opacity-50">
                <AltArrowDownIcon class="h-4 w-4 -rotate-90" />
            </button>
        </div>

        <!-- Page size selector -->
        <div class="relative">
            <!-- Dropdown -->
            <More class="w-full">
                <MoreTrigger @click="" class="w-full">
                    <button @click="togglePageSizeDropdown"
                        class="flex items-center space-x-1 rounded-lg border px-3 py-1.5 hover:bg-gray-50 w-[108px]">
                        <span>{{ pageSize }}条/页</span>
                        <AltArrowDownIcon class="h-4 w-4 transition-transform rotate-180"/>
                    </button>
                </MoreTrigger>
                <MoreContent class="min-w-[100px]">
                    <MoreItem v-for="size in pageSizeOptions" :key="size" class="cursor-pointer"
                        @click="selectPageSize(size)">
                        {{ size }}条/页
                    </MoreItem>
                </MoreContent>
            </More>
        </div>
    </div>
</template>