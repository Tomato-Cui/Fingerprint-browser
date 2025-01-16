<script setup>
import {computed, onMounted, reactive, ref, watch} from 'vue'
import {
  PlusIcon,
  ClockIcon,
  XIcon,
  MoreVerticalIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  Search as SearchIcon,
  CheckCircle as CheckCircleIcon,
  Download, TrashIcon, PencilIcon,
} from 'lucide-vue-next'
import UploadModel from "@/views/application-center/upload-model.vue";
import { MoreVertical } from 'lucide-vue-next'
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
import { Dialog, DialogPanel, DialogTitle, TransitionRoot, TransitionChild } from '@headlessui/vue'
import {Popover, PopoverContent, PopoverTrigger} from "@/components/ui/popover/index";
import TooltipButton from "@/components/tooltip-button.vue";

const extensions = ref([])
const activeMenu = ref(null)
const currentPage = ref(1)
const perPage = ref(10)
let totalItems = ref(0)
const activeTab = ref("team");
const removeApp = ref();
let totalPages = computed(() => Math.ceil(totalItems.value / perPage.value));

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
  return loadUserExtension;
});

const loadUserExtension = () => {
  extension_query_by_user(currentPage.value, perPage.value)
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
          totalItems = extensions.value.length
          totalPages = totalItems/10+ totalItems%10 ? 1:0
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

const isMenuOpen = ref(false)
const activeModal = ref(null)
const rangeSelection = ref('all')
const selected = ref()
const fixedSelection = ref('independent')
const enableSelection = ref('independent')

const openModal = (modalName) => {
  isMenuOpen.value = false
  activeModal.value = modalName
}

const closeModal = () => {
  activeModal.value = null
}

</script>
<template>
  <div class="bg-white rounded-xl px-2 py-2 w-full h-full flex flex-col overflow-hidden" >
  <div class="bg-white flex-shrink-0 flex-1 overflow-hidden flex flex-col">
    <!-- Header -->
    <header class="mb-2">
      <div class="flex justify-between items-center">
        <p class="text-[20px] font-bold ml-4">拓展</p>
        <div class="flex items-center space-x-4 mr-2">
          <div class="relative">
            <input
                type="search"
                placeholder="搜索"
                class="pl-10 pr-4 py-2 border rounded-lg w-64 focus:outline-none focus:ring-2 focus:ring-indigo-500 "
            >
            <SearchIcon class="w-5 h-5 absolute left-3 top-2.5 text-gray-400" />
          </div>
          <button
              class="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-700 flex items-center space-x-2"
              @click="addExtension"
          >
            <PlusIcon class="w-5 h-5" />
            <p class="text-[14px]">添加拓展</p>
          </button>
        </div>
      </div>
    </header>

    <!-- Tabs -->
    <div class="border-b mb-3 ml-3 ">
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
    <div class=" overflow-y-auto">
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

      <div v-else class=" grid md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3 ml-3  h-auto ">
        <div
            v-for="extension in extensions"
            :key="extension.id"
            class="border rounded-lg p-2 relative group hover:shadow-lg transition-shadow  hover:border-blue-600 hover:border-spacing-1"
        >
          <div class="relative h-16 ">
            <img
                src="../../assets/images/2021051521244130.jpg"
                alt="Extension header"
                class="w-full h-full object-cover rounded-md"
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
                <div class="" style="width: 100%;">
                  <div class="flex justify-between " >
                    <p class="font-bold text-[14px]">
                      {{ extension.name.length > 16 ? extension.name.substring(0, 16) + '...' : extension.name }}
                    </p>


                    <div class="relative">
                      <Popover v-model:open="isMenuOpen">
                        <PopoverTrigger as-child>
                          <TooltipButton title="设置"
                                         class="p-2 rounded hover:bg-gray-0  border-gray-300  active:bg-blue-50 hover:text-blue-500">
                            <MoreVertical class="w-4 h-4" />
                          </TooltipButton>
                        </PopoverTrigger>
                        <PopoverContent class="p-2 w-[120px] text-sm" align="center" >
                          <div class="py-1">
                            <button
                                @click="openModal('range')"
                                class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 w-full text-center"
                            >
                              配置范围
                            </button>
                            <button
                                @click="openModal('settings')"
                                class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 w-full text-center"
                            >
                              拓展设置
                            </button>
                            <button
                                @click="openModal('delete')"
                                class="block px-4 py-2 text-sm text-red-600 hover:bg-gray-100 w-full text-center"
                            >
                              删除环境
                            </button>
                          </div>
                        </PopoverContent>


                      </Popover>
                    </div>

                  </div>
                  <div class="flex items-center space-x-2 text-sm text-gray-500">
                    <span>提供方</span>
                    <a href="#" class="text-indigo-600 hover:underline">
                      Chrome Web Store
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
<!--            <p-->
<!--                class="text-black text-sm  mt-1 line-clamp-2 "-->
<!--                :title="extension.description"-->
<!--            >-->
<!--              {{ extension.description }}-->
<!--            </p>-->
          <div
              class="description-wrapper  overflow-y-auto"
              v-html="extension.description"
          />
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
  <div class="bg-white border-t border-gray-200 p-3 flex items-center justify-between h-[60px] mt-3">
    <p class="text-sm text-gray-500">共 {{ totalItems }} 项</p>
    <div class="flex items-center space-x-2">
      <button
          class="p-2 rounded hover:bg-gray-100"
          :disabled="currentPage === 1"
          @click="currentPage-- && loadData()"
      >
        <ChevronLeftIcon class="w-5 h-5" />
      </button>
      <span class="px-4 py-2 rounded bg-gray-100">
        {{ currentPage }}
      </span>
      <button
          class="p-2 rounded hover:bg-gray-100"
          :disabled="currentPage >= totalPages"
          @click="currentPage++ && loadData()"
      >
        <ChevronRightIcon class="w-5 h-5" />
      </button>
      <select
          v-model="perPage"
          class="ml-2 border rounded-lg px-2 py-1"
          @change="loadUserExtension"
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

  <Teleport to="body">
    <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
    >
      <div v-if="activeModal === 'range'" class="fixed inset-0 z-50">
        <!-- Overlay -->
        <div class="absolute inset-0 bg-black/30" @click="closeModal"></div>

        <!-- Modal -->
        <div class="flex items-center justify-center min-h-screen p-4">
          <div
              class="relative bg-white rounded-lg w-full max-w-md shadow-lg"
              @click.stop
          >
            <!-- Header -->
            <div class="flex items-center justify-between p-3 border-b">
              <h3 class="font-[600]">配置范围</h3>
              <button
                  @click="closeModal"
                  class="text-gray-400 hover:text-gray-500 transition-colors"
              >
                <XIcon class="w-5 h-5" />
              </button>
            </div>

            <!-- Content -->
            <div class="px-6 py-4">
              <div class="mb-4 flex">
                <div class="space-y-3 ml-14">
                  <h3 class="text-sm text-gray-700 mb-3 font-[600] text-[12px]">环境窗口排序</h3>
                </div>

                <div class="space-y-3 ml-8">
                  <label class="flex items-center text-sm">
                    <input
                        type="radio"
                        name="config"
                        value="all"
                        v-model="selected"
                        class="w-4 h-4 text-blue-600 border -blue-300 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-gray-700 font-[600] text-[12px]">全部环境</span>
                  </label>
                  <label class="flex items-center text-sm">
                    <input
                        type="radio"
                        name="config"
                        value="group"
                        v-model="selected"
                        class="w-4 h-4 text-blue-600 border-gray-300 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-gray-700 font-[600] text-[12px]">按分组</span>
                  </label>
                  <label class="flex items-center text-sm">
                    <input
                        type="radio"
                        name="config"
                        value="platform"
                        v-model="selected"
                        class="w-4 h-4 text-blue-600 border-gray-300 focus:ring-blue-500"
                    />
                    <span class="ml-2 text-gray-700 font-[600] text-[12px]">按平台</span>
                  </label>
                </div>
              </div>
            </div>

            <!-- Footer -->
            <div class="flex justify-end gap-2 p-2 border-t">
              <button
                  @click="closeModal"
                  class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                取消
              </button>
              <button
                  @click="closeModal"
                  class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                确认
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
  <!-- Settings Modal -->
  <Teleport to="body">
    <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
    >
      <div v-if="activeModal === 'settings'" class="fixed inset-0 z-50">
        <!-- Overlay -->
        <div class="absolute inset-0 bg-black/30" @click="closeModal"></div>

        <!-- Modal -->
        <div class="flex items-center justify-center min-h-screen p-4">
          <div
              class="relative bg-white rounded-lg w-[500px] max-w-md shadow-lg "
              @click.stop
          >
            <!-- Header -->
            <div class="flex items-center justify-between p-3 border-b">
              <h3 class=" font-[500] text-[14px]">配置范围</h3>
              <button
                  @click="closeModal"
                  class="text-gray-400 hover:text-gray-500 transition-colors"
              >
                <XIcon class="w-5 h-5" />
              </button>
            </div>

            <div class="space-y-6 p-4">
              <!-- 固定方式 -->
              <div class="flex items-start">
                <div class="w-1/4 text-right pr-4">
                  <h4 class="mb-4  font-[600] text-[12px]">固定方式</h4>
                </div>
                <div class="w-3/4 space-y-4">
                  <div class="flex items-center space-x-2">
                    <input type="radio" id="independent1" v-model="fixedSelection" value="independent" class="h-3 w-3 text-blue-600" />
                    <label for="independent1 " class=" font-[500] text-xs">不控制，在环境中单独设置</label>
                  </div>
                  <div class="flex items-center space-x-2">
                    <input type="radio" id="fixed" v-model="fixedSelection" value="fixed" class="h-3 w-3 text-blue-600" />
                    <label for="fixed" class=" font-[500] text-[12px]">固定至工作栏</label>
                  </div>
                  <div class="flex items-center space-x-2">
                    <input type="radio" id="unfixed" v-model="fixedSelection" value="unfixed" class="h-3 w-3 text-blue-600" />
                    <label for="unfixed" class=" font-[500] text-[12px]">不固定至工作栏</label>
                  </div>
                </div>
              </div>

              <!-- 启用/禁用 -->
              <div class="flex items-start">
                <div class="w-1/4 text-right pr-4">
                  <h4 class="font-[600]  text-[12px] mb-4">启用/禁用</h4>
                </div>
                <div class="w-3/4 space-y-4">
                  <div class="flex items-center space-x-2">
                    <input type="radio" id="independent2" v-model="enableSelection" value="independent" class="h-3 w-3 text-blue-600" />
                    <label for="independent2" class="font-[500]  text-[12px]">不控制，在环境中单独设置</label>
                  </div>
                  <div class="flex items-center space-x-2">
                    <input type="radio" id="enable" v-model="enableSelection" value="enable" class="h-3 w-3 text-blue-600" />
                    <label for="enable" class="font-[500]  text-[12px]">自动启用</label>
                  </div>
                  <div class="flex items-center space-x-2">
                    <input type="radio" id="disable" v-model="enableSelection" value="disable" class="h-3 w-3 text-blue-600" />
                    <label for="disable" class="font-[500]  text-[12px]">自动禁用</label>
                  </div>
                </div>
              </div>
            </div>


            <!-- Footer -->
            <div class="flex justify-end gap-2 p-2 border-t">
              <button
                  @click="closeModal"
                  class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                取消
              </button>
              <button
                  @click="closeModal"
                  class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
              >
                确认
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
  <!-- Delete Confirmation Modal -->
  <TransitionRoot appear :show="activeModal === 'delete'" as="template">
    <Dialog as="div" @close="closeModal" class="relative z-50">
      <TransitionChild
          enter="duration-300 ease-out"
          enter-from="opacity-0"
          enter-to="opacity-100"
          leave="duration-200 ease-in"
          leave-from="opacity-100"
          leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-black/25" />
      </TransitionChild>

      <div class="fixed inset-0 overflow-y-auto">
        <div class="flex min-h-full items-center justify-center p-4">
          <TransitionChild
              enter="duration-300 ease-out"
              enter-from="opacity-0 scale-95"
              enter-to="opacity-100 scale-100"
              leave="duration-200 ease-in"
              leave-from="opacity-100 scale-100"
              leave-to="opacity-0 scale-95"
          >
            <DialogPanel class="w-full max-w-md transform overflow-hidden rounded-lg bg-white p-6 shadow-xl transition-all">
              <DialogTitle class="text-[14px] font-bold mb-1">删除该拓展吗</DialogTitle>

              <p class="text-[13px]">删除后，未安装此扩展的浏览器环境将不再安装。</p>
              <p class="text-[13px]">是否删除该扩展？</p>

              <div class="mt-6 flex justify-end space-x-3">
                <button @click="closeModal" class="px-4 py-1 border rounded-md hover:bg-gray-50 text-[14px]">
                  取消
                </button>
                <button @click="closeModal" class="px-4 py-1 bg-blue-600 text-white rounded-md hover:bg-blue-700 text-[14px]">
                  确认
                </button>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<style >
.description-wrapper p:first-of-type {
  @apply h-auto py-4 text-gray-800 block text-xs;
}
.description-wrapper p {
  @apply text-xs h-16 text-gray-500 block;
}
</style>