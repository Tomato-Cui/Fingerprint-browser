<template>
  <div class="proxy-config-page">
    <div class="proxy-config-content">
      <el-form :model="proxyConfig" label-width="100px" label-position="left">
        <el-form-item label="主机 : 端口">
          <el-select
            v-model="proxyConfig.protocol"
            placeholder="选择协议"
            style="width: 100px"
          >
            <el-option label="HTTP" value="http" />
            <el-option label="HTTPS" value="https" />
          </el-select>
          <el-input
            v-model="proxyConfig.host"
            placeholder="主机地址"
            style="width: 300px; margin-left: 10px"
          />
          <el-input
            v-model="proxyConfig.port"
            placeholder="端口号"
            style="width: 100px; margin-left: 10px"
          />
          <el-button
            type="primary"
            @click="checkProxy"
            style="margin-left: 10px"
            >检查代理</el-button
          >
        </el-form-item>

        <el-form-item label="代理账号">
          <el-input
            v-model="proxyConfig.username"
            placeholder="请输入账号"
            style="width: 400px"
          />
        </el-form-item>

        <el-form-item label="代理密码">
          <el-input
            v-model="proxyConfig.password"
            placeholder="请输入密码"
            style="width: 400px"
          />
        </el-form-item>

        <el-divider />

        <el-form-item label="代理密钥">
          <el-input
            v-model="proxyConfig.secret"
            placeholder="请输入密钥"
            style="width: 400px"
          />
          <el-checkbox v-model="syncProxy" style="margin-left: 10px"
            >同步代理</el-checkbox
          >
        </el-form-item>

        <el-button type="primary" @click="saveProxyConfig">保存设置</el-button>
      </el-form>
    </div>

    <!-- More Resources Section -->
    <div class="resources-section">
      <el-row :gutter="20">
        <el-col :span="6" v-for="(resource, index) in resources" :key="index">
          <el-card class="resource-card" shadow="hover">
            <img :src="resource.image" class="resource-image" />
            <div class="resource-content">
              <h3>{{ resource.title }}</h3>
              <el-button type="text" @click="redirectTo(resource.link)"
                >查看详情</el-button
              >
            </div>
          </el-card>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { ElMessage } from "element-plus";
import Config from "@/views/Agency/config.vue";

// Active tab
const activeTab = ref("proxyConfig");

// Proxy Configuration Data
const proxyConfig = ref({
  protocol: "http",
  host: "",
  port: "",
  username: "",
  password: "",
  secret: "",
});

// Synchronize Proxy Option
const syncProxy = ref(false);

// Resource Data
const resources = ref([
  { title: "IPFoxy", image: "path/to/image1.png", link: "https://ipfoxy.com" },
  {
    title: "Bright Data",
    image: "path/to/image2.png",
    link: "https://brightdata.com",
  },
  { title: "Oxylabs", image: "path/to/image3.png", link: "https://oxylabs.io" },
]);

// Methods
const checkProxy = () => {
  ElMessage.success("检查代理成功！");
};

const saveProxyConfig = () => {
  ElMessage.success("代理配置保存成功！");
};

const redirectTo = (url: string) => {
  window.open(url, "_blank");
};
</script>

<style scoped>
.proxy-config-page {
  padding: 20px;
}

.proxy-config-content {
  padding: 20px;
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.resource-section {
  padding: 20px;
}

.resource-card {
  text-align: center;
}

.resource-image {
  width: 100%;
  height: 150px;
  object-fit: cover;
}

.resource-content {
  margin-top: 10px;
}
</style>
