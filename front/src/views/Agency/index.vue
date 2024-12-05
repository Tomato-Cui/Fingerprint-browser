<template>
  <Layout>
    <div class="Env">
      <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
        <el-tab-pane label="代理列表" name="first"><Table /></el-tab-pane>
        <el-tab-pane label="代理配置" name="second"><Config /></el-tab-pane>
        <el-tab-pane label="更多资源" name="third"
          ><MoreResource
        /></el-tab-pane>
      </el-tabs>
    </div>
  </Layout>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import Layout from "@/layouts/index.vue";

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
import Table from "@/views/Agency/table.vue";
import { ElMessage, ElMessageBox, type TabsPaneContext } from "element-plus";

const activeName = ref("first");

const handleClick = (tab: TabsPaneContext, event: Event) => {
  console.log(tab, event);
};
</script>

<style scoped></style>
