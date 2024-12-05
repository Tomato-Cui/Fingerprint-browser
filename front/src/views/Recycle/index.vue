<template>
  <Layout>
    <div class="page-container">
      <!-- 工具栏 -->
      <div class="toolbar">
        <!-- 日期范围选择器 -->
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="至"
          start-placeholder="开始"
          end-placeholder="结束"
          format="YYYY-MM-DD"
          value-format="YYYY-MM-DD"
          :picker-options="pickerOptions"
          @change="handleDateChange"
          size="small"
          class="short-width"
        />

        <!-- 分组下拉选择框 -->
        <el-select
          v-model="selectedGroup"
          placeholder="全部分组"
          size="small"
          class="short-width"
        >
          <el-option
            v-for="group in groupOptions"
            :key="group.value"
            :label="group.label"
            :value="group.value"
          />
        </el-select>

        <!-- 操作按钮 -->
        <el-button type="primary" icon="el-icon-search" size="small"
          >筛选</el-button
        >
        <el-button type="default" icon="el-icon-refresh" size="small"
          >恢复</el-button
        >
        <el-button type="danger" icon="el-icon-delete" size="small"
          >彻底删除</el-button
        >
      </div>

      <!-- 分组管理表格 -->
      <el-table :data="paginatedData" style="width: 100%">
        <el-table-column type="selection" width="55"></el-table-column>
        <el-table-column prop="id" label="环境编号"></el-table-column>
        <el-table-column
          prop="name"
          label="分组名称"
          width="180"
        ></el-table-column>
        <el-table-column prop="count" label="分组环境总数"></el-table-column>
        <el-table-column prop="remark" label="备注"></el-table-column>
        <el-table-column prop="time" label="创建时间"></el-table-column>
        <el-table-column prop="user" label="创建用户"></el-table-column>
        <el-table-column prop="mail" label="邮箱"></el-table-column>
        <el-table-column label="操作" width="300">
          <template #default="scope">
            <el-button size="mini">启动</el-button>
            <el-button size="mini">暂停</el-button>
            <el-dropdown>
              <el-button size="mini">
                更多<i class="el-icon-arrow-down el-icon--right"></i>
              </el-button>
              <el-dropdown-menu #dropdown>
                <el-dropdown-item>编辑</el-dropdown-item>
                <el-dropdown-item>删除</el-dropdown-item>
                <el-dropdown-item>复制</el-dropdown-item>
              </el-dropdown-menu>
            </el-dropdown>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <el-pagination
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
        :current-page="currentPage"
        :page-sizes="[10, 20, 50, 100]"
        :page-size="pageSize"
        layout="total, sizes, prev, pager, next, jumper"
        :total="tableData.length"
      >
        <template #total="{ total }"> 总计: {{ total }} 条 </template>
      </el-pagination>
    </div>
  </Layout>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import Layout from "@/layouts/index.vue";

// 日期范围选择
const dateRange = ref<Date[]>([]);
const handleDateChange = (value: Date[]) => {
  console.log("日期范围变更:", value);
};

// 快捷选项
const pickerOptions = {
  shortcuts: [
    {
      text: "今天",
      onClick(picker: any) {
        const today = new Date();
        picker.$emit("pick", [today, today]);
      },
    },
    {
      text: "昨天",
      onClick(picker: any) {
        const yesterday = new Date();
        yesterday.setDate(yesterday.getDate() - 1);
        picker.$emit("pick", [yesterday, yesterday]);
      },
    },
    {
      text: "近 7 天",
      onClick(picker: any) {
        const end = new Date();
        const start = new Date();
        start.setDate(start.getDate() - 6);
        picker.$emit("pick", [start, end]);
      },
    },
    {
      text: "近 30 天",
      onClick(picker: any) {
        const end = new Date();
        const start = new Date();
        start.setDate(start.getDate() - 29);
        picker.$emit("pick", [start, end]);
      },
    },
  ],
};

// 分组选择
const selectedGroup = ref("");
const groupOptions = [
  { value: "all", label: "全部分组" },
  { value: "group1", label: "分组 1" },
  { value: "group2", label: "分组 2" },
];

const recordsCount = 100;
const tableData = ref(
  Array.from({ length: recordsCount }, (v, i) => ({
    id: i + 1, // Assign a unique ID starting from 1
    name: "张三",
    count: 99,
    remark: "-",
    time: "2024.8.10 14:47",
    user: "user",
    mail: "12345678@163.com",
  }))
);

const currentPage = ref(1);
const pageSize = ref(10);

const paginatedData = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return tableData.value.slice(start, end);
});

const handleSizeChange = (newSize: number) => {
  pageSize.value = newSize;
};

const handleCurrentChange = (newPage: number) => {
  currentPage.value = newPage;
};
</script>

<style scoped>
.page-container {
  padding: 20px;
  background-color: #f5f7fa;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px;
  background: #fff;
  border: 1px solid #ebebeb;
  border-radius: 5px;
}

.toolbar .el-date-picker,
.toolbar .el-select {
  width: 200px;
}

.short-width {
  width: 150px !important;
}
</style>
