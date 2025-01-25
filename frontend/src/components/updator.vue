<script setup lang="ts">
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { onMounted, ref, watch } from "vue";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { BadgePlusIcon } from "lucide-vue-next";
import { IconLogo } from "@/assets/icons/sidebar";
import { useUpdateStore } from "@/stores/update";

let versionInfo = ref<any>(null);

const updateStore = useUpdateStore();

onMounted(async () => {
  const update = await check();
  if (!update || !update.available) return;

  updateStore.updateDialogOpen();

  let downloaded = 0;
  let contentLength = 0;

  try {
    await update.download((event) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength as number;
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          break;
        case "Finished":
          break;
      }
    });

    versionInfo.value = update;
    contentLength;
  } catch {}
});

watch(versionInfo, () => {
  if (!versionInfo.value) return;
  updateStore.setVersion(versionInfo.value.version, versionInfo.value.body);
});

const installHanle = async () => {
  const update = await check();
  if (!update || !update.available) return;

  versionInfo.value = update;
  updateStore.updateDialogOpen();

  let downloaded = 0;
  let contentLength = 0;

  try {
    await update.download((event) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength as number;
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          break;
        case "Finished":
          break;
      }
    });

    versionInfo.value = update;
    contentLength;
  } catch {}

  await update.install();

  await relaunch();
};

const before = () => {
  updateStore.updateDialogClose();
  setTimeout(() => {
    updateStore.updateDialogOpen();
  }, 1000 * 60 * 30);
};
</script>

<template>
  <Dialog :open="updateStore.updateDialog">
    <DialogContent
      class="p-4 shadow-[0_20px_50px_rgba(8,_112,_184,_0.1)] rounded-sm max-w-[388px]"
    >
      <div class="w-full pt-4 flex justify-center items-center">
        <IconLogo class="w-12 h-12" />
      </div>
      <div
        class="bg-purple-50 p-2 min-h-20 max-h-48 rounded-md text-sm flex items-start gap-x-2 overflow-y-auto"
      >
        <div class="w-6">
          <BadgePlusIcon class="text-purple-800" />
        </div>
        <p class="">
          {{
            versionInfo
              ? versionInfo.body
              : "软件存在更新，请更新应用程序，获取全新体验。"
          }}
        </p>
      </div>
      <div class="flex flex-col space-y-4">
        <button
          @click="installHanle"
          class="w-full text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:outline-[#5050FA] bg-[#5050FA] text-white active:scale-[.98]"
        >
          立即重启
        </button>
        <div class="w-full border"></div>
        <button
          @click="before"
          class="text-sm border rounded-md px-2 py-2 flex justify-center gap-x-2 font-[500] outline-offset-0 hover:outline-offset-[.5px] transition-all ease-in-out duration-150 hover:bg-gray-100 bg-white text-black active:scale-[.98]"
        >
          稍后提醒
        </button>
      </div>
    </DialogContent>
  </Dialog>
</template>
