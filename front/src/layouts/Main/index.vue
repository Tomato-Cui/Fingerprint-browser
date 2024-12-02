<template>
  <div class="aside">
    <!-- <router-view /> -->
    <!-- <div class="tem">
            <main class="main-content">
                <router-view />
            </main>
        </div> -->
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.aside {
  flex: 1; /* 占据剩余空间 */
  background-color: rgb(244, 249, 254);
  /* background-color: rgb(207, 231, 255); */
  /* padding: 20px; */
  overflow-y: auto; /* 如果内容过多，支持滚动 */
  position: relative;
  /* position: absolute; */
}
.tem {
  position: absolute;
  /* left: 262px; */
  /* top: 80px; */
  width: 1130px;
  height: 649px;
  border-radius: 16px;
  opacity: 1;
  margin-left: 14px;
  background: #ffffff;
}
</style>
