<template>
    <div v-if="props.open" class="fixed inset-0 bg-black/50 flex items-center justify-center z-[99]"
        @click="() => emit('close')"></div>
    <Transition name="modal-fade">
        <div v-if="props.open"
            class="z-[100] h-[550px] w-[950px] inset-0 m-auto bg-[#5E51FF] fixed rounded-xl overflow-hidden">
            <div class="p-4 flex justify-between mb-8">
                <div class="text-white">联系我们</div>
                <!-- <span class="cursor-pointer z-[110]"> -->
                    <CloseIcon class="size-5 cursor-pointer z-[110]" @click="emit('close')" />
                <!-- </span> -->
            </div>
            <div class="flex flex-col items-center justify-between gap-3">
                <p class="text-white text-[23px] font-semibold">🙋 联系我们获得帮助</p>
                <div class="flex">
                    <div class="flex flex-col items-center">
                        <img :src="WeChat" alt="" srcset="">
                        <div class="flex items-center gap-2 text-white">
                            <span class="p-0.5 bg-[#06C662] rounded-md">
                                <SocialMediaWeChatIcon class="size-5" />
                            </span>
                            扫码获得微信
                        </div>
                    </div>
                    <div class="flex flex-col items-center">
                        <img :src="FlyBook" alt="" srcset="">
                        <div class="flex items-center gap-2 text-white">
                            <span class="p-0.5 bg-[#2aabee] rounded-full">
                                <SocialMediaFlyBookIcon class="size-5" />
                            </span>
                            扫码获得Telegram
                        </div>
                    </div>
                </div>
            </div>
            <GroupIcon class="size-[220px] opacity-30 absolute bottom-0" />
            <GroupIcon class="size-[220px] opacity-30 rotate-180 absolute top-0 right-0" />
        </div>
    </Transition>
</template>

<script setup lang="ts">
import { GroupIcon, SocialMediaFlyBookIcon, SocialMediaWeChatIcon } from '@/assets/icons/system-operation/index';
import { CloseIcon } from '@/assets/icons/synchronizer';
import WeChat from "@/assets/icons/system-operation/WeChat.png"
import FlyBook from "@/assets/icons/system-operation/fly book.png"
import Model from '@/components/model/model.vue'

const props = defineProps<{
    open: boolean
}>()

const emit = defineEmits(['close'])
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
    transition: opacity 0.3s ease, transform 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
    opacity: 0;
    transform: translateY(20px); /* 元素向下移动20px，实现自下而上消失的效果 */
}
</style>