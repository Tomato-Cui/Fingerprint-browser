<template>
    <div class="h-main bg-gray-50 p-4 flex">
        <div class="bg-white rounded-xl flex-1 px-4 py-2 w-full">
            <!-- Header Section -->
            <header class="mb-4 flex items-center justify-between">
                <div class="flex items-center space-x-4 w-auto">
                    <h1 class="text-xl font-semibold w-[105px]">环境管理</h1>
                    <div class="w-full">
                        <div class="flex items-center justify-between px-6 h-[35px] space-x-14">
                            <div class="space-x-6 rounded-md h-11 px-1 bg-gray-100 md:flex">
                                <button class="px-3 my-1 rounded-md whitespace-nowrap text-primary"
                                    :class="'bg-[#FFFFFFFF]', 'shadow-lg'">浏览器环境</button>
                                <button class="px-3 my-1 rounded-md whitespace-nowrap cursor-not-allowed">云手机环境</button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex items-center space-x-4">
                    <div class="flex flex-col relative">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center border rounded-lg pl-2">
                                <SearchIcon class="size-5" />
                                <input type="search" placeholder="搜索"
                                    class="rounded-md px-3 py-1.5 text-sm outline-none w-[260px] h-[35px]" />
                            </div>
                            <button @click="filterOpt"
                                class="flex border border-gray-300 px-2 py-1 gap-2 rounded-lg h-[35px] items-center">
                                <component :is="FilterIcon" class="size-5" />筛选
                            </button>
                        </div>
                        <!-- 筛选 -->
                        <SearchChoose :open="showSearchChoose" @close="showSearchChoose = false"
                            class="absolute top-10 shadow-xl border" />
                    </div>

                    <button v-for="item in headOperate" @click="item.go"
                        class="flex border border-gray-300 px-2 py-1 gap-2 rounded-lg h-[35px] items-center">
                        <component :is="item.icon" class="size-5" />
                        {{ item.label }}
                    </button>
                </div>
            </header>

            <!-- Tabs -->
            <div class="mb-4 border-b">
                <button class="border-b-2 border-blue-600 px-4 py-2 text-sm font-medium text-blue-600">
                    默认分组
                </button>
                <button class="px-4 py-2 text-sm text-gray-600">
                    测试分组
                </button>
            </div>

            <!-- Action Buttons -->
            <div class="mb-4 flex items-center space-x-2">
                <OpenBrowserIcon class="w-[106px] h-[43px]" />
                <div class="flex flex-1 overflow-x-auto space-x-2">
                    <button v-for="action in visibleActions" :key="action.key" v-show="action.key !== 'start'"
                        class="flex items-center rounded-md bg-white px-3 py-1.5 text-sm shadow-sm hover:bg-gray-50 min-w-fit h-[43px] border gap-2">
                        <component :is="action.icon" class="mr-1.5 h-4 w-4" />
                        {{ action.label }}
                    </button>
                </div>

                <div class="relative">
                    <button @click="showQuickSettings = !showQuickSettings" class="">
                        <MoreOperatorIcon class="size-10" />
                    </button>
                    <!-- Quick Settings Modal -->
                    <Operate :open="showQuickSettings" @close="showQuickSettings = false" @select="operateSelect"
                        class="absolute right-0 top-0 w-[320px]" />
                </div>
            </div>

            <!-- Table -->
            <div class="rounded-lg bg-white shadow">
                <table class="min-w-full">
                    <thead class="w-full">
                        <tr class="border-b bg-gray-50 ">
                            <th class="w-12 py-3 pl-4">
                                <input type="checkbox" class="rounded border-gray-300" />
                            </th>
                            <th v-for="column in visibleColumns" :key="column.key"
                                class="px-4 py-3 text-left text-sm font-medium text-gray-600">
                                {{ column.label }}
                            </th>
                            <th class="px-4 py-3 text-right relative">
                                <button @click="showColumnsModal = !showColumnsModal">
                                    <WrapperIcon class="size-5" />
                                </button>
                                <!-- Columns Modal -->
                                <TableTheadChoose :open="showColumnsModal" @close="showColumnsModal = false"
                                    @select="theadChoose" class="absolute right-0 top-0 w-[320px]" />
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="item in tableData" :key="item.id" class="border-b">
                            <td class="py-3 pl-4">
                                <input type="checkbox" class="rounded border-gray-300" />
                            </td>
                            <td class="px-4 py-3 text-sm">{{ item.id }}</td>
                            <!-- <td class="px-4 py-3 text-sm">{{ item.name }}</td>
                            <td class="px-4 py-3">
                                <button class="rounded bg-blue-50 px-3 py-1 text-sm text-blue-600">
                                    启动
                                </button>
                            </td>
                            <td class="px-4 py-3">
                                <span :class="{
                                    'flex items-center text-sm': true,
                                    'text-blue-600': item.status === 'ready',
                                    'text-green-600': item.status === 'complete',
                                    'text-red-600': item.status === 'not_ready'
                                }">
                                    <div :class="{
                                        'mr-1.5 h-2 w-2 rounded-full': true,
                                        'bg-blue-600': item.status === 'ready',
                                        'bg-green-600': item.status === 'complete',
                                        'bg-red-600': item.status === 'not_ready'
                                    }"></div>
                                    {{ statusText[item.status] }}
                                </span>
                            </td>
                            <td class="px-4 py-3 text-sm">{{ item.account }}</td>
                            <td class="px-4 py-3 text-right">
                                <button>
                                </button>
                            </td> -->
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import Operate from "./com/operate.vue"
import TableTheadChoose from "./com/table-thead-choose.vue"
import SearchChoose from "./com/search-choose.vue"
import { FilterIcon, SearchIcon, GroupIcon, BookmarkIcon, ApiIcon, SynchronizerIcon, OpenBrowserIcon, MoreOperatorIcon, WrapperIcon } from "@/assets/icons/environment/index.ts"

const showSearchChoose = ref(false)
const showQuickSettings = ref(false)
const showColumnsModal = ref(false)

const allActions = ref([])  //全部操作
const allColumns = ref([])  //全部列

const operateSelect = (val) => {
    allActions.value = val
}
const theadChoose = (val) => {
    allColumns.value = val
}

const visibleActions = computed(() => {
    // console.log("??:", allActions.value);
    return allActions.value.filter(action => action.visible)
})
const visibleColumns = computed(() => {
    return allColumns.value.filter(column => column.visible)
})

const filterOpt = () => {
    showSearchChoose.value = !showSearchChoose.value
}

const headOperate = ref([
    {
        label: '分组管理',
        icon: GroupIcon,
        go: 0,  //前往分组管理
    },
    {
        label: '书签',
        icon: BookmarkIcon,
        go: 0,
    },
    {
        label: 'api',
        icon: ApiIcon,
        go: 0,
    },
    {
        label: '同步器',
        icon: SynchronizerIcon,
        go: 0,
    }
])

const statusText = {
    ready: '准备好',
    complete: '完成',
    not_ready: '未准备好'
}

const tableData = ref([
    { id: 1, name: 'P-1', operate: '启动', status: 'ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', createTime: '2025-1-13 20:21:54', startTime: '2025-1-13 20:22:12' },
    { id: 2, name: 'P-1', operate: '启动', status: 'complete', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', createTime: '2025-1-13 20:21:54', startTime: '2025-1-13 20:22:12' },
    { id: 3, name: 'P-1', operate: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', createTime: '2025-1-13 20:21:54', startTime: '2025-1-13 20:22:12' },
])


const saveColumnSettings = () => {
    showColumnsModal.value = false
    // Save column settings logic here
}
</script>