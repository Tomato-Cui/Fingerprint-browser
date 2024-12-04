<template>
    <div class="member-management">
        <div class="member-management-content">
            <!-- Header Buttons -->
            <div class="header-buttons">
                <el-button type="primary" size="lager" @click="addMemberInAll(null)">+ 添加成员</el-button>
                <el-button size="lager" @click="inviteMember">邀请成员</el-button>
            </div>

            <!-- Filters -->
            <div class="filters">
                <el-select v-model="selectedGroup" placeholder="请选择分组" size="lager" class="filter-select">
                    <el-option v-for="group in [{ id: 0, name: '全部' }].concat(groups)" :key="group.id"
                        :label="group.name" :value="group.id" />
                </el-select>
                <el-input v-model="search" style="max-width: 600px" size="lager" placeholder="请输入"
                    class="input-with-select" clearable>
                    <template #prepend>
                        <el-select v-model="selectedFilter" placeholder="姓名" style="width: 115px">
                            <el-option label="姓名" value="1" />
                            <el-option label="备注名" value="2" />
                            <el-option label="手机/邮箱" value="3" />
                        </el-select>
                    </template>
                </el-input>
            </div>
        </div>

        <!-- Member List TODO过滤器没改 -->
        <el-card class="member-card" shadow="never" v-for="member in allMembers" :key="member.id">
            <div class="all-members">
                <img src="../../../assets/容器 240@4x.png" alt="" srcset=""
                    style="width: 40px;height: 40px;margin-right: 10px;">
                <div class="member-card-info">
                    <div class="member-card-info-top">
                        <span class="name">{{ member.User.Username }}</span>
                        <el-tag size="small"
                            :type="member.group === 'BOSS' ? 'warning' : member.group === '管理员' ? 'success' : 'info'"
                            style="margin: 0 10px;">{{
                                member.group
                            }}</el-tag>
                    </div>
                    <div class="member-card-info-bottom">
                        <div>姓名：{{ member.User.Username }}</div>
                        <div>邮箱：{{ member.User.Email }}</div>
                        <div>分组：<el-tag effect="plain" type="info">{{ member.group }}</el-tag></div>
                        <div class="acc-1"><span>授权环境：</span><span>{{ member.accreditEnv }}</span></div>
                    </div>
                </div>
                <div style="display: flex;align-items: center;">
                    <div v-if="member.group === 'BOSS' || member.group === '管理员'" class="all-m-option"><el-button
                            type="primary" text size="large" @click="openMemberDrawer(member)">详情</el-button></div>
                    <div v-else class="all-m-option"><el-button type="primary" text size="large"
                            @click="openEnvironmentSelector(member)">授权环境
                        </el-button> <!-- 授权环境: 打开授权环境对话框 -->
                        <span style="color:	#C0C0C0;display: flex;align-items: center;">|</span>
                        <el-button type="primary" text size="large" @click="addMemberInAll(member)">编辑</el-button>
                        <div style="display: flex;align-items: center;">
                            <el-dropdown>
                                <el-icon>
                                    <More />
                                </el-icon>
                                <template #dropdown>
                                    <el-dropdown-menu>
                                        <el-dropdown-item @click="openMemberDrawer(member)">查看详情</el-dropdown-item>
                                        <el-dropdown-item @click="disableMember(member)">禁用成员</el-dropdown-item>
                                        <el-dropdown-item divided @click="deleteMember(member)">删除成员</el-dropdown-item>
                                    </el-dropdown-menu>
                                </template>
                            </el-dropdown>

                        </div>
                    </div>
                </div>
            </div>
        </el-card>

    </div>
    <!-- 添加成员弹出框 -->
    <el-dialog :title="!editMemberFlag ? '添加成员' : '编辑成员'" v-model="memberDialogVisible" width="1000"
        :close-on-click-modal="false" class="add-member-dialog" @close="handleClose">
        <!-- Form -->
        <el-form :model="memberForm" :rules="rules" ref="formRef" label-width="100px">
            <!-- Login Account -->
            <el-form-item label="登录账号" prop="phoneAndEmail">
                <el-input v-model="memberForm.phoneAndEmail" placeholder="请输入手机号/邮箱号" :disabled="editMemberFlag" />
            </el-form-item>

            <!-- Name -->
            <el-form-item label="姓名">
                <el-input v-model="memberForm.name" placeholder="输入手机号/邮箱号后自动获取" :disabled="editMemberFlag" />
            </el-form-item>

            <!-- Remark -->
            <el-form-item label="备注名" prop="remark">
                <el-input v-model="memberForm.remark" placeholder="请输入备注名" />
            </el-form-item>

            <!-- Group Name -->
            <el-form-item label="分组名称">
                <div v-if="!addMemFlag">{{ memberForm.groupName }}</div>
                <div v-else style="width: 100%;display: flex;">
                    <el-select v-model="memberForm.groupId" placeholder="Select" size="large" style="width: 100%">
                        <el-option v-for="item in groups" :key="item.id" :label="item.name" :value="item.id" />
                    </el-select>
                    <el-button type="primary" size="large" @click="addMemGroupDialog" text>+添加分组</el-button>
                </div>
            </el-form-item>

            <!-- Assign Environment -->
            <div v-show="!editMemberFlag">
                <el-form-item label="授权环境">
                    <!-- {{ addEnvFlag }} -->
                    <div style="display: flex; flex-direction: column;gap: 10px;width: 100%;">
                        <div style="border: 1px solid #e8e8e8;padding: 10px 20px;"
                            v-if="memberForm.selectedEnvironment.length !== 0">
                            <!-- {{ memberForm.selectedEnvironment }} -->
                            <div>已选择{{ environmentNames.length }}个环境</div>
                            <ul style="padding-left: 0;list-style-type: none;display: flex;flex-wrap: wrap;gap: 10px;">
                                <li v-for="envName in environmentNames"
                                    style="width: 15%;border: 1px solid #cfdbff;color: #3569fdd9;background: #f0f4ff;padding: 2px 10px;">
                                    {{ envName.name }}</li>
                            </ul>
                        </div>
                        <div style="display: flex;gap: 10px;">
                            <el-button type="primary" @click="openEnvironmentSelector(null)">+ 选择环境</el-button>
                            <div v-show="memberForm.selectedEnvironment.length === 0" class="error-text">
                                请选择需要授权给该员工管理的环境
                            </div>
                        </div>
                    </div>


                </el-form-item>
            </div>
        </el-form>

        <!-- Footer -->
        <template #footer>
            <el-button @click="addHandleClose">取消</el-button>
            <el-button type="primary" @click="submitForm">确定</el-button>
        </template>
    </el-dialog>
    <!-- 添加环境 -->
    <el-dialog title="授权环境" v-model="envDialogVisible" width="1000px" :close-on-click-modal="false"
        class="add-environment-dialog">
        <!-- Sidebar -->
        <div class="dialog-content">
            <aside class="sidebar">
                <div class="sidebar-item">
                    <el-icon>
                        <Edit />
                    </el-icon>
                    备注名
                </div>
                <div class="sidebar-value">{{ memberForm.remark == '' ? "未设置" : memberForm.remark }}</div>

                <div class="sidebar-item">
                    <el-icon>
                        <User />
                    </el-icon>
                    姓名
                </div>
                <div class="sidebar-value">{{ memberForm.name == '' ? "未设置" : memberForm.name }}</div>
            </aside>

            <!-- Main content -->
            <main class="main-content">
                <div class="header">
                    <p>请选择需要授权的环境</p>
                    <span>已选择 {{ selectedEnvironments.length }} 个环境</span>
                </div>

                <!-- Filters -->
                <div class="filters">
                    <!-- <el-checkbox v-model="selectAll">全选</el-checkbox> -->
                    <el-select v-model="selectedEnvironmentType" placeholder="所有环境" size="large" class="filter-item">
                        <el-option v-for="item in environmentType" :key="item.id" :label="item.value"
                            :value="item.id" />
                    </el-select>
                    <el-select v-model="selectedOperationType" placeholder="所有操作类型" size="large" class="filter-item">
                        <el-option v-for="item in operationType" :key="item.id" :label="item.value" :value="item.id" />
                    </el-select>
                    <el-select v-model="selectedGroupEv" placeholder="所有分组" size="large" class="filter-item">
                        <el-option v-for="item in groupType" :key="item.id" :label="item.value" :value="item.id" />
                    </el-select>
                    <!-- <el-input v-model="searchQuery" placeholder="请输入环境名称" size="large" clearable class="filter-item" /> -->
                    <el-input v-model="searchQuery" placeholder="Please input" class="input-with-select"
                        style="width: 300px;">
                        <template #prepend>
                            <el-select v-model="selectInput" placeholder="Select" class="filter2">
                                <el-option label="环境名称" value="1" />
                                <el-option label="IP地址" value="2" />
                                <el-option label="备注" value="3" />
                            </el-select>
                        </template>
                        <template #suffix>
                            <el-icon>
                                <Search />
                            </el-icon>
                        </template>
                    </el-input>
                </div>

                <!-- Environment List -->
                <el-table :data="filteredEnvironments" border style="width: 100%;" ref="environmentTable"
                    :row-key="'id'" @selection-change="handleSelectionChange" v-model:selection="selectedRowsEnv">
                    <el-table-column type="selection" width="55" label="选择" />
                    <el-table-column prop="name" label="环境名称" />
                    <el-table-column prop="ip" label="IP 地址" />
                    <el-table-column prop="location" label="位置" />
                </el-table>
            </main>
        </div>

        <!-- Footer -->
        <template #footer>
            <el-button @click="handleCloseEnv">取消</el-button>
            <el-button type="primary" @click="handleConfirmEv">确定</el-button>
        </template>
    </el-dialog>
    <!-- 邀请成员 -->
    <el-dialog v-model="invDialogVisible" title="邀请成员" width="550px" @close="handleClose" class="invite-member-dialog">
        <p style="margin: 20px 0;">成员可通过在加入团队中输入团队码的方式，在审核之后加入团队</p>
        <div style="display: flex;align-items: center;justify-content: space-between;margin: 20px 0;">
            <div style="width: 100px;">分组：</div>
            <el-select v-model="InvValue" clearable placeholder="选择" style="width: 100%;margin: 0 10px;">
                <el-option v-for="item in groups" :key="item.id" :label="item.name" :value="item.id" />
            </el-select>
            <el-button :type="InvValue == null ? 'info' : 'primary'" :disabled="InvValue == null ? true : false"
                @click="copyCode">复制团队码</el-button>
        </div>
        <!-- 团队码显示区 -->
        <div class="team-code">
            <div v-if="!InvValue">团队码</div>
            <div v-else>{{ getCode(6) }}</div>
        </div>
    </el-dialog>
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
import { ref, watch } from "vue";
import {
    Search,
    More,
    Edit,
    User,
} from "@element-plus/icons-vue";
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus';
import { computed } from 'vue';
import { ElTable } from 'element-plus';
import teamApi from '@/api/team';
import { onMounted, nextTick } from "vue";
import type { TableInstance } from 'element-plus'

const router = useRouter();
// console.log("router", router.currentRoute.value.path);
const search = ref(""); // Search input binding
const groups = ref([
    { id: 1, name: "员工", accounts: 1, remark: "无", role: [1, 101, 10101] },
    { id: 2, name: "经理", accounts: 0, remark: "无", role: [1, 101, 10102] },
    { id: 3, name: "管理员", accounts: 0, remark: "无", role: [1, 101, 10103] },
    { id: 4, name: "BOSS", accounts: 1, remark: "无", role: [1, 101, 10104] },
]);
const envDialogVisible = ref(false);  //环境选择弹出框
const drawerMember = ref<any>(null) //成员详情弹出框的成员数据
const memberDrawer = ref(false);  //成员详情弹出框
const environmentType = ref([{ id: 1, value: '所有环境' }, { id: 2, value: '已授权' }, { id: 3, value: '未授权' }])  //环境类型
const operationType = ref([{ id: 1, value: '所有操作类型' }, { id: 2, value: 'Windows' }, { id: 3, value: 'Android' }, { id: 4, value: 'macOS' }, { id: 5, value: 'iOS' }])  //操作类型
const groupType = ref([{ id: 1, value: '所有分组' }])  //分组类型
const selectedEnvironments = ref<number[]>([]);// 多选的环境的id
// Filters
const selectInput = ref('1');  //搜索类型
const selectedEnvironmentType = ref(1);  //环境类型
const selectedOperationType = ref(1);  //操作类型
const selectedGroupEv = ref(1);  //分组类型
const searchQuery = ref("");  //搜索内容
const selectedRowsEnv = ref<any>([]); //选择的环境的id
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
// const allMembers = ref([  //所有成员数据
//     { id: 1, username: "yuan", name: "张三", phoneAndEmail: "13800000000", groupId: 1, group: "员工", remark: "无", accreditEnv: 1, accEnvIds: [1] },
//     { id: 2, username: "li", name: "李四", phoneAndEmail: "13800000001", groupId: 2, group: "经理", remark: "无", accreditEnv: 1, accEnvIds: [2] },
//     { id: 3, username: "wang", name: "王五", phoneAndEmail: "yingchao04@qq.com", groupId: 3, group: "管理员", remark: "无", accreditEnv: 1, accEnvIds: [2] },
//     { id: 4, username: "zhao", name: "赵六", phoneAndEmail: "13800000003", groupId: 4, group: "BOSS", remark: "无", accreditEnv: 1, accEnvIds: [1] },
// ])
const allMembers = ref<any[]>([])
const selectedGroup = ref();
const selectedFilter = ref("1"); //所有成员搜索类型
const addMemFlag = ref(false) //所有成员路由下的添加成员标志// 过滤筛选环境列表
const filteredEnvironments = computed(() => {
    return environments.value.filter((env) => {
        const matchesType = selectedEnvironmentType.value === 1 || env.type === selectedEnvironmentType.value;
        const matchesOperation =
            selectedOperationType.value === 1 || env.operation === selectedOperationType.value;
        const matchesGroup = selectedGroupEv.value === 1 || env.group === selectedGroupEv.value;
        let matchesQuery = false
        // console.log(selectInput.value);

        if (selectInput.value === '1') {
            matchesQuery = env.name.includes(searchQuery.value);
            console.log(matchesType && matchesOperation && matchesGroup && matchesQuery);

        } else if (selectInput.value === '2') {
            matchesQuery = env.ip.includes(searchQuery.value);
        } else {
            matchesQuery = env.remark.includes(searchQuery.value);
        }
        // matchesQuery.value = env.name.includes(searchQuery.value)
        return matchesType && matchesOperation && matchesGroup && matchesQuery;
    });
});
// 标志
const editMemberFlag = ref(false) //编辑成员标志
const accEnvFlag = ref(false) //授权环境打开环境对话框标志
//接收所授权环境的用户
const selectedMember = ref<any>(null)
const teamId = ref(1) //团队id
const environmentTable = ref<TableInstance>() //环境表格
const memberObj = ref<any>(null)  //修改成员时成员对象
const memberForm = ref<MemberForm>({
    phoneAndEmail: "",
    name: "",
    remark: "",
    groupName: "员工", // Example group name
    groupId: 1,
    selectedEnvironment: <number[]>[],
});
const memberDialogVisible = ref(false);
// 邀请成员弹出框
const InvValue = ref(null)
const textToCopy = ref('')
// 控制弹出框是否显示
const invDialogVisible = ref(false);
//环境名称列表
const environmentNames = ref([
    { id: 1, name: "环境1" },
    { id: 2, name: "环境2" },
    { id: 3, name: "环境3" },
    { id: 4, name: "环境4" },
    { id: 5, name: "环境5" },
    { id: 6, name: "环境6" },
    { id: 6, name: "环境6" },
    { id: 6, name: "环境6" },
])

//添加成员按钮
const addMember = () => {
}

// 添加成员和编辑
const addMemberInAll = (member: any) => {
    addMemFlag.value = true
    if (member != null) { //编辑成员
        editMemberFlag.value = true  //编辑成员标志
        memberForm.value.id = member.id
        memberForm.value.groupId = member.groupId
        memberForm.value.selectedEnvironment = member.accEnvIds
        memberForm.value.name = member.name
        memberForm.value.remark = member.remark
        memberForm.value.phoneAndEmail = member.phoneAndEmail
        //给environmentNames赋值
        environmentNames.value = member.accEnvIds.map((envId: any) => environments.value.find((env: any) => env.id === envId));
        // 打开选择环境时，给memberObj赋值
        memberObj.value = member
    }
    console.log("ZXCVBNM<>:", memberForm.value);

    memberDialogVisible.value = true
}
// 取消添加成员
const addHandleClose = () => {
    setTimeout(() => {
        editMemberFlag.value = false  //编辑成员标志
    }, 1000)
    memberDialogVisible.value = false
    resetForm()
}
// 添加成员提交
const submitForm = () => {
    formRef.value?.validate((valid: any) => {
        if (valid) {
            if (memberForm.value.selectedEnvironment.length === 0) {
                ElMessage.error("请选择需要授权的环境��");
                return;
            }
            // console.log("表单提交数据:", memberForm.value);
            // console.log("表单提交数据-环境:", memberForm.value.selectedEnvironment);
            memberDialogVisible.value = false;
            if (memberForm.value.id) { //修改成员
                console.log("修改成员:_________", memberForm.value);

                teamApi.updateMemberApi(memberForm.value)
                memberObj.value = null //成员对象置空
            } else { //添加成员
                console.log("添加成员:_________", memberForm.value);
                teamApi.addMemberApi(memberForm.value)
            }
            resetForm()
            setTimeout(() => {
                editMemberFlag.value = false  //编辑成员标志
            }, 1000)
        } else {
            console.log("表单校验失败");
        }
    });
};
watch(memberDialogVisible, (newVal) => {
})
const openMemberDrawer = (member: any) => { //打开成员详情弹出框
    drawerMember.value = member
    memberDrawer.value = true
    teamApi.getMemberInfoApi(member.id)
}
const disableMember = (member: any) => {  //禁用成员
    // console.log("禁用成员", member);
    ElMessageBox.confirm(`确定禁用${member.username}吗？请谨慎操作`, '禁用成员', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    }).then(() => {
        //禁用成员
        teamApi.enableMemberApi(member.id);
        ElMessage.warning("未连接后端");
    }).catch(() => {
        console.log("取消禁用成员", member);
    });
}
const deleteMember = (member: any) => {  //删除成员
    // console.log("删除成员", member);
    ElMessageBox.confirm(`确定删除${member.username}吗？请谨慎操作`, '删除成员', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    }).then(() => {
        //删除成员
        teamApi.deleteMemberApi(member.id);
        ElMessage.warning("未连接后端");
    }).catch(() => {
        console.log("取消禁用成员", member);
    });
}

const addMemGroupDialog = () => {
    // console.log("添加分组弹窗打开");
    memberDialogVisible.value = false
    //跳转至分组页
    router.push({ path: '/team/group' });
}
const inviteMember = () => {
    // console.log("Invite Member clicked");
    invDialogVisible.value = true
};
// 成员表单数据类型
interface MemberForm {
    id?: number;
    phoneAndEmail: string;
    name: string;
    remark: string;
    groupName: string;
    groupId: number;
    selectedEnvironment: number[];
}

const resetForm = () => { //表单重置
    memberForm.value.id = undefined
    memberForm.value.phoneAndEmail = ""
    memberForm.value.name = ""
    memberForm.value.remark = ""
    memberForm.value.groupName = "员工"
    memberForm.value.selectedEnvironment = []
}


// Form validation rules
const rules = {
    account: [{ required: true, message: "请输入登录账号", trigger: "blur" }],
    remark: [{ required: true, message: "请输入备注名", trigger: "blur" }],
};
// Reference to form
const formRef = ref<FormInstance>();




const openEnvironmentSelector = (member: any) => {
    // console.log("环境选择弹窗打开:", member.accEnvIds);
    //查询所有环境----需要一个团队id
    teamApi.getEnvironmentsApi(teamId.value);
    if (member) {
        //将member中的数据赋值给成员对象
        memberObj.value = member
        console.log("memberObj.value:", memberObj.value);

        // addEnvFlag.value = true
        accEnvFlag.value = true
    }
    envDialogVisible.value = true
};
watch(envDialogVisible, (newVal) => {

    nextTick(() => {
        // console.log("-----??????????::::", environmentTable.value?.toggleRowSelection);

        if (newVal === true) {

            if (accEnvFlag.value) {  //当授权环境进入的时候，为多选框自动选中
                const selectedRows = environments.value.filter(row => {
                    return memberObj.value.accEnvIds.includes(row.id)
                });
                selectedRows.forEach(row => {
                    environmentTable.value?.toggleRowSelection(row, true);
                });
                //将选中的用户ID赋值给表单
                console.log("memberForm.value.id:", memberForm.value.id);

                selectedEnvironments.value = memberObj.value.accEnvIds //环境IDs赋值给表格选择的id列表
            }
        } else {
            // ElMessage.warning("环境选择对话框关闭");
            if (accEnvFlag.value) { //授权环境出去的
                memberForm.value.selectedEnvironment = []
                accEnvFlag.value = false
                // ElMessage.warning("环境选择对话框关闭" + memberForm.value.selectedEnvironment);
            } else {
                memberForm.value.selectedEnvironment = selectedEnvironments.value
                environmentNames.value = environments.value.filter(env => selectedEnvironments.value.includes(env.id))
                // ElMessage.warning("环境选择对话框关闭___" + selectedEnvironments.value);
            }
            environmentTable.value?.clearSelection()//取消选中状态
            memberObj.value = null //成员对象置为空
        }
        // console.log("?_?_?_?:", memberForm.value);
    });
})

//环境选择
const handleSelectionChange = (selection: any) => {
    selectedEnvironments.value = selection.map((env: any) => env.id);  //选择的环境
    // console.log("selectedEnvironments.value----------:", selectedEnvironments.value);

};
// 环境选择对话框完成（点击确定）
const handleConfirmEv = () => {
    // memberForm.value.selectedEnvironment = selectedEnvironments.value //环境id赋值给表单
    // selectedEnvironments.value = [] //置空
    // console.log("Selected Environments:", selectedEnvironments.value);
    if (accEnvFlag.value) {//修改当前用户的环境，而非添加新用户及环境
        memberForm.value.id = memberObj.value.id
        memberForm.value.selectedEnvironment = selectedEnvironments.value   //授权环境仅需传用户id和环境ids

        teamApi.addEnvironmentApi(selectedMember)
        resetForm()
    }
    //将选中的环境id赋值给表单
    memberForm.value.selectedEnvironment = selectedEnvironments.value
    getEnvironmentNames(memberForm.value.selectedEnvironment);  //根据环境id查询环境名称
    envDialogVisible.value = false;
    // addEnvFlag.value = false  //标志已存在成员授权结束
    // memberObj.value = null //成员对象置为空
};
// 环境选择对话框关闭（点击取消）
const handleCloseEnv = () => {
    selectedEnvironments.value = [] //置空
    // memberForm.value.selectedEnvironment = [] //置空
    // console.log("太难了：", selectedRowsEnv.value);

    // selectedRowsEnv.value = [] //置空
    // console.log("selectedRowsEnv.value----------w:", selectedRowsEnv.value);
    // console.log("selectedEnvironments.value----------w:", selectedEnvironments.value);
    // addEnvFlag.value = false  //标志已存在成员授权结束
    envDialogVisible.value = false;
    // memberObj.value = null //成员对象置为空
}
// 过滤筛选所有成员
const filteredAllMembers = computed(() => {
    return allMembers.value.filter((member) => {
        const matchGroup = selectedGroup.value === undefined || selectedGroup.value === 0 || member.groupId === selectedGroup.value
        let matchQuery = false
        if (selectedFilter.value === '1') {
            matchQuery = member.name.includes(search.value)
        } else if (selectedFilter.value === '2') {
            matchQuery = member.username.includes(search.value)
        } else {
            matchQuery = member.phoneAndEmail.includes(search.value)
        }
        return matchQuery && matchGroup;
    });
});


// 关闭弹出框时的回调
const handleClose = () => {
    // console.log('弹出框关闭');
    addMemFlag.value = false
};

const getCode = (length: number) => {
    // console.log("获取团队码");
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    let result = '';
    for (let i = 0; i < length; i++) {
        const randomIndex = Math.floor(Math.random() * characters.length);
        result += characters.charAt(randomIndex);
    }
    textToCopy.value = result
    return result
}
const copyCode = async () => {
    // console.log("复制团队码");
    try {
        await navigator.clipboard.writeText(textToCopy.value);
        ElMessage({
            message: '复制成功',
            type: 'success',
        })
    } catch (err) {
        console.error('Failed to copy text: ', err);
        alert('复制失败');
    }
}

//根据多个环境id批量查询环境名称
const getEnvironmentNames = (ids: number[]) => {
    // environmentNames.value = teamApi.getEnvironmentNamesApi(ids)
}

//
onMounted(() => {
    const teams = ref()
    //当前用户的第一个团队的所有成员
    teamApi.getAllTeam().then((res: any) => {
        // console.log("123456:", res);
        teams.value = res.data.data
        // console.log("_+_+_+_+_!!!!:", teams.value);
        teamApi.getAllMemberByTeamId(teams.value[0].ID).then((res: any) => {
            console.log("_~_~_~_:", res);
            allMembers.value = res
            // console.log("!@#$%^&*():",allMembers.value);
        })
    })



})
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

// 所有成员
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

.add-member-dialog {
    height: 650px !important;

    .el-form {
        .el-form-item {
            min-height: 70px;
            display: flex;
            align-items: center;

            .el-input {
                // width: 300px;
                height: 50px;
            }
        }
    }

    // 添加成员弹出框
    .error-text {
        margin-top: 5px;
        color: red;
        font-size: 12px;
    }
}

.dialog-content {
    height: 640px;
    display: flex;
    gap: @gap-size;

    .sidebar {
        width: 150px;
        background: @sidebar-bg-color;
        padding: 10px;
        border-radius: @border-radius;

        .sidebar-item {
            display: flex;
            align-items: center;
            gap: 5px;
            font-size: 14px;
            color: @text-color-primary;
            margin-bottom: 8px;
        }

        .sidebar-value {
            font-size: 14px;
            color: @text-color-secondary;
            margin-bottom: 15px;
        }
    }

    .main-content {
        flex: 1;

        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
            font-size: 14px;
            color: @text-color-primary;

            p {
                margin: 0;
            }
        }

        .filters {
            display: flex;
            gap: 10px;
            margin-bottom: 10px;

            .filter-item {
                width: 150px;
            }

            :deep(.filter2) {
                width: 95px;
                height: 100%;

                .el-select__wrapper {
                    height: 100%;
                }
            }
        }
    }
}

// 邀请成员对话框
.invite-member-dialog {
    .team-code {
        width: 100%;
        height: 80px;
        font-size: 28px;
        border-radius: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #d2d7e1;
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