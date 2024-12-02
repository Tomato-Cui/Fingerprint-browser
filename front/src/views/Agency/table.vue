<!-- 代理管理 -->
<template>
  <div class="Env">
    <div class="search-bar">
      <!-- 按钮触发搜索框 -->
      <el-button
        type="text"
        class="filter-btn no-focus"
        @click="showSearch = !showSearch"
      >
        <el-icon><Search /></el-icon>筛选
      </el-button>

      <!-- 弹出的搜索框 -->
      <el-dialog
        v-model="showSearch"
        title="筛选条件"
        width="500px"
        :modal-append-to-body="false"
        custom-class="search-dialog"
      >
        <div class="search-box">
          <div
            v-for="(condition, index) in conditions"
            :key="index"
            class="search-row"
          >
            <el-select
              v-model="condition.field"
              placeholder="请选择筛选字段"
              style="width: 150px"
            >
              <el-option
                v-for="option in fieldOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              />
            </el-select>
            <el-input
              v-if="condition.field"
              v-model="condition.value"
              :placeholder="`请输入${condition.field}`"
              style="flex: 1"
            />
            <el-button
              type="text"
              size="small"
              @click="removeCondition(index)"
              class="remove-condition-btn"
            >
              <el-icon><Minus /></el-icon>删除
            </el-button>
          </div>

          <el-button
            type="text"
            size="small"
            @click="addCondition"
            class="add-condition-btn"
          >
            <el-icon><Plus /></el-icon>添加条件
          </el-button>
        </div>

        <template #footer>
          <el-button type="primary" @click="applyFilters">确认</el-button>
          <el-button @click="clearFilters">清除</el-button>
        </template>
      </el-dialog>
      <div class="left">
        <el-button type="text" @click="addProxyDialogVisible = true">
          <el-icon><Plus /></el-icon>添加代理
        </el-button>
        <el-button type="text" @click="handleCheckProxy">
          <el-icon><Search /></el-icon>检查代理
        </el-button>
        <el-button type="text" @click="handleDelete">
          <el-icon><Minus /></el-icon>
          删除
        </el-button>
      </div>
    </div>
    <div>
      <div>
        <el-table
          :data="paginatedData"
          style="width: 100%"
          @selection-change="handleSelectionChange"
        >
          <el-table-column type="selection" width="55"></el-table-column>
          <el-table-column prop="id" label="代理ID"></el-table-column>
          <el-table-column prop="text" label="代理信息"></el-table-column>
          <el-table-column prop="ip" label="出口IP"></el-table-column>
          <el-table-column prop="numEnv" label="关联环境数"></el-table-column>
          <el-table-column prop="ipQuery" label="IP查询渠道"></el-table-column>
          <!-- <el-table-column prop="ipQuery" label="IP查询渠道"></el-table-column> -->
          <el-table-column prop="remark" label="备注"></el-table-column>
          <el-table-column prop="status" label="代理状态"></el-table-column>
          <el-table-column prop="enddate" label="到期日期"></el-table-column>
          <el-table-column label="操作" width="300">
            <template #default="scope">
              <el-button size="mini">刷新</el-button>

              <el-dropdown>
                <el-button size="mini">
                  更多<el-icon><ArrowDown /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="openEditDialog(scope.row)">
                      <el-icon><Edit /></el-icon>
                      编辑
                    </el-dropdown-item>
                    <el-dropdown-item @click="handleDelete(scope.row)">
                      <el-icon><Delete /></el-icon>
                      删除
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </template>
          </el-table-column>
        </el-table>
        <el-pagination
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
          :current-page="currentPage"
          :page-sizes="[10, 20, 50, 100]"
          :page-size="pageSize"
          layout="total, sizes, prev, pager, next"
          :total="tableData.length"
        >
          <template #total="{ total }"> 总计: {{ total }} 条 </template>
        </el-pagination>
      </div>
    </div>
  </div>

  <!-- 添加代理对话框 -->
  <el-dialog v-model="addProxyDialogVisible" title="添加代理" width="50%">
    <div>
      <p>说明：</p>
      <ol>
        <li>不填写代理类型时，将默认为HTTP类型</li>
        <li>仅支持HTTP、HTTPS、SSH、SOCKS5四种代理类型</li>
        <li>每一行一个代理，一次最多添加500个代理</li>
        <li>主机支持IPv4、IPv6，IPv6的主机需填写在 [] 内</li>
        <li>填写格式 (支持IPv4, IPv6):</li>
        <ul>
          <li>192.168.0.1:8000{备注}</li>
          <li>192.168.0.1:8000:代理账号:代理密码{备注}</li>
          <li>socks5://192.168.0.1:8000[刷新URL]{备注}</li>
          <li>http://[2001:db8:2de:0:0:0:0:e13]:8000[刷新URL]{备注}</li>
        </ul>
      </ol>
    </div>
    <el-form>
      <el-form-item label="IP查询渠道">
        <el-select v-model="ipQueryChannel" placeholder="请选择IP查询渠道">
          <el-option label="IPLocation" value="IPLocation"></el-option>
          <el-option label="ip-api" value="ip-api"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="代理信息">
        <el-input
          type="textarea"
          v-model="proxyInfo"
          placeholder="请在此填写您的代理信息"
          rows="10"
        ></el-input>
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button type="primary" @click="handleAddProxy">确定</el-button>
        <el-button @click="addProxyDialogVisible = false">取消</el-button>
      </span>
    </template>
  </el-dialog>

  <!-- 编辑代理对话框 -->
  <el-dialog
    v-model="editDialogVisible"
    title="编辑代理"
    width="600px"
    :before-close="handleClose"
  >
    <el-form
      :model="editForm"
      :rules="rules"
      ref="editFormRef"
      label-width="100px"
    >
      <el-form-item label="代理ID">
        <el-input v-model="editForm.proxyId" disabled></el-input>
      </el-form-item>

      <el-form-item label="代理类型" prop="type">
        <el-select v-model="editForm.type" placeholder="请选择代理类型">
          <el-option label="HTTP" value="HTTP"></el-option>
          <el-option label="HTTPS" value="HTTPS"></el-option>
          <el-option label="SOCKS5" value="SOCKS5"></el-option>
        </el-select>
      </el-form-item>

      <el-form-item label="IP查询渠道" prop="ipChannel">
        <el-select v-model="editForm.ipChannel" placeholder="请选择IP查询渠道">
          <el-option label="IP2Location" value="IP2Location"></el-option>
          <el-option label="ip-api" value="ip-api"></el-option>
        </el-select>
      </el-form-item>

      <el-form-item label="主机 : 端口" prop="host">
        <div style="display: flex; align-items: center; gap: 8px">
          <el-input
            v-model="editForm.host"
            placeholder="请输入主机IP"
            style="flex: 2"
          ></el-input>
          :
          <el-input
            v-model="editForm.port"
            placeholder="请输入端口"
            style="flex: 1"
          ></el-input>
        </div>
      </el-form-item>

      <el-form-item label="代理账号">
        <el-input
          v-model="editForm.username"
          placeholder="请输入账号"
        ></el-input>
      </el-form-item>

      <el-form-item label="代理密码">
        <el-input
          v-model="editForm.password"
          placeholder="请输入密码"
          show-password
        ></el-input>
      </el-form-item>

      <el-form-item label="刷新URL">
        <el-input
          v-model="editForm.refreshUrl"
          placeholder="请输入刷新URL"
        ></el-input>
      </el-form-item>

      <el-form-item label="备注">
        <el-input
          type="textarea"
          v-model="editForm.note"
          placeholder="请输入备注"
          maxlength="200"
          show-word-limit
        ></el-input>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="handleCancel">取消</el-button>
      <el-button type="primary" @click="handleConfirm">确定</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import {
  Search,
  Plus,
  Minus,
  ArrowDown,
  Edit,
  Delete,
} from "@element-plus/icons-vue";

import Config from "@/views/Agency/config.vue";
import MoreResource from "@/views/Agency/moreresource.vue";
import { ElMessage, ElMessageBox, type TabsPaneContext } from "element-plus";

const activeName = ref("first");

const handleClick = (tab: TabsPaneContext, event: Event) => {
  console.log(tab, event);
};

const activeIndex = ref("/agency/list");
const input3 = ref("");
const select = ref("");
const addProxyDialogVisible = ref(false);
const ipQueryChannel = ref("");
const proxyInfo = ref("");
const showSearch = ref(false);
const selectedField = ref("");
const inputValue = ref("");
const fieldOptions = ref([
  { label: "代理ID", value: "代理ID" },
  { label: "代理类型", value: "代理类型" },
  { label: "备注", value: "备注" },
  { label: "代理信息", value: "代理信息" },
  { label: "关联环境数", value: "关联环境数" },
  { label: "代理商", value: "代理商" },
  { label: "IP查询渠道", value: "IP查询渠道" },
  { label: "代理状态", value: "代理状态" },
  { label: "国家/地区", value: "国家/地区" },
]);

// 编辑相关
const editDialogVisible = ref(false);
const editFormRef = ref(null);
const editForm = ref({
  proxyId: "",
  type: "HTTP",
  ipChannel: "IP2Location",
  host: "",
  port: "",
  username: "",
  password: "",
  refreshUrl: "",
  note: "",
});

const rules = {
  type: [{ required: true, message: "请选择代理类型", trigger: "change" }],
  ipChannel: [
    { required: true, message: "请选择IP查询渠道", trigger: "change" },
  ],
  host: [{ required: true, message: "请输入主机IP", trigger: "blur" }],
  port: [
    {
      required: true,
      message: "请输入端口",
      trigger: "blur",
    },
    {
      pattern: /^[0-9]{1,5}$/,
      message: "端口号必须为数字（1-5位）",
      trigger: "blur",
    },
  ],
};

const openEditDialog = (row) => {
  editForm.value = {
    proxyId: row.id,
    type: "HTTP", // 根据实际数据设置
    ipChannel: row.ipQuery,
    host: row.ip,
    port: "8000", // 根据实际数据设置
    username: "",
    password: "",
    refreshUrl: "",
    note: row.remark,
  };
  editDialogVisible.value = true;
};

const handleClose = () => {
  editDialogVisible.value = false;
};

const handleCancel = () => {
  editDialogVisible.value = false;
};

const handleConfirm = async () => {
  editFormRef.value.validate((valid) => {
    if (valid) {
      console.log("表单提交数据:", editForm.value);
      ElMessage.success("代理信息修改成功！");
      editDialogVisible.value = false;
    } else {
      ElMessage.error("请完善表单信息！");
    }
  });
};

const recordsCount = 100;
const tableData = ref(
  Array.from({ length: recordsCount }, (v, i) => ({
    id: i + 1,
    text: "http://123123123.456",
    ip: "192.168.1.1",
    numEnv: 99,
    ipQuery: "2024.8.10 14:47",
    remark: "-",
  }))
);

const currentPage = ref(1);
const pageSize = ref(10);

const paginatedData = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return tableData.value.slice(start, end);
});

const handleSizeChange = (newSize) => {
  pageSize.value = newSize;
};

const handleCurrentChange = (newPage) => {
  currentPage.value = newPage;
};

const handleAddProxy = () => {
  addProxyDialogVisible.value = false;
};

const selectedRows = ref([]);

const handleSelectionChange = (selection) => {
  selectedRows.value = selection;
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
      // 获取选中记录的ID
      const selectedIds = selectedRows.value.map((row) => row.id);
      // 从tableData中过滤掉选中的记录
      tableData.value = tableData.value.filter(
        (row) => !selectedIds.includes(row.id)
      );
      ElMessage.success("删除成功");
    })
    .catch(() => {
      ElMessage.info("已取消删除");
    });
};

const handleCheckProxy = () => {
  if (selectedRows.value.length === 0) {
    ElMessage.warning("请选择要检查的代理");
    return;
  }

  // Logic to check the status of the selected proxies
  selectedRows.value.forEach((proxy) => {
    // Simulate checking proxy status
    console.log(`Checking status of proxy with ID: ${proxy.id}`);
  });

  ElMessage.success("代理检查完成");
};

const conditions = ref([{ field: "", value: "" }]);

const addCondition = () => {
  conditions.value.push({ field: "", value: "" });
};

const removeCondition = (index) => {
  conditions.value.splice(index, 1);
};

const applyFilters = () => {
  console.log("应用筛选条件", conditions.value);
};

const clearFilters = () => {
  conditions.value = [{ field: "", value: "" }];
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
.function {
  margin: 10px;
}
.search-input {
  flex-grow: 1; /* 搜索框占满剩余空间 */
}
.el-button {
  color: black;
  background-color: #f7f8fa;
}
.function {
  display: flex;
  justify-content: space-between; /* 将子元素分布到容器的两端 */
}

.function-left,
.function-right {
  display: flex; /* 使用 Flexbox 对按钮进行水平布局 */
  gap: 10px; /* 设置按钮之间的间距 */
}
.filter-btn {
  display: flex;
  align-items: center;
  font-size: 14px;
  /* color: #409eff; */
  /* border: 1px solid #409eff; */
  border-radius: 8px;
  padding: 6px 12px;
}

.filter-btn.no-focus:focus {
  outline: none;
}

.search-dialog {
  border-radius: 12px;
}

.search-box {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.search-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.add-condition-btn {
  color: #409eff;
  font-size: 14px;
}

.remove-condition-btn {
  color: #f56c6c;
  font-size: 14px;
}

.el-dialog__footer {
  display: flex;
  justify-content: space-between;
}

/* .el-range-input {
  width: 50px;
} */
</style>
