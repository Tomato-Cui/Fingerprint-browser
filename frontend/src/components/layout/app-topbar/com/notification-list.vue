<!-- 消息列表 -->
<script setup>
import { ref } from 'vue'
import { user_receive_query, user_allow } from '@/commands/user-team-temp'
import { query_current_team_info } from "@/commands/team"
import { toast } from 'vue-sonner'

//同意邀请
const allowInv = (note) => {
    user_allow(note.id, note.team_id).then(res => {
        toast.message(res.message)
        getList()
    })
}

//消息列表
const noteList = ref([])

const getList = () => {
    user_receive_query(1, 10000).then(res => {
        noteList.value = res.data.data
        console.log("noteList:::", noteList.value);
    })
}
getList()
</script>

<template>
    <div class="w-full h-full">
        <ul class="w-full h-full overflow-auto">
            <li v-for="note in noteList" class="w-full border mb-3 h-[90px] px-2 py-4 rounded-xl shadow-sm">
                <div class="h-full flex items-center">
                    <img src="../../../../assets/images/note-avatar.jpg" alt="" class="h-full w-[58px]">
                    <div class="flex flex-col justify-around ml-2 h-full w-[100px]">
                        <span class="text-sm truncate">{{ note.team_name }}</span>
                        <span class="font-medium truncate">{{ note.description }}</span>
                    </div>
                    <div class="flex flex-1 justify-end" v-if="note.allow_1 === 0">
                        <button class="border w-[100px] h-[40px] rounded-xl hover:bg-blue-100 transition-color duration-500" @click="allowInv(note)">立即加入</button>
                    </div>
                    <div class="flex flex-1 justify-end" v-else>
                        <span class="h-[30px] w-[70px] bg-green-200 flex items-center justify-center rounded-md">已同意</span>
                    </div>
                </div>
            </li>
        </ul>
    </div>
</template>