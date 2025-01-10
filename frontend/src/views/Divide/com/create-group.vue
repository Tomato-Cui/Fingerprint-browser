<script setup>
import { Model } from "@/components/model/index";
import { PrimaryButton, CancelButton } from "@/components/button";
import { ref, defineEmits, defineProps, reactive, watch } from 'vue'
import { UserPlus2Icon, UsersIcon } from 'lucide-vue-next'
import { IconCreateTeam, IconJoinTeam } from "@/assets/icons";
import { environment_group_create, environment_group_query } from "@/commands/environment-group";
import { toast } from "vue-sonner";
import Input from "@/components/input.vue";

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
const errorRepeat = ref(false)  //分组名称是否重复
//确认
const subMit = () => {
    if (addGroup.groupName === "") {
        toast.error("团队名称不能为空")
        return
    }

    // environment_group_query(1, 1000).then(res => {
    //     console.log("add", addGroup.description);
        
    //     res.data.data.forEach(item => {
    //         console.log(item.name === addGroup.groupName);
            
    //         if (item.name === addGroup.groupName) {
    //             errorRepeat.value = true
    //             console.log("分组名称重复");
                
    //             return
    //         }
    //     })
    // })
    environment_group_create(addGroup.groupName, addGroup.description).then(res => {
        // console.log("res", res)
        toast.message(res.message)
    })
    emit('update:createGroupDialog', false)
}
watch(() => props.createGroupDialog, (val) => {
    clearForm()
})
</script>

<template>
    <!-- 创建团队 -->
    <Model class="" title="添加环境分组" :open="props.createGroupDialog"
        @close="() => emit('update:createGroupDialog', false)">
        <!-- 主体 -->
        <div class="flex flex-col mb-6 gap-y-4 px-2">
            <div class="flex items-center gap-4">
                <label class="w-[80px] flex justify-end">分组名称</label>
                <div class="w-full">
                    <Input v-model="addGroup.groupName" type="text" placeholder="请输入分组名称" class="flex-1 w-full" />
                    <p v-show="errorRepeat" class="text-red-500 text-sm">分组名称重复</p>
                </div>
            </div>
            <div class="flex items-center gap-4">
                <label class="w-[80px] flex justify-end">备注</label>
                <Input v-model="addGroup.description" type="text" placeholder="请输入备注" class="flex-1" />
            </div>
        </div>

        <div class="flex justify-end border-t">
            <div class="flex justify-start py-4 gap-x-4 mr-2">
                <PrimaryButton class="px-8" @click="subMit">确定
                </PrimaryButton>
                <CancelButton class="px-8" @click="() => emit('update:createGroupDialog', false)">取消
                </CancelButton>
            </div>
        </div>
    </Model>
</template>