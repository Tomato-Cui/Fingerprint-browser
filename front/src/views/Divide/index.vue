<template>
  <Layout>
    <div class="Env">
      <!-- 搜索栏 -->
      <div class="search-bar">
        <el-input
          v-model="searchKey"
          style="max-width: 300px"
          placeholder="请输入搜索关键词"
          class="input-with-select"
        >
          <template #prepend>
            <el-select
              v-model="selectedGroup"
              placeholder="全部分组"
              style="width: 115px"
              filterable
            >
              <el-option
                v-for="group in filteredGroups"
                :key="group.id"
                :label="group.name"
                :value="group.name"
              />
            </el-select>
          </template>
          <template #append>
            <el-button :icon="Search" @click="handleSearch" />
          </template>
        </el-input>
        <div class="left">
          <el-button type="default" @click="handleAuthClick">
            <el-icon><img src="@/icons/svg/shouquan.svg" alt="" /></el-icon
            >分组授权
          </el-button>
          <el-button type="default" @click="dialogVisible = true">
            <el-icon><img src="@/icons/svg/xinjianfenzu.svg" alt="" /></el-icon
            >创建分组
          </el-button>
          <el-button type="default" @click="handleDelete">
            <el-icon><img src="@/icons/svg/shanchu.svg" alt="" /></el-icon>删除
          </el-button>
        </div>
      </div>

      <!-- 数据表 -->
      <div>
        <el-table
          :data="paginatedData"
          border
          style="width: 100%"
          @selection-change="handleSelectionChange"
        >
          <el-table-column type="selection" width="55"></el-table-column>
          <el-table-column prop="id" label="ID/编码"></el-table-column>
          <el-table-column prop="name" label="分组名称" width="180"
            ><template #default="{ row }">
              <el-button @click="goToEnvironmentPage(row.id)" type="text">{{
                row.name
              }}</el-button>
            </template></el-table-column
          >
          <el-table-column
            prop="group_count"
            label="分组环境总数"
          ></el-table-column>
          <el-table-column prop="remark" label="备注"></el-table-column>
          <el-table-column prop="createdAt" label="创建时间"></el-table-column>
          <el-table-column prop="ownerId" label="创建用户"></el-table-column>
          <!-- <el-table-column prop="email" label="邮箱"></el-table-column> -->
          <el-table-column label="操作" width="300">
            <template #default="scope">
              <el-button size="mini" @click="handleRefresh(scope.row)"
                >刷新</el-button
              >
              <el-dropdown trigger="click">
                <el-button size="mini">
                  更多<el-icon><ArrowDown /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="handleEditClick(scope.row)"
                      >修改</el-dropdown-item
                    >
                    <el-dropdown-item @click="exportDialogVisible = true"
                      >导出</el-dropdown-item
                    >
                    <el-dropdown-item @click="handleMergeClick(scope.row)"
                      >合并</el-dropdown-item
                    >
                    <el-dropdown-item>分享</el-dropdown-item>
                    <el-dropdown-item @click="handleTop(scope.row)">
                      {{ scope.row.isTop ? "取消置顶" : "置顶" }}
                    </el-dropdown-item>
                    <el-dropdown-item @click="deleteDivide(scope.row.id)"
                      >删除</el-dropdown-item
                    >
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </template>
          </el-table-column>
        </el-table>
        <!-- 分页组件 -->
        <el-pagination
          background
          layout="sizes, prev, pager, next, total"
          :total="totalItems"
          :page-size="pageSize"
          :page-sizes="[5, 10, 15, 20]"
          v-model:current-page="currentPage"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>

      <!-- 新建分组对话框 -->
      <el-dialog v-model="dialogVisible" title="创建分组" width="30%">
        <el-card>
          <el-form
            :model="form"
            :rules="rules"
            ref="formRef"
            label-width="100px"
          >
            <!-- 分组名称 -->
            <el-form-item label="分组名称" prop="name">
              <el-input v-model="form.name" placeholder="请输入分组名称" />
            </el-form-item>
            <!-- 备注 -->
            <el-form-item label="备注" prop="remark">
              <el-input
                type="textarea"
                v-model="form.remark"
                placeholder="请输入备注"
                rows="3"
              />
            </el-form-item>
            <!-- 操作按钮 -->
            <el-form-item>
              <el-button type="primary" @click="submitForm">提交</el-button>
              <el-button @click="resetForm">重置</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-dialog>

      <!-- 导出数据对话框 -->
      <el-dialog v-model="exportDialogVisible" title="导出数据" width="30%">
        <el-form :model="exportForm">
          <el-form-item label="导出字段">
            <el-checkbox-group v-model="exportForm.fields">
              <el-checkbox label="id">ID/编码</el-checkbox>
              <el-checkbox label="name">分组名称</el-checkbox>
              <el-checkbox label="count">分组环境总数</el-checkbox>
              <el-checkbox label="remark">备注</el-checkbox>
              <el-checkbox label="time">创建时间</el-checkbox>
              <el-checkbox label="user">创建用户</el-checkbox>
              <el-checkbox label="mail">邮箱</el-checkbox>
            </el-checkbox-group>
          </el-form-item>
          <el-form-item label="文件格式">
            <el-radio-group v-model="exportForm.fileType">
              <el-radio label="excel">Excel</el-radio>
              <el-radio label="txt">TXT</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-form>
        <template #footer>
          <span class="dialog-footer">
            <el-button type="primary" @click="handleExport">导出</el-button>
            <el-button @click="exportDialogVisible = false">取消</el-button>
          </span>
        </template>
      </el-dialog>

      <!-- 合并分组对话框 -->
      <el-dialog v-model="mergeDialogVisible" title="合并分组" width="30%">
        <el-form :model="mergeForm">
          <el-form-item label="当前分组">
            <el-input v-model="mergeForm.currentGroup" disabled></el-input>
          </el-form-item>
          <el-form-item label="选择目标分组">
            <el-select
              v-model="mergeForm.targetGroup"
              placeholder="请选择要合并的分组"
            >
              <el-option
                v-for="group in availableGroups"
                :key="group.name"
                :label="group.name"
                :value="group.name"
              />
            </el-select>
          </el-form-item>
        </el-form>
        <template #footer>
          <span class="dialog-footer">
            <el-button type="primary" @click="handleMergeConfirm"
              >确定</el-button
            >
            <el-button @click="mergeDialogVisible = false">取消</el-button>
          </span>
        </template>
      </el-dialog>

      <!-- 修改分组对话框 -->
      <el-dialog v-model="editDialogVisible" title="修改分组" width="30%">
        <el-form :model="editForm">
          <el-form-item label="分组名称">
            <el-input
              v-model="editForm.name"
              placeholder="请输入分组名称"
            ></el-input>
          </el-form-item>
          <el-form-item label="备注">
            <el-input
              v-model="editForm.remark"
              type="textarea"
              placeholder="请输入备注"
            ></el-input>
          </el-form-item>
        </el-form>
        <template #footer>
          <span class="dialog-footer">
            <el-button type="primary" @click="handleEditConfirm"
              >确定</el-button
            >
            <el-button @click="editDialogVisible = false">取消</el-button>
          </span>
        </template>
      </el-dialog>

      <!-- 分组授权对话框 -->
      <el-dialog v-model="authDialogVisible" title="分组授权" width="30%">
        <el-form :model="authForm">
          <el-form-item label="分组名称">
            <el-input v-model="authForm.groupName" disabled></el-input>
          </el-form-item>
          <el-form-item label="子用户信息">
            <el-input
              v-model="authForm.subUserInfo"
              type="textarea"
              placeholder="请输入子用户信息"
            ></el-input>
          </el-form-item>
        </el-form>
        <template #footer>
          <span class="dialog-footer">
            <el-button type="primary" @click="handleAuthConfirm"
              >确定</el-button
            >
            <el-button @click="authDialogVisible = false">取消</el-button>
          </span>
        </template>
      </el-dialog>
    </div>
  </Layout>
</template>

<script setup>
import { ref, computed, watch, onMounted } from "vue";
import { Search, ArrowDown } from "@element-plus/icons-vue";
import { ElMessage, ElMessageBox } from "element-plus";
import axios from "axios";
import { getDivides, createDivide, deleteDivide } from "@/api/divideService";
import Layout from "@/layouts/index.vue";
import { useRouter } from "vue-router";

const router = useRouter();

// 计算总页数
const totalPages = computed(() => Math.ceil(totalItems.value / pageSize.value));

// 获取分页数据
const paginatedData = computed(() => {
  const start = (page.value - 1) * pageSize.value;
  return divides.value.slice(start, start + pageSize.value);
});

// 跳转到环境页面，传递 group_name 参数
const goToEnvironmentPage = (groupName) => {
  router.push({
    path: "/environment",
    query: { group: groupName }, // 将 group_name 作为查询参数传递
  });
};
// 切换页码
const handlePageChange = async (newPage) => {
  page.value = newPage;
  const res = await getDivides(page.value, pageSize.value);
  totalItems.value = res.meta.total_items;
  divides.value = res.data;
};

const page = ref(1); // 当前页
const pageSize = ref(10); // 每页显示的条数
const totalItems = ref(0); // 总数据条数
const divides = ref([]); // 分组数据

onMounted(async () => {
  try {
    const res = await getDivides(page.value, pageSize.value);

    // 检查返回数据的有效性
    if (res && res.code === 1 && res.data) {
      totalItems.value = res.data.total_count || 0; // 获取总条数
      divides.value = res.data.data || []; // 获取分组数据，防止为空
    } else {
      console.error("响应数据格式错误或缺少必要字段");
    }

    // 判断是否有数据
    if (totalItems.value === 0) {
      console.log("没有符合条件的数据");
      // 你可以在这里做一些处理，例如显示一个 "没有数据" 的提示
    }
  } catch (error) {
    console.error("请求出错:", error);
  }
});

const selectedGroup = ref("");

const filteredData = computed(() => {
  let filtered = divides.value;

  if (selectedGroup.value) {
    filtered = filtered.filter((item) =>
      item.name.includes(selectedGroup.value)
    );
  }

  if (searchKey.value.trim()) {
    const key = searchKey.value.trim().toLowerCase();
    filtered = filtered.filter(
      (item) =>
        item.name.toLowerCase().includes(key) ||
        item.remark.toLowerCase().includes(key) ||
        item.user.toLowerCase().includes(key)
    );
  }

  return filtered;
});

const handleSearch = () => {
  page.value = 1;
};

const selectedRows = ref([]);
const handleSelectionChange = (selection) => {
  selectedRows.value = selection;
};

const filteredGroups = computed(() => {
  if (!divides.value || !Array.isArray(divides.value)) {
    return [];
  }
  return Array.from(
    new Set(
      divides.value.map((item) => ({
        id: item.id,
        name: item.name,
      }))
    )
  );
});

const dialogVisible = ref(false);
const form = ref({
  name: "",
  remark: "",
});
const rules = {
  name: [
    { required: true, message: "分组名称不能为空", trigger: "blur" },
    {
      min: 2,
      max: 50,
      message: "分组名称长度需在 2 到 50 字符之间",
      trigger: "blur",
    },
  ],
};

const submitForm = () => {
  formRef.value.validate((valid) => {
    if (valid) {
      ElMessage.success("分组创建成功！");
      console.log("提交数据：", form.value);
      handleCreateGroup();
    } else {
      ElMessage.error("请检查表单填写是否正确！");
    }
  });
};

const resetForm = () => {
  formRef.value.resetFields();
};

const formRef = ref(null);

const handleCreateGroup = () => {
  if (divides.value.some((group) => group.name === form.value.name)) {
    ElMessage.error("分组名称已存在，请使用其他名称");
    return;
  }

  const newGroup = {
    id: Date.now(),
    name: form.value.name,
    remark: form.value.remark,
    count: 0,
    time: new Date().toLocaleString(),
    user: "currentUser",
    mail: "currentUser@example.com",
    isTop: false,
    topTime: null,
  };

  divides.value.push(newGroup);
  dialogVisible.value = false;
  form.value = {
    name: "",
    remark: "",
  };
};

const authDialogVisible = ref(false);
const authForm = ref({
  groupName: "",
  subUserInfo: "",
});

const handleAuthClick = () => {
  if (selectedRows.value.length !== 1) {
    ElMessage.warning("请选择一条记录进行授权");
    return;
  }

  authForm.value.groupName = selectedRows.value[0].name;
  authDialogVisible.value = true;
};

const handleAuthConfirm = () => {
  authDialogVisible.value = false;
  authForm.value = {
    groupName: "",
    subUserInfo: "",
  };
};

const handleDelete = () => {
  if (selectedRows.value.length === 0) {
    ElMessage.warning("请选择要删除的记录");
    return;
  }

  ElMessageBox.confirm("确定要删除选中的记录吗？", "提示", {
    cancelButtonText: "取消",
    confirmButtonText: "确定",
    type: "warning",
  })
    .then(() => {
      const selectedIds = selectedRows.value.map((row) => row.id);
      divides.value = divides.value.filter(
        (row) => !selectedIds.includes(row.id)
      );
      ElMessage.success("删除成功");
    })
    .catch(() => {
      ElMessage.info("已取消删除");
    });
};

const handleRefresh = (row) => {
  ElMessage({
    message: "正在刷新数据...",
    type: "info",
  });

  setTimeout(() => {
    const index = divides.value.findIndex((item) => item.id === row.id);
    if (index !== -1) {
      divides.value[index] = {
        ...row,
        time: new Date().toLocaleString(),
        count: Math.floor(Math.random() * 100),
      };
    }

    ElMessage({
      message: "数据刷新成功",
      type: "success",
    });
  }, 1000);
};

const handleTop = (row) => {
  if (row.isTop) {
    row.isTop = false;
    divides.value.push(divides.value.splice(divides.value.indexOf(row), 1)[0]);
    ElMessage.info(`已取消置顶：${row.name}`);
  } else {
    row.isTop = true;
    divides.value.unshift(
      divides.value.splice(divides.value.indexOf(row), 1)[0]
    );
    ElMessage.success(`已置顶：${row.name}`);
  }
};

const mergeDialogVisible = ref(false);
const mergeForm = ref({
  currentGroup: "",
  targetGroup: "",
});

const availableGroups = computed(() => {
  return divides.value.filter(
    (item) => item.name !== mergeForm.value.currentGroup
  );
});

const handleMergeClick = (row) => {
  mergeForm.value.currentGroup = row.name;
  mergeForm.value.targetGroup = "";
  mergeDialogVisible.value = true;
};

const handleMergeConfirm = () => {
  if (!mergeForm.value.targetGroup) {
    ElMessage.warning("请选择要合并的目标分组");
    return;
  }

  const currentGroupData = divides.value.filter(
    (item) => item.name === mergeForm.value.currentGroup
  );
  const targetGroupData = divides.value.filter(
    (item) => item.name === mergeForm.value.targetGroup
  );

  const totalCount =
    currentGroupData.reduce((sum, item) => sum + item.count, 0) +
    targetGroupData.reduce((sum, item) => sum + item.count, 0);

  divides.value = divides.value.filter(
    (item) =>
      item.name !== mergeForm.value.currentGroup &&
      item.name !== mergeForm.value.targetGroup
  );

  divides.value.push({
    id: Date.now(),
    name: mergeForm.value.targetGroup,
    count: totalCount,
    remark:
      "合并自: " +
      mergeForm.value.currentGroup +
      ", " +
      mergeForm.value.targetGroup,
    time: new Date().toLocaleString(),
    user: "system",
    mail: "-",
    isTop: false,
    topTime: null,
  });

  ElMessage.success("分组合并成功");
  mergeDialogVisible.value = false;
};

const editDialogVisible = ref(false);
const editForm = ref({
  name: "",
  remark: "",
});

const handleEditClick = (row) => {
  editForm.value.name = row.name;
  editForm.value.remark = row.remark;
  editDialogVisible.value = true;
};

const handleEditConfirm = () => {
  const index = divides.value.findIndex(
    (item) => item.name === editForm.value.name
  );
  if (index !== -1) {
    divides.value[index].remark = editForm.value.remark;
    ElMessage.success("分组修改成功");
    editDialogVisible.value = false;
  } else {
    ElMessage.error("分组修改失败");
  }
};

const confirmDelete = (row) => {
  ElMessageBox.confirm(`确定要删除 "${row.name}" 吗？`, "提示", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning",
  })
    .then(() => {
      deleteRow(row);
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "已取消删除",
      });
    });
};

const deleteRow = (row) => {
  divides.value = divides.value.filter((item) => item.id !== row.id);
  ElMessage({
    type: "success",
    message: "删除成功！",
  });
};

const searchKey = ref("");

const currentPage = ref(1);
const total = ref(0);

const handleSizeChange = async (newSize) => {
  pageSize.value = newSize;
  currentPage.value = 1;
  const res = await getDivides(currentPage.value, pageSize.value);
  totalItems.value = res.meta.total_items;
  divides.value = res.data;
};

const newDivide = ref({
  idCode: "",
  group: "",
  remark: "",
  createdBy: "",
  email: "",
  actions: "",
});

const createNewDivide = async () => {
  try {
    const result = await createDivide(newDivide.value);
    console.log("分组创建成功:", result);
  } catch (error) {
    console.error("创建分组失败:", error);
  }
};

const deleteGroup = async (id) => {
  try {
    const result = await deleteDivide(id);
    console.log("分组删除成功:", result);
  } catch (error) {
    console.error("删除分组失败:", error);
  }
};
</script>

<style scoped>
.search-bar {
  display: flex;
  align-items: center;
  gap: 10px;
}
.input-with-select {
  margin: 10px;
}
.Env {
  background-color: #ffffff;
  border-radius: 16px 16px;
}
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>
