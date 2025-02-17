<script setup lang="ts">
import { AlertModel } from "@/components/alert-model";
import { environment_tag_query, environment_tag_delete } from "@/commands/environment-tag";
import { toast } from "vue-sonner";

const props = defineProps<{
    open: Boolean
}>()

const emit = defineEmits(['close'])
const submit = () => {
    environment_tag_query(1, 10000).then(res => {
        res.data.data.forEach((tag: any) => {
            environment_tag_delete(tag.id)
        })
    })
    toast.success('清空成功')
    emit('close')
}
</script>

<template>
    <AlertModel title="删除提醒" :open="props.open" @close="emit('close')" @cancel="emit('close')" @submit="submit">
        <p class="h-[70px]">
            是否确认清空已选环境的全部标签？
        </p>
    </AlertModel>
</template>