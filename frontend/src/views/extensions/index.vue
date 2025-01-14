<script setup>
import {computed, onMounted, reactive, ref, watch} from 'vue'
import {
  PlusIcon,
  ClockIcon,
  MoreVerticalIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  Search as SearchIcon,
  CheckCircle as CheckCircleIcon,
  Download,
} from 'lucide-vue-next'
import UploadModel from "@/views/application-center/upload-model.vue";
import {
  extension_query,
  extension_query_by_user, extension_remove_by_user_uuid,
  extension_user_create,
  user_toggle_extension
} from "@/commands/extension";
import {toast} from "vue-sonner";
import {app} from "@tauri-apps/api";
import { Switch } from "@/components/ui/switch";
import { AlertModel } from "@/components/alert-model";


const extensions = ref([])
const activeMenu = ref(null)
const currentPage = ref(1)
const perPage = ref(10)
const totalItems = ref(extensions.value.length)
const activeTab = ref("team");
const storeApps = ref([]);
const removeApp = ref();

const showMenu = (id) => {
  activeMenu.value = activeMenu.value === id ? null : id
}
const uploadModelProps = reactive({
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

const addExtension = () => {
  uploadModelProps.open = true;
  uploadModelProps.title = "上传应用";
};

const submitInstallSoftwareHandle = () => {
  uploadModelProps.open = false;
  activeTab.value = "team";
};

const loadData = computed(() => {
  return activeTab.value === "team" ? loadUserExtension : loadStoreExtension;
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
                extensions.value.findIndex((i) => i.id === item.uuid) !== -1,
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
          extensions.value = res.data.data.map((item) => ({
            id: item.uuid,
            name: item.name,
            description: item.description,
            icon: item.avatar_url,
            release_url: item.release_url,
            enabled: item.open === 1,
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
  let current_app = extensions.value.find(
      (item) => item.id === extension_uuid
  );
  let res = await user_toggle_extension(extension_uuid, !current_app.enabled);
  if (res.code === 1) {
    current_app.enabled = !current_app.enabled;
  }
};


const addExtensionSubmitHandle = async () => {
  let res = await extension_user_create(
      addExtensionToUserModelProps.extension_uuid
  );

  if (res.code === 1) {
    toast.success(res.message);
  } else {
    toast.warning(res.message);
  }

  addExtensionToUserModelProps.open = false;
};

const removeSoftwareHandle = async () => {
  alertModelProps.open = false;

  if (alertModelProps.title === "移除应用" && removeApp) {
    removeApp.installed = false;

    let id = removeApp.value.id;
    if (id) {
      let res = await extension_remove_by_user_uuid(removeApp.value.id);
      if (res.code === 1) {
        extensions.value = extensions.value.filter(
            (item) => item.id !== id
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

// Click outside to close menu
const closeMenu = (e) => {
  if (!e.target.closest('.menu-trigger')) {
    activeMenu.value = null
  }
}

// Add click outside listener
if (typeof window !== 'undefined') {
  window.addEventListener('click', closeMenu)
}
onMounted(() => {
    loadUserExtension();
});

</script>
<template>
  <div class="flex flex-col  h-screen ">
  <div class="bg-white flex-shrink-0">
    <!-- Header -->
    <header class="mb-6">
      <div class="flex justify-between items-center">
        <h1 class="text-2xl font-bold">拓展</h1>
        <div class="flex items-center space-x-4">
          <div class="relative">
            <input
                type="search"
                placeholder="搜索"
                class="pl-10 pr-4 py-2 border rounded-lg w-64 focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
            <SearchIcon class="w-5 h-5 absolute left-3 top-2.5 text-gray-400" />
          </div>
          <button
              class="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-700 flex items-center space-x-2"
              @click="addExtension"
          >
            <PlusIcon class="w-5 h-5" />
            <span>添加拓展</span>
          </button>
        </div>
      </div>
    </header>

    <!-- Tabs -->
    <div class="border-b mb-6">
      <nav class="flex space-x-8">
        <a
            href="#"
            class="border-b-2 border-indigo-600 text-indigo-600 pb-4 px-1"
        >
          浏览器拓展
        </a>
        <a
            href="#"
            class="text-gray-500 hover:text-gray-700 pb-4 px-1"
        >
          云手机拓展
        </a>
      </nav>
    </div>

    <!-- Content -->
    <div class="flex-grow overflow-y-auto">
      <div v-if="extensions.length === 0" class="text-center py-20">
        <div class="w-32 h-32 mx-auto mb-6 relative">
          <div class="absolute inset-0 bg-indigo-100 rounded-lg"></div>
          <img
              :src="'https://hebbkx1anhila5yf.public.blob.vercel-storage.com/image-1v98O1EMy5KGJgQ2dbrP32Vn8myv8x.png'"
              alt="Empty state illustration"
              class="relative z-10"
          />
        </div>
        <h3 class="text-xl font-medium mb-2">轻松把扩展加到环境里</h3>
        <p class="text-gray-500 mb-6">在浏览器扩展里加的扩展，会同步给有权限的成员，在环境里加的扩展就不会同步啦。</p>
        <button
            class="bg-indigo-600 text-white px-6 py-2 rounded-md hover:bg-indigo-700 inline-flex items-center space-x-2"
            @click="addExtension"
        >
          <ClockIcon class="w-5 h-5" />
          <span>添加拓展</span>
        </button>
      </div>

      <div v-else class="grid md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-6">
        <div
            v-for="extension in extensions"
            :key="extension.id"
            class="border rounded-lg p-2 relative group hover:shadow-lg transition-shadow min-w-[300px]"
        >
          <div class="relative h-16">
            <img
                src="../../assets/images/2021051521244130.jpg"
                alt="Extension header"
                class="w-full h-full object-cover"
            />
            <div class="absolute top-3 right-3">
              <Switch
                  :checked="extension.enabled"
                  @update="extension.enabled = !extension.enabled"
                  @click="() => switchOpenHandle(extension)"
              />
            </div>

          </div>


          <div class="flex items-start space-x-4 mt-2">
            <img :src="extension.icon" alt="" class="w-8 h-8 rounded-lg" />
            <div class="flex-1">
              <div class="flex items-start justify-between">
                <div class="" style="width: 90%;">
                  <div class="flex justify-between " >
                    <p class="font-bold text-[14px]">
                      {{ extension.name.length > 16 ? extension.name.substring(0, 16) + '...' : extension.name }}
                    </p>

                    <button
                        @click="showMenu(extension.id)"
                        class="text-gray-400 hover:text-gray-600"
                    >
                      <MoreVerticalIcon class="w-5 h-5" />
                    </button>
                  </div>
                  <div class="flex items-center space-x-2 text-sm text-gray-500">
                    <span>提供方</span>
                    <a href="#" class="text-indigo-600 hover:underline">
                      Chrome Web Store
                    </a>
                  </div>
                </div>
              </div>
              <p
                  class="text-gray-500 text-sm mt-2 line-clamp-2 w-11/12"
                  :title="extension.description"
              >
                {{ extension.description }}
              </p>

            </div>
          </div>

          <!-- Dropdown Menu -->
          <div
              v-if="activeMenu === extension.id"
              class="absolute right-4 top-16 w-48 bg-white rounded-lg shadow-lg border py-1 z-10"
          >
            <button
                class="w-full px-4 py-2 text-left text-sm hover:bg-gray-50"
            >
              配置范围
            </button>
            <button
                class="w-full px-4 py-2 text-left text-sm text-red-600 hover:bg-gray-50"
            >
              删除环境
            </button>
          </div>
        </div>
      </div>
    </div>


  </div>

  <!-- Pagination -->
  <div class="bg-white border-t border-gray-200 p-2 flex items-center justify-between min-h-[60px] flex-shrink-0 mt-2">
    <p class="text-sm text-gray-500">共 {{ totalItems }} 项</p>
    <div class="flex items-center space-x-2">
      <button
          class="p-2 rounded hover:bg-gray-100"
          :disabled="currentPage === 1"
      >
        <ChevronLeftIcon class="w-5 h-5" />
      </button>
      <span class="px-4 py-2 rounded bg-gray-100">{{ currentPage }}</span>
      <button
          class="p-2 rounded hover:bg-gray-100"
      >
        <ChevronRightIcon class="w-5 h-5" />
      </button>
      <select
          v-model="perPage"
          class="ml-2 border rounded-lg px-2 py-1"
      >
        <option value="10">10条/页</option>
        <option value="20">20条/页</option>
        <option value="50">50条/页</option>
      </select>
    </div>
  </div>

  </div>

  <UploadModel
      :disableTab="activeTab === 'recommended' && 'package'"
      :open="uploadModelProps.open"
      :title="uploadModelProps.title"
      @close="() => (uploadModelProps.open = false)"
      @submit="submitInstallSoftwareHandle"
      @load="() => loadData()"
  />

  <AlertModel
      :title="alertModelProps.title"
      :open="alertModelProps.open"
      @close="() => (alertModelProps.open = false)"
      @cancel="() => (alertModelProps.open = false)"
      @submit="
          () => {
            alertModelProps.title === '开启应用'
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

</template>

