<script setup>
import { computed, onMounted, reactive, ref, watch } from "vue";
import {
  UploadCloud as UploadCloudIcon,
  Search as SearchIcon,
  Settings2 as Settings2Icon,
  SquarePen as SquarePenIcon,
  CheckCircle as CheckCircleIcon,
  Download,
  Ellipsis,
  Trash2 as Trash2Icon,
  CircleAlert as CircleAlertIcon,
} from "lucide-vue-next";
import { More, MoreContent, MoreItem, MoreTrigger } from "@/components/more";
import { Switch } from "@/components/ui/switch";
import Input from "@/components/input.vue";
import UploadModel from "./upload-model.vue";
import SettingModel from "./setting-model.vue";
import UpdateModel from "./update-model.vue";
import { AlertModel } from "@/components/alert-model";
import EnableSoftwareSwitch from "./enable-software-switch.vue";
import {
  extension_query,
  extension_query_by_user,
  extension_remove_by_user_uuid,
  extension_user_create,
  user_toggle_extension,
} from "@/commands/extension";
import { toast } from "vue-sonner";
import { app } from "@tauri-apps/api";

const uploadModelProps = reactive({
  open: false,
  title: "",
});
const settingModelProps = reactive({
  open: false,
  title: "",
  extension_uuid: "",
});
const updateModelProps = reactive({
  open: false,
  title: "",
});
const alertModelProps = reactive({
  open: false,
  title: "",
  message: "",
  showWarn: false,
  extension_title: "",
  extension_uuid: "",
});
const addExtensionToUserModelProps = reactive({
  open: false,
  title: "",
  message: "",
  extension_uuid: "",
  extension_title: "",
  extension_avatar: "",
  extension_description: "",
  extension_release_url: "",
});

const loadStoreExtension = () => {
  extension_query(1, 10)
    .then((res) => {
      if (res.code) {
        storeApps.value = res.data.data.map((item) => ({
          id: item.uuid,
          name: item.name,
          description: item.description,
          icon: item.avatar_url,
          enabled: true,
          downloading: true,
          downloaded: false,
          installed:
            installedApps.value.findIndex((i) => i.id == item.uuid) != -1,
        }));
      } else {
        toast.warning(res.message);
      }
    })
    .catch((err) => {
      toast.warning(err);
    });
}

const loadUserExtension = () => {
  extension_query_by_user(1, 10)
    .then((res) => {
      if (res.code) {
        installedApps.value = res.data.data.map((item) => ({
          id: item.uuid,
          name: item.name,
          description: item.description,
          icon: item.avatar_url,
          release_url: item.release_url,
          enabled: item.open == 1 ? true : false,
          downloading: true,
          downloaded: false,
          installed: true,
        }));
      } else {
        toast.warning(res.message);
      }
    })
    .catch((err) => {
      toast.warning(err);
    });
};

onMounted(() => {
  if (activeTab.value === "team") {
    loadUserExtension();
  }
});

const activeTab = ref("team");
const storeApps = ref([]);
const removeApp = ref();
const installedApps = ref([]);
const switchStatus = ref(false);

const addExtensionHandle = (app) => {
  if (app) {
    addExtensionToUserModelProps.open = true;
    addExtensionToUserModelProps.title = "下载应用";
    addExtensionToUserModelProps.extension_title = app.name;
    addExtensionToUserModelProps.extension_uuid = app.id;
    addExtensionToUserModelProps.extension_avatar = app.icon;
    addExtensionToUserModelProps.extension_description = app.description;
    addExtensionToUserModelProps.extension_release_url = app.release_url;
  }
};
const addExtensionSubmitHandle = async () => {
  let res = await extension_user_create(
    addExtensionToUserModelProps.extension_uuid
  );

  if (res.code == 1) {
    toast.success(res.message);
  } else {
    toast.warning(res.message);
  }

  addExtensionToUserModelProps.open = false;
};

const submitInstallSoftwareHandle = () => {
  uploadModelProps.open = false;
  activeTab.value = "team";
};

const uploadSoftwareHandle = () => {
  uploadModelProps.open = true;
  uploadModelProps.title = "上传应用";
};

const removeSoftwareOpenHandle = (app) => {
  alertModelProps.title = "移除应用";
  alertModelProps.open = true;
  alertModelProps.showWarn = true;
  alertModelProps.message = "移除后，所有浏览器将不会安装此应用。";
  removeApp.value = app;
};

const removeSoftwareHandle = async () => {
  alertModelProps.open = false;

  if (alertModelProps.title == "移除应用" && removeApp) {
    removeApp.installed = false;

    let id = removeApp.value.id;
    if (id) {
      let res = await extension_remove_by_user_uuid(removeApp.value.id);
      if (res.code == 1) {
        installedApps.value = installedApps.value.filter(
          (item) => item.id != id
        );
      } else {
        toast.warning(res.message);
      }
    } else {
      toast.warning("应用不存在");
    }
  }
  removeApp.value = undefined;
};

const switchOpenHandle = (app) => {
  alertModelProps.title = "开启应用";
  alertModelProps.open = true;
  alertModelProps.showWarn = false;
  alertModelProps.message =
    "开启后，如在新建浏览器环境时选择此应用，将安装到浏览器上使用。您确定开启以下应用吗？";
  alertModelProps.extension_title = app.name;
  alertModelProps.extension_uuid = app.id;
};

const switcExtensionHandle = async () => {
  let extension_uuid = alertModelProps.extension_uuid;
  let current_app = installedApps.value.find(
    (item) => item.id == extension_uuid
  );
  let res = await user_toggle_extension(extension_uuid, !current_app.enabled);
  if (res.code == 1) {
    current_app.enabled = !current_app.enabled;
  }
};

const settingOpenHandle = (app) => {
  settingModelProps.title = "配置环境";
  settingModelProps.open = true;
  settingModelProps.extension_uuid = app.id;
};
const updateOpenHandle = () => {
  updateModelProps.title = "更新应用";
  updateModelProps.open = true;
};
const loadData = computed(() => {
  return activeTab.value == "team" ? loadUserExtension : loadStoreExtension;
});

watch(activeTab, (newV) => {
  loadData.value();
});
</script>

<template>
  <div class="p-4 h-main rounded-md flex flex-col">
    <!-- Navigation Tabs -->
    <div class="flex flex-col gap-y-4 bg-white z-10 rounded-md pb-4">
      <nav class="flex gap-8 border-b px-4">
        <button
          @click="activeTab = 'team'"
          class="px-1 py-4 -mb-px font-medium"
          :class="
            activeTab === 'team'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-600'
          "
        >
          应用
        </button>
        <button
          @click="activeTab = 'recommended'"
          class="px-1 py-4 -mb-px font-medium"
          :class="
            activeTab === 'recommended'
              ? 'text-blue-600 border-b-2 border-blue-600'
              : 'text-gray-600'
          "
        >
          推荐应用
        </button>
      </nav>
      <!-- Search Bar and Actions -->
      <div
        :class="[
          'flex px-4 gap-x-8',
          activeTab !== 'team' ? 'justify-between' : 'justify-start',
        ]"
      >
        <div v-if="activeTab === 'team'" class="flex gap-4">
          <!-- <UploadModel v-if="activeTab === 'team'"> -->
          <button
            @click="uploadSoftwareHandle"
            class="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2"
          >
            <UploadCloudIcon class="h-5 w-5" />
            上传应用
          </button>
          <!-- </UploadModel> -->
        </div>
        <div class="relative w-80">
          <SearchIcon
            class="absolute left-3 top-2.5 h-5 w-5 text-gray-400 z-10"
          />
          <Input type="text" placeholder="关键字搜索" class="pl-10" />
        </div>
        <!-- No Results Message (Only on Recommended Page) -->
        <div
          v-if="activeTab === 'recommended'"
          class="h-full flex items-center justify-end mb-6 text-sm"
        >
          <span class="text-gray-500">没有找到你想要的应用？</span>
          <a href="#" class="text-blue-600 hover:underline"
            >前往Chrome 应用商店搜索</a
          >
        </div>
      </div>
    </div>
    <!-- Application Cards Grid -->
    <div class="flex-auto bg-white overflow-y-auto">
      <div
        class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-6 px-4 pb-8"
      >
        <div
          v-for="app in activeTab === 'team' ? installedApps : storeApps"
          :key="app.id"
          class="bg-white rounded-lg p-4 border overflow-y-auto"
        >
          <div class="flex flex-col">
            <div class="flex justify-between items-start mb-2">
              <div class="flex gap-x-2">
                <div>
                  <img
                    :src="app.icon"
                    alt="App icon"
                    class="w-16 h-16 rounded"
                  />
                </div>
                <div>
                  <h3 class="font-medium">{{ app.name }}</h3>
                  <span class="text-gray-400 text-xs">{{ app.version }}</span>
                </div>
              </div>

              <div
                v-if="activeTab !== 'recommended'"
                class="flex flex-col items-end gap-y-6"
              >
                <!-- <EnableSoftwareSwitch @click="() => switchOpenHandle(app)" /> -->
                <Switch
                  :checked="app.enabled"
                  @update="app.enabled = !app.enabled"
                  @click="() => switchOpenHandle(app)"
                />
                <div class="flex gap-1">
                  <p
                    v-if="app.downloading"
                    class="text-orange-500 text-xs whitespace-nowrap flex items-center gap-1"
                  >
                    <Download class="h-4 w-4" />
                    下载中
                  </p>
                  <p
                    v-if="app.downloaded"
                    class="text-green-500 text-xs whitespace-nowrap flex items-center gap-1"
                  >
                    <CheckCircleIcon class="h-4 w-4" />
                    下载完成
                  </p>
                </div>

              </div>
              <button
                v-if="activeTab === 'recommended'"
                @click="() => addExtensionHandle(app)"
                class="flex items-center justify-center gap-2 px-2 py-1 bg-white rounded-sm border text-xs l: text-blue-600 border-blue-600 whitespace-nowrap"
              >
                {{ app.installed ? "已添加" : "添加" }}
              </button>
            </div>

            <div
              class="description-wrapper h-36 overflow-y-auto"
              v-html="app.description"
            />
            <div class="mt-4 flex items-center justify-between">
              <div class="flex items-center gap-1 text-sm">
                <span class="text-gray-500">提供方：</span>
                <a href="#" class="text-blue-600 hover:underline"
                  >Chrome Web Store</a
                >
              </div>
              <More>
                <MoreTrigger>
                  <Ellipsis
                    v-if="activeTab !== 'recommended'"
                    class="h-6 w-6 cursor-pointer"
                  />
                </MoreTrigger>
                <MoreContent>
                  <MoreItem
                    class="cursor-pointer"
                    @click="() => settingOpenHandle(app)"
                  >
                    <Settings2Icon class="w-4 h-4" />配置
                  </MoreItem>
                  <MoreItem
                    class="cursor-pointer hidden"
                    @click="false && updateOpenHandle"
                  >
                    <SquarePenIcon class="w-4 h-4" />更新
                  </MoreItem>
                  <MoreItem
                    class="cursor-pointer"
                    @click="() => removeSoftwareOpenHandle(app)"
                  >
                    <Trash2Icon class="w-4 h-4" />移除
                  </MoreItem>
                </MoreContent>
              </More>
            </div>
          </div>
        </div>
      </div>
      <div
        v-if="!(activeTab === 'team' ? installedApps : storeApps).length"
        class="text-gray-400 text-center text-xs h-96 justify-center items-center flex"
      >
        内容为空
      </div>
      <AlertModel
        :title="alertModelProps.title"
        :open="alertModelProps.open"
        @close="() => (alertModelProps.open = false)"
        @cancel="() => (alertModelProps.open = false)"
        @submit="
          () => {
            alertModelProps.title == '开启应用'
              ? switcExtensionHandle()
              : removeSoftwareHandle();
            alertModelProps.open = false;
          }
        "
      >
        <div
          v-if="alertModelProps.showWarn"
          class="text-orange-400 border-[1px] p-2 px-4 border-orange-400 rounded-md bg-orange-100 flex items-center gap-x-4 text-sm"
        >
          <CircleAlertIcon />
          {{ alertModelProps.message }}
        </div>
        <div class="text-sm flex flex-col gap-y-4 py-4">
          <p>
            {{ alertModelProps.message }}
          </p>
          <p>
            应用名称
            <span class="bg-blue-200 p-2 rounded-md text-blue-600 ml-4">{{
              alertModelProps.extension_title
            }}</span>
          </p>
        </div>
      </AlertModel>

      <AlertModel
        title="添加扩展到个人中心"
        :open="addExtensionToUserModelProps.open"
        @close="() => (addExtensionToUserModelProps.open = false)"
        @cancel="() => (addExtensionToUserModelProps.open = false)"
        @submit="addExtensionSubmitHandle"
      >
        <div class="text-sm flex flex-col gap-y-4 py-4">
          <p>
            {{ addExtensionToUserModelProps.message }}
          </p>
          <p>
            应用名称
            <span class="bg-blue-200 p-2 rounded-md text-blue-600 ml-4">{{
              addExtensionToUserModelProps.extension_title
            }}</span>
          </p>
        </div>
      </AlertModel>

      <UploadModel
        :disableTab="activeTab == 'recommended' && 'package'"
        :open="uploadModelProps.open"
        :title="uploadModelProps.title"
        @close="() => (uploadModelProps.open = false)"
        @submit="submitInstallSoftwareHandle"
        @load="() => loadData()"
      />

      <SettingModel
        :open="settingModelProps.open"
        :title="settingModelProps.title"
        :extensionUuid="settingModelProps.extension_uuid"
        @close="() => (settingModelProps.open = false)"
      />

      <UpdateModel
        :open="updateModelProps.open"
        :title="updateModelProps.title"
        @close="() => (updateModelProps.open = false)"
      />
    </div>
  </div>
</template>

<style >
.toggle-checkbox:checked {
  @apply right-0 border-blue-600;
}
.toggle-checkbox:checked + .toggle-label {
  @apply bg-blue-600;
}
.description-wrapper p:first-of-type {
  @apply h-auto text-sm py-4 text-gray-800;
}
.description-wrapper p {
  @apply text-xs h-32 text-gray-500;
}
</style>
