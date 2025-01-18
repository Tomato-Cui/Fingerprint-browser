<template>
    <div class="px-4 py-3 font-semibold text-[20px]">帮助中心</div>
    <div class="min-h-screen flex">
        <!-- Sidebar -->
        <aside class="w-64 bg-white border-r border-gray-200">
            <nav class="p-4">
                <div v-for="item in menuItems" :key="item.id"
                    @click="() => { activeItem = item.id, router.push(item.href) }" :class="[
                        'flex items-center justify-between px-4 py-3 rounded-lg mb-1 cursor-pointer transition-colors duration-150',
                        activeItem === item.id
                            ? 'bg-[#5050FA] text-white'
                            : 'text-gray-700 hover:bg-gray-100'
                    ]">
                    <div class="flex items-center gap-3">
                        <component :is="item.icon" class="w-5 h-5" />
                        <span>{{ item.name }}</span>
                    </div>
                    <span class="text-sm">{{ item.count }}</span>
                </div>
            </nav>
        </aside>

        <!-- Main Content -->
        <main class="flex-1 p-8">
            <!-- Breadcrumb -->
            <nav class="flex items-center space-x-2 text-sm mb-6">
                <span>帮助中心</span>
                <span>/</span>
                <span class="text-gray-900">{{ getCurrentTitle }}</span>
            </nav>

            <h1 class="text-2xl font-semibold text-gray-900 mb-6">{{ getCurrentTitle }}</h1>
            <slot />
        </main>
    </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router';
import { CloseIcon } from '@/assets/icons/synchronizer';
import { ApiIcon, CostIcon, CreateEnvIcon, EnvironmentIcon, ExpandIcon, FAQIcon, IntentIcon, QuickIcon, SysIcon, TeamIcon } from "@/assets/icons/system-operation/help/index"

const router = useRouter();
const menuItems = [
    { id: 'quickstart', name: '快速入门', icon: QuickIcon, count: 5, href: '/quickStart' },
    { id: 'environment', name: '创建环境', icon: CreateEnvIcon, count: 5, href: '/quickStart' },
    { id: 'management', name: '环境管理', icon: EnvironmentIcon, count: 5, href: '/quickStart' },
    { id: 'network', name: '网络设置', icon: IntentIcon, count: 5, href: '/quickStart' },
    { id: 'extensions', name: '扩展与应用', icon: ExpandIcon, count: 5, href: '/quickStart' },
    { id: 'sync', name: '同步器', icon: SysIcon, count: 5, href: '/quickStart' },
    { id: 'api', name: 'API', icon: ApiIcon, count: 5, href: '/quickStart' },
    { id: 'team', name: '团队管理', icon: TeamIcon, count: 5, href: '/quickStart' },
    { id: 'billing', name: '费用', icon: CostIcon, count: 5, href: '/quickStart' },
    { id: 'faq', name: '常见问题', icon: FAQIcon, count: 5, href: '/quickStart' }
]
const activeItem = ref('quickstart')

const getCurrentTitle = computed(() => {
    const item = menuItems.find(item => item.id === activeItem.value)
    return item ? item.name : ''
})


</script>