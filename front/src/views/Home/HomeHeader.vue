<!-- 头部固定菜单 -->
<template>
  <div class="head">
    <div>
      <h1>纵狐指纹</h1>
    </div>
    <nav class="head-nav">
      <li>
        <el-icon>
          1
          <!-- <ChatLineRound /> -->
        </el-icon>
      </li>
      <li>
        <el-icon>
          1
          <!-- <ChatLineRound /> -->
        </el-icon>
      </li>
      <li>
        <el-icon>
          1
          <!-- <ChatLineRound /> -->
        </el-icon>
      </li>
      <li>
        <el-icon>
          1
          <!-- <ChatLineRound /> -->
        </el-icon>
      </li>
      <li>下拉框</li>
    </nav>
  </div>
</template>

<script setup></script>

<style scoped>
.head {
  /* 自动布局 */
  display: flex;
  flex-direction: row;
  height: 90px;
  /* align-items: center; */
  /* padding: 16px 48px; */
  gap: 62.78%;
  /* background-color: rgb(244, 249, 254); */
  background-color: rgb(249, 89, 255);
}

.head div h1 {
  font-size: 18px;
  font-weight: 400;
  width: 200px;
  color: #000000;
}
.head-nav {
  display: flex;
  flex-direction: row;
  padding: 0px;
  gap: 16px;
  z-index: 1;
}
.head-nav li {
  list-style: none;
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 20% 0% 20% 0%;
  gap: 8px;
  background: #ffffff;
}
</style>
