<template>
    <Model :open="props.open" @close="emit('close')" title="新增标签" description="输入后按下回车将标签加入">
        <!-- Content -->
        <div class="p-6 space-y-2">
            <!-- <div class="text-sm text-gray-600">已选择项目 {{ props.data?.length }}</div> -->

            <!-- Tag Input -->
            <div class="relative flex gap-4 items-center">
                <span class="min-w-fit">新增标签</span>
                <div class="min-h-[40px] w-full border rounded-md px-3 py-2 flex items-center gap-2 focus-within:border-indigo-500"
                    @focusin="search" @focusout="() => { if (!colorSelectionActive) inputFocused = false }">
                    <div v-for="item in tagList"
                        class="bg-[#F3F3FF] px-2 py-1 rounded flex items-center gap-1 text-sm min-w-fit`"
                        :class="[`bg-${selectedColor}-50`]">
                        <span :class="[`text-${selectedColor}-600`]">{{ item.name }}</span>
                        <XIcon class="w-4 h-4 cursor-pointer" @click="removeTag(item.id)" />
                    </div>
                    <input v-model="tagInput" type="text" placeholder="新增标签" class="flex-1 outline-none text-sm"
                        @keydown.enter="addTag" />
                </div>
            </div>
            <!-- {{ inputFocused || currentTag }} -->
            <!-- {{ searchList.length }} -->

            <!-- Color Selection -->
            <transition enter-active-class="transition ease-out duration-200"
                enter-from-class="opacity-0 transform scale-95" enter-to-class="opacity-100 transform scale-100"
                leave-active-class="transition ease-in duration-150" leave-from-class="opacity-100 transform scale-100"
                leave-to-class="opacity-0 transform scale-95">
                <div v-if="searchList.length != 0 && inputFocused" class="space-y-3 border rounded-xl p-2 shadow-lg"
                    @mouseenter="colorSelectionActive = true" @mouseleave="colorSelectionActive = false">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center gap-2 text-sm cursor-pointer" @click="addTag">
                            <PlusIcon class="w-4 h-4" />
                            <span>新建</span>
                        </div>

                        <!-- <div class="flex items-center gap-3">
                            <button v-for="color in colors" :key="color.value"
                                @click="selectedColor = color.value; inputFocused = true;"
                                class="w-6 h-6 rounded-full border-2 transition-all duration-200" :class="[
                                    color.class,
                                    selectedColor === color.value ? 'border-gray-400 scale-110' : 'border-transparent'
                                ]"></button>
                        </div> -->
                    </div>

                    <div class="flex gap-2 p-3 rounded flex-col max-h-[200px] overflow-auto">
                        <span v-for="item in searchList"
                            class="bg-blue-100 text-blue-400 border-blue-400 border max-w-fit px-2 py-1 rounded-md">{{
                            item.name }}</span>
                    </div>
                </div>
            </transition>
        </div>

        <!-- Footer -->
        <div class="flex justify-end gap-3 p-4 border-t">
            <button @click="close" class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800">
                取消
            </button>
            <button @click="confirm" class="px-4 py-2 text-sm text-white bg-indigo-600 rounded hover:bg-indigo-700">
                确认
            </button>
        </div>
    </Model>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { XIcon, PlusIcon } from 'lucide-vue-next'
import { Model } from '@/components/model'
import { toast } from 'vue-sonner'
import { environment_tag_create, environment_tag_query } from '@/commands/environment-tag'

const props = defineProps({
    open: Boolean,
    data: Array,
    selectedCount: {
        type: Number,
        default: 1
    }
})
watch(() => props.open, (_) => {
    selectedColor.value = 'indigo'
    tagList.value = [],
        tagInput.value = ''
})

const emit = defineEmits(['close', 'confirm'])

// const colors = [
//     { value: 'gray', class: 'bg-gray-200' },
//     { value: 'blue', class: 'bg-blue-200' },
//     { value: 'green', class: 'bg-green-200' },
//     { value: 'yellow', class: 'bg-yellow-200' },
//     { value: 'pink', class: 'bg-pink-200' },
//     { value: 'purple', class: 'bg-purple-200' },
//     { value: 'orange', class: 'bg-orange-200' },
//     { value: 'indigo', class: 'bg-indigo-200' }
// ]
const tagList = ref<{ id: number; name: string }[]>([])
const searchList = ref<any[]>([])

const tagInput = ref('')
const selectedColor = ref('indigo')
const inputFocused = ref(false)
const colorSelectionActive = ref(false)

const addTag = () => {
    if (!tagInput.value) {
        toast.warning('标签不能为空')
        return
    }

    environment_tag_create(tagInput.value, tagInput.value).then(res => {
        toast.success(res.message)
        getList()
        tagInput.value = ''
    })
}
const removeTag = (id: number) => {
    tagList.value = tagList.value.filter(tag => tag.id !== id);
};

const search = () => {
    inputFocused.value = true
    getList()
}

const close = () => {
    emit('close')
}

const confirm = () => {
    // if (tagList.value.length === 0) {
    //     toast.error('标签不能为空')
    //     return
    // }

    close()
}

const getList = () => {
    environment_tag_query(1, 100).then(res => {
        searchList.value = res.data.data
    })
}

watch(() => props.open, (v) => {
    if (v) {
        // getList()
    }
})
</script>