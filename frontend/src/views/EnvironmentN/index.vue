<template>
    <div class="bg-white rounded-xl flex-1 px-4 py-2 w-full h-full flex flex-col">
        <!-- Header Section -->
        <header class="mb-4 flex items-c enter justify-between h-[40px]">
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
                            <input type="search" placeholder="搜索" v-model="searchQuery"
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
                        class="flex border text-[14px] border-gray-300 px-2 py-1 gap-2 rounded-lg h-[35px] items-center hover:bg-blue-50">
                        <component :is="item.icon" class="size-5" />
                        {{ item.label }}
                    </button>
                    <SynchronizerIndex :open="synchronizerDialog" @close="synchronizerDialog = false"
                        class="absolute top-10" />
                </div>
            </div>
        </header>

        <!-- Tabs -->
        <GroupChoose class="flex flex-col flex-1">
            <!-- Action Buttons -->
            <div class="mb-4 flex items-center space-x-2 h-[60px]">
                <!-- <OpenBrowserIcon class="w-[106px] h-[43px]" :class="{'cursor-not-allowed': selectedItems.length==0, 'hover:bg-gray-50 cursor-pointer': selectedItems.length!=0}" @click="selectedItems.length!=0 ? startAll() : void(0)" /> -->
                <div class="w-[100px] h-[40px] bg-[#7744ff] flex rounded-lg items-center relative"
                    :class="{ 'cursor-not-allowed opacity-50': selectedItems.length == 0, 'cursor-pointer': selectedItems.length != 0 }"
                    @click="selectedItems.length != 0 ? startAll() : void (0)">
                    <RoundArrowRight class="size-6 mx-3" />
                    <span class="text-white absolute left-12">启动</span>
                    <div class="flex-1 flex justify-end">
                        <HelfGlobalIcon class="size-[51px] opacity-10" />
                    </div>
                </div>
                <div class="flex flex-1 overflow-x-auto space-x-2">
                    <!-- <button v-for="action in visibleActions" :key="action.key" v-show="action.key !== 'start'"
                        @click="selectedItems.length != 0 ? action.action : void (0)"
                        :class="{ 'cursor-not-allowed opacity-50': selectedItems.length == 0, 'hover:bg-gray-50': selectedItems.length != 0 }"
                        class="flex items-center rounded-md bg-[#EDEDFF] px-3 py-[2px] text-sm shadow-sm min-w-fit h-[35px] gap-2">
                        <component :is="action.icon" class="mr-1.5 h-4 w-4" />
                        <span>{{ action.label }}</span>
                        <AltArrowDownIcon v-show="action.children" class="size-5" />
                    </button> -->

                    <More class="flex min-w-fit" v-for="action in visibleActions" :key="action.key">
                        <MoreTrigger class="min-w-fit">
                            <button v-show="action.key !== 'start'"
                                @click="selectedItems.length != 0 ? action.action : void (0)"
                                :class="{ 'cursor-not-allowed opacity-50': selectedItems.length == 0, 'hover:bg-gray-50': selectedItems.length != 0 }"
                                class="flex items-center rounded-md bg-[#EDEDFF] px-3 py-[2px] text-sm shadow-sm min-w-fit h-[35px] gap-2">
                                <component :is="action.icon" class="mr-1.5 h-4 w-4" />
                                <span>{{ action.label }}</span>
                                <AltArrowDownIcon v-show="action.children" class="size-5" />
                            </button>
                        </MoreTrigger>
                        <MoreContent class="w-[140px]" v-if="action.children && selectedItems.length > 0">
                            <MoreItem v-for="item in action.children" :key="item" class="cursor-pointer" :class="{'hover:bg-white': item.key === 'addGroup'}">
                                <div v-if="item.key === 'addGroup'" class="border border-[#5050FA] bg-[#EDEDFF] w-full p-1 rounded-sm flex items-center justify-center"><GroupAddIcon class="size-5"/>{{ item.label }}</div>
                                <div v-else>{{ item.label }}</div>
                            </MoreItem>
                        </MoreContent>
                    </More>
                </div>

                <div class="relative">
                    <button @click="showQuickSettings = !showQuickSettings" class="">
                        <MoreOperatorIcon class="size-10" />
                    </button>
                    <!-- Quick Settings Modal -->
                    <Operate :open="showQuickSettings" @close="showQuickSettings = false" @select="operateSelect"
                        @stopAll="stopAll" class="absolute right-0 top-0 w-[320px]" />
                </div>
            </div>

            <!-- Table -->
            <div class="rounded-lg bg-white flex-1 overflow-auto flex flex-col">
                <table class="min-w-full">
                    <!-- {{ sortColumn }} -->
                    <thead class="w-full sticky top-0 z-10">
                        <tr class="border-b bg-gray-50">
                            <th class="w-12 py-3 pl-4 text-left">
                                <input type="checkbox" class="rounded border-gray-300" :checked="isAllSelected"
                                    @change="toggleSelectAll" :indeterminate="isIndeterminate" />
                            </th>
                            <template v-for="column in visibleColumns" :key="column.key">
                                <th v-if="column.key === 'id'"
                                    class="px-2 py-3 text-left text-sm font-medium text-gray-600 cursor-pointer"
                                    @click="handleSort('id')">
                                    <div class="flex items-center justify-between border-r">
                                        <span>序号</span>
                                        <div class="mr-2">
                                            <CaretDownIcon class="size-3" />
                                            <CaretUpIcon class="size-3" />
                                        </div>
                                    </div>
                                </th>
                                <th v-if="column.key === 'name'"
                                    class="px-2 py-3 text-left text-sm font-medium text-gray-600 cursor-pointer"
                                    @click="handleSort('name')">
                                    <div class="flex items-center justify-between border-r">
                                        <span>环境名称</span>
                                        <div class="mr-2">
                                            <CaretDownIcon class="size-3" />
                                            <CaretUpIcon class="size-3" />
                                        </div>
                                    </div>
                                </th>
                                <th v-if="column.key !== 'id' && column.key !== 'name'"
                                    class="px-2 py-3 text-left text-sm font-medium text-gray-600">
                                    <div class="flex items-center justify-between border-r">
                                        <span>{{ column.label }}</span>
                                    </div>
                                </th>
                            </template>
                            <th class="px-4 py-3 text-right relative w-8">
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
                        <tr v-for="(item, index) in filterData" :key="index" class="border-b"
                            :class="{ 'bg-[#EDF1F7]': selectedItems.includes(item.uuid) }">
                            <td class="py-3 pl-4">
                                <input type="checkbox" class="rounded border-gray-300"
                                    :checked="selectedItems.includes(item.uuid)"
                                    @change="toggleSelectItem(item.uuid)" />
                            </td>
                            <!-- <td class="px-2 py-3">{{ item }}</td> -->
                            <!-- 每一格 -->
                            <template v-for="(value, key) in item as Record<string, any>" :key="key">
                                <td class="px-2 py-3 text-sm" v-if="key != 'uuid'">
                                    <!-- 操作格 -->
                                    <div v-if="key === 'action'" class="flex justify-between items-center ">
                                        <span @click="true ? startEnv(item.uuid) : void (0)"
                                            :class="{ 'border-[#5050fa] bg-[#EDEDFF] text-blue-500': value == '启动', 'border-red-300 bg-red-50 text-red-500': value == '停止' }"
                                            class="border w-[70px] flex justify-center py-1 rounded-lg border-[#5050fa] bg-[#EDEDFF] cursor-pointer hover:bg-blue-100">{{
                                                value
                                            }}</span>
                                        <span class="">
                                            <More class="top-8 relative">
                                                <MoreTrigger>
                                                    <OneFrameIcon class="h-5 w-5 cursor-pointer" />
                                                </MoreTrigger>
                                                <MoreContent class="min-w-[200px]">
                                                    <!-- 小尖头 -->
                                                    <!-- <div
                                                class="size-4 bg-white border-l border-t transform rotate-45 absolute top-[-7px] left-5 z-[99999]">  
                                            </div> -->
                                                    <MoreItem v-for="item in regularItems" :key="item"
                                                        class="cursor-pointer" @click="">
                                                        {{ item }}
                                                    </MoreItem>
                                                </MoreContent>
                                            </More>
                                        </span>
                                    </div>
                                    <!-- 其他格 -->
                                    <div v-else class="flex justify-between items-center text-sm">
                                        <span>{{ value || '--' }}</span>
                                    </div>
                                </td>
                            </template>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
                <div v-show="filterData.length == 0" class="flex items-center justify-center flex-col space-y-3 flex-1">
                    <AddCardIcon
                        class=" size-[200px]  text-blue-400 border-gray-300 flex items-center justify-center" />
                    <p>您可创建具有独立高质量指纹的profile，也可自定义编辑指纹信息</p>
                    <button @click="router.push('/environment-action/create')"
                        class="p-2 bg-[#5050FA] text-white rounded-lg flex items-center gap-3">
                        <AssCircleIcon class="size-5" />
                        新建环境
                    </button>
                </div>
            </div>

            <!-- page -->
            <Page :total="PageObj.total" :modelValue="PageObj.pageNum" @update:model-value="getPageNum"
                @update:page-size="getPageSize" />
        </GroupChoose>

    </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted, onUnmounted } from 'vue'
import Operate from "./com/operate.vue"
import TableTheadChoose from "./com/table-thead-choose.vue"
import SearchChoose from "./com/search-choose.vue"
import Page from './com/page.vue'
import GroupChoose from './com/group-choose.vue'
// import Auth from './synchronizer/auth.vue'
import SynchronizerIndex from './synchronizer/index.vue'
import { AssCircleIcon, AddCardIcon, FilterIcon, SearchIcon, GroupIcon, BookmarkIcon, ApiIcon, SynchronizerIcon, GroupAddIcon, MoreOperatorIcon, WrapperIcon, AltArrowDownIcon, CaretDownIcon, CaretUpIcon } from "@/assets/icons/environment/index.ts"
import { HelfGlobalIcon, RoundArrowRight } from '@/assets/icons/environment/index'
import { environment_query, environment_query_by_group } from '@/commands/environment'
import { useRouter, useRoute } from 'vue-router'
import { OneFrameIcon } from '@/assets/icons/environment/index.ts'
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { browser_starts, browser_stops, browser_start } from '@/commands/browser'
import { toast } from 'vue-sonner'

const router = useRouter()
const route = useRoute()
const showSearchChoose = ref(false)
const showQuickSettings = ref(false)
const showColumnsModal = ref(false)
const tableOperateModal = ref(false)
const synchronizerDialog = ref(false)
const searchQuery = ref('')
const sortColumn = ref('id');
const sortOrder = ref('asc');

watch(() => tableOperateModal.value, (val) => {
    // console.log("bianh：", chooseIndex.value);
})

const allActions = ref<any>([])  //全部操作
const allColumns = ref<any>([])  //全部列
const regularItems = [
    "刷新指纹",
    "编辑环境",
    "导出Profile",
    "转移",
    "复制环境ID",
    "同步Cookie",
    "运维授权",
    "清除缓存",
    "删除环境",
];

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
//启动单个环境
const startEnv = (uuid: string) => {
    browser_start(uuid).then((res: any) => {
        toast.success(res.message)
    })
}
// 批量启动
const startAll = () => {
    browser_starts(selectedItems.value).then((res: any) => {
        toast.success(res.message)
    })
}
// 批量关闭
const stopAll = () => {
    browser_stops(selectedItems.value).then((res: any) => {
        toast.success(res.message)
    })
}
const visibleColumns = computed(() => {
    return allColumns.value.filter((column: any) => column.visible)
})

const filterOpt = () => {
    setTimeout(() => {
        showSearchChoose.value = !showSearchChoose.value
    }, 10)
}
// 打开同步器
const openSynchronizer = () => {
    synchronizerDialog.value = !synchronizerDialog.value
}
const toBookMark = () => {
    router.push('/environment-manager-bookmark')
}
const toGroupManager = () => {
    router.push('/environment-group-manager')
}
const headOperate = ref([
    {
        label: '分组管理',
        icon: GroupIcon,
        go: toGroupManager,  //前往分组管理
    },
    {
        label: '书签',
        icon: BookmarkIcon,
        go: toBookMark,
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

const tableData = ref([])
const filterData = computed(() => {
    // Filter the tableData based on the search query
    const filteredTableData = tableData.value.filter((item: any) =>
        item.name.toLowerCase().includes(searchQuery.value.toLowerCase())
    );

    // Sort the filtered data
    const sortedData = [...filteredTableData].sort((a: any, b: any) => {
        if (sortColumn.value === 'id') {
            return sortOrder.value === 'asc' ? a.ind - b.ind : b.ind - a.ind;
        } else if (sortColumn.value === 'name') {
            return sortOrder.value === 'asc'
                ? a.name.localeCompare(b.name)
                : b.name.localeCompare(a.name);
        }
        return 0;
    });


    // Map the sorted and filtered data to include only visible columns
    return sortedData.map((item: any) => {
        const visibleKeys = ['ind', ...visibleColumns.value.map((col: any) => col.key)];
        const filteredItem: Record<string, any> = { uuid: item.uuid };
        visibleKeys.forEach((key: any) => {
            if (key !== 'id' && key in item) {
                filteredItem[key] = item[key];
            }
        });
        return filteredItem;
    });
});

//分页
const PageObj = reactive({
    total: 0,
    pageNum: 1,
    pageSize: 17,
})
watch(() => PageObj.pageSize, (v) => {
    console.log("??:", PageObj);
    getList()
})
const getPageNum = (val: number) => {
    PageObj.pageNum = val
    getList()
}
const getPageSize = (val: number) => {
    PageObj.pageNum = 1
    PageObj.pageSize = val
    getList()
}

// 多选
const selectedItems = ref<string[]>([])
const isAllSelected = computed(() => {
    return tableData.value.length > 0 && selectedItems.value.length === tableData.value.length
})
const isIndeterminate = computed(() => {
    return selectedItems.value.length > 0 && selectedItems.value.length < tableData.value.length
})
const toggleSelectAll = (event: Event) => {
    const target = event.target as HTMLInputElement
    if (target.checked) {
        selectedItems.value = filterData.value?.map(item => item.uuid)
    } else {
        selectedItems.value = []
    }
}
const toggleSelectItem = (envUuid: string) => {
    const index = selectedItems.value.indexOf(envUuid)
    if (index === -1) {
        selectedItems.value.push(envUuid)
    } else {
        selectedItems.value.splice(index, 1)
    }
}
defineExpose({
    selectedItems
})

//监听理由变化
watch(() => route.params.id, () => {
    getList()
})

const getList = () => {
    if (route.params.id === '0') {
        environment_query(PageObj.pageNum, PageObj.pageSize).then((res: any) => {
            // tableData.value = res.data
            // console.log("res:", res.data);
            let ind = 1;
            PageObj.total = res.data.total
            // PageObj.pageNum =
            // 将字段与表头对上
            tableData.value = res.data.data.map((item: any) => {
                return {
                    ind: ind++,
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
        })
    } else {
        environment_query_by_group(+route.params.id, PageObj.pageNum, PageObj.pageSize).then((res: any) => {
            let ind = 1;
            PageObj.total = res.data.total
            // PageObj.pageNum =
            // 将字段与表头对上
            tableData.value = res.data.data.map((item: any) => {
                return {
                    ind: ind++,
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
        })
    }
}
onMounted(() => {
    getList()
})
const handleSort = (column: string) => {
    if (sortColumn.value === column) {
        sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
    } else {
        sortColumn.value = column;
        sortOrder.value = 'asc';
    }
};
</script>
