<template>
  <div class="message-box" v-show="visible">
    <div class="message-content">
      <!-- <span>{{ message }}</span> -->
    </div>
    <img
      @click="visible = false"
      v-if="showClose"
      src="./images/close.svg"
      alt=""
    />
  </div>
</template>

<script setup lang="ts">
import type { MessageProps } from "./types";
import { ref, withDefaults, onMounted } from "vue";

const visible = ref(false);

const props = withDefaults(defineProps<MessageProps>(), {
  duration: 3000,
  type: "success",
});

function startTimer() {
  if (props.duration === 0) return;
  setTimeout(() => {
    visible.value = false;
  }, props.duration);
}

onMounted(() => {
  visible.value = true;
  startTimer();
});
</script>

<style scoped>
.message-box {
  width: max-content;
  position: fixed;
  left: 50%;
  top: 20px;
  transform: translateX(-50%);
  border: 1px solid skyblue;
}
img {
  width: 16px;
  height: 16px;
}
</style>
