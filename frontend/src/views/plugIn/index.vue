<template>
  <Layout>
    <div class="app-container">
      <div class="header">
        <el-input
          v-model="searchQuery"
          placeholder="搜索应用"
          prefix-icon="el-icon-search"
        ></el-input>
        <el-button
          type="primary"
          icon="el-icon-plus"
          @click="dialogVisible = true"
          >添加应用</el-button
        >
      </div>
      <el-dialog v-model="dialogVisible" title="上传应用" width="600px">
        <div>
          <el-tabs v-model="activeTab" class="upload-type-tabs">
            <el-tab-pane label="安装包" name="installPackage"></el-tab-pane>
            <el-tab-pane
              label="Chrome应用商店"
              name="chromeStore"
            ></el-tab-pane>
          </el-tabs>

          <!-- 安装包上传区域 -->
          <div v-if="activeTab === 'installPackage'" class="upload-section">
            <el-form :model="form" label-width="100px">
              <!-- 安装包上传 -->
              <el-form-item label="安装包">
                <el-upload
                  class="upload-demo"
                  action="#"
                  list-type="text"
                  :limit="1"
                  :on-exceed="handleExceed"
                >
                  <el-button>上传文件</el-button>
                </el-upload>
                <div class="upload-tip">支持zip格式，限制60M</div>
              </el-form-item>
            </el-form>
          </div>

          <!-- Chrome 应用商店上传区域 -->
          <div
            v-else-if="activeTab === 'chromeStore'"
            class="chrome-store-section"
          >
            <el-form :model="form" label-width="100px">
              <!-- URL 输入框 -->
              <el-form-item label="应用URL">
                <el-input
                  v-model="form.appURL"
                  placeholder="请输入Chrome商店的插件URL"
                ></el-input>
              </el-form-item>
            </el-form>

            <!-- 提示信息 -->
            <div class="tip-section">
              <p>温馨提示：</p>
              <ul>
                <li>1. 此上传类型需使用非中国大陆网络。</li>
                <li>2. 应用由第三方提供，最终解释权归第三方所有。</li>
                <li>
                  3. 前往
                  <a
                    href="https://chrome.google.com/webstore"
                    target="_blank"
                    class="link"
                  >
                    Chrome 应用商店
                  </a>
                  ，复制需要上传的应用链接。
                </li>
              </ul>
              <!-- 示例图片 -->
              <div>
                <!-- <img
                src="https://via.placeholder.com/600x120?text=Chrome+Web+Store+Example"
                alt="Chrome Store Example"
                style="width: 100%; margin-top: 10px"
              /> -->
              </div>
            </div>
          </div>
          <div v-if="activeTab === 'installPackage'" class="upload-section">
            <el-form :model="form" label-width="80px">
              <el-form-item label="安装包">
                <el-upload
                  class="upload-demo"
                  action="#"
                  list-type="text"
                  :on-preview="handlePreview"
                  :on-remove="handleRemove"
                  :limit="1"
                  :on-exceed="handleExceed"
                >
                  <el-button>上传文件</el-button>
                </el-upload>
                <div class="upload-tip">支持zip格式，限制60M</div>
              </el-form-item>
              <el-form-item label="图标">
                <el-upload
                  class="upload-demo"
                  drag
                  action="#"
                  list-type="picture-card"
                  :limit="1"
                  :on-preview="handlePreview"
                  :on-remove="handleRemove"
                  :on-exceed="handleExceed"
                >
                  <i class="el-icon-upload"></i>
                  <div class="el-upload__text">点击上传或拖拽到此处</div>
                </el-upload>
                <div class="upload-tip">
                  至少60*60像素，支持jpg/jpeg/png格式，限制1M
                </div>
              </el-form-item>
              <el-form-item label="应用名称">
                <el-input
                  v-model="form.appName"
                  maxlength="20"
                  show-word-limit
                  placeholder="必填，请填写应用名称"
                />
              </el-form-item>
              <el-form-item label="简介">
                <el-input
                  type="textarea"
                  v-model="form.description"
                  maxlength="200"
                  show-word-limit
                  placeholder="选填，请填写应用的简介说明"
                />
              </el-form-item>
            </el-form>
          </div>
        </div>
        <span slot="footer" class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleConfirm">确定</el-button>
        </span>
      </el-dialog>
      <div class="content">
        <div class="app-card" v-for="app in filteredApps" :key="app.id">
          <el-card>
            <div slot="header" class="clearfix">
              <span
                ><img
                  src="https://img.freepik.com/free-vector/ap-monogram-logo-template_23-2150227278.jpg?ga=GA1.1.332564243.1730535881"
                  alt=""
                />{{ app.name }}</span
              >
              <MyButton />
            </div>
            <div class="app-body">
              <p>{{ app.description }}</p>
            </div>
          </el-card>
        </div>
      </div>
    </div>
  </Layout>
</template>

<script setup>
import { ref, computed } from "vue";
import MyButton from "@/components/MyButton/index.vue";
import Layout from "@/layouts/index.vue";

const apps = ref([
  {
    id: 1,
    name: "团队应用-Imageye",
    description: "你可以从网上搜索、浏览和下载图片",
  },
  {
    id: 2,
    name: "推荐应用-Imageye",
    description: "浏览你喜欢的图片，并加以管理",
  },
]);

const searchQuery = ref("");
const dialogVisible = ref(false);
const activeTab = ref("installPackage");

const form = ref({
  appName: "",
  description: "",
  installPackage: null,
  icon: null,
});

const filteredApps = computed(() => {
  return apps.value.filter((app) => app.name.includes(searchQuery.value));
});

const handlePreview = (file) => {
  console.log("预览文件:", file);
};

const handleRemove = (file, fileList) => {
  console.log("删除文件:", file, fileList);
};

const handleExceed = () => {
  console.error("文件数量超出限制");
};

const handleConfirm = () => {
  console.log("提交表单:", form.value);
  dialogVisible.value = false;
};
</script>

<style scoped>
img {
  height: 36px;
  width: 36px;
}

.app-container {
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 20px;
  padding: 20px;
}

.header {
  display: grid;
  grid-template-columns: 250px 100px;
  gap: 15px;
  align-items: center;
}

.content {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.app-card {
  display: flex;
}

.upload-section {
  padding: 10px 0;
}

.upload-demo {
  width: 100%;
}

.upload-tip {
  font-size: 12px;
  color: #909399;
  margin-top: 5px;
}

.dialog-footer {
  text-align: right;
}

.upload-type-tabs {
  margin-bottom: 20px;
}
</style>
