<template>
  <!-- <Home /> -->
  <router-view />
</template>

<script setup lang="ts">
import { platform_str } from '@/command/os'

console.log(platform_str);

</script>

<style>
body {
  margin: 0px;
}
</style>
