<template>
  <!-- 左侧菜单栏容器 -->

  <div class="aside">
    <el-row class="tac">
      <el-col>
        <!-- 菜单主体内容 -->
        <div class="menu">
          <el-menu
            :default-active="activeMenu"
            router
            background-color="#FAFDFF"
            class="el-menu-vertical-demo"
            @open="handleOpen"
            @close="handleClose"
          >
            <!-- 新建浏览器菜单项 -->
            <el-menu-item index="/">
              <el-icon><document /></el-icon>
              <span>新建浏览器</span>
            </el-menu-item>
            <!-- 环境管理菜单项 -->
            <el-menu-item index="/environment">
              <el-icon><icon-menu /></el-icon>
              <span>环境管理</span>
            </el-menu-item>
            <!-- 分组管理菜单项 -->
            <el-menu-item index="/divide">
              <el-icon><document /></el-icon>
              <span>分组管理</span>
            </el-menu-item>
            <!-- 代理管理菜单项 -->
            <el-menu-item index="/agency">
              <el-icon><document /></el-icon>
              <span>代理管理</span>
            </el-menu-item>
            <!-- 插件管理菜单项 -->
            <el-menu-item index="/plugIn">
              <el-icon><document /></el-icon>
              <span>插件管理</span>
            </el-menu-item>
            <!-- 回收站菜单项 -->
            <el-menu-item index="/recycle">
              <el-icon><document /></el-icon>
              <span>回收站</span>
            </el-menu-item>
            <!-- 自动化子菜单 -->
            <el-sub-menu index="1-1">
              <template #title
                ><el-icon><document /></el-icon>自动化</template
              >
              <el-menu-item index="/api"
                ><el-icon><document /></el-icon>API</el-menu-item
              >
            </el-sub-menu>
            <!-- 团队管理子菜单 -->
            <el-sub-menu index="/team">
              <template #title
                ><el-icon><setting /></el-icon>团队管理</template
              >
            </el-sub-menu>
          </el-menu>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script lang="ts" setup>
// 导入Element Plus图标组件
import {
  Document,
  Menu as IconMenu,
  Location,
  Setting,
} from "@element-plus/icons-vue";
import { ref, watch } from "vue";
import { useRoute } from "vue-router";

// 菜单打开事件处理函数
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath);
};
// 菜单关闭事件处理函数
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath);
};
// 获取当前路由实例
const route = useRoute();
// 根据当前路由路径设置激活的菜单项
const activeMenu = ref(route.path);

// 监听路由变化，更新激活的菜单项
watch(route, () => {
  activeMenu.value = route.path;
});
</script>

<style>
/* 左侧菜单栏容器样式 */
.tac {
  width: 200px;
  height: 100%; /* 使菜单栏填满左侧区域 */
  border-radius: 12px 12px 12px 12px;
  background-color: #fafdff;
  overflow-y: auto;
}

/* 菜单基础样式 */
.el-menu {
  border-right: none; /* 隐藏右侧的线条 */
  box-shadow: none; /* 去除阴影 */
  border-radius: 8px 8px 8px 8px;
}

/* 菜单项样式 */
.el-menu-item {
  margin-bottom: 1.8%;
  border-radius: 8px 8px 8px 8px;
  border-bottom: none; /* 隐藏菜单项的下边框 */
}

/* 菜单内容区域样式 */
.menu {
  margin-top: 2.47%;
  margin-left: 7%;
  margin-right: 7%;
  margin-bottom: 5.86%;
  background-color: #fafdff;
}
</style>
