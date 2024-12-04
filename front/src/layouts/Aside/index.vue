<template>
  <div class="aside">
    <el-row class="tac">
      <el-col>
        <div class="menu">
          <el-menu :default-active="activeIndex" class="el-menu-vertical-demo" theme="dark" @select="handleSelect"
            @open="handleOpen" @close="handleClose" router>
            <el-menu-item index="/home">
              <el-icon>
                <img src="@/icons/svg/xinjian.svg" alt="Document Icon" />
                <!-- <Icon
                  iconName="xinjian"
                  alt="新建文档"
                  iconClass="custom-icon"
                /> -->
                <!-- <svg-icon icon="xinjian" class="svg-container"></svg-icon> -->
              </el-icon>
              <span>新建浏览器</span>
            </el-menu-item>
            <el-menu-item index="/environment">
              <el-icon>
                <img src="@/icons/svg/huanjing.svg" alt="Document Icon" />
              </el-icon>
              <span>环境管理</span>
            </el-menu-item>
            <el-menu-item index="/divide">
              <el-icon>
                <img src="@/icons/svg/fenzu.svg" alt="Document Icon" />
              </el-icon>
              <span>分组管理</span>
            </el-menu-item>
            <el-menu-item index="/agency">
              <el-icon>
                <img src="@/icons/svg/daili.svg" alt="Document Icon" />
              </el-icon>
              <span>代理管理</span>
            </el-menu-item>
            <el-menu-item index="/plugIn">
              <el-icon>
                <img src="@/icons/svg/chajian.svg" alt="Document Icon" />
              </el-icon>
              <span>插件管理</span>
            </el-menu-item>
            <el-menu-item index="/recycle">
              <el-icon>
                <img src="@/icons/svg/huishou.svg" alt="Document Icon" />
              </el-icon>
              <span>回收站</span>
            </el-menu-item>
            <el-sub-menu index="1-1">
              <template #title>
                <el-icon>
                  <img src="@/icons/svg/zidong.svg" alt="Document Icon" />
                </el-icon>
                <span>自动化</span>
              </template>
              <el-menu-item index="/api">
                <el-icon>
                  <img src="@/icons/svg/api.svg" alt="Document Icon" />
                </el-icon>
                <span>API</span>
              </el-menu-item>
            </el-sub-menu>
            <el-sub-menu index="1-2">
              <template #title>
                <el-icon>
                  <img src="@/icons/svg/tuandui.svg" alt="Document Icon" />
                </el-icon>
                <span>团队管理</span>
              </template>
              <el-menu-item index="/team/group">
                <!-- <i class="el-icon-s-management"></i> -->
                <span><el-icon>
                    <Grid />
                  </el-icon>分组管理</span>
              </el-menu-item>
              <el-menu-item index="/team/member">
                <!-- <i class="el-icon-user"></i> -->
                <span><el-icon>
                    <UserFilled />
                  </el-icon>所有成员</span>
              </el-menu-item>
              <el-menu-item index="/team/forbid">
                <!-- <i class="el-icon-message"></i> -->
                <span><el-icon>
                    <CircleCloseFilled />
                  </el-icon>已禁用</span>
              </el-menu-item>
              <el-menu-item index="/team/apply">
                <!-- <i class="el-icon-document"></i> -->
                <span><el-icon>
                    <Avatar />
                  </el-icon>成员申请</span>
              </el-menu-item>
              <!-- <el-menu-item index="/team">
                <el-icon>
                  <img src="@/icons/svg/tuandui.svg" alt="Document Icon" />
                </el-icon>
                <span>团队管理</span>
              </el-menu-item> -->
            </el-sub-menu>
          </el-menu>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script lang="ts" setup>
import { useRouter, useRoute } from "vue-router";
import { ref, watch } from "vue";
import Icon from "@/components/icon.vue";
import {
  Grid,
  UserFilled,
  CircleCloseFilled,
  Avatar,
} from "@element-plus/icons-vue";

const router = useRouter();
const route = useRoute();
const activeIndex = ref(route.path);

watch(route, (newRoute) => {
  activeIndex.value = newRoute.path;
});

function handleSelect(index: string) {
  router.push(index);
}

const handleOpen = (key: string, keyPath: string[]) => {
  console.log("Menu opened:", key, keyPath);
};

const handleClose = (key: string, keyPath: string[]) => {
  console.log("Menu closed:", key, keyPath);
};
</script>

<style>
.tac {
  width: 200px;
  height: 100%;
  border-radius: 12px;
  background-color: #fafdff;
  overflow-y: auto;
}

:deep(.el-menu) {
  border-right: none;
  box-shadow: none;
  border-radius: 8px;

  .el-menu-item {
    margin-bottom: 1.8%;
    border-radius: 8px;
    border-bottom: none;
    padding-left: 0;
  }

}

.menu {
  margin: 2.47% 7% 5.86% 7%;
  background-color: #fafdff;
}
</style>
