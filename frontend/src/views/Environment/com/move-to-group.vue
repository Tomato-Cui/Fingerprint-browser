<script setup>
import { onMounted, ref, watch } from "vue";
import { Model } from "@/components/model/index";
import Input from "@/components/input.vue";
import { PrimaryButton, CancelButton } from "@/components/button";
import { user_query_search_by_email } from "@/commands/user";
import { environment_batch_move_to_group } from "@/commands/environment";
import { environment_group_query } from "@/commands/environment-group";
import { SearchX } from "lucide-vue-next";
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/select";
import Button from "@/components/ui/button/Button.vue";
import { toast } from "vue-sonner";

const searchOn = ref(false);
const searchList = ref([]);
const selectGroupId = ref("");
const props = defineProps({
    open: false,
    title: "",
    name: [],
    uuid: [],
});
watch(() => props.open, async (v) => {
    // console.log("name", props.name);
    // props.name = [];
    
    if (v) {
        getList();
    }else{
    }
});

const emit = defineEmits(["close", "submit", "load"]);
const onSubmit = () => {
    if(!selectGroupId.value){
        toast.error("请选择分组");
        return;
    }
    environment_batch_move_to_group(
        props.uuid, //环境ID
        selectGroupId.value //分组ID
    ).then((res) => {
        if (res.code == 1) {
            toast.success("移动成功");
        } else {
            toast.error("移动失败");
        }
        emit("close");
    });
};

const getList = () => {
    environment_group_query(1, 1000).then((res) => {
        searchList.value = res.data.data;
        // selectGroupId.value = (searchList.length > 0 && searchList[0]?.id) || 0;
    });
}

</script>

<template>
    <Model class="max-w-[725px]" :title="props.title" :open="props.open" @close="() => emit('close')">
        <div class="flex flex-col gap-y-4 items-center w-full">
            <div class="flex gap-x-2 items-center px-2 w-3/4">
                <span class="whitespace-nowrap w-[120px] flex justify-end">目标环境:</span>
                <div class="w-full flex flex-wrap">
                    <span v-for="item in props.name" class="bg-blue-200 m-1 px-2 py-1 rounded-sm">{{ item }}</span>
                </div>
            </div>
            <div class="flex gap-x-2 items-center px-2 w-3/4">
                <span class="whitespace-nowrap w-[120px] flex justify-end">移动到分组:</span>
                <div class="flex gap-x-2 w-full">
                    <Select v-model="selectGroupId">
                        <SelectTrigger class="w-full">
                            <SelectValue :placeholder="'请选择分组'" class="p-2 rounded-lg outline-none" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup class="max-h-[200px] overflow-auto">
                                <SelectItem :value="item.id" :key="item" v-for="item in searchList">
                                    {{ item.name }}
                                </SelectItem>
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                </div>
            </div>
        </div>
        <div class="flex gap-x-2 justify-end pr-10 my-2 w-full">
            <CancelButton @click="() => emit('close')">取消</CancelButton>
            <PrimaryButton @click="onSubmit">移动</PrimaryButton>
        </div>
    </Model>
</template>
