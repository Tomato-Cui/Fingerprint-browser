<script setup lang="ts">
import { Model } from '@/components/model';
import { reactive } from 'vue';
import primaryButton from '@/components/button/primary-button.vue';
import cancelButton from '@/components/button/cancel-button.vue';
import Input from '@/components/input.vue';
import { environment_group_create } from '@/commands/environment-group'
import { toast } from 'vue-sonner';

const props = defineProps<{
    open: Boolean
}>()
const emit = defineEmits(['close', 'searchGroup'])

const form = reactive({
    groupName: "",
    description: "",
})
const cleanFrom = () => {
    form.description = ""
    form.groupName = ""
}

const commit = () => {
    if (!form.groupName) {
        toast.warning('分组名称不能为空')
        return
    }
    environment_group_create(form.groupName, form.description).then((res: any) => {
        toast.success(res.message)
        emit('close')
        cleanFrom()
        // window.location.reload()
        emit('searchGroup')
    })
}
</script>

<template>
    <Model :open="props.open" title="新建分组">
        <div class="w-full space-y-3 px-6 py-4">
            <div class="flex w-full items-center">
                <div>分组名称：</div>
                <Input v-model="form.groupName" class="flex-1" placeholder="请输入分组名称"/>
            </div>
            <div class="flex w-full items-center">
                <div>分组备注：</div>
                <Input v-model="form.description" class="flex-1" placeholder="请输入备注"/>
            </div>
        </div>
        <!-- footer -->
        <div class="w-full px-4 py-2 flex justify-end items-center border-t">
            <cancelButton @click="emit('close')">取消</cancelButton>
            <primaryButton @click="commit" @keydown.enter="commit">确定</primaryButton>
        </div>
    </Model>
</template>