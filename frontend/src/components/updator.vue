<script setup lang="ts">
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { onMounted } from "vue";

onMounted(async () => {
  const update = await check();
  if (!update) return;

  console.log(
    `Found update ${update.version} from ${update.date} with notes ${update.body}`
  );

  let downloaded = 0;
  let contentLength = 0;

  // 下载更新
  await update.download((event) => {
    switch (event.event) {
      case "Started":
        contentLength = event.data.contentLength as number;
        console.log(`Started downloading ${event.data.contentLength} bytes`);
        break;
      case "Progress":
        downloaded += event.data.chunkLength;
        console.log(`Downloaded ${downloaded} from ${contentLength}`);
        break;
      case "Finished":
        console.log("Downloaded file size:", downloaded);
        break;
    }
  });

  console.log("Download completed successfully");

  // 安装更新
  console.log("Starting installation...");
  await update.install();
  console.log("Installation completed successfully");

  // 重启应用程序
  console.log("Attempting to relaunch...");
  await relaunch();
  console.log("Relaunch completed successfully");
});
</script>

<template></template>
