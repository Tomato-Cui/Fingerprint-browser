<script setup>
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "../ui/sidebar/index";
import { Collapsible, CollapsibleTrigger, CollapsibleContent } from "../ui/collapsible/index";
import { ChevronDown, Search as SearchIcon, Folder as FolderIcon, Component as ComponentIcon } from 'lucide-vue-next'
import { useRoute } from "vue-router";
import MenuDrag from './menu-drag.vue'
import { onMounted, watch } from "vue";
import { environment_group_query } from '@/commands/environment-group';
import { team_is_leader, query_current_team_info } from "@/commands/team";
import { toast } from "vue-sonner";
import { ref } from "vue";
import { environment_proxy_group_query } from '@/commands/environment-proxy-group';
// import { Search as SearchIcon } from 'lucide-vue-next'
// import Input from '@/components/input.vue'

const props = defineProps(['menus'])
const route = useRoute();
const isLeader = ref(true)

const envMenus = ref([]) //环境分组子菜单
const proxyMenus = ref([])  //代理分组子菜单

const getList = () => {
    // 查询环境子菜单
    environment_group_query(1, 1000).then(res => {
        //获取到的数组依次添加到props.menu中
        res.data.data.forEach(item => {
            envMenus.value.push(item)
        })

        //添加之后，处理字段
        envMenus.value = envMenus.value.map(item => {
            return {
                id: item.id,
                title: item.name,
                url: `/divide/${item.id}`,
                icon: ComponentIcon,
                activeIcon: ComponentIcon
            }
        })

        props.menus?.forEach(item => {
            if (item.title == '环境分组') {
                item.children = envMenus.value
            } 
        })
    }).catch(err => {
        console.log(err)
    })

    // // 查询代理分组子菜单
    // environment_proxy_group_query(1, 1000).then(res => {
    //     //获取到的数组依次添加到props.menu中
    //     res.data.data.forEach(item => {
    //         proxyMenus.value.push(item)
    //     })

    //     //添加之后，处理字段
    //     proxyMenus.value = proxyMenus.value.map(item => {
    //         return {
    //             id: item.id,
    //             title: item.name,
    //             url: `/mydaili/${item.id}`,
    //             icon: ComponentIcon,
    //             activeIcon: ComponentIcon
    //         }
    //     })
    // }).catch(err => {
    //     console.log(err)
    // })
}

onMounted(() => {
    getList()

    query_current_team_info().then(res => {
        team_is_leader(res.data.id).then(res2 => {
            isLeader.value = res2.data
        })
    })
})
</script>

<template>
    <SidebarMenuItem v-for="item in menus" :key="item.title">
        <SidebarMenuButton v-if="!item.children" asChild :class="`text-[16px] hover:bg-white  rounded-md h-12 min-h-12 flex justify-start items-center px-6 text-[#858C94] ${item.url == route.path && 'bg-white text-primary'
            }`">
            <router-link :to="item.url">
                <component :is="item.url == route.path ? item.activeIcon : item.icon"></component>
                <span>{{ item.title }}</span>
            </router-link>
        </SidebarMenuButton>
        <Collapsible v-else class="group/collapsible">
            <SidebarGroup>
                <CollapsibleTrigger class="w-full flex items-center  hover:bg-white  rounded-md">
                    <div :class="`text-[16px] rounded-md h-12 min-h-14 flex justify-start items-center w-full px-6
                            text-[#858C94] ${item.url == route.path && 'bg-white text-primary'}`">
                        <component class="w-4 h-4" :is="item.url == route.path ? item.activeIcon : item.icon" />
                        <span class="ml-2">{{ item.title }}</span>
                    </div>
                    <ChevronDown className="transition-transform group-data-[state=open]/collapsible:rotate-180"
                        color="#858C94" />
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <SidebarGroupContent>
                        <SidebarGroupContent>
                            <SidebarMenu class="flex flex-col max-h-[240px] overflow-auto">
                                <MenuDrag :menu="item.children" />
                                <SidebarMenuItem
                                    v-for="i in item.children.filter(i => isLeader || (i?.title != '分组管理' && i?.title != '成员申请' && i?.title != '黑名单'))"
                                    :key="i.title" class="ml-2 mt-2">
                                    <SidebarMenuButton asChild :class="`hover:bg-white text-[16px] rounded-md h-12 min-h-12 flex justify-start items-center px-6 text-[#858C94] ${item.title == '分组管理' ? 'h-10 min-h-10' : ''} ${i.url == route.path && 'bg-white text-primary'
                                        }`">
                                        <router-link :to="i.url">
                                            <component :is="i.url == route.path ? i.activeIcon : i.icon" />
                                            <span>{{ i.title }}</span>
                                        </router-link>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarGroupContent>
                    </SidebarGroupContent>
                </CollapsibleContent>
            </SidebarGroup>
        </Collapsible>
    </SidebarMenuItem>
</template>