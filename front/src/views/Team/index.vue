<template>
  <Layout>
    <div class="group-management">
      <!-- <Group /> -->
      <el-container>
        <!-- Sidebar Menu -->
        <!-- <el-aside width="200px" class="sidebar">
        <el-menu :default-active="defaultActive" class="el-menu-vertical" @select="handleSelect">
          <el-menu-item index="/team/group">
            <i class="el-icon-s-management"></i>
            <span><el-icon>
                <Grid />
              </el-icon>分组管理</span>
          </el-menu-item>
          <el-menu-item index="/team/member">
            <i class="el-icon-user"></i>
            <span><el-icon>
                <UserFilled />
              </el-icon>所有成员</span>
          </el-menu-item>
          <el-menu-item index="/team/forbid">
            <i class="el-icon-message"></i>
            <span><el-icon>
                <CircleCloseFilled />
              </el-icon>已禁用</span>
          </el-menu-item>
          <el-menu-item index="/team/apply">
            <i class="el-icon-document"></i>
            <span><el-icon>
                <Avatar />
              </el-icon>成员申请</span>
          </el-menu-item>
        </el-menu>
      </el-aside> -->

        <!-- Main Content -->
        <el-container>
          <!-- {{ router.currentRoute.value.path }}
        {{ res }} -->
          <Group v-if="router.currentRoute.value.path === '/team/group'" />
          <Member v-if="router.currentRoute.value.path === '/team/member'" />
          <Forbid v-if="router.currentRoute.value.path === '/team/forbid'" />
          <Apply v-if="router.currentRoute.value.path === '/team/apply'" />
        </el-container>
      </el-container>
    </div>
  </Layout>
</template>

<script setup lang="ts">
import Group from './group/index.vue';
import Member from './member/index.vue';
import Forbid from './forbid/index.vue';
import Apply from './apply/index.vue';
import { onMounted } from 'vue';
import { useRouter } from "vue-router";
import { ref, watch } from "vue";
import Layout from "@/layouts/index.vue"
import {
  Grid,
  UserFilled,
  CircleCloseFilled,
  Avatar,
} from "@element-plus/icons-vue";
const router = useRouter();
import { ElMessage } from 'element-plus';
console.log("router", router.currentRoute.value.path);

onMounted(() => {
  //   if (router.currentRoute.value.path === '/team') {
  //     router.push('/team/group')
  //   }
  //   defaultActive.value = router.currentRoute.value.path;
})

const defaultActive = ref(router.currentRoute.value.path);

const res = ref()
const handleSelect = (key: string, keyPath: string[]) => {
  // console.log(key, keyPath);
  defaultActive.value = router.currentRoute.value.path;
  // res.value = keyPath[0]
  router.push(keyPath[0])
  router.currentRoute.value.path = keyPath[0];
  // console.log("de------------:", typeof defaultActive.value);

};


// 邀请成员
// 控制弹出框是否显示
const invDialogVisible = ref(false);
// 打开弹出框
const openDialog = () => {
  invDialogVisible.value = true;
};
// 弹出框确认按钮的回调
const confirmDialog = () => {
  console.log('确认按钮被点击');
  invDialogVisible.value = false;
};

watch(() => router.currentRoute.value.path, (newPath) => {
  // ElMessage.success("newPath:" + newPath)
  // if (newPath === '/team') {
  //   router.push('/team/group')
  // }
  defaultActive.value = newPath;
});
</script>

<style scoped lang="less">
.group-management {
  height: calc(100vh - 90px);
  background-color: #f5f5f5;
  display: flex;
  flex-direction: row;

  .sidebar {
    // height: 100vh;
    border-right: 1px solid #dcdfe6;
    background-color: #fff;
  }
}
</style>
