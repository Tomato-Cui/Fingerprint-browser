<template>
    <div class="member-management">
        <div class="filters forbid-filters">
            <el-select v-model="selectedStatusApply" placeholder="请选择审核状态" size="lager" class="filter-select"
                style="width: 150px;margin-right: 10px;">
                <el-option label="待审核" :value="0" />
                <el-option label="同意" :value="1" />
                <el-option label="拒绝" :value="-1" />
            </el-select>
            <el-input v-model="searchForApply" style="width: 300px;" size="lager" placeholder="请输入"
                class="input-with-select" clearable>
                <template #prepend>
                    <el-select v-model="selectInputApply" placeholder="请输入" style="width: 115px">
                        <el-option label="姓名" value="1" />
                        <el-option label="手机/邮箱" value="2" />
                    </el-select>
                </template>
            </el-input>
        </div>

        <!-- 申请成员列表 -->
        <el-card class="member-card" shadow="never" v-for="member in filteredAllMembersApply" :key="member.id">
            <!-- {{ member }} -->
            <div class="all-members" style="width: 100%;">
                <img src="../../../assets/容器 240@4x.png" alt="" srcset=""
                    style="width: 40px;height: 40px;margin-right: 10px;">
                <div class="member-card-info"
                    style="flex: 1;display: flex;justify-content: space-between;flex-direction: row;align-items: center;">
                    <div class="member-c-i-main1" style="">
                        <div>{{ member.username }}
                            <el-tag size="small"
                                :type="member.groupId === 4 ? 'warning' : member.groupId === 3 ? 'success' : 'info'"
                                style="margin: 0 10px;">{{
                                    groups.find(item => item.id === member.groupId)?.name
                                }}</el-tag>
                        </div>
                        <div style="color: #C0C0C0;">手机号/邮箱：{{ member.phoneAndEmail }}</div>
                        <div style="color: #C0C0C0;">邀请人：{{ member.invitePeople }}</div>
                    </div>
                    <div class="member-c-i-main2" style="color: #C0C0C0;">
                        用户组<el-tag size="small" style="margin: 0 10px;">{{
                            groups.find(item => item.id === member.groupId)?.name
                        }}</el-tag>
                    </div>
                    <div class="member-c-i-main3">
                        <div style="color: #C0C0C0;">申请时间：{{ member.applyTime }}</div>
                        <div style="color: #C0C0C0;">状态：<span style="color: black;">{{ member.state === 0 ? '待审核' :
                            member.state
                                === 1 ? '同意' : '拒绝' }}</span></div>
                    </div>
                </div>
                <div style="display: flex;align-items: center;">
                    <div class="all-m-option">
                        <el-button v-show="member.state !== 1" type="success" text size="large"
                            @click="agreeApply(member)">同意
                        </el-button> <!-- 授权环境: 打开授权环境对话框 -->
                        <span v-show="member.state === 0"
                            style="color:	#C0C0C0;display: flex;align-items: center;">|</span>
                        <el-button v-show="member.state !== -1" type="danger" text size="large"
                            @click="refuseApply(member)">拒绝</el-button>
                    </div>
                </div>
            </div>
        </el-card>
        <!-- Pagination -->
        <div class="pagination" style="flex: 1;display: flex;align-items: flex-end;">
            <div class="pagination-">共{{ filteredAllMembersApply.length }}条数据</div>
            <el-pagination background layout="sizes, prev, pager, next" :total="filteredAllMembersApply.length"
                :page-size="[5, 10, 20, 50]" @current-change="handlePageChange"></el-pagination>
        </div>
    </div>
</template>
<script setup lang="ts">

import { useRouter } from "vue-router";
import { ref } from "vue";
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus';
import { computed } from 'vue';

const groups = ref([
    { id: 1, name: "员工", accounts: 1, remark: "无", role: [1, 101, 10101] },
    { id: 2, name: "经理", accounts: 0, remark: "无", role: [1, 101, 10102] },
    { id: 3, name: "管理员", accounts: 0, remark: "无", role: [1, 101, 10103] },
    { id: 4, name: "BOSS", accounts: 1, remark: "无", role: [1, 101, 10104] },
]);
const defaultActive = ref("1");

const handlePageChange = (page: any) => {
    console.log("Page changed to:", page);
};

const selectedStatusApply = ref(0);  //审核状态
const selectInputApply = ref('1');  //审核搜索类型
const searchForApply = ref("");  //审核搜索内容
const applyMember = ref([  //申请成员列表
    { id: 1, username: "zhang", phoneAndEmail: "1243534636", invitePeople: 'hubstudio', groupId: 1, applyTime: '2024/11/28', state: 1 },
    { id: 2, username: "wang", phoneAndEmail: "1243534637", invitePeople: 'hubstudio', groupId: 2, applyTime: '2024/11/28', state: 0 },
    { id: 3, username: "li", phoneAndEmail: "1243534637", invitePeople: 'hubstudio', groupId: 1, applyTime: '2024/11/28', state: -1 },
    { id: 4, username: "wi", phoneAndEmail: "1243534637", invitePeople: 'hubstudio', groupId: 1, applyTime: '2024/11/28', state: -1 },
])
const filteredAllMembersApply = computed(() => {  //申请成员列表过滤器
    return applyMember.value.filter((member) => {
        //根据审核状态的选择过滤
        const matchState = member.state === selectedStatusApply.value
        let matchQuery = false
        if (selectInputApply.value === '1') {
            matchQuery = member.username.includes(searchForApply.value)
        } else if (selectInputApply.value === '2') {
            matchQuery = member.phoneAndEmail.includes(searchForApply.value)
        } else {
            matchQuery = true
        }
        return matchState && matchQuery;
    });
});
// 同意申请
const agreeApply = (member: any) => {
    ElMessageBox.confirm(`确定同意${member.username}吗？`, '启用成员', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    }).then(() => {
        ElMessage.warning("未连接后端");
    }).catch(() => {
        console.log("取消启用成员", member);
    });
}
// 拒绝申请
const refuseApply = (member: any) => {
    ElMessageBox.confirm(`确定拒绝${member.username}吗？`, '启用成员', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    }).then(() => {
        ElMessage.warning("未连接后端");
    }).catch(() => {
        console.log("取消启用成员", member);
    });
}
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
    padding: @padding-normal;
    width: 100%;
    background-color: #fff;

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
</style>