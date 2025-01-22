<template>
  <!-- <div v-if="props.open"  class="fixed inset-0 bg-black/0 flex items-center justify-center z-10" @click="() => emit('close')"></div> -->
  <div
    v-if="props.open"
    ref="targetDiv"
    class="bg-white rounded-xl w-[244px] absolute shadow-xl border z-50 top-[60px]"
  >
    <div class="p-2 space-y-2">
      <!-- Profile Section -->
      <div class="border-b pb-2 cursor-pointer">
        <div
          class="flex items-center gap-4 hover:bg-gray-100 p-1 rounded-md transition-all ease-in-out duration-300 delay-100 px-2"
        >
          <div class="flex-1">
            <h3 class="font-medium text-sm text-black">
              {{ userInfo.account }}
            </h3>
            <p class="text-gray-500 text-xs">{{ userInfo.email }}</p>
          </div>
          <ChevronRightIcon class="w-5 h-5 text-gray-400" />
        </div>
      </div>

      <!-- Menu Items -->
      <div class="space-y-2">
        <!-- Language Selector -->
        <div class="relative">
          <button
            @click="toggleSubmenu('language')"
            class="flex items-center gap-4 hover:bg-gray-100 rounded-md transition-all ease-in-out duration-300 delay-100 p-2 w-full"
            :class="{ 'bg-gray-50': activeSubmenu === 'language' }"
          >
            <WidgetIcon class="w-4 h-4 text-sm" />
            <span class="flex-1 text-sm text-left">简体中文</span>
            <AltArrowDownIcon class="w-5 h-5 text-gray-400 -rotate-90" />
          </button>

          <!-- Language Submenu -->
          <Transition name="submenu">
            <div
              v-if="activeSubmenu === 'language'"
              class="absolute left-[230px] top-0 ml-2 w-48 bg-white rounded-xl shadow-lg border"
            >
              <div class="p-1 space-y-1">
                <button
                  v-for="lang in languages"
                  :key="lang.value"
                  @click="lang.handle"
                  class="w-full text-left p-2 rounded-lg hover:bg-gray-100 transition-colors text-xs"
                  :class="{ 'bg-gray-100': currentLanguage === lang.value }"
                >
                  {{ lang.label }}
                </button>
              </div>
            </div>
          </Transition>
        </div>

        <!-- Theme Mode -->
        <div class="relative">
          <button
            @click="toggleSubmenu('theme')"
            class="flex items-center gap-4 hover:bg-gray-100 rounded-md transition-all ease-in-out duration-300 delay-100 p-2 w-full"
            :class="{ 'bg-gray-50': activeSubmenu === 'theme' }"
          >
            <PaintRollerIcon class="w-4 h-4" />
            <span class="flex-1 text-sm text-left">主题模式</span>
            <AltArrowDownIcon class="w-5 h-5 text-gray-400 -rotate-90" />
          </button>

          <!-- Theme Submenu -->
          <Transition name="submenu">
            <div
              v-if="activeSubmenu === 'theme'"
              class="absolute left-[230px] top-0 ml-2 w-48 bg-white rounded-xl shadow-lg border"
            >
              <div class="p-1 space-y-1">
                <button
                  v-for="theme in themes"
                  :key="theme.value"
                  @click="theme.handle"
                  class="w-full text-left p-2 rounded-lg hover:bg-gray-100 transition-colors text-xs"
                  :class="{ 'bg-gray-100': currentTheme === theme.value }"
                >
                  {{ theme.label }}
                </button>
              </div>
            </div>
          </Transition>
        </div>

        <!-- Logout Button -->
        <button
          @click="logout"
          class="w-full flex items-center gap-3 p-2 border-[#ED003F] border rounded-lg text-[#ED003F] hover:bg-red-100 bg-red-50 transition-colors"
        >
          <LoginIcon class="w-4 h-4" />
          <span class="flex-1 text-left text-sm">退出登录</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import {
  WidgetIcon,
  PaintRollerIcon,
  LoginIcon,
} from "@/assets/icons/system-operation";
import { AltArrowDownIcon } from "@/assets/icons/environment";
import { ChevronRightIcon } from "lucide-vue-next";
import { useUserStore } from "@/stores/user";
import { useColorMode } from "@vueuse/core";
import { set_theme } from "@/commands";
import { setLocale } from "@/plugins/i18n";

const userInfo = useUserStore().getUserInfo();

const props = defineProps<{
  open: boolean;
}>();
watch(
  () => props.open,
  (_) => {
    activeSubmenu.value = null;
  }
);

const emit = defineEmits(["close", "logout"]);

const activeSubmenu = ref<string | null>(null);
const currentLanguage = ref<"zh-CN" | "en-US">("zh-CN");
const currentTheme = ref<"light" | "dark" | "auto">("light");

let mode = useColorMode();

const languages = [
  {
    label: "简体中文",
    value: "zh-CN",
    handle: () => {
      currentLanguage.value = "zh-CN";
      setLocale("zh-CN");
    },
  },
  {
    label: "English",
    value: "en-US",
    handle: () => {
      currentLanguage.value = "en-US";
      setLocale("en-US");
    },
  },
];

const themes = [
  {
    label: "浅色主题",
    value: "light",
    handle: () => {
      mode.value = "light";
      currentTheme.value = "light";
      set_theme("light");
    },
  },
  {
    label: "深色主题",
    value: "dark",
    handle: () => {
      mode.value = "dark";
      currentTheme.value = "dark";
      set_theme("dark");
    },
  },
  {
    label: "跟随系统",
    value: "auto",
    handle: () => {
      mode.value = "auto";
      currentTheme.value = "auto";
      set_theme("auto");
    },
  },
];

const toggleSubmenu = (menu: string) => {
  activeSubmenu.value = activeSubmenu.value === menu ? null : menu;
};

const logout = () => {
  emit("logout");
};

// Close submenus when clicking outside
const handleClickOutside = (event: MouseEvent) => {
  if (
    activeSubmenu.value &&
    !(event.target as HTMLElement).closest(".relative")
  ) {
    activeSubmenu.value = null;
  }
};

// Add/remove click outside listener
onMounted(() => {
  document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
});

// 获取指定的 div 元素
const targetDiv = ref<any>(null);

// 监听点击事件
const handleClick = (event: MouseEvent) => {
  // 判断是否点击到了指定的 div
  if (targetDiv.value && !targetDiv.value.contains(event.target)) {
    if (props.open === true) emit("close"); //未点击到则关闭弹窗
  }
};

// 在组件挂载时添加事件监听
onMounted(() => {
  document.addEventListener("click", handleClick);
});

// 在组件卸载时移除事件监听
onUnmounted(() => {
  document.removeEventListener("click", handleClick);
});
</script>

<style scoped>
.submenu-enter-active,
.submenu-leave-active {
  transition: all 0.2s ease;
}

.submenu-enter-from,
.submenu-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}
</style>
