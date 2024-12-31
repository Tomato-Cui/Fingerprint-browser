<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineEmits, defineProps, reactive, watch } from 'vue'
import { UserPlus2Icon, UsersIcon } from 'lucide-vue-next'
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";
import { environment_group_create } from "@/commands/environment-group";
import { toast } from "vue-sonner";

const props = defineProps({
    createGroupDialog: Boolean
})
const emit = defineEmits(['update:createGroupDialog'])

const addGroup = reactive({
    groupName: "",
    description: ""
})
//清空表单
const clearForm = () => {
    addGroup.groupName = ""
    addGroup.description = ""
}
//确认
const subMit = () => {
    emit('update:createGroupDialog', false)
    if(addGroup.groupName === "") {
        toast.error("团队名称不能为空")
        return
    }
    environment_group_create(addGroup.groupName, addGroup.description).then(res => {
        // console.log("res", res)
        toast.message(res.message)
    })
}
watch(() => props.createGroupDialog, (val) => {
    clearForm()
})
</script>

<template>
    <!-- 创建团队 -->
    <Model class="" :title="'创建团队'" :open="props.createGroupDialog" @close="() => emit('update:createGroupDialog', false)">
        <div class="space-x-4 p-4 flex flex-col">
            <!-- 在这里书写弹出框主题内容代码 -->

            <!-- 主体 -->
            <div class="h-[100px] flex flex-col justify-between mb-6">
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">分组名称</label>
                    <input v-model="addGroup.groupName" type="text" placeholder="请输入团队名称"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
                <div class="flex justify-center items-center gap-4">
                    <label class="w-[80px] flex justify-end">备注</label>
                    <input v-model="addGroup.description" type="text" placeholder="请输入备注"
                        class="border border-gray-400 rounded-sm h-[40px] px-5" />
                </div>
            </div>

            <div class="flex justify-center border-t">
                <div class="flex justify-start py-8 gap-x-4">
                    <PrimaryButton class="px-8" @click="subMit">确定
                    </PrimaryButton>
                    <CancelButton class="px-8" @click="() => emit('update:createGroupDialog', false)">取消
                    </CancelButton>
                </div>
            </div>
        </div>
    </Model>
</template>