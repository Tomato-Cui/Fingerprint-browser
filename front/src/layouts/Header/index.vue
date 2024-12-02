<!-- 头部固定菜单 -->
<template>
  <div class="header">
    <div class="header-left">
      <h1>
        <img src="@\icons\svg\index.svg" alt="Document Icon" />
        <span>纵狐指纹</span>
      </h1>
    </div>
    <div class="header-right">
      <li>
        <el-icon
          ><img src="@\icons\svg\yuyan.svg" alt="Document Icon"
        /></el-icon>
      </li>
      <li>
        <el-icon
          ><img src="@\icons\svg\xiaoxi.svg" alt="Document Icon"
        /></el-icon>
      </li>
      <li>
        <el-icon
          ><img src="@\icons\svg\shuaxin.svg" alt="Document Icon"
        /></el-icon>
      </li>
      <div class="xiala">
        <DropBox />
      </div>
    </div>
  </div>
</template>

<script setup >
import { ref } from "vue";
import DropBox from "./components/DropBox.vue";
</script>

<style scoped>
.header {
  /* 自动布局 */
  display: flex;
  height: 100%;
  flex-direction: row;
  justify-content: space-between;
  box-sizing: border-box;
  /* background-color: rgb(255, 182, 121); */
  background-color: #f5fafe;
}
.header-left {
  margin-left: 3.33%;
  display: flex; /* 启用 flexbox */
  align-items: center; /* 垂直居中 */
  justify-content: center; /* 水平居中 */
  height: 100%; /* 容器高度与父级相同 */
}
.header-left h1 {
  font-size: 18px;
  font-weight: 400;
  color: #000000;
  margin: 0; /* 移除默认的 margin 以防止偏移 */
  display: flex; /* 对 h1 也使用 flexbox */
  align-items: center; /* 保证 h1 内部的文本和图标垂直居中 */
}
.header-left h1 span {
  margin-left: 8px;
}
.header-right {
  /* display: flex;
flex-direction: row;
padding: 0px;
z-index: 1; */
  margin-right: 3.33%;
  display: flex;
  align-items: center;
  flex-direction: row;
}
.header-right li {
  width: 48px;
  height: 48px;
  border-radius: 8px 8px 8px 8px;
  margin-left: 16px;
  list-style: none;
  background: #ffffff;
  display: grid;
  place-items: center;
}
.header-right li img {
  align-items: center;
}
.xiala {
  margin-left: 15px;
  width: 130px;
  height: 48px;
  /* background-color: brown; */
}
.el-sub-menu {
  padding: 0px;
}
</style>