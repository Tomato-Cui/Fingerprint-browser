<template>
  <div class="group-content">
    <el-header class="header">
      <el-button type="primary" @click="createGroup" class="create-group-button"
        >+ 创建分组</el-button
      >
      <el-input
        v-model="search"
        placeholder="请输入分组名称"
        class="search-input"
        @keyup.enter="handleEnter"
        clearable
      >
        <template #suffix>
          <el-icon size="large" @click="handleEnter" style="cursor: pointer">
            <Search />
          </el-icon>
        </template>
      </el-input>
    </el-header>
    <el-main class="group-main">
      <el-table :data="filteredGroups" stripe border class="group-table">
        <el-table-column
          prop="group_name"
          label="分组名称"
          align="center"
        ></el-table-column>
        <el-table-column
          prop="member_count"
          label="包含账号"
          align="center"
          class="font-color"
        ></el-table-column>
        <el-table-column
          prop="description"
          label="备注"
          align="center"
          class="remark-column font-color"
        ></el-table-column>
        <el-table-column label="操作" width="500" align="center">
          <template #default="scope">
            <el-button
              type="primary"
              plain
              size="large"
              @click="addMember(scope.row)"
              v-show="scope.row.id !== 4"
              >+ 添加成员</el-button
            >
            <span
              style="
                color: #c0c0c0;
                display: flex;
                align-items: center;
                margin-left: 19px;
              "
              v-show="scope.row.id !== 3 && scope.row.id !== 4"
            >
              |
            </span>
            <el-button
              type="primary"
              text
              size="large"
              @click="editGroup(scope.row)"
              v-show="scope.row.id !== 3 && scope.row.id !== 4"
              >编辑</el-button
            >
            <span
              style="
                color: #c0c0c0;
                display: flex;
                align-items: center;
                margin-left: 19px;
              "
              v-show="scope.row.id !== 4"
            >
              |
            </span>
            <el-button
              type="primary"
              text
              size="large"
              @click="viewMembers(scope.row)"
              v-show="scope.row.id !== 4"
              >查看成员</el-button
            >
            <el-dropdown v-show="scope.row.id !== 3 && scope.row.id !== 4">
              <el-icon size="large" style="">
                <MoreFilled />
              </el-icon>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="deleteGroup(scope.row)"
                    >删除分组</el-dropdown-item
                  >
                  <el-dropdown-item @click="copyGroup(scope.row)"
                    >复制分组</el-dropdown-item
                  >
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
        </el-table-column>
      </el-table>

      <!-- Pagination -->
      <div class="pagination">
        <div class="pagination-">共{{ groups.length }}条数据</div>
        <el-pagination
          background
          layout="sizes, prev, pager, next"
          :page-size="[5, 10, 20, 50]"
          @current-change="handlePageChange"
        ></el-pagination>
      </div>
    </el-main>
    <!-- 创建分组弹出框 -->
    <!-- Dialog -->
    <el-dialog
      title="新建分组"
      v-model="groupDialogVisible"
      width="1000"
      :close-on-click-modal="false"
      class="create-group-dialogs"
    >
      <!-- Form -->
      <div>
        <el-form :model="form" label-width="180px">
          <!-- Group Name -->
          <el-form-item
            label="分组名称:"
            :rules="[
              { required: true, message: '请输入分组名称', trigger: 'change' },
            ]"
          >
            <el-input v-model="form.groupName" placeholder="请输入分组名称" />
          </el-form-item>

          <!-- Remark -->
          <el-form-item label="备注:">
            <el-input
              v-model="form.remark"
              placeholder="请输入备注内容"
              type="textarea"
              :rows="2"
            />
          </el-form-item>

          <!-- Permission Management -->
          <!-- <el-form-item style="display: flex; flex-direction: column;width: 100%;"> -->
          <div style="display: flex; flex-direction: column">
            <div style="display: flex; margin: 10px 0">
              权限管理
              <div style="font-size: 12px; color: #909399; margin-left: 10px">
                请为分组分配权限功能，同个分组的成员功能权限一致
              </div>
            </div>
            <!-- <el-cascader :options="options" :props="props" clearable size="large" /> -->
            <el-cascader-panel
              style="width: fit-content"
              :props="props"
              :options="options"
              v-model="form.role"
            />
          </div>
          <!-- </el-form-item> -->
        </el-form>
      </div>

      <!-- Footer -->
      <template #footer>
        <el-button @click="(groupDialogVisible = false), resetForm()"
          >取消</el-button
        >
        <el-button type="primary" @click="handleConfirm">确定</el-button>
      </template>
    </el-dialog>
    <!-- 添加成员弹出框 -->
    <el-dialog
      title="添加成员"
      v-model="memberDialogVisible"
      width="1000"
      :close-on-click-modal="false"
      class="add-member-dialog"
      @close="handleClose"
    >
      <!-- Form -->
      <el-form
        :model="memberForm"
        :rules="rules"
        ref="formRef"
        label-width="100px"
      >
        <!-- Login Account -->
        <el-form-item label="登录账号" prop="email">
          <el-input
            v-model="memberForm.email"
            placeholder="请输入邮箱"
          />
        </el-form-item>

        <!-- Name -->
        <el-form-item label="姓名">
          <el-input
            v-model="memberForm.name"
            placeholder="输入手机号/邮箱号后自动获取"
            disabled
          />
        </el-form-item>

        <!-- Remark -->
        <!-- <el-form-item label="备注名" prop="remark">
          <el-input v-model="memberForm.remark" placeholder="请输入备注名" />
        </el-form-item> -->

        <!-- Group Name -->
        <el-form-item label="分组名称">
          <div v-if="!addMemFlag">{{ memberForm.groupName }}</div>
          <div v-else style="width: 100%; display: flex">
            <el-select
              v-model="addMemGroup"
              placeholder="Select"
              size="large"
              style="width: 100%"
            >
              <el-option
                v-for="item in groups"
                :key="item.id"
                :label="item.name"
                :value="item.id"
              />
            </el-select>
            <el-button
              type="primary"
              size="large"
              @click="addMemGroupDialog"
              text
              >+添加分组</el-button
            >
          </div>
        </el-form-item>

        <!-- Assign Environment -->
        <el-form-item label="授权环境">
          <el-button type="primary" @click="openEnvironmentSelector(null)"
            >+ 选择环境</el-button
          >
          <!-- {{ memberForm.selectedEnvironment.length === 0 }} -->
          <div
            v-if="memberForm.selectedEnvironment.length === 0"
            class="error-text"
          >
            请选择需要授权给该员工管理的环境
          </div>
        </el-form-item>
      </el-form>

      <!-- Footer -->
      <template #footer>
        <el-button @click="(memberDialogVisible = false), resetForm()"
          >取消</el-button
        >
        <el-button type="primary" @click="submitForm">确定</el-button>
      </template>
    </el-dialog>
    <!-- 添加环境 -->
    <el-dialog
      title="授权环境"
      v-model="envDialogVisible"
      width="1000px"
      :close-on-click-modal="false"
      class="add-environment-dialog"
    >
      <!-- Sidebar -->
      <div class="dialog-content">
        <aside class="sidebar">
          <div class="sidebar-item">
            <el-icon>
              <Edit />
            </el-icon>
            备注名
          </div>
          <div class="sidebar-value">
            {{ memberForm.remark == "" ? "未设置" : memberForm.remark }}
          </div>

          <div class="sidebar-item">
            <el-icon>
              <User />
            </el-icon>
            姓名
          </div>
          <div class="sidebar-value">
            {{ memberForm.name == "" ? "未设置" : memberForm.name }}
          </div>
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
            <el-select
              v-model="selectedEnvironmentType"
              placeholder="所有环境"
              size="large"
              class="filter-item"
            >
              <el-option
                v-for="item in environmentType"
                :key="item.id"
                :label="item.value"
                :value="item.id"
              />
            </el-select>
            <el-select
              v-model="selectedOperationType"
              placeholder="所有操作类型"
              size="large"
              class="filter-item"
            >
              <el-option
                v-for="item in operationType"
                :key="item.id"
                :label="item.value"
                :value="item.id"
              />
            </el-select>
            <el-select
              v-model="selectedGroupEv"
              placeholder="所有分组"
              size="large"
              class="filter-item"
            >
              <el-option
                v-for="item in groupType"
                :key="item.id"
                :label="item.value"
                :value="item.id"
              />
            </el-select>
            <!-- <el-input v-model="searchQuery" placeholder="请输入环境名称" size="large" clearable class="filter-item" /> -->
            <el-input
              v-model="searchQuery"
              placeholder="Please input"
              class="input-with-select"
              style="width: 300px"
            >
              <template #prepend>
                <el-select
                  v-model="selectInput"
                  placeholder="Select"
                  class="filter2"
                >
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
          <el-table
            :data="filteredEnvironments"
            border
            style="width: 100%"
            @selection-change="handleSelectionChange"
            v-model:selection="selectedRowsEnv"
            ref="multipleTable"
          >
            <el-table-column type="selection" width="55" label="选择" />
            <el-table-column prop="name" label="环境名称" />
            <el-table-column prop="ip" label="IP 地址" />
            <el-table-column prop="location" label="位置" />
          </el-table>
        </main>
      </div>

      <!-- Footer -->
      <template #footer>
        <el-button @click="envDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleConfirmEv">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>
<script setup lang="ts">
import { useRouter } from "vue-router";
import { ref, reactive, watch } from "vue";
import { Search, MoreFilled, Edit, User } from "@element-plus/icons-vue";
import { ElMessage, ElMessageBox, type FormInstance } from "element-plus";
import { computed } from "vue";
import { ElTable } from "element-plus";
import teamApi from "@/api/team";
const router = useRouter();
console.log("router", router.currentRoute.value.path);
import { onMounted } from "vue";

function validateEmail(email: string): boolean {  //邮箱规则校验
    const emailPattern = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
    return emailPattern.test(email);
}
const search = ref(""); // Search input binding
const groups = ref([]);
const defaultActive = ref("1");

// Event handlers 创建分组
const createGroup = () => {
  groupDialogVisible.value = true;
  // console.log("Create group clicked:", groupDialogVisible.value);
};

const addMember = (group: any) => {
  console.log("Adding member to:", group.name);
  memberForm.value.groupName = group.group_name;

  memberDialogVisible.value = true;
};
const editGroup = (group: any) => {
  console.log("Editing group:", group.name);
  //将当前组的数据赋值给表单
  form.value.groupName = group.name;
  form.value.remark = group.remark;
  form.value.role = group.role;
  //查询数据，打开“创建分组弹出框”
  groupDialogVisible.value = true;
};

const viewMembers = (group: any) => {
  console.log("Viewing members of:", group.name);
  //跳转到所有成员页，并在路由附带分组id
  router.push({ path: "/team/member", query: { groupId: group.id } });
};

//删除分组
const deleteGroup = (group: any) => {
  //询问是否确认删除
  ElMessageBox.confirm("确定要删除该分组吗？", "提示", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning",
  }).then(() => {
    console.log("删除：", group);
    const teamID = ref();
    //查询出当前用户团队ID
    teamApi.getAllTeam().then((res: any) => {
      teamID.value = res.data.data[0].ID;
      teamApi.deleteGroupApi(teamID.value, group.group_id);
      groups.value = groups.value.filter(
        (item: any) => item.group_id !== group.group_id
      );
    });
  });
};

const copyGroup = (group: any) => {
  console.log("复制分组", group.name);
  //TODO: 连接后端添加分组，以下为假数据
  groups.value.forEach((value: any, index: any, array: any) => {
    // console.log("value", value);
    if (value.group_name === group.group_name) {
      // groups.value.unshift(value)
      //新分组对象
      let gCopyObj = { ...value };
      gCopyObj.accounts = 0;
      gCopyObj.remark = value.description;
      //   gCopyObj.id = groups.value.length + 1;
      //修改name--当name加上(1)后，数组中已经有与其相同的name，则数字加一
      let name = gCopyObj.group_name + "(1)";
      let index = groups.value.findIndex(
        (item: any) => item.group_name === name
      );
      if (index !== -1) {
        //如果数组中已经有与其相同的name，则数字加一
        let index = name.lastIndexOf("(");
        name =
          name.slice(0, index) +
          "(" +
          (parseInt(name.slice(name.lastIndexOf("(") + 1)) + 1) +
          ")";
      }
      gCopyObj.groupName = name;
      gCopyObj.group_name = name;
      //赋值teamID
      teamApi.getAllTeam().then((res: any) => {
        gCopyObj.teamID = res.data.data[0].ID;
        console.log("gCopyObj:", gCopyObj);
        //   发送添加分组请求
        teamApi.createGroupApi(gCopyObj);
        groups.value = [gCopyObj, ...groups.value];
      });

      return;
    }
  });
};

const handlePageChange = (page: any) => {
  console.log("Page changed to:", page);
};

const addMemFlag = ref(false); //所有成员路由下的添加成员标志
const addMemGroup = ref(1); //所有成员路由下的添加成员分组
const addMemGroupDialog = () => {
  // console.log("添加分组弹窗打开");
  //进入分组管理页，关闭对话框
  defaultActive.value = "1";
  memberDialogVisible.value = false;
};
// 分组名称搜索
const handleEnter = () => {
  console.log("Search clicked");
};
// ----------------------------------------------------创建分组弹出框
// Dialog visibility
const groupDialogVisible = ref(false);
const form = reactive({
  teamID: "",
  groupName: "",
  remark: "",
  role: [],
});
const resetForm = () => {
  //表单重置
  form.value.groupName = "";
  form.value.remark = "";
  form.value.role = [];
  memberForm.value.email = "";
  memberForm.value.name = "";
  memberForm.value.remark = "";
  memberForm.value.groupName = "员工";
  memberForm.value.selectedEnvironment = [];
};
// 创建分组/编辑分组权限结构
const props = { multiple: true };
const options = [
  {
    value: 1,
    label: "环境",
    children: [
      {
        value: 101,
        label: "我的环境",
        children: [
          { value: 10101, label: "查看数据" },
          { value: 10102, label: "创建环境" },
          { value: 10103, label: "九宫格" },
          { value: 10104, label: "排序" },
          { value: 10105, label: "编辑账号" },
          { value: 10106, label: "修改代理" },
          { value: 10107, label: "CookieRobot" },
          { value: 10108, label: "转移环境" },
        ],
      },
      {
        value: 102,
        label: "已转移环境",
        children: [
          { value: 10201, label: "查看数据" },
          { value: 10202, label: "详情" },
          { value: 10203, label: "删除" },
        ],
      },
    ],
  },
  {
    value: 2,
    label: "云手机",
    children: [
      {
        value: 201,
        label: "我的云手机",
        children: [
          { value: 20101, label: "查看数据" },
          { value: 20102, label: "详情" },
          { value: 20103, label: "新建云手机" },
        ],
      },
    ],
  },
  {
    value: 23,
    label: "North America",
    children: [
      {
        value: 24,
        label: "US",
        children: [
          { value: 25, label: "New York" },
          { value: 26, label: "Los Angeles" },
          { value: 27, label: "Washington" },
        ],
      },
      {
        value: 28,
        label: "Canada",
        children: [
          { value: 29, label: "Toronto" },
          { value: 30, label: "Montreal" },
          { value: 31, label: "Ottawa" },
        ],
      },
    ],
  },
];
// Confirm button handler
const handleConfirm = () => {
  //获取当前用户的团队，为form.value.teamID赋值
  teamApi.getAllTeam().then((res: any) => {
    form.teamID = res.data.data[0].ID;
    // console.log("res.data.data[0].ID:", res.data.data[0].ID);
    console.log("form.value:", form);
    teamApi.createGroupApi(form);
    // 刷新此页面
    window.location.reload();
  });

  groupDialogVisible.value = false;
  resetForm();
};

// ---------------------------------------------------添加成员弹出框
// Dialog visibility
const memberDialogVisible = ref(false);
// Form data
const memberForm = ref({
  email: "",
  name: "",
  remark: "",
  groupName: "员工", // Example group name
  selectedEnvironment: <number[]>[],
});
// Form validation rules
const rules = {
  email: [
    { required: true, message: "请输入登录账号", trigger: "blur" },
    {
      pattern: /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
      message: "请输入有效的邮箱地址",
      trigger: "change",
    },
  ],
  remark: [{ required: true, message: "请输入备注名", trigger: "blur" }],
};
watch(() => memberForm.value.email, async (newVal) => { //监听邮箱是否正确
    // console.log("newVal:", newVal);
  if (validateEmail(newVal)) {
    // ElMessage.success("有效的邮箱地址");
    const res = await teamApi.searchByEmail(newVal)
    console.log("res-----------=:", res);
    if(res.data.code == 1){        
        // 用户存在
        memberForm.value.name = res.data.data.Username
        // console.log("memberForm.value.name:", res.data.data.username);
    }else{  //TODO else进不去？
        console.log("12467842342");
        
        memberForm.value.name = "" //查无此人，清空name
    }
  }else{
    memberForm.value.name = "" //邮箱格式不正确，清空name
  }
});


// Reference to form
const formRef = ref<FormInstance>();

// Submit the form
const submitForm = () => {
  formRef.value?.validate((valid: any) => {
    if (valid) {
      if (!memberForm.value.selectedEnvironment) {
        ElMessage.error("请选择需要授权的环境��");
        return;
      }
      console.log("表单提交数据:", memberForm.value);
      memberDialogVisible.value = false;
      resetForm();
    } else {
      console.log("表单校验失败");
    }
  });
};

// -----------------------------------------------------添加环境
// Dialog visibility
const envDialogVisible = ref(false); //环境选择弹出框
const environmentType = ref([
  { id: 1, value: "所有环境" },
  { id: 2, value: "已授权" },
  { id: 3, value: "未授权" },
]); //环境类型
const operationType = ref([
  { id: 1, value: "所有操作类型" },
  { id: 2, value: "Windows" },
  { id: 3, value: "Android" },
  { id: 4, value: "macOS" },
  { id: 5, value: "iOS" },
]); //操作类型
const groupType = ref([{ id: 1, value: "所有分组" }]); //分组类型
const selectedEnvironments = ref<number[]>([]); // 多选的环境的id
// Filters
const selectInput = ref("1"); //搜索类型
const selectedEnvironmentType = ref(1); //环境类型
const selectedOperationType = ref(1); //操作类型
const selectedGroupEv = ref(1); //分组类型
const searchQuery = ref(""); //搜索内容
const multipleTable = ref(null); //选择环境的表格
const selectedRowsEnv = ref<any>([]); //选择的环境的id
const environments = ref([
  //环境列表
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
// 过滤筛选环境列表
const filteredEnvironments = computed(() => {
  return environments.value.filter((env) => {
    const matchesType =
      selectedEnvironmentType.value === 1 ||
      env.type === selectedEnvironmentType.value;
    const matchesOperation =
      selectedOperationType.value === 1 ||
      env.operation === selectedOperationType.value;
    const matchesGroup =
      selectedGroupEv.value === 1 || env.group === selectedGroupEv.value;
    let matchesQuery = false;
    // console.log(selectInput.value);

    if (selectInput.value === "1") {
      matchesQuery = env.name.includes(searchQuery.value);
      console.log(
        matchesType && matchesOperation && matchesGroup && matchesQuery
      );
    } else if (selectInput.value === "2") {
      matchesQuery = env.ip.includes(searchQuery.value);
    } else {
      matchesQuery = env.remark.includes(searchQuery.value);
    }
    // matchesQuery.value = env.name.includes(searchQuery.value)
    return matchesType && matchesOperation && matchesGroup && matchesQuery;
  });
});
// 过滤分组
const filteredGroups = computed(() => {
  return groups.value.filter((group: any) => {
    // console.log("search.value:", group.group_name + "----" + searchQuery.value);
    return group.group_name.includes(search.value);
  });
});

// Handle selection change
const handleSelectionChange = (selection: any) => {
  selectedEnvironments.value = selection; //选择的环境
  console.log(
    "selectedEnvironments.value----------:",
    selectedEnvironments.value
  );
};
// Confirm selection
const handleConfirmEv = () => {
  memberForm.value.selectedEnvironment = selectedEnvironments.value; //将选择的环境id赋值给memberForm
  console.log("Selected Environments:", selectedEnvironments.value);
  selectedEnvironments.value = [];
  envDialogVisible.value = false;
};
const openEnvironmentSelector = (member: any) => {
  // console.log("环境选择弹窗打开:", member.accEnvIds);
  if (member) {
    // selectedEnvironments.value = member.accEnvIds
    // console.log("selectedEnvironments.value", selectedEnvironments.value);
    // TODO: 自动选中以数组member.accEnvIds为id的环境  无效
    // 1. 遍历member.accEnvIds，找到environments中id与member.accEnvIds相同的元素
    // 2. 将找到的元素添加到selectedRowsEnv中
    // 3. 调用multipleTable的toggleRowSelection方法，选中找到的元素
    // selectedRowsEnv.value = member.accEnvIds.filter((row: any) => environments.value.find((env: any) => env.id === row));
    // if (multipleTable.value) {
    //   multipleTable.value.toggleRowSelection(selectedRowsEnv.value, true);
    // }
    // console.log("selectedRowsEnv.value----------:", selectedRowsEnv.value);
  }

  envDialogVisible.value = true;
  // You can implement the logic to open another dialog or component for environment selection.
};

// 关闭弹出框时的回调
const handleClose = () => {
  // console.log('弹出框关闭');
  addMemFlag.value = false;
};

const getList = () => {
  teamApi.getGroups().then((res: any) => {
    groups.value = res.data.data;
    // console.log("groups.value:", groups.value);
  });
};

onMounted(() => {
  // TODO: 获取分组列表并赋值
  // teamApi.getDefaultTeam().then((res: any) => {
  // groups.value = res.data.data
  // })
  getList();
});
</script>

<style scoped lang="less">
// 添加环境
@sidebar-bg-color: #f5f5f5;
@text-color-primary: #606266;
@text-color-secondary: #909399;
@border-radius: 4px;
@gap-size: 20px;

.group-content {
  background-color: #fff;
  width: 100%;
  padding: 20px;

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 20px;
    background-color: #fff;

    .create-group-button {
      width: 150px;
      height: 50px;
      font-size: 20px;
      margin-right: 10px;
    }

    .search-input {
      margin-left: 10px;
      width: 350px;
      height: 50px;
    }
  }

  .group-main {
    // height: 100%;
    min-height: calc(100vh - 190px);
    padding: 0;
    width: 100%;
    display: flex;
    flex-direction: column;

    :deep .group-table {
      margin-top: 20px;
      width: 100%;
      max-height: calc(100vh - 290px);
      overflow: scroll;

      .el-table__inner-wrapper {
        .el-table__header-wrapper {
          .el-table__header {
            min-height: 60px; // You can adjust this value as needed
          }
        }

        .el-table__body-wrapper {
          .el-table__body {
            .el-table__row {
              height: 90px;
              font-size: 19px;

              .cell {
                display: flex;
                // justify-content: space-around;

                .el-dropdown {
                  display: flex;
                  align-items: center;
                }

                .el-button {
                  font-size: 20px;
                }
              }
            }
          }
        }
      }
    }

    .pagination {
      width: 100%;
      display: flex;
      flex: 1;
      justify-content: space-between;
      align-items: flex-end;
      margin-top: 20px;
    }
  }

  // 创建分组弹出框
  :deep(.create-group-dialogs) {
    height: 800px;
    padding: 40px 80px;
    display: flex;
    flex-direction: column;

    // justify-content: space-between;
    .el-dialog__footer {
      flex: 1;
      display: flex;
      justify-content: flex-end;
      align-items: flex-end;
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
    gap: 20px;

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
}
</style>
