<template>
    <Teleport to="body" class="z-[60]">
        <div v-if="props.invMember"
            class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-[60]">
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
                <div class="bg-white rounded-lg w-[600px]">
                    <!-- Header -->
                    <div class="flex justify-between items-center p-4 border-b">
                        <h2 class="text-lg font-medium">邀请成员</h2>
                        <button @click="closeModal" class="text-gray-400 hover:text-gray-600 transition-colors">
                            <XIcon class="w-5 h-5" />
                        </button>
                    </div>

                    <!-- Content -->
                    <div class="p-6">
                        <p class="text-gray-600 mb-6 text-sm">
                            成员可通过在加入团队中输入团队码的方式，在审核之后加入团队
                        </p>

                        <!-- Team Selection -->
                        <div class="mb-6 flex items-center justify-between gap-5">
                            <label class="block text-sm text-gray-600 mb-2">分组:</label>
                            <div class="relative flex-1">
                                <Select v-model="selectedTeam" class="">
                                    <!-- 修改宽度 -->
                                    <!-- <SelectTrigger class="w-32"> -->
                                    <SelectTrigger>
                                        <SelectValue placeholder="请选择分组" class="w-full p-2 rounded-lg outline-none" />
                                    </SelectTrigger>
                                    <SelectContent class="z-[80]">
                                        <SelectGroup>
                                            <SelectItem v-for="team in teams" :value="team.id">
                                                {{ team.name }}
                                            </SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select>
                                <!-- <select v-model="selectedTeam"
                                    class="w-full appearance-none bg-gray-50 border border-gray-200 rounded-md px-4 py-2 pr-8 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                                    <option value="" disabled>请选择接邀请成员的分组</option>
                                    <option v-for="team in teams" :key="team.id" :value="team.id">
                                        {{ team.name }}
                                    </option>
                                </select>
                                <ChevronDownIcon
                                    class="absolute right-2 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 pointer-events-none" /> -->
                            </div>
                            <button
                                class="w-[130px] bg-blue-500 hover:bg-blue-600 text-white rounded-md py-2 px-4 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                                :class="{ 'cursor-not-allowed opacity-50': (!isClicked1 && !isClicked2) || selectedTeam === 0 }"
                                @click="(isClicked1 || isClicked2) && selectedTeam != 0 ? copyTeamCode(0) : null"
                                >
                                <!-- {{ !isClicked1 && !isClicked2 && selectedTeam === 0 }} -->
                                复制{{ isClicked1 ? '团队码' : (isClicked2 ? '链接' : '内容') }}
                            </button>

                            <!-- <button @click="copyTeamCode(1)"
                                class=" bg-blue-500 hover:bg-blue-600 text-white rounded-md py-2 px-4 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
                                复制连接
                            </button> -->
                        </div>

                        <!-- Team Code Display -->
                        <div class="relative mb-6">
                            <div class="h-[100px] bg-gray-50 rounded-lg p-8 flex items-center justify-center border"
                                :class="{ 'border-blue-600': isClicked1 }" @click="choseSelect1">
                                <span v-if="!selectedTeam" class="text-gray-300 text-2xl font-medium">团队码</span>
                                <span v-else>{{ valueCode }}</span>
                            </div>
                            <div class="h-[65px] bg-gray-50 rounded-lg py-4 flex justify-center my-5 border"
                                :class="{ 'border-blue-600': isClicked2 }" @click="choseSelect2">
                                <span v-if="!selectedTeam" class="text-gray-300 text-2xl font-medium">邀请链接</span>
                                <span v-else>{{ textLink }}</span>
                            </div>

                            <!-- <div class="bg-gray-50 rounded-lg py-4 flex justify-center my-5 text-blue-400">
                                <span v-if="!selectedTeam" class="text-gray-300 text-2xl font-medium">邀请链接</span>
                                <span v-else class="border-b border-blue-500">{{ textLink }}</span>
                            </div> -->
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<script setup>
import { ref, defineEmits, defineProps, onMounted, watch } from 'vue';
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
    invMember: Boolean
})
const emit = defineEmits(['update:invMember'])
const InvValue = ref(null)
const textToCopy = ref('')  //团队码
const textLink = ref('http://192.168.6.115:5173/')  //链接

const selectedTeam = ref(0)
const teams = ref([
    {id: 0, name: "请选择分组"},
    { id: 1, name: '团队 A' },
    { id: 2, name: '团队 B' },
    { id: 3, name: '团队 C' },
])
const valueCode = ref()
watch(() => selectedTeam.value, (val) => {
    valueCode.value = getCode(6)
})

const closeModal = () => {
    // Emit close event to parent
    selectedTeam.value = ''
    console.log("取消");

    emit('update:invMember', false)
}

const copyTeamCode = async (index) => { //复制团队码
    try {
        if (index === 0) {
            await navigator.clipboard.writeText(textToCopy.value);
            alert('团队码已复制')
        } else {
            await navigator.clipboard.writeText(textLink.value);
            alert('链接已复制')
        }
    } catch (err) {
        console.error('Failed to copy text: ', err);
        alert('复制失败');
    }
}



const getCode = (length) => {  //生成团队码
    // console.log("获取团队码");
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    let result = '';
    for (let i = 0; i < length; i++) {
        const randomIndex = Math.floor(Math.random() * characters.length);
        result += characters.charAt(randomIndex);
    }
    textToCopy.value = result
    return result
}

const isClicked1 = ref(false)
const choseSelect1 = () => {
    // console.log("选择当前项---");

    isClicked1.value = !isClicked1.value
    if (isClicked1.value) {
        isClicked2.value = false
    }
}

const isClicked2 = ref(false)
const choseSelect2 = () => {
    isClicked2.value = !isClicked2.value
    if (isClicked2.value) {
        isClicked1.value = false
    }
    // console.log("选择当前项---:", isClicked2.value);
}
</script>

<style>
.team-code {
    width: 100%;
    height: 80px;
    font-size: 28px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #d2d7e1;
}
</style>