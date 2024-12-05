<template>
  <div class="login-container">
    <!-- 右上角的登录按钮 -->
    <button class="login-button" @click="toggleDropdown">登录</button>

    <!-- 弹出框，显示在按钮的下方 -->
    <div v-if="isDropdownVisible" class="dropdown">
      <ul>
        <li @click="selectOption('登录')">登录</li>
        <li @click="selectOption('注册')">注册</li>
        <li @click="selectOption('忘记密码')">忘记密码</li>
      </ul>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  name: 'LoginDropdown',
  setup() {
    const isDropdownVisible = ref(false); // 控制弹出框的显示和隐藏

    const toggleDropdown = () => {
      isDropdownVisible.value = !isDropdownVisible.value;
    };

    const selectOption = (option) => {
      alert(`您选择了: ${option}`);
      isDropdownVisible.value = false; // 选择后关闭弹出框
    };

    return {
      isDropdownVisible,
      toggleDropdown,
      selectOption,
    };
  },
};
</script>

<style scoped>
/* 登录按钮样式 */
.login-button {
  position: absolute;
  top: 10px;
  right: 10px;
  padding: 8px 16px;
  background-color: #007bff;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
}

/* 弹出框样式 */
.dropdown {
  position: absolute;
  top: 50px; /* 将弹出框显示在按钮下方 */
  right: 10px;
  width: 150px;
  background-color: #f9f9f9;
  border: 1px solid #ddd;
  padding: 8px 0;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  z-index: 10;
}

.dropdown ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.dropdown li {
  padding: 8px 16px;
  cursor: pointer;
  color: #007bff;
}

.dropdown li:hover {
  background-color: #f0f0f0;
}
</style>
