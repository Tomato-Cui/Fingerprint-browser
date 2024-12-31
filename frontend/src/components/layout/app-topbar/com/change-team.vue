<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineProps, defineEmits, onMounted } from 'vue'
import { team_query, switch_team, query_current_team_info } from "@/commands/team"
import { useRouter } from "vue-router";
import { defineAsyncComponent } from 'vue';
import { XIcon, CheckIcon } from 'lucide-vue-next'
import { toast } from "vue-sonner";

const router = useRouter()
const props = defineProps({
    switchTeam: Boolean,
    isLoadDialog: Boolean
})
const emit = defineEmits(['update:switchTeam', 'update:isLoadDialog'])
const teams = ref([
    { id: 1, name: '12343234' },
    { id: 2, name: '这是测试' },
    { id: 3, name: 'wrewg' },
])

const selectedTeam = ref(2) // Default selected team

const selectTeam = (teamId) => {
    selectedTeam.value = teamId
}

const isLoad = ref(true)
const confirmSelection = () => {
    emit('update:switchTeam', false)
    switch_team(selectedTeam.value).then(res => {
        toast.message(res.message)
        router.push('/environment')
        getList()
        // console.log("teams----:", teams.value);
        emit("update:isLoadDialog", true)  //开启加载效果
        setTimeout(() => {
            emit("update:isLoadDialog", false)  //关闭加载效果
        }, 1000)
        //刷新
        window.location.reload()
    })
}

const getList = () => {
    team_query(1, 1000).then(res => {
        teams.value = res.data.data
    });
    query_current_team_info().then(res => {
        selectedTeam.value = res.data.id
    })
}

onMounted(() => {
    getList()
})
</script>

<template>
    <!-- class: 这里class的宽度就是弹出框的宽度.请自行设置 -->
    <Model class="min-w-[800px]" :title="'切换团队'" :open="props.switchTeam"
        @close="() => emit('update:switchTeam', false)">
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
                            <span class="text-gray-900">{{ team.name }}</span>
                            <CheckIcon v-if="selectedTeam === team.id" class="w-5 h-5 text-blue-500" />
                        </div>
                    </div>
                </div>

                <div class="">
                    <div class="flex justify-start py-8 gap-x-4">
                        <PrimaryButton class="px-8" @click="confirmSelection">确定
                        </PrimaryButton>
                        <CancelButton class="px-8" @click="() => emit('update:switchTeam', false)">取消
                        </CancelButton>
                    </div>
                </div>
            </div>
        </div>
    </Model>
    
</template>