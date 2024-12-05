<template>
    <div class="member-management">
        <!-- Filters -->
        <div class="filters forbid-filters">
            <el-select v-model="selectedGroupForbid" placeholder="请选择分组" size="lager" class="filter-select">
                <el-option v-for="group in [{ id: 0, name: '全部' }].concat(groups)" :key="group.id" :label="group.name"
                    :value="group.id" />
            </el-select>
            <el-input v-model="searchForbid" style="width: 300px;" size="lager" placeholder="请输入"
                class="input-with-select" clearable>
                <template #prepend>
                    <el-select v-model="selectedFilterForbid" placeholder="姓名" style="width: 115px">
                        <el-option label="姓名" value="1" />
                        <el-option label="备注名" value="2" />
                        <el-option label="手机/邮箱" value="3" />
                    </el-select>
                </template>
            </el-input>
        </div>

        <!-- Member List -->
        <el-card class="member-card" shadow="never" v-for="member in filteredAllMembersForbid" :key="member.id">
            <div class="all-members" style="color: #C0C0C0;">
                <img src="../../../assets/容器 240@4x.png" alt="" srcset=""
                    style="width: 40px;height: 40px;margin-right: 10px;">
                <div class="member-card-info">
                    <div class="member-card-info-top">
                        <span class="name">{{ member.username }}</span>
                        <el-tag size="small"
                            :type="member.group === 'BOSS' ? 'warning' : member.group === '管理员' ? 'success' : 'info'"
                            style="margin: 0 10px;">{{
                                member.group
                            }}</el-tag>
                    </div>
                    <div class="member-card-info-bottom">
                        <div>姓名：{{ member.name }}</div>
                        <div>手机号/邮箱：{{ member.phoneAndEmail }}</div>
                        <div>分组：<el-tag effect="plain" type="info">{{ member.group }}</el-tag></div>
                        <div class="acc-1"><span>授权环境：</span><span>{{ member.accreditEnv }}</span></div>
                    </div>
                </div>
                <div style="display: flex;align-items: center;">
                    <div class="all-m-option"><el-button type="success" text size="large"
                            @click="enableMember(member)">启用
                        </el-button> <!-- 授权环境: 打开授权环境对话框 -->
                        <span style="color:	#C0C0C0;display: flex;align-items: center;">|</span>
                        <el-button type="danger" text size="large" @click="deleteMember(member)">删除</el-button>
                        <div style="display: flex;align-items: center;">
                            <el-dropdown>
                                <el-icon>
                                    <More />
                                </el-icon>
                                <template #dropdown>
                                    <el-dropdown-menu>
                                        <el-dropdown-item @click="openMemberDrawer(member)">查看详情</el-dropdown-item>
                                    </el-dropdown-menu>
                                </template>
                            </el-dropdown>

                        </div>
                    </div>
                </div>
            </div>
        </el-card>
        <!-- Pagination -->
        <div class="pagination" style="flex: 1;display: flex;align-items: flex-end;">
            <div class="pagination-">共{{ filteredAllMembersForbid.length }}条数据</div>
            <el-pagination background layout="sizes, prev, pager, next" :total="filteredAllMembersForbid.length"
                :page-size="[5, 10, 20, 50]" @current-change="handlePageChange"></el-pagination>
        </div>
    </div>
    <!-- 成员详情 -->
    <el-drawer v-model="memberDrawer" title="成员详情" size="600px">
        <div class="member-drawer">
            <div class="member-d-header">
                <img src="../../../assets/容器 240@4x.png" alt="" srcset="" style="width: 40px;height: 40px;">
                <div style="font-weight: 800;font-size: 18px;margin: 0 15px;">{{ drawerMember?.username }}</div>
                <el-tag size="small"
                    :type="drawerMember.group === 'BOSS' ? 'warning' : drawerMember.group === '管理员' ? 'success' : 'info'">{{
                        drawerMember.group
                    }}</el-tag>
            </div>
            <div class="member-d-main2">
                <el-row>
                    <el-col :span="12">
                        <div class="member-d-m-title">备注名:</div>
                        <div class="member-d-m-info">{{ drawerMember.username }}</div>
                    </el-col>
                    <el-col :span="12">
                        <div class="member-d-m-title">姓名:</div>
                        <div class="member-d-m-info">{{ drawerMember.name }}</div>
                    </el-col>
                </el-row>
                <el-row>
                    <el-col :span="12">
                        <div class="member-d-m-title">手机号/邮箱号:</div>
                        <div class="member-d-m-info">{{ drawerMember.phoneAndEmail }}</div>
                    </el-col>
                    <el-col :span="12">
                        <div class="member-d-m-title">分组:</div>
                        <div class="member-d-m-info">{{ drawerMember.group }}</div>
                    </el-col>
                </el-row>
            </div>
            <div
                style="width: 100px;height: 40px;background-color: #fff;display: flex;align-items: center;justify-content: center;border: 2px solid #f5f5f5">
                已授权环境</div>
            <div class="member-d-main3">
                <div class="member-d-m3-main">
                    <!-- 用表格显示当前成员的已授权环境 -->
                    <el-table
                        :data="drawerMember.accEnvIds.map((envId: any) => environments.find((env: any) => env.id === envId))"
                        border style="width: 100%;">
                        <el-table-column prop="name" label="环境名称" />
                        <el-table-column prop="ip" label="IP 地址" />
                        <!-- <el-table-column prop="location" label="位置" />   -->
                    </el-table>
                </div>
            </div>
        </div>
    </el-drawer>
</template>
<script setup lang="ts">

import { useRouter } from "vue-router";
import { ref } from "vue";
import {
    More,
} from "@element-plus/icons-vue";
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus';
import { computed } from 'vue';
import { ElTable } from 'element-plus';

const router = useRouter();
console.log("router", router.currentRoute.value.path);


const groups = ref([
    { id: 1, name: "员工", accounts: 1, remark: "无", role: [1, 101, 10101] },
    { id: 2, name: "经理", accounts: 0, remark: "无", role: [1, 101, 10102] },
    { id: 3, name: "管理员", accounts: 0, remark: "无", role: [1, 101, 10103] },
    { id: 4, name: "BOSS", accounts: 1, remark: "无", role: [1, 101, 10104] },
]);
const openMemberDrawer = (member: any) => { //打开成员详情弹出框
    drawerMember.value = member
    memberDrawer.value = true
}
const deleteMember = (member: any) => {  //删除成员
    // console.log("删除成员", member);
    ElMessageBox.confirm(`确定删除${member.username}吗？请谨慎操作`, '删除成员', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    }).then(() => {
        ElMessage.warning("未连接后端");
    }).catch(() => {
        console.log("取消禁用成员", member);
    });
}
const enableMember = (member: any) => {  //启用成员
    // console.log("启用成员", member);
    ElMessageBox.confirm(`确定启用${member.username}吗？`, '启用成员', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    }).then(() => {
        ElMessage.warning("未连接后端");
    }).catch(() => {
        console.log("取消启用成员", member);
    });
}

const handlePageChange = (page: any) => {
    console.log("Page changed to:", page);
};

// ----------------------------------------------------所有成员
const selectedFilterForbid = ref("1"); //禁用成员搜索类型
const addMemFlag = ref(false) //所有成员路由下的添加成员标志
// ----------------------------------------------------创建分组弹出框
// Dialog visibility
const groupDialogVisible = ref(false);
const form = ref({
    groupName: "",
    remark: "",
    role: [],
});

// ---------------------------------------------------添加成员弹出框
// Dialog visibility
const memberDialogVisible = ref(false);
// Form data
const memberForm = ref({
    phoneAndEmail: "",
    name: "",
    remark: "",
    groupName: "员工", // Example group name
    selectedEnvironment: null,
});
// Reference to form
const formRef = ref<FormInstance>();

// -----------------------------------------------------添加环境
const drawerMember = ref<any>(null) //成员详情弹出框的成员数据
const memberDrawer = ref(false);  //成员详情弹出框
const searchForbid = ref("");  //禁用成员搜索内容
const selectedGroupForbid = ref();  //禁用成员分组类
const memberForbid = ref([  //禁用成员列表
    { id: 1, username: "yuan", name: "张三", phoneAndEmail: "13800000000", groupId: 4, group: "BOSS", remark: "无", accreditEnv: 1, accEnvIds: [1] },
    { id: 2, username: "li", name: "李四", phoneAndEmail: "13800000001", groupId: 3, group: "管理员", remark: "无", accreditEnv: 1, accEnvIds: [2] },
    { id: 3, username: "wang", name: "王五", phoneAndEmail: "yingchao04@qq.com", groupId: 3, group: "管理员", remark: "无", accreditEnv: 1, accEnvIds: [2] },
])
const environments = ref([  //环境列表
    {
        id: 1,
        name: "test2",
        ip: "36.24.241.70",
        location: "China, Zhejiang",
        type: 1,
        operation: 1,
        group: 1,
        remark: "无",
    },
    {
        id: 2,
        name: "test1",
        ip: "91.186.218.38",
        location: "Hong Kong",
        type: 2,
        operation: 2,
        group: 2,
        remark: "无",
    },
]);
const filteredAllMembersForbid = computed(() => {  //禁用成员列表过滤器
    return memberForbid.value.filter((member) => {
        const matchGroup = selectedGroupForbid.value === undefined || selectedGroupForbid.value === 0 || member.groupId === selectedGroupForbid.value
        let matchQuery = false
        if (selectedFilterForbid.value === '1') {
            matchQuery = member.name.includes(searchForbid.value)
        } else if (selectedFilterForbid.value === '2') {
            matchQuery = member.username.includes(searchForbid.value)
        } else {
            matchQuery = member.phoneAndEmail.includes(searchForbid.value)
        }
        return matchQuery && matchGroup;
    });
});
// 控制弹出框是否显示
const invDialogVisible = ref(false);
</script>

<style scoped lang="less">
// 定义一些常用的变量
@background-color-light: #f5f5f5;
@border-color: #dcdfe6;
@font-size-normal: 16px;
@padding-normal: 20px;
@margin-normal: 10px;
@margin-large: 20px;
@color-primary: #409eff;
@color-text-secondary: #909399;
@color-text-tertiary: #606266;
@sidebar-bg-color: #f5f5f5;
@text-color-primary: #606266;
@text-color-secondary: #909399;
@border-radius: 4px;
@gap-size: 20px;

.member-management {
    padding: @padding-normal;
    width: 100%;
    background-color: #fff;
    display: flex;
    flex-direction: column;

    .member-management-content {
        width: 100%;
        display: flex;
        justify-content: space-between;
    }

    .header-buttons {
        margin-bottom: @margin-normal;

        .el-button:first-child {
            margin-right: @margin-normal;
        }
    }

    .filters {
        display: flex;
        align-items: center;
        margin-bottom: @margin-large;
        background-color: #fff;

        .filter-select {
            margin-right: @margin-normal;
            width: 150px;
        }

        .filter-input {
            flex: 1;
        }
    }

    .forbid-filters {
        // width: 500px;
        display: flex;
        align-items: center;
        justify-content: flex-end;
    }

    :deep(.member-card) {
        background-color: #fff;
        padding: 5px 10px;
        border: 1px solid #eaeaea;
        border-radius: 8px;

        .el-card__body {
            padding: 5px 10px;
        }

        .card-header {
            display: flex;
            align-items: center;
            margin-bottom: 15px;

            .icon {
                margin-right: @margin-normal;
                font-size: 24px;
                color: @color-primary;
            }
        }

        .member-info {
            display: flex;
            flex-direction: column;

            .info-top {
                display: flex;
                align-items: center;
                gap: 10px;
                font-size: @font-size-normal;
                font-weight: bold;
            }

            .info-bottom {
                display: flex;
                gap: 20px;
                color: @color-text-secondary;
                margin-top: 5px;
            }
        }

        .card-details {
            display: flex;
            align-items: center;
            justify-content: space-between;
            color: @color-text-tertiary;

            span {
                margin-right: @margin-normal;
            }
        }
    }

    .pagination {
        width: 100%;
        display: flex;
        flex: 1;
        justify-content: space-between;
        align-items: flex-end;
        margin-top: @margin-large;
    }

    .all-members {
        height: 70px;
        display: flex;
        align-items: center;

        .member-card-info {
            margin: 0 10px;
            flex: 1;
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: space-around;

            .member-card-info-bottom {
                display: flex;
                justify-content: space-between;
            }
        }

        .all-m-option {
            width: 200px;
            justify-content: center;
            display: flex;
        }
    }
}

// 成员详情弹出框
.member-drawer {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;

    .member-d-header {
        width: 100%;
        height: 80px;
        display: flex;
        align-items: center;
    }

    .member-d-main2 {
        width: 100%;
        height: 200px;
        display: flex;
        flex-direction: column;
        gap: 30px;
    }

    .member-d-main3 {
        flex: 1;
        // width: 100%;
        background-color: #f5f5f5;
        padding: 20px 30px;
        padding-bottom: 60px;
        // overflow: scroll;
    }

    .member-d-m-title {
        font-weight: 800;
    }

    .member-d-m-info {
        font-weight: 400;
        color: #909399;
    }
}
</style>