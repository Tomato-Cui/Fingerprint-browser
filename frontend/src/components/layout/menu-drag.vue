<script setup>
import { cn } from '@/util/lib'
import { onMounted, ref } from 'vue';
import { Sortable } from 'sortablejs'
import { useRoute } from 'vue-router';
import {
    SidebarMenuButton,
    SidebarMenuItem,
} from "../ui/sidebar/index";
import { environment_group_query } from '@/commands/environment-group';
import { ChevronDown, Search as SearchIcon, Folder as FolderIcon, Component as ComponentIcon } from 'lucide-vue-next'
import { PrimaryButton } from "@/components/button";
import CreateGroup from "@/views/Divide/com/create-group.vue"
import { watch } from 'vue';

const props = defineProps({
    menu: [],
});

const hsSortable = ref(null);
const route = useRoute();
const menus = ref([])
const createGroupDialog = ref(false)  // 创建分组

const createGroup = () => {
    createGroupDialog.value = true
};
watch(createGroupDialog, (val) => {
    
    if (!val) {
        getList()
    }
})

const getList = () => {
    environment_group_query(1, 1000).then(res => {
        //获取到的数组依次添加到props.menu中
        res.data.data.forEach(item => {
            menus.value.push(item)
        })

        //添加之后，处理字段
        menus.value = menus.value.map(item => {
            return {
                id: item.id,
                title: item.name,
                url: `/divide/${item.id}`,
                icon: ComponentIcon,
                activeIcon: ComponentIcon
            }
        })
    }).catch(err => {
        console.log(err)
    })
}

onMounted(() => {
    getList()

    new Sortable(hsSortable.value, {
        animation: 150,
        handle: '.hs-handle',
        dragClass: '!rounded-none'
    });
});
</script>

<template>
    <!-- {{ menus }} -->
    <ul ref="hsSortable" class="max-w-xs flex flex-col hs-handle max-h-[230px] overflow-auto">
        <SidebarMenuItem v-for="item in menus" :key="item.id" class="ml-2 mt-2 group ">
            <SidebarMenuButton asChild :class="`  text-[16px] rounded-md h-12 cursor-grab min-h-12 flex justify-start items-center px-6 text-[#858C94] ${item.title == '分组管理' ? 'h-10 min-h-10' : ''} ${item.url == route.path && 'bg-white text-primary'
                }`">
                <router-link :to="item.url">
                    <component :is="item.url == route.path ? item.activeIcon : item.icon" :class="cn('shrink-0 text-transparent group-hover:text-[#858C94] size-4 dark:text-neutral-500 transform ease-in-out delay-50 duration-300',
                        item.url == route.path && 'bg-white text-primary'
                    )" />
                    <span>{{ item.title }}</span>
                </router-link>
            </SidebarMenuButton>
        </SidebarMenuItem>
    </ul>
    <!-- 创建分组 -->
    <CreateGroup v-model:createGroupDialog="createGroupDialog" />
</template>