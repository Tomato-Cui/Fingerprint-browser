<template>
  <span class="myButton">
    <button @click="dianji" >{{tianjia}}</button>
    <MyDialog v-model="isActive.value" />
  </span>
</template>
<script setup>
import { ref } from 'vue';
import { useStateStore } from '@/stores/modules/global';
import MyDialog from '@/components/MyDialog/index.vue';


const tianjia=ref('添加')
const isActive = useStateStore();
const dianji = () => {
  isActive.value = !isActive.value;
  if (isActive.value) {
    tianjia.value = '添加'
    console.log(isActive.value);
  } else {
    tianjia.value = '已添加'
    
  }
}



</script>

<style>
.myButton button{
  width: 65px;
  height: 40px;
  color:#366ef4;
  font-size: 14px;
  border-radius: 8px;
  border: 1px solid #366ef4;
  background-color: rgb(255, 255, 255);
  cursor: pointer;  /* 鼠标悬浮时显示手形 */
  transition: all 0.3s ease; /* 为了平滑过渡效果 */
}
.myButton button:hover{
  background-color: #366ef4;
  color: rgb(255, 255, 255);
}
.myButton button:active {
  background-color: #2a5eab;  /* 点击时按钮颜色变暗 */
  color: rgb(255, 255, 255);
}
</style>