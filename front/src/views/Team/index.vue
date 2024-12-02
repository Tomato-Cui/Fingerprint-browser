<!-- 团队管理 -->
<template>
  <Layout>
    <el-container style="height: 100vh">
      <el-aside width="200px" style="display: flex; flex-direction: column">
        <div class="sticky-button">
          <el-button type="primary" icon="el-icon-plus" @click="addGroup"
            >添加分组</el-button
          >
        </div>
        <el-menu
          class="el-menu-vertical-demo"
          @select="handleGroupSelect"
          style="flex-grow: 1; overflow: auto"
          unique-opened
        >
          <el-menu-item
            v-for="group in groups"
            :key="group.id"
            :index="group.id.toString()"
            @click="navigateToGroup(group.id)"
            class="menu-item-group"
          >
            <template #title>
              <div class="group-title">
                {{ group.name }}
                <el-button
                  type="text"
                  icon="el-icon-delete"
                  @click.stop="removeGroup(group.id)"
                  class="delete-button"
                ></el-button>
              </div>
            </template>
          </el-menu-item>
        </el-menu>
      </el-aside>

      <el-container>
        <el-main>
          <el-table :data="selectedGroupData" style="width: 100%">
            <el-table-column prop="name" label="姓名"></el-table-column>
            <el-table-column prop="email" label="邮箱"></el-table-column>
            <el-table-column prop="phone" label="手机号"></el-table-column>
            <el-table-column prop="status" label="状态"></el-table-column>
            <el-table-column label="操作">
              <template #default="scope">
                <el-button size="mini">编辑</el-button>
                <el-button size="mini" type="danger">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-main>
      </el-container>
    </el-container>
  </Layout>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { ElMessage } from "element-plus";
import Layout from "@/layouts/index.vue";

const groups = ref([
  // 示例数据
  // { id: 1, name: '分组1', data: [{ name: '张三', email: 'zhangsan@example.com', phone: '12345678901', status: '在线' }] },
  // { id: 2, name: '分组2', data: [{ name: '李四', email: 'lisi@example.com', phone: '09876543211', status: '离线' }] },
  // { id: 3, name: '分组3', data: [{ name: '王五', email: 'wangwu@example.com', phone: '11223344556', status: '忙碌' }] }
]);
const selectedGroupData = ref([]);

const router = useRouter();

function handleGroupSelect(index) {
  const group = groups.value.find((g) => g.id.toString() === index);
  selectedGroupData.value = group ? group.data : [];
}

function navigateToGroup(groupId) {
  console.log(`Navigating to group with ID: ${groupId}`); // 调试输出
  router.push({ name: "team", params: { id: groupId } });
}

function removeGroup(groupId) {
  groups.value = groups.value.filter((group) => group.id !== groupId);
  selectedGroupData.value = [];
  ElMessage({
    message: "分组已删除",
    type: "error",
  });
}

function addGroup() {
  const newGroupId = groups.value.length + 1;
  groups.value.push({
    id: newGroupId,
    name: `分组${newGroupId}`,
    data: [
      {
        name: `新成员`,
        email: `${newGroupId}newmember@example.com`,
        phone: "1234567890",
        status: "新加入",
      },
    ],
  });
  ElMessage({
    message: "分组已添加",
    type: "success",
  });
}
</script>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.search-input {
  width: 300px;
}

.el-header {
  display: flex;
  align-items: center;
  padding: 20px;
}
.sticky-button {
  position: sticky;
  top: 0;
  z-index: 1000;
  background: #fff;
  padding: 10px 0;
  width: 100%;
}
.group-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.delete-button {
  margin-left: auto; /* Push the button to the right */
  background-color: rgb(255, 0, 0);
}
</style>
