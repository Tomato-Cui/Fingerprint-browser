<template>
  <Layout>
    <div class="Env">
      <div class="search-bar">
        <el-input-group>
          <!-- 筛选字段选择框 -->
          <el-select v-model="select" placeholder="请选择字段" style="width: 200px" class="adaptive-select">
            <el-option v-for="item in fields" :key="item.value" :label="item.label" :value="item.value"></el-option>
          </el-select>
          <!-- 操作符选择框 -->
          <el-select v-model="operator" placeholder="操作符" style="width: 150px" class="adaptive-select">
            <el-option v-for="item in operators" :key="item.value" :label="item.label" :value="item.value"></el-option>
          </el-select>
          <!-- 输入内容框 -->
          <el-input v-model="input3" placeholder="请输入内容" style="width: 300px" class="adaptive-input"></el-input>
          <!-- 删除条件按钮 -->
          <!-- <el-button
          icon="el-icon-close"
          type="text"
          @click="removeCondition()"
        ></el-button> -->
          <!-- 搜索按钮 -->
          <el-button :icon="Search" @click="searchData()" />
        </el-input-group>
      </div>
      <div class="function">
        <div class="function-left">
          <!-- 批量打开按钮 -->
          <el-button type="text" :onclick="openBrowsers">
            <img src="@/icons/svg/dakai.svg" alt="" />批量打开</el-button>
          <!-- 批量导出按钮 -->
          <el-button type="text">
            <img src="@/icons/svg/daochu.svg" alt="" />批量导出</el-button>
          <!-- 关闭按钮 -->
          <el-button type="text">
            <img src="@/icons/svg/guanbi.svg" alt="" />关闭</el-button>
          <!-- 移动按钮 -->
          <el-button type="text" @click="moveDialogVisible = true">
            <img src="@/icons/svg/yidong.svg" alt="" />移动</el-button>

          <!-- 移动环境对话框 -->
          <el-dialog v-model="moveDialogVisible" title="移动环境" width="600px" center>
            <el-form :model="moveForm" label-width="80px">
              <el-form-item label="环境编号">
                <span>{{ selectedEnvId }}</span>
              </el-form-item>
              <el-form-item label="分组">
                <el-select v-model="moveForm.group" placeholder="请选择分组" style="width: 100%">
                  <el-option label="分组1" value="1" />
                  <el-option label="分组2" value="2" />
                  <el-option label="分组3" value="3" />
                </el-select>
              </el-form-item>
              <el-form-item label="授权用户">
                <el-select v-model="moveForm.users" multiple placeholder="请选择用户" style="width: 100%">
                  <el-option label="用户1" value="1" />
                  <el-option label="用户2" value="2" />
                  <el-option label="用户3" value="3" />
                </el-select>
              </el-form-item>
            </el-form>
            <template #footer>
              <span class="dialog-footer">
                <el-button @click="moveDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleMoveConfirm">确定</el-button>
              </span>
            </template>
          </el-dialog>

          <el-dropdown>
            <!-- 修改环境按钮 -->
            <el-button type="text" :disabled="selectedRows.length !== 1" style="display: none">
              <img src="@/icons/svg/xiugaihuanjing.svg" alt="" />
              修改环境
              <el-icon class="el-icon--right"><arrow-down /></el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item @click="proxyDialogVisible = true">
                  <el-icon>
                    <Connection />
                  </el-icon>修改代理
                </el-dropdown-item>
                <el-dropdown-item @click="fingerprintDialogVisible = true">
                  <el-icon>
                    <Stamp />
                  </el-icon>修改指纹
                </el-dropdown-item>
                <el-dropdown-item @click="accountDialogVisible = true">
                  <el-icon>
                    <User />
                  </el-icon>修改账号
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>

          <!-- 修改代理对话框 -->
          <el-dialog v-model="proxyDialogVisible" title="修改代理" width="50%" center>
            <el-form :model="proxyForm" label-width="100px">
              <el-form-item label="环境编号">
                <span>{{ selectedEnvId }}</span>
              </el-form-item>
              <el-form-item label="代理方式">
                <el-radio-group v-model="proxyForm.method">
                  <el-radio label="custom">自定义</el-radio>
                  <el-radio label="added">已添加的代理</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="代理类型">
                <el-radio-group v-model="proxyForm.type">
                  <el-radio label="SSH">SSH</el-radio>
                  <el-radio label="HTTPS">HTTPS</el-radio>
                  <el-radio label="HTTP">HTTP</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="IP查询渠道">
                <el-radio-group v-model="proxyForm.ipChannel">
                  <el-radio label="IP2Location">IP2Location</el-radio>
                  <el-radio label="ip-api">ip-api</el-radio>
                </el-radio-group>
              </el-form-item>
            </el-form>
            <template #footer>
              <span class="dialog-footer">
                <el-button @click="proxyDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleProxyConfirm">确定</el-button>
              </span>
            </template>
          </el-dialog>

          <!-- 修改指纹对话框 -->
          <el-dialog v-model="fingerprintDialogVisible" title="修改指纹" width="50%" center :style="{ padding: '20px' }">
            <el-form :model="fingerprintForm" label-width="100px" label-position="left">
              <el-form-item label="环境编号">
                <span>{{ selectedEnvId }}</span>
              </el-form-item>
              <el-form-item label="操作系统">
                <el-radio-group v-model="fingerprintForm.os">
                  <el-radio label="windows">Windows</el-radio>
                  <el-radio label="mac">Mac</el-radio>
                  <el-radio label="linux">Linux</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="WebRTC">
                <el-radio-group v-model="fingerprintForm.webrtc">
                  <el-radio label="forward">转发</el-radio>
                  <el-radio label="replace">替换</el-radio>
                  <el-radio label="real">真实</el-radio>
                  <el-radio label="disabled">禁用</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="User Agent">
                <el-select v-model="fingerprintForm.userAgent" placeholder="请选择User Agent">
                  <el-option label="Chrome" value="chrome"></el-option>
                  <el-option label="Firefox" value="firefox"></el-option>
                  <el-option label="Safari" value="safari"></el-option>
                </el-select>
              </el-form-item>
              <el-form-item label="时区">
                <el-radio-group v-model="fingerprintForm.timezone">
                  <el-radio label="GMT+0">GMT+0</el-radio>
                  <el-radio label="GMT+8">GMT+8</el-radio>
                  <el-radio label="GMT-5">GMT-5</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="地理位置">
                <el-radio-group v-model="fingerprintForm.location">
                  <el-radio label="美国">美国</el-radio>
                  <el-radio label="中国">中国</el-radio>
                  <el-radio label="欧洲">欧洲</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="语言">
                <el-radio-group v-model="fingerprintForm.language">
                  <el-radio label="中文">中文</el-radio>
                  <el-radio label="英文">英文</el-radio>
                  <el-radio label="法文">法文</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="界面语言">
                <el-radio-group v-model="fingerprintForm.interfaceLanguage">
                  <el-radio label="中文">中文</el-radio>
                  <el-radio label="英文">英文</el-radio>
                  <el-radio label="法文">法文</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="分辨率">
                <el-radio-group v-model="fingerprintForm.resolution">
                  <el-radio label="1920x1080">1920x1080</el-radio>
                  <el-radio label="1366x768">1366x768</el-radio>
                  <el-radio label="1280x720">1280x720</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="字体">
                <el-radio-group v-model="fingerprintForm.font">
                  <el-radio label="Arial">Arial</el-radio>
                  <el-radio label="Times New Roman">Times New Roman</el-radio>
                  <el-radio label="Courier New">Courier New</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="硬件噪音">
                <el-checkbox-group v-model="fingerprintForm.hardwareNoise">
                  <el-checkbox label="CPU">CPU</el-checkbox>
                  <el-checkbox label="GPU">GPU</el-checkbox>
                  <el-checkbox label="Memory">Memory</el-checkbox>
                </el-checkbox-group>
              </el-form-item>
              <el-form-item label="WebGL元数据">
                <el-radio-group v-model="fingerprintForm.webglMetadata">
                  <el-radio label="启用">启用</el-radio>
                  <el-radio label="禁用">禁用</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item label="WebGPU">
                <el-radio-group v-model="fingerprintForm.webgpu">
                  <el-radio label="启用">启用</el-radio>
                  <el-radio label="禁用">禁用</el-radio>
                </el-radio-group>
              </el-form-item>
            </el-form>
            <template #footer>
              <span class="dialog-footer">
                <el-button @click="fingerprintDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleFingerprintConfirm">确定</el-button>
              </span>
            </template>
          </el-dialog>

          <!-- 修改账号对话框 -->
          <el-dialog v-model="accountDialogVisible" title="修改账号" width="50%" center>
            <el-form :model="accountForm" label-width="100px">
              <el-form-item label="环境编号">
                <span>{{ selectedEnvId }}</span>
              </el-form-item>
              <el-form-item label="账号平台">
                <el-input v-model="accountPlatformForm.account_platform" placeholder="请输入平台" />
              </el-form-item>
              <el-form-item label="标签">
                <el-input type="textarea" v-model="accountForm.tags" placeholder="请输入标签，多个标签用逗号分隔" />
              </el-form-item>
              <el-form-item label="用户账号">
                <el-input v-model="accountForm.username" placeholder="请输入用户账号" />
              </el-form-item>
              <el-form-item label="用户密码">
                <el-input v-model="accountForm.password" type="password" placeholder="请输入密码" />
              </el-form-item>
              <el-form-item label="2FA密钥">
                <el-input v-model="accountForm.twoFactorKey" placeholder="请输入2FA密钥" />
              </el-form-item>
              <el-form-item label="Cookie">
                <el-input type="textarea" v-model="accountForm.cookies" placeholder="请输入Cookie" :rows="3" />
              </el-form-item>
            </el-form>
            <template #footer>
              <span class="dialog-footer">
                <el-button @click="accountDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleAccountConfirm">确定</el-button>
              </span>
            </template>
          </el-dialog>

          <el-dropdown trigger="hover">
            <!-- 更多按钮 -->
            <el-button type="text">
              <img src="@/icons/svg/gengduo.svg" alt="" />更多</el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item>更新环境</el-dropdown-item>
                <el-dropdown-item>环境信息</el-dropdown-item>
                <el-dropdown-item>代理信息</el-dropdown-item>
                <el-dropdown-item>指纹信息</el-dropdown-item>
                <el-dropdown-item>清除缓存</el-dropdown-item>
                <el-dropdown-item>自定义快捷区</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
        <div class="function-right">
          <!-- 刷新按钮 -->
          <el-button type="text" @click="refreshData">
            <img src="@/icons/svg/shuaxin-small.svg" alt="" />刷新</el-button>
        </div>
      </div>
      <div>
        <!-- 数据表格 -->
        <el-table :data="tableData" border style="width: 100%" @selection-change="handleSelectionChange">
          <el-table-column type="selection" width="55"></el-table-column>

          <el-table-column prop="ID" label="ID/编码" width="auto"></el-table-column>

          <el-table-column prop="group_id" label="分组" width="auto"></el-table-column>
          <el-table-column label="名称" width="auto">
            <template #default="scope">
              <div style="
                  display: flex;
                  justify-content: space-between;
                  align-items: center;
                ">
                <span>{{ scope.row.name }}</span>
                <el-button type="text" @click="openNameDialog(scope.row)">修改名称</el-button>
              </div>
            </template>
          </el-table-column>
          <el-table-column label="IP" width="auto">
            <template #default="scope">
              <div style="
                  display: flex;
                  justify-content: space-between;
                  align-items: center;
                ">
                <span>{{ scope.row.proxy }}</span>
                <el-button type="text" @click="proxyDialogVisible = true"
                  >修改IP</el-button
                >
              </div>
            </template>
          </el-table-column>
          <el-table-column prop="lasted_at" label="最近打开" width="auto"></el-table-column>
          <el-table-column prop="domain_name" label="账号平台" width="auto">
            <template #default="scope">
              <div style="
                  display: flex;
                  justify-content: space-between;
                  align-items: center;
                ">
                <span>{{ scope.row.domain_name }}</span>
                <el-button type="text" @click="openAccountPlatformDialog(scope.row)">修改平台</el-button>
              </div>
            </template>
          </el-table-column>

          <el-table-column label="备注" width="auto" prop="remark">
            <template #default="scope">
              <div style="
                  display: flex;
                  justify-content: space-between;
                  align-items: center;
                ">
                <span>{{ scope.row.remark }}</span>
                <el-button type="text" @click="openRemarksDialog(scope.row)">修改备注</el-button>
              </div>
            </template>
          </el-table-column>

          <el-table-column label="创建时间" width="auto" prop="CreatedAt">
            <template #default="scope">
              <span>{{
                scope.row.CreatedAt
                  ? new Date(scope.row.CreatedAt).toLocaleString()
                  : "N/A"
              }}</span>
            </template>
          </el-table-column>
          <el-table-column label="浏览器" width="auto">
            <template #default="scope">
              <div style="
                  display: flex;
                  justify-content: space-between;
                  align-items: center;
                ">
                <!-- <span>浏览器</span> -->
                <el-button type="text"
                  @click="() => isOpened(scope.row) ? closeBrowser(scope.row) : openBrowser(scope.row)">{{
                    isOpened(scope.row) ? "关闭" : "打开"
                  }}浏览器</el-button>
              </div>
            </template>
          </el-table-column>

          <el-table-column label="操作" width="auto">
            <template #default="scope">
              <el-dropdown>
                <el-button type="text">
                  更多<el-icon><arrow-down /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="handleEdit(scope.row)">编辑</el-dropdown-item>
                    <el-dropdown-item @click="handleCopy(scope.row)">复制</el-dropdown-item>
                    <el-dropdown-item @click="handleDeleteClickWrapper(scope.row)">删除</el-dropdown-item>
                    <el-dropdown-item @click="handleClearCache(scope.row)">清除缓存</el-dropdown-item>
                    <el-dropdown-item @click="openProxyDialog(scope.row)">修改代理</el-dropdown-item>
                    <el-dropdown-item @click="openAccountDialog(scope.row)">修改账号</el-dropdown-item>
                    <el-dropdown-item @click="openFingerprintDialog(scope.row)">修改指纹</el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </template>
          </el-table-column>
        </el-table>
        <!-- 分页组件 -->
        <el-pagination background layout="prev, pager, next, sizes, total" :current-page="currentPage"
          :page-sizes="[5, 10, 20, 50]" :page-size="pageSize" :total="total" @size-change="handleSizeChange"
          @current-change="handleCurrentChange"></el-pagination>
      </div>
    </div>

    <!-- 修改状态对话框 -->
    <el-dialog v-model="accountPlatformDialogVisible" title="修改账号平台" width="50%" center>
      <el-form :model="statusForm" label-width="100px">
        <el-form-item label="网址">
          <el-input v-model="accountPlatformForm.account_platform" placeholder="请输入网址" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="accountPlatformDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="confirmUpdateAccountPlatform">确定</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 修改名称对话框 -->
    <el-dialog v-model="countDialogVisible" title="修改名称" width="50%" center>
      <el-form :model="countForm" label-width="100px">
        <el-form-item label="名称">
          <el-input v-model="countForm.count" placeholder="请输入名称" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="countDialogVisible = false">取消</el-button>

          <el-button type="primary" @click="confirmUpdateName()">确定</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 修改IP对话框 -->
    <el-dialog v-model="proxyDialogVisible" title="修改IP" width="50%" center>
      <el-form :model="ipForm" label-width="100px">
        <el-form-item label="IP地址">
          <!-- <el-input v-model="ipForm.ip" placeholder="请输入IP地址" /> -->
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="proxyDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleProxyConfirm">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </Layout>
</template>

<script setup lang="ts">
import Layout from "@/layouts/index.vue";

import { ref, onMounted } from "vue";
import {
  Search,
  Connection,
  Stamp,
  User,
  ArrowDown,
} from "@element-plus/icons-vue";

import {
  // updateEnvironmentIP,
  // handleNameUpdate,
  loadData,
  handleDeleteClick,
  updateEnvironmentIP,
  updateEnvironmentName,
  updateEnvironmentAccountPlatform,
  getEnvironmentGroup,
} from "@/api/environmentService";

import { ElMessageBox, ElMessage } from "element-plus";
import router from "@/router";
import { useRoute } from "vue-router";
import { default_fingerprint, start, starts, stops } from "@/command/environment";

const input3 = ref("");
const select = ref("");
const operator = ref("");

// 搜索框可选字段
const fields = [
  { label: "名称", value: "name" },
  { label: "环境编号", value: "env_code" },
  { label: "序号", value: "order" },
  { label: "标签", value: "tag" },
  { label: "环境ID", value: "env_id" },
  { label: "账号平台", value: "account_platform" },
  { label: "平台账号", value: "platform_account" },
  { label: "备注", value: "remark" },
  { label: "IP", value: "ip" },
];

// 可选操作符
const operators = [
  { label: "包含", value: "contains" },
  { label: "等于", value: "equals" },
  { label: "开始于", value: "starts_with" },
  { label: "结束于", value: "ends_with" },
  { label: "不包含", value: "not_contains" },
  { label: "为空", value: "is_empty" },
  { label: "不为空", value: "is_not_empty" },
];

// 对话框显示状态
const moveDialogVisible = ref(false);
const proxyDialogVisible = ref(false);
const fingerprintDialogVisible = ref(false);
const accountDialogVisible = ref(false);
const statusDialogVisible = ref(false);
const countDialogVisible = ref(false);

const tableData = ref<any>([]);

const originalData = ref([...tableData.value]); // 这里保存原始数据

const searchQuery = ref(""); // 用于存储搜索的内容

// 处理搜索数据的方法
const searchData = () => {
  if (searchQuery.value.trim() === "") {
    tableData.value = [...originalData.value]; // 如果搜索框为空，恢复原始数据
  } else {
    tableData.value = originalData.value.filter((row) =>
      row.group_name.includes(searchQuery.value)
    );
  }
};

const total = ref(0);
const currentPage = ref(1);
const pageSize = ref(10);

// 每页大小改变时触发
const handleSizeChange = (newSize: number) => {
  pageSize.value = newSize; // 更新每页大小
  console.log("pageSize:", pageSize.value);
  const obj = loadData(currentPage.value, pageSize.value);

  Promise.resolve(obj).then((res) => {
    tableData.value = res.tableData;
    total.value = Number(res.total);
  });
};

// 当前页码改变时触发
const handleCurrentChange = (newPage: number) => {
  currentPage.value = newPage; // 更新当前页码
  const obj = loadData(currentPage.value, pageSize.value);

  Promise.resolve(obj).then((res) => {
    tableData.value = res.tableData;
    total.value = Number(res.total);
  });
};

// 表单数据
const moveForm = ref({
  group: "",
  users: [],
});

const proxyForm = ref({
  method: "custom",
  type: "SSH",
  ipChannel: "IP2Location",
});

const fingerprintForm = ref({
  os: "windows",
  webrtc: "forward",
  userAgent: "",
  timezone: "",
  location: "",
  language: "",
  interfaceLanguage: "",
  resolution: "",
  font: "",
  hardwareNoise: [],
  webglMetadata: "",
  webgpu: "",
});

const accountForm = ref({
  platform: "",
  tags: "",
  username: "",
  password: "",
  twoFactorKey: "",
  cookies: "",
});

const statusForm = ref({
  status: "",
});

const countForm = ref({
  count: "",
});

const selectedEnvId = ref("ENV001");

const selectedRows = ref<any[]>([]);

// 处理确认事件
const handleMoveConfirm = () => {
  moveDialogVisible.value = false;
};

const handleProxyConfirm = () => {
  proxyDialogVisible.value = false;
};

const handleFingerprintConfirm = () => {
  fingerprintDialogVisible.value = false;
};

const handleAccountConfirm = () => {
  accountDialogVisible.value = false;
};

const refreshData = async () => {
  try {
    await loadData();
  } catch (error) {
    console.error("刷新数据失败:", error);
  }
};

// const openNameDialog = (row: any) => {
//   // 非空和长度限制验证
//   if (!row.name || row.name.length === 0) {
//     ElMessage.error("名称不能为空");
//     return; // 退出函数，不继续执行
//   }

//   if (row.name.length > 50) {
//     // 假设名称长度最大为 50
//     ElMessage.error("名称不能超过 50 个字符");
//     return; // 退出函数，不继续执行
//   }

//   // 如果名称通过验证，则继续执行以下逻辑
//   selectedRow.value = row;
//   currentRow.value.id = row.id; // 需要修改的哪一行的id
//   console.log("------:", currentRow.value.id);

//   // 显示 countDialogVisible 对话框
//   countForm.value.count = row.count;
//   countDialogVisible.value = true;
// };

const openNameDialog = (row: any) => {
  console.log("tableData:", tableData.value);
  console.log("row.id:" + row.ID);
  if (!row.ID) {
    ElMessage.error("ID 无效");
    return; // 如果 ID 无效，不继续执行
  }

  // 非空和长度限制验证
  if (!row.name || row.name.length === 0) {
    ElMessage.error("名称不能为空");
    return; // 退出函数，不继续执行
  }

  if (row.name.length > 50) {
    // 假设名称长度最大为 50
    ElMessage.error("名称不能超过 50 个字符");
    return; // 退出函数，不继续执行
  }

  // 如果名称通过验证，则继续执行以下逻辑
  selectedRow.value = row;
  currentRow.value.id = row.ID; // 需要修改的哪一行的id

  // 显示 countDialogVisible 对话框
  countForm.value.count = row.count;
  countDialogVisible.value = true;
};

const openProxyDialog = (row: any) => {
  selectedRow.value = row;
  proxyForm.value = { ...row.proxy };
  proxyDialogVisible.value = true;
};

const openAccountDialog = (row: any) => {
  selectedRow.value = row;
  accountForm.value = { ...row.account };
  accountDialogVisible.value = true;
};

const openFingerprintDialog = (row: any) => {
  selectedRow.value = row;
  fingerprintForm.value = { ...row.fingerprint };
  fingerprintDialogVisible.value = true;
};

const handleSelectionChange = (val: any) => {
  selectedRows.value = val;
};

const handleEdit = (row: any) => {
  // 编辑逻辑
};

const handleCopy = (row: any) => {
  // 复制逻辑
};

const handleClearCache = (row: any) => {
  // 清除缓存逻辑
};

const newIp = ref(""); // 用户输入的新 IP 地址

// 修改 IP 对话框
const ipDialogVisible = ref(false);
const ipForm = ref({ ip: "" });
const selectedRow = ref(null);

// 打开 IP 对话框
const openIpDialog = (row: any) => {
  selectedRow.value = row;
  ipForm.value.ip = row.ip;
  ipDialogVisible.value = true;
};

// IP 校验
const isValidIPv4 = (ip: string): boolean => {
  const ipv4Pattern =
    /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
  return ipv4Pattern.test(ip);
};

// 确认修改 IP
const handleIpConfirm = async () => {
  console.log("ipForm.value.ip:", ipForm.value.ip);
  try {
    if (!isValidIPv4(ipForm.value.ip)) {
      ElMessage.error("请输入有效的 IPv4 地址");
      return;
    }

    if (!selectedRow.value) {
      ElMessage.error("未选择要修改的行");
      return;
    }

    await updateEnvironmentIP(selectedRow.value.ID, ipForm.value.ip);
    ElMessage.success("IP 更新成功");
    ipDialogVisible.value = false;
    loadData(currentPage.value, pageSize.value); // 重新加载数据
  } catch (error) {
    ElMessage.error("IP 更新失败");
  }
};

// 当前选择的行和名称
// const nameDialogVisible = ref(false); // 控制修改名称对话框的显示
// const currentName = ref(""); // 存储当前要修改的名称
const currentRow = ref<any>({
  id: "",
}); // 存储当前选中的行

// 加载表格数据

// // 提交修改名称的请求
// const confirmUpdateName = async () => {
//   if (countForm.value.count === "") {
//     ElMessage.error("名称不能为空！");
//     return;
//   }

//   try {
//     await updateEnvironmentName(currentRow.value.id, countForm.value.count);
//     // ElMessage.success("名称修改成功！");
//     countDialogVisible.value = false;
//     await loadData(currentPage.value, pageSize.value); // 重新加载数据
//     console.log("调用loadData");

//     window.location.reload();
//   } catch (error) {
//     console.error("更新名称失败：", error);
//     ElMessage.error("名称修改失败，请稍后重试！");
//   }
// };

const confirmUpdateName = async () => {
  if (countForm.value.count === "") {
    ElMessage.error("名称不能为空！");
    return;
  }

  try {
    // 调用修改名称的接口
    await updateEnvironmentName(currentRow.value.id, countForm.value.count);

    // 关闭名称修改对话框
    countDialogVisible.value = false;

    // 重新加载数据，刷新表格内容
    window.location.reload();
    console.log("数据已更新");

    // 如果您想要显示修改成功的信息，可以使用以下消息提示
    ElMessage.success("名称修改成功！");
  } catch (error) {
    console.error("更新名称失败：", error);
    ElMessage.error("名称修改失败，请稍后重试！");
  }
};

const accountPlatformDialogVisible = ref(false);
// const currentRow = ref<any>(null);
const accountPlatformForm = ref({
  account_platform: "",
});

// 打开对话框
const openAccountPlatformDialog = (row: any) => {
  console.log(
    "openAccountPlatformDialog拿到的数据行的数据",
    row.account_platform
  );
  currentRow.value = row;
  accountPlatformForm.value.account_platform = row.account_platform;
  accountPlatformDialogVisible.value = true;
  console.log(
    "accountPlatformForm的值",
    accountPlatformForm.value.account_platform
  );
};

// 提交修改请求
const confirmUpdateAccountPlatform = async () => {
  console.log("变量的值", accountPlatformForm.value.account_platform);
  console.log(
    "accountPlatformForm.value.account_platform:",
    accountPlatformForm.value.account_platform
  );
  if (!accountPlatformForm.value.account_platform.trim()) {
    ElMessage.error("账号平台不能为空！");
    return;
  }

  try {
    console.log("currentRow.value.id:", currentRow.value.id);
    console.log(
      "accountPlatformForm.value.account_platform:",
      accountPlatformForm.value.account_platform
    );
    await updateEnvironmentAccountPlatform(
      currentRow.value.id,
      accountPlatformForm.value.account_platform
    );
    ElMessage.success("账号平台修改成功！");
    accountPlatformDialogVisible.value = false;

    // 更新表格数据
    const updatedRow = tableData.value.find(
      (row) => row.id === currentRow.value.id
    );
    if (updatedRow) {
      updatedRow.account_platform = accountPlatformForm.value.account_platform;
    }
  } catch (error) {
    console.error("更新账号平台失败:", error);
    ElMessage.error("账号平台修改失败，请稍后重试！");
  }
};

onMounted(async () => {
  const route = useRoute();
  let group_id = route.query.group;

  try {
    if (group_id) {
      let data = await getEnvironmentGroup(group_id as any, [
        currentPage.value,
        pageSize.value,
      ]);
      tableData.value = data.data.data;
      console.log(data.data.data);
      total.value = Number(data.data.total_count);
    } else {
      const data = await loadData(currentPage.value, pageSize.value);

      tableData.value = data.data.data;
      console.log(data.data.data);
      total.value = Number(data.data.total_count);
    }
  } catch (error) {
    console.error("更新账号平台失败:", error);
    ElMessage.error("账号平台修改失败，请稍后重试！");
  }
});

const handleDeleteClickWrapper = async (row: any) => {
  let ok = await handleDeleteClick(row);
  if (ok) {
    tableData.value = tableData.value.filter((item: any) => item.ID != row.ID);
  }
};

const openedBrowser = ref<Array<string>>([]);
const isOpened = (row: any) => {
  let exist = openedBrowser.value.find(item => item == row.ID)
  return exist
}
const openBrowser = async (row: any) => {
  let response = await start(row.ID, default_fingerprint)
  let rows = response.data;

  console.log(row.ID, response);

  for (let key of Object.keys(rows)) {
    let { status } = rows[key];
    if (status) { openedBrowser.value.push(key) }
  }
}
const closeBrowser = async (row: any) => {
  let response = await stops([row.ID])
}

const openBrowsers = async () => {
  let checkEnvs = selectedRows.value.map(item => ({
    environment_id: item.ID,
    fingerprint: default_fingerprint,
  }));

  let response = await starts(checkEnvs);

  let rows = response.data;
  for (let key of Object.keys(rows)) {
    let { status } = rows[key];
    if (status) { openedBrowser.value.push(key) }
  }
}

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

.adaptive-select {
  min-width: 120px;
}

.adaptive-input {
  min-width: 180px;
}

.Env {
  background-color: #ffffff;
  border-radius: 16px 16px;
}

.function {
  margin: 10px;
}

.search-input {
  flex-grow: 1;
}

.el-button {
  color: black;
  background-color: #f7f8fa;
}

.function {
  display: flex;
  justify-content: space-between;
}

.function-left,
.function-right {
  display: flex;
  gap: 10px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

/* 操作按钮样式 */
.operation-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
}

@media (max-width: 768px) {
  .function {
    flex-direction: column;
  }

  .function-left,
  .function-right {
    justify-content: center;
  }
}
</style>
