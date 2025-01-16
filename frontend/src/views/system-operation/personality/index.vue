<template>
    <!-- <div v-if="props.open"  class="fixed inset-0 bg-black/0 flex items-center justify-center z-10" @click="() => emit('close')"></div> -->
    <div v-if="props.open" ref="targetDiv" class="bg-white rounded-2xl w-[300px] absolute shadow-xl border z-50 top-[60px]">
        <!-- Header -->
        <div class="flex justify-between items-center px-6 py-3 border-b">
            <h2 class="text-xl">个人中心</h2>
            <!-- <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
                <div class="size-7 text-xl">X</div>
            </button> -->
        </div>

        <!-- Content -->
        <div class="p-4">
            <!-- Profile Section -->
            <div class="bg-gray-50 rounded-xl p-2 mb-4">
                <div class="flex items-center gap-4">
                    <img :src="'https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000'"
                        alt="Profile" class="w-12 h-12 rounded-full" />
                    <div class="flex-1">
                        <h3 class="font-medium">Jojolin</h3>
                        <p class="text-gray-500 text-sm">Jojolin@gmail.com</p>
                    </div>
                    <AltArrowDownIcon class="w-5 h-5 text-gray-400 -rotate-90" />
                </div>
            </div>

            <!-- Menu Items -->
            <div class="space-y-2">
                <!-- Language Selector -->
                <div class="relative">
                    <button @click="toggleSubmenu('language')"
                        class="w-full flex items-center gap-3 p-2 rounded-xl hover:bg-gray-50 transition-colors"
                        :class="{ 'bg-gray-50': activeSubmenu === 'language' }">
                        <WidgetIcon class="w-5 h-5" />
                        <span class="flex-1 text-left">简体中文</span>
                        <AltArrowDownIcon class="w-5 h-5 text-gray-400 -rotate-90" />
                    </button>

                    <!-- Language Submenu -->
                    <Transition name="submenu">
                        <div v-if="activeSubmenu === 'language'"
                            class="absolute left-[280px] top-0 ml-2 w-48 bg-white rounded-xl shadow-lg border">
                            <div class="p-2">
                                <button v-for="lang in languages" :key="lang.value" @click="selectLanguage(lang.value)"
                                    class="w-full text-left px-4 py-2 rounded-lg hover:bg-gray-50 transition-colors"
                                    :class="{ 'text-[#5B5BFA]': currentLanguage === lang.value }">
                                    {{ lang.label }}
                                </button>
                            </div>
                        </div>
                    </Transition>
                </div>

                <!-- Theme Mode -->
                <div class="relative">
                    <button @click="toggleSubmenu('theme')"
                        class="w-full flex items-center gap-3 p-2 rounded-xl hover:bg-gray-50 transition-colors"
                        :class="{ 'bg-gray-50': activeSubmenu === 'theme' }">
                        <PaintRollerIcon class="w-5 h-5" />
                        <span class="flex-1 text-left">主题模式</span>
                        <AltArrowDownIcon class="w-5 h-5 text-gray-400 -rotate-90"/>
                    </button>

                    <!-- Theme Submenu -->
                    <Transition name="submenu">
                        <div v-if="activeSubmenu === 'theme'"
                            class="absolute left-[280px] top-0 ml-2 w-48 bg-white rounded-xl shadow-lg border">
                            <div class="p-2">
                                <button v-for="theme in themes" :key="theme.value" @click="selectTheme(theme.value)"
                                    class="w-full text-left px-4 py-2 rounded-lg hover:bg-gray-50 transition-colors"
                                    :class="{ 'text-[#5B5BFA]': currentTheme === theme.value }">
                                    {{ theme.label }}
                                </button>
                            </div>
                        </div>
                    </Transition>
                </div>

                <!-- Logout Button -->
                <button @click="logout"
                    class="w-full flex items-center gap-3 p-2 border-red-400 border rounded-xl text-red-500 hover:bg-red-100 bg-red-50 transition-colors">
                    <LoginIcon class="w-5 h-5" />
                    <span class="flex-1 text-left">退出登录</span>
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { WidgetIcon, PaintRollerIcon, LoginIcon } from '@/assets/icons/system-operation';
import { AltArrowDownIcon } from '@/assets/icons/environment';

const props = defineProps<{
    open: boolean
}>()
watch(() => props.open, (_) => {
    activeSubmenu.value = null
})

const emit = defineEmits(['close', 'logout', 'updateLanguage', 'updateTheme'])

const activeSubmenu = ref<string | null>(null)
const currentLanguage = ref('zh')
const currentTheme = ref('light')

const languages = [
    { label: '简体中文', value: 'zh' },
    { label: 'English', value: 'en' }
]

const themes = [
    { label: '浅色主题', value: 'light' },
    { label: '深色主题', value: 'dark' },
    { label: '跟随系统主题', value: 'system' }
]

const toggleSubmenu = (menu: string) => {
    activeSubmenu.value = activeSubmenu.value === menu ? null : menu
}

const selectLanguage = (lang: string) => {
    currentLanguage.value = lang
    emit('updateLanguage', lang)
    activeSubmenu.value = null
}

const selectTheme = (theme: string) => {
    currentTheme.value = theme
    emit('updateTheme', theme)
    activeSubmenu.value = null
}

const logout = () => {
    emit('logout')
}

// Close submenus when clicking outside
const handleClickOutside = (event: MouseEvent) => {
    if (activeSubmenu.value && !(event.target as HTMLElement).closest('.relative')) {
        activeSubmenu.value = null
    }
}

// Add/remove click outside listener
onMounted(() => {
    document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
})

// 获取指定的 div 元素
const targetDiv = ref<any>(null);

// 监听点击事件
const handleClick = (event: MouseEvent) => {

  // 判断是否点击到了指定的 div
  if (targetDiv.value && !targetDiv.value.contains(event.target)) {
    if (props.open === true) emit('close');  //未点击到则关闭弹窗
  }
};

// 在组件挂载时添加事件监听
onMounted(() => {
  document.addEventListener('click', handleClick);
});

// 在组件卸载时移除事件监听
onUnmounted(() => {
  document.removeEventListener('click', handleClick);
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