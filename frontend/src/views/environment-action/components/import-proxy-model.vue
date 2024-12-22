<script setup>
import { ref } from 'vue'
import { Model } from '@/components/model/index'
import { PrimaryButton, CancelButton } from '@/components/button';

const props = defineProps({
    open: false,
})
const emits = defineEmits(['close'])
const batchForm = ref({
    service: 'IPRust.io',
    autoRefresh: false
})

</script>
<template>
    <Model class="min-w-[1250px]" title="导入代理" :open="props.open" @close="() => emits('close')">
        <div class="mx-8 p-6">
            <div class="grid grid-cols-2 gap-6">
                <div class="space-y-4">
                    <h3 class="text-sm font-medium text-gray-700">代理服务器</h3>

                    <textarea placeholder="请输入代理信息"
                        class="flex-1 border-[1px] border-gray-300 rounded-md shadow-sm p-2 text-sm h-96 overflow-y-auto resize-none"
                        style="width: 100%"></textarea>

                    <div class="flex items-center justify-between space-x-2">
                        <div class="flex items-center space-x-2">
                            <!-- 下拉框 -->
                            <select v-model="batchForm.service" class="border border-gray-300 rounded-md shadow-sm p-2"
                                style="height:40px;width: 150px">
                                <option value="IPRust.io">IPRust.io</option>
                                <option value="IPRust.io">IPRust.io</option>
                                <option value="IPRust.io">IPRust.io</option>
                                <option value="IPRust.io">IPRust.io</option>
                                <option value="IPRust.io">IPRust.io</option>
                            </select>

                            <!-- 自动去重 checkbox -->
                            <label class="flex items-center">
                                <input type="checkbox" v-model="batchForm.autoRefresh" class="h-4 w-4 text-blue-600">
                                <span class="ml-2 text-sm">自动去重</span>
                            </label>
                        </div>

                        <!-- 按钮右对齐 -->
                        <button class="bg-blue-500 text-white px-4 py-2 rounded-md text-sm hover:bg-blue-600 ml-auto"
                            style="height:40px;width: 80px;">
                            解析
                        </button>
                    </div>
                </div>

                <div class="bg-gray-50 p-4 rounded-md">
                    <h4 class="text-sm font-medium text-gray-700 mb-2">写入规则</h4>
                    <div class="text-xs text-gray-500 space-y-1">
                        <p>支持 IPv4, IPv6</p>
                        <p>支持 HTTP, HTTPS, SOCKS5 三种类型代理</p>
                        <p>每一行一个代理，必须完整添加</p>
                        <p>检测IP代理时间，测试自动添加IP，主机端口必须填写对应位置</p>
                    </div>
                </div>

            </div>

            <div class="flex justify-between">
                <div class="text-sm text-gray-500">已解析代理</div>
                <div class="space-x-2">
                    <button
                        class="text-sm text-gray-500  border border-gray-300 p-2 rounded-md hover:text-blue-500 hover:border-blue-500 active:bg-blue-50">删除失败</button>
                    <button
                        class="text-sm text-gray-500  border border-gray-300 p-2 rounded-md hover:text-blue-500 hover:border-blue-500 active:bg-blue-50">测试全部</button>
                </div>
            </div>



        </div>
        <div class="flex items-center gap-4 border-t p-6 px-14">
            <PrimaryButton @click="() => emits('close')">
                保存
            </PrimaryButton>
            <CancelButton @click="() => emits('close')">
                取消
            </CancelButton>
        </div>
    </Model>
</template>