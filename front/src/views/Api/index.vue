<template>
  <Layout>
    <h1>API</h1>
    <div class="a">
      <Button />
      <Downup />
      <MyDialog />
      <Mymodal />
      <!-- <Message message="hello world" :duration="0" :showClose="false"/> -->
    </div>
  </Layout>
</template>

<script setup lang="ts">
import Button from "@/components/MyButton/index.vue";
import Downup from "@/components/Downup/index.vue";
import MyDialog from "@/components/MyDialog/index.vue";
import Mymodal from "@/components/Mymodal/index.vue";
import Layout from "@/layouts/index.vue";
// import Message from '@/components/Message/index.vue'
import { onMounted } from "vue";
import { createMessage } from "@/components/Message/method";
import { ref } from "vue";
onMounted(() => {
  createMessage({
    message: "hello world",
    duration: 0,
    showClose: false,
  });
});
</script>

<style>
.a {
  height: 100vh;
}
</style>
