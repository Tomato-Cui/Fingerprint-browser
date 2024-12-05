<template>
  <Layout>
    <div class="api-settings">
      <!-- API Settings -->
      <div class="api-card">
        <h3>API设置</h3>
        <div class="api-status">
          <!-- 接口状态 -->
          <el-row>
            <el-col :span="6">
              <span>接口状态</span>
            </el-col>
            <el-col :span="18">
              <!-- <el-alert type="custom" class="custom-alert" description="该功能仅限付费套餐可用，如需使用，请升级套餐。" show-icon /> -->
              <div class="custom-alert">
                <el-icon>
                  <WarningFilled />
                </el-icon>
                该功能仅限付费套餐可用，如需使用，请升级套餐。
              </div>
            </el-col>
          </el-row>
          <!-- API KEY -->
          <el-row>
            <el-col :span="6">
              <el-popover placement="top-start" :width="300" trigger="hover" content="命令行界面下，使用API key进行验证"
                popper-class="popper-sty">
                <template #reference>
                  <span style="margin-right: 5px;">API Key
                    <el-icon>
                      <Warning />
                    </el-icon>
                  </span>
                </template>
              </el-popover>
            </el-col>
            <el-col :span="18">
              <el-button type="primary" link><el-icon>
                  <Refresh />
                </el-icon>&nbsp;生成Key</el-button>
            </el-col>
          </el-row>
          <!-- 接口文档 -->
          <el-row>
            <el-col :span="6">
              <span>接口文档</span>
            </el-col>
            <el-col :span="18">
              <el-link href="#" type="primary"><el-icon>
                  <Document />
                </el-icon>&nbsp;查看文档</el-link>
            </el-col>
          </el-row>
        </div>
      </div>

      <!-- Migration Settings -->
      <div class="migration-card">
        <h3>迁移环境</h3>
        <div class="platform-tabs">
          <div class="platform-t-header">平台类型</div>
          <el-tabs v-model="activeTab" type="border-card">
            <el-tab-pane label="Dolphin{anty}" name="dolphin">
              <div>
                <div style="margin: 20px 0;">
                  <span class="platform-tabs-1">使用Dolphin的免费套餐时迁移的环境不包含Cookie。</span>
                  <el-link type="primary">操作指引</el-link>
                </div>
                <div class="platform-t-h-form">
                  <!-- Input for Dolphin_Token -->
                  <el-form :model="form" label-width="150px" class="gologin-form">
                    <el-form-item label="Dolphin_Token">
                      <el-input v-model="form.Dolphin_Token" placeholder="请输入Dolphin_Token"
                        style="width: 300px; height: 40px;" />
                    </el-form-item>
                  </el-form>

                  <!-- Start Migration Button -->
                  <div class="button-container">
                    <el-button type="primary" @click="startMigration" plain>
                      开始迁移
                    </el-button>
                  </div>
                </div>
              </div>
            </el-tab-pane>
            <el-tab-pane label="GoLogin" name="gologin">
              <div>
                <el-link type="primary" style="margin: 15px 0;">操作指引</el-link>
                <div class="platform-t-h-form">
                  <!-- Input for GoLogin_Token -->
                  <el-form :model="form" label-width="150px" class="gologin-form">
                    <el-form-item label="GoLogin_Token">
                      <el-input v-model="form.gologinToken" placeholder="请输入GoLogin_Token"
                        style="width: 300px; height: 40px;" />
                    </el-form-item>
                  </el-form>

                  <!-- Start Migration Button -->
                  <div class="button-container">
                    <el-button type="primary" @click="startMigration" plain>
                      开始迁移
                    </el-button>
                  </div>
                </div>
              </div>
            </el-tab-pane>
            <el-tab-pane label="Multilogin" name="multilogin">
              <div class="migration-options">
                <div style="margin: 10px 0;">
                  <span class="platform-tabs-1">仅支持迁移 Multilogin 付费套餐的环境。</span>
                  <el-link type="primary">操作指引</el-link>
                </div>
                <el-checkbox v-model="migrateCookie">迁移环境Cookie</el-checkbox>

                <div class="platform-t-h-form">
                  <el-form :model="form" label-width="100px" class="multilogin-form">
                    <el-form-item label="Multilogin_Port">
                      <el-input v-model="form.multiloginPort" placeholder="请输入Multilogin_Port"
                        style="width: 300px; height: 40px;" />
                    </el-form-item>
                    <el-form-item label="Multilogin_Token">
                      <el-input v-model="form.multiloginToken" placeholder="请输入Multilogin_Token"
                        style="width: 300px; height: 40px;" />
                    </el-form-item>
                  </el-form>

                  <el-button type="primary" @click="startMigration">开始迁移</el-button>
                </div>

              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </div>

      <!-- 功能概述、用户须知 -->
      <div class="bottom-info">
        <div class="bottom-i-top">
          <div style="font-size: 18px; margin: 20px 0; font-weight: 800;">API功能概述</div>
          <div class="bottom-i-top-info">
            我们的API可以协助用户通过程序化的方式来读写环境配置信息、启动和关闭环境、查询环境等基础接口，还可以配合Selenium和Puppeteer等自动化框架来实现浏览器环境操作的自动化。</div>
        </div>
        <div class="bottom-i-bottom">
          <span style="font-weight: 800;">使用须知</span>
          <div class="bottom-i-top-info">
            <ol class="instruction-list">
              <li>
                <p>1.使用前准备：</p>
                <ul>
                  <li>- 检查登录的账号是否拥有API的功能权限。</li>
                  <li>- 检查接口状态是否正常。</li>
                </ul>
              </li>
              <li>
                <p>2.参数说明：</p>
                <ul>
                  <li>-API的参数均为字符串类型。</li>
                  <li>-POST的内容均为JSON格式。</li>
                  <li>-非必填参数均为可选参数，可以不传递。</li>
                </ul>
              </li>
              <li>
                <p>3.接口上限：</p>
                <ul>
                  <li>-访问频次，所有接口每秒最多请求1次。</li>
                </ul>
              </li>
              <li>
                <p>
                  4.支持CLI模式下启动API服务。
                  <el-link href="#" type="primary" underline>了解更多</el-link>
                </p>
              </li>
            </ol>
          </div>
        </div>
      </div>
    </div>
  </Layout>
</template>

<script setup>
import { ref } from "vue"
import { Refresh, Document, WarningFilled, Warning } from '@element-plus/icons-vue'
import axios from "axios";
import Layout from "@/layouts/index.vue"
import { fun1, StartingMigrations } from '@/api/apis'

// Reactive state variables
const activeTab = ref('dolphin'); // Set default active tab
const migrateCookie = ref(false);   // Checkbox state for migrating cookies
const form = ref({
  multiloginPort: '',  // Placeholder for Multilogin port input
  multiloginToken: '',  // Placeholder for Multilogin token input
  gologinToken: '',
  Dolphin_Token: ''
});

// Method for starting migration
const startMigration = () => {
  // if (!form.value.multiloginPort || !form.value.multiloginToken) {
  //   console.error('Both Multilogin Port and Token are required!');
  //   // axios.post("/", )
  //   return;
  // }
  // startMigration(form);
  // apiApi.startMigration(form)
  fun1()
  StartingMigrations(form.value)
  // console.log('Starting migration with data:', form.value);
};
</script>

<style scoped>
/* API设置 */
.custom-alert {
  padding: 5px 3px;
  padding-left: 10px;
  font-size: 14px;
  border-radius: 10px;
  background-color: #dcedfc;
  /* 自定义背景色 */
  border: 1px solid #00008b;
  /* 自定义边框颜色 */
  color: #00008b;
  /* 自定义文字颜色 */
}

/* el-popover弹出框 */
.el-popover {
  font-size: 12px;
}

.api-settings {
  padding: 20px;
  background-color: #fff;
}

.api-card,
.migration-card {
  margin-bottom: 20px;
}

.api-status {
  margin: 10px 0;
  padding-left: 40px;
}

.api-status>.el-row {
  width: 500px;
  min-height: 50px;
  display: flex;
  align-items: center;
}

.api-key-section {
  margin: 15px 0;
}

.api-doc-section {
  margin-top: 10px;
}

/* 迁移环境 */
.el-tabs--border-card {
  border: none;
}

:deep .el-tabs--border-card>.el-tabs__header {
  background-color: #fff;
  border: none;
}

:deep .el-tabs--border-card>.el-tabs__content {
  padding: 15px 0;
}

:deep .el-tabs__nav {
  padding: 0 10px;
  background-color: #e4e7ed;
  border-radius: 5px;
  /* border: none; */
}

:deep .el-tabs__item {
  /* width: 100px; */
  padding: 0;
  margin-top: 5px !important;
  margin-bottom: 5px;
  margin-right: 10px;
  padding-left: 20px;
  padding-right: 20px;
  border-radius: 5px;
}

:deep(.el-tabs__item):focus {
  box-shadow: 5px 5px 10px #cfcfd2;
}

.platform-tabs {
  margin-top: 20px;
  padding-left: 40px;
  display: flex;
}

.platform-t-header {
  width: 150px;
  display: flex;
  justify-content: center;
  line-height: 50px;
}

.platform-tabs-1 {
  font-size: 15px;
  color: gray;
}

.platform-t-h-form {
  background-color: #f5f5f5;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 10px;
}

.migration-options {
  margin-top: 15px;
}

.multilogin-form {
  padding: 10px 40px;
  margin-top: 10px;
}

/* 功能概述、用户须知 */
.bottom-i-top-info {
  font-size: 12px;
  color: #8f92a0;
  margin-bottom: 40px;
}

.instruction-list {
  list-style-type: none;
  padding: 0;
  color: #8f92a0;
  font-size: 12px;
  /* padding-left: 20px;  */
  /* margin: 0; */
}

.instruction-list p {
  margin: 0;
}

.instruction-list>li {
  margin-bottom: 10px;
}

.instruction-list ul {
  list-style-type: none;
  /* Bulleted sub-list */
  padding-left: 20px;
  /* margin-left: 20px; */
}

.el-link {
  margin-left: 5px;
  /* Space before the "了解更多" link */
}
</style>
