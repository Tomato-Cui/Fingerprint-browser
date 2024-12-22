<script setup>
import { cn } from '@/util/lib'
import { onMounted, ref } from 'vue';
import { Sortable } from 'sortablejs'
import { useRoute } from 'vue-router';
import {
    SidebarMenuButton,
    SidebarMenuItem,
} from "../ui/sidebar/index";

const props = defineProps({
    menu: [],
});

const hsSortable = ref(null);
const route = useRoute();

onMounted(() => {
    new Sortable(hsSortable.value, {
        animation: 150,
        handle: '.hs-handle',
        dragClass: '!rounded-none'
    });
});
console.log(route.path);

</script>

<template>
    <ul ref="hsSortable" class="max-w-xs flex flex-col hs-handle ">
        <SidebarMenuItem v-for="item in props.menu" :key="item.title" class="ml-2 mt-2 group ">
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
</template>