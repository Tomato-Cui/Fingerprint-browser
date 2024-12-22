<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineProps, defineEmits, onMounted } from 'vue'

const props = defineProps({
    switchTeam: Boolean
})
const emit = defineEmits(['update:switchTeam'])
import { XIcon, CheckIcon } from 'lucide-vue-next'

const teams = ref([
  { id: 1, name: '12343234' },
  { id: 2, name: '这是测试' },
  { id: 3, name: 'wrewg' },
  { id: 4, name: 'wrewg' },
  { id: 5, name: 'wrewg' },
  { id: 6, name: 'wrewg' },
  { id: 7, name: 'wrewg' },
  { id: 8, name: 'wrewg' },
  { id: 9, name: 'wrewg' },
  { id: 10, name: 'wrewg' },
])

const selectedTeam = ref(2) // Default selected team

const selectTeam = (teamId) => {
  selectedTeam.value = teamId
}

const confirmSelection = () => {
  if (selectedTeam.value) {
    console.log('Selected team:', selectedTeam.value)
    // Emit selected team to parent
    emit('select', selectedTeam.value)
  }
}
</script>

<template>
    <!-- class: 这里class的宽度就是弹出框的宽度.请自行设置 -->
    <Model class="min-w-[800px]" :title="'切换团队'" :open="props.switchTeam" @close="() => emit('update:switchTeam', false)">
        <div class="space-x-4 pt-6 flex">
            <!-- 在这里书写弹出框主题内容代码 -->
            <div class="px-6 py-4 w-full">
                <p class="text-gray-600 mb-4">请选择一个团队登录</p>

                <!-- Team List -->
                <div class="overflow-y-auto pr-2 mb-6 custom-scrollbar h-[300px]">
                    <div v-for="team in teams" :key="team.id"
                        class="mb-3 p-3 rounded-lg border cursor-pointer transition-all" :class="{
                            'border-blue-500 bg-blue-50': selectedTeam === team.id,
                            'border-gray-200 hover:border-gray-300': selectedTeam !== team.id
                        }" @click="selectTeam(team.id)">
                        <div class="flex justify-between items-center">
                            <span class="text-gray-900">{{ team.teamName }}</span>
                            <CheckIcon v-if="selectedTeam === team.id" class="w-5 h-5 text-blue-500" />
                        </div>
                    </div>
                </div>


                <div class="">
                    <div class="flex justify-start py-8 gap-x-4">
                        <PrimaryButton class="px-8" @click="() => emit('update:switchTeam', false)">确定
                        </PrimaryButton>
                        <CancelButton class="px-8" @click="() => emit('update:switchTeam', false)">取消
                        </CancelButton>
                    </div>
                </div>
            </div>
        </div>
    </Model>
</template>