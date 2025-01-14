<template>
    <div class="bg-white rounded-xl flex-1 px-4 py-2 w-full h-full flex flex-col">
        <!-- Header Section -->
        <header class="mb-4 flex items-center justify-between h-[40px]">
            <div class="flex items-center space-x-4 w-auto">
                <h1 class="text-xl font-semibold w-[105px]">环境管理</h1>
                <!-- <div class="w-full">
                    <div class="flex items-center justify-between px-6 h-[35px] space-x-14">
                        <div class="space-x-6 rounded-md h-11 px-1 bg-gray-100 md:flex">
                            <button class="px-3 my-1 rounded-md whitespace-nowrap text-primary"
                                :class="'bg-[#FFFFFFFF]' + ' ' + 'shadow-lg'">浏览器环境</button>
                            <button class="px-3 my-1 rounded-md whitespace-nowrap cursor-not-allowed">云手机环境</button>
                        </div>
                    </div>
                </div> -->
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

                <div class="flex gap-2 relative">
                    <button v-for="item in headOperate" @click="item.go"
                        class="flex border border-gray-300 px-2 py-1 gap-2 rounded-lg h-[35px] items-center">
                        <component :is="item.icon" class="size-5" />
                        {{ item.label }}
                    </button>
                    <SynchronizerIndex :open="synchronizerDialog" @close="synchronizerDialog = false"
                        class="absolute top-10" />
                </div>
            </div>
        </header>

        <!-- Tabs -->
        <div class="mb-4 border-b h-[40px]">
            <button class="border-b-2 border-blue-600 px-4 py-2 text-sm font-medium text-blue-600">
                默认分组
            </button>
            <button class="px-4 py-2 text-sm text-gray-600">
                测试分组
            </button>
        </div>

        <!-- Action Buttons -->
        <div class="mb-4 flex items-center space-x-2 h-[60px]">
            <OpenBrowserIcon class="w-[106px] h-[43px]" />
            <div class="flex flex-1 overflow-x-auto space-x-2">
                <button v-for="action in visibleActions" :key="action.key" v-show="action.key !== 'start'"
                    class="flex items-center rounded-md bg-[#EDEDFF] px-3 py-[2px] text-sm shadow-sm hover:bg-gray-50 min-w-fit h-[35px] gap-2">
                    <component :is="action.icon" class="mr-1.5 h-4 w-4" />
                    <span>{{ action.label }}</span>
                    <AltArrowDownIcon v-show="action.children" class="size-5" />
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
        <div class="rounded-lg bg-white flex-1 overflow-auto flex flex-col">
            <table class="min-w-full">
                <thead class="w-full">
                    <tr class="border-b bg-gray-50">
                        <th class="w-12 py-3 pl-4 text-left">
                            <input type="checkbox" class="rounded border-gray-300" :checked="isAllSelected"
                                @change="toggleSelectAll" :indeterminate="isIndeterminate" />
                        </th>
                        <th v-for="column in visibleColumns" :key="column.key"
                            class="px-2 py-3 text-left text-sm font-medium text-gray-600">
                            <div class="flex items-center justify-between border-r">
                                <span>{{ column.label }}</span>
                            </div>
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
                    <tr v-for="(item, index) in filterData" :key="index" class="border-b">
                        <td class="py-3 pl-4">
                            <input type="checkbox" class="rounded border-gray-300"
                                :checked="selectedItems.includes(item.id)" @change="toggleSelectItem(item.id)" />
                        </td>
                        <td class="px-2 py-3">{{ index + 1 }}</td>
                        <!-- 每一格 -->
                        <td v-for="(value, key) in item as Record<string, any>" :key="key" class="px-2 py-3"
                            :class="{ 'min-w-[150px] cursor-pointer': key === 'action' }"
                            @click="key === 'action' ? handleTableOperate(index) : void (0)">
                            <!-- 操作格 -->
                            <div v-if="key === 'action'" class="flex justify-between items-center ">
                                <span class="border px-2 py-1 rounded-lg border-[#5050fa] bg-[#EDEDFF]">{{ value
                                    }}</span>
                                <span class="">
                                    <OneFrameIcon class="size-5 relative" />
                                    <TableOperate :open="tableOperateModal" @close="tableOperateModal = false"
                                        v-if="index === chooseIndex" class="top-[14px] right-[18px]" />
                                </span>
                            </div>
                            <!-- 其他格 -->
                            <div v-else class="flex justify-between items-center ">
                                <span>{{ value || '--' }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
            <div v-show="tableData.length == 0" class="flex items-center justify-center flex-col space-y-3 flex-1">
                <AddCardIcon class=" size-[200px]  text-blue-400 border-gray-300 flex items-center justify-center" />
                <p>您可创建具有独立高质量指纹的profile，也可自定义编辑指纹信息</p>
                <button @click="router.push('/environment-action/create')"
                    class="p-4 bg-[#5050FA] text-white rounded-lg flex items-center gap-3">
                    <AssCircleIcon class="size-5" />
                    新建环境
                </button>
            </div>
        </div>

        <!-- page -->
        <div class="flex items-center space-x-4 text-sm h-12 justify-end border-t">
            <!-- Total count -->
            <span class="text-gray-500">共 {{ total }} 项</span>

            <!-- Navigation -->
            <div class="flex items-center space-x-2">
                <button @click="prevPage" :disabled="currentPage === 1"
                    class="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 hover:bg-gray-100 disabled:opacity-50">
                    <AltArrowDownIcon class="h-4 w-4 rotate-90" />
                </button>

                <div class="flex h-8 min-w-[32px] items-center justify-center rounded-lg bg-gray-100 px-3">
                    {{ currentPage }}
                </div>

                <button @click="nextPage" :disabled="isLastPage"
                    class="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 hover:bg-gray-100 disabled:opacity-50">
                    <AltArrowDownIcon class="h-4 w-4 -rotate-90" />
                </button>
            </div>

            <!-- Page size selector -->
            <div class="relative">
                <button @click="togglePageSizeDropdown"
                    class="flex items-center space-x-1 rounded-lg border px-3 py-1.5 hover:bg-gray-50">
                    <span>{{ pageSize }}条/页</span>
                    <AltArrowDownIcon class="h-4 w-4 transition-transform"
                        :class="{ 'rotate-180': showPageSizeDropdown }" />
                </button>

                <!-- Dropdown -->
                <div v-if="showPageSizeDropdown"
                    class="absolute right-0 mt-1 w-24 rounded-lg border bg-white py-1 shadow-lg">
                    <button v-for="size in pageSizeOptions" :key="size" @click="selectPageSize(size)"
                        class="w-full px-3 py-1.5 text-left hover:bg-gray-50"
                        :class="{ 'text-blue-600': pageSize === size }">
                        {{ size }}条/页
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import Operate from "./com/operate.vue"
import TableTheadChoose from "./com/table-thead-choose.vue"
import SearchChoose from "./com/search-choose.vue"
import TableOperate from './com/table-operate.vue'
// import Auth from './synchronizer/auth.vue'
import SynchronizerIndex from './synchronizer/index.vue'
import { AssCircleIcon, AddCardIcon, FilterIcon, SearchIcon, GroupIcon, BookmarkIcon, ApiIcon, SynchronizerIcon, OpenBrowserIcon, MoreOperatorIcon, WrapperIcon, AltArrowDownIcon } from "@/assets/icons/environment/index.ts"
import { environment_query } from '@/commands/environment'
import { useRouter } from 'vue-router'
import { OneFrameIcon } from '@/assets/icons/environment/index.ts'

const router = useRouter()
const showSearchChoose = ref(false)
const showQuickSettings = ref(false)
const showColumnsModal = ref(false)
const tableOperateModal = ref(false)
const synchronizerDialog = ref(false)
watch(() => tableOperateModal.value, (val) => {
    // console.log("bianh：", chooseIndex.value);
})

const allActions = ref<any>([])  //全部操作
const allColumns = ref<any>([])  //全部列

const operateSelect = (val: any) => {
    allActions.value = val
}
const theadChoose = (val: any) => {
    allColumns.value = val
}

const chooseIndex = ref(0)
const handleTableOperate = (index: number) => {
    tableOperateModal.value = true
    // console.log("HHHH:", tableData.value[index].id);
    if (chooseIndex.value != index) {
        chooseIndex.value = index
    }
}

const visibleActions = computed(() => {
    // console.log("??:", allActions.value);
    return allActions.value.filter((action: any) => action.visible)
})
const visibleColumns = computed(() => {
    return allColumns.value.filter((column: any) => column.visible)
})

const filterOpt = () => {
    showSearchChoose.value = !showSearchChoose.value
}
// 打开同步器
const openSynchronizer = () => {
    synchronizerDialog.value = !synchronizerDialog.value
}

const headOperate = ref([
    {
        label: '分组管理',
        icon: GroupIcon,
        go: filterOpt,  //前往分组管理
    },
    {
        label: '书签',
        icon: BookmarkIcon,
        go: filterOpt,
    },
    {
        label: 'api',
        icon: ApiIcon,
        go: filterOpt,
    },
    {
        label: '同步器',
        icon: SynchronizerIcon,
        go: openSynchronizer,
    }
])

// const statusText = {
//     ready: '准备好',
//     complete: '完成',
//     not_ready: '未准备好'
// }

const tableData = ref([
    { id: 1, name: 'P-1', action: '启动', status: 'ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    { id: 2, name: 'P-1', action: '启动', status: 'complete', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
    //     { id: 3, name: 'P-1', action: '启动', status: 'not_ready', account: '--', proxy: '--', description: '测试', tab: '--', groupName: '--', create_at: '2025-1-13 20:21:54', lastOpen: '2025-1-13 20:22:12' },
])

const filterData = computed(() => {
    return tableData.value.map((item: any) => {
        // 获取 visibleColumns 中的所有 key
        const visibleKeys = visibleColumns.value.map((col: any) => col.key);

        // 过滤 item 中的键，保留 item 中的 key 在 visibleKeys 中的键值对
        return Object.keys(item).reduce((filteredItem: any, key: string) => {
            if (visibleKeys.includes(key) && key != 'id') {
                filteredItem[key] = item[key]; // 仅包含 visibleColumns 中的 key 和对应的值
            }
            return filteredItem;
        }, {});
    });
});

//分页
const props = defineProps({
    total: {
        type: Number,
        default: 0
    },
    modelValue: {
        type: Number,
        default: 1
    }
})
const emit = defineEmits(['update:modelValue', 'update:pageSize'])
const currentPage = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
})
const pageSize = ref(10)
const pageSizeOptions = [10, 20, 50, 100]
const showPageSizeDropdown = ref(false)
const isLastPage = computed(() => {
    return currentPage.value * pageSize.value >= props.total
})
const prevPage = () => {
    if (currentPage.value > 1) {
        currentPage.value--
    }
}
const nextPage = () => {
    if (!isLastPage.value) {
        currentPage.value++
    }
}
const togglePageSizeDropdown = () => {
    showPageSizeDropdown.value = !showPageSizeDropdown.value
}
const selectPageSize = (size: number) => {
    pageSize.value = size
    emit('update:pageSize', size)
    showPageSizeDropdown.value = false
}

// 多选
const selectedItems = ref<number[]>([])
const isAllSelected = computed(() => {
    return tableData.value.length > 0 && selectedItems.value.length === tableData.value.length
})
const isIndeterminate = computed(() => {
    return selectedItems.value.length > 0 && selectedItems.value.length < tableData.value.length
})
const toggleSelectAll = (event: Event) => {
    const target = event.target as HTMLInputElement
    if (target.checked) {
        selectedItems.value = tableData.value.map(item => item.id)
    } else {
        selectedItems.value = []
    }
}
const toggleSelectItem = (itemId: number) => {
    const index = selectedItems.value.indexOf(itemId)
    if (index === -1) {
        selectedItems.value.push(itemId)
    } else {
        selectedItems.value.splice(index, 1)
    }
}
defineExpose({
    selectedItems
})



const getList = () => {
    environment_query(1, 10).then((res: any) => {
        // tableData.value = res.data
        console.log("res:", res.data);
        // 将字段与表头对上
        tableData.value = res.data.data.map((item: any) => {
            return {
                id: item.id,
                name: item.name,
                action: '启动',
                status: '启动中',
                account: '账号',
                proxy: item.proxy_username,
                description: item.description,
                tab: '标签',
                groupName: item.group_name,
                create_at: item.created_at,
                lastOpen: '最后启动',
                uuid: item.uuid,
            }
        })

        console.log("table:", tableData.value);
        
    })
}
onMounted(() => {
    getList()
})
</script>