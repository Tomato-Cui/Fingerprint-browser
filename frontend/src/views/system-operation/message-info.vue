<template>
    <div v-if="props.open" class="fixed inset-0 bg-black/50 flex items-center justify-center z-[9999]"
        @click="() => emit('close')"></div>
    <Transition name="modal-fade">
        <Model :open="props.open" title="消息详情" @close="emit('close')" class="min-w-[900px] z-[1000]">
            <!-- <div class="bg-white rounded-2xl w-[900px] relative"> -->
            <!-- Header -->
            <!-- <div class="flex justify-between items-center p-6">
                    <h2 class="text-xl font-medium">XXy 官方通知</h2>
                    <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
                        <div class="size-5 text-[20px]">X</div>
                    </button>
                </div> -->

            <!-- Content -->
            <div class="px-6 pb-6 space-y-6 max-h-[80vh] overflow-y-auto">
                <!-- Banner -->
                <div class="h-[200px] rounded-xl bg-gradient-to-r from-[#8B5CF6] via-[#6366F1] to-[#60A5FA]"></div>

                <!-- Text Content -->
                <div class="space-y-6 text-gray-700">
                    <p class="text-lg">亲爱的 Roxy 用户，感谢您一直以来的信任与支持！</p>

                    <p>自最初版本上线至今8个月以来(2024.05.09-2025.01.09)Roxy 已完成:35次APP版本迭代、29次内核更新。若干UX/UI优化如今，Roxy
                        已拥有超过17600名注册用户，累计创建窗口超过4940000个在此期间，我们不断升级服务资源与安全策略以对抗异常行为，并持续提供完全免费的服务</p>

                    <p class="font-medium text-black">在这样的用户增长趋势及负载下，我们决定于【2025.03.26】启动商业化运营</p>

                    <p>而在此之前，我们仍会以可控成本提供尽可能的免费服务:</p>

                    <ul class="space-y-2 list-disc pl-5">
                        <li>普通用户:20个窗口，1个额外工作空间(受邀注册或参与推广计划)</li>
                        <li>特邀用户:60个窗口，2个额外工作空间(通过老用户特邀码兑换)</li>
                        <li>创始用户:100个窗口，3个额外工作空间(2024.11.05前注册用户自动获得)。。此外，所有创始用户和特邀用户都将在未来享有永久优惠权益</li>
                    </ul>

                    <p>您可以随时点击右上角【我的权益】查看详情</p>

                    <p>在正式推出收费方案前，我们将持续提升产品性能及功能，包括但不限于:</p>

                    <ol class="space-y-2 list-decimal pl-5">
                        <li>窗口同步及排列功能、自定义UA支持(1月)</li>
                        <li>全局交互体验一致性优化</li>
                        <li>团队、工作空间及扩展功能逻辑优化</li>
                    </ol>

                    <p>您的建议对 Roxy 的发展至关重要，欢迎通过技术支持或在官方社群中与我们联系。</p>

                    <div class="pt-4">
                        <p>xy 官方团队</p>
                        <p class="text-gray-500">2025.01.13</p>
                    </div>
                </div>
            </div>

            <!-- Footer -->
            <div class="px-6 py-4 border-t flex justify-end">
                <button @click="$emit('close')"
                    class="bg-[#5B5BFA] hover:bg-[#4A4AE5] text-white rounded-lg py-2 px-4 transition-colors">
                    明白
                </button>
            </div>
            <!-- </div> -->
        </Model>
    </Transition>
</template>

<script setup lang="ts">
import Model from '@/components/model/model.vue'

const props = defineProps<{
    open: boolean
    messageId: number
}>()

const emit = defineEmits(['close'])
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
    transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
    opacity: 0;
}
</style>