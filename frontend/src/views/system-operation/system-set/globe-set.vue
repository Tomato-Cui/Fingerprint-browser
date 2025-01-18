<template>
    <layout>
        <div class="min-h-screen flex p-4">
            <!-- Sidebar -->
            <aside class="w-64 bg-white border-gray-200">
                <nav class="p-4 space-y-1 border-r">
                    <button v-for="item in menuItems" :key="item.id" @click="activeItem = item.id" :class="[
                        'w-full text-left px-4 py-3 rounded-lg transition-colors duration-150',
                        activeItem === item.id
                            ? 'bg-[#5050FA] text-white'
                            : 'text-gray-700 hover:bg-gray-100'
                    ]">
                        {{ item.name }}
                    </button>
                </nav>
            </aside>

            <main class="flex-1 p-8">
                <!-- 安全锁 -->
                <div v-if="activeItem === 'security'" class="">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">安全锁</h1>

                    <!-- Security Settings -->
                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="flex items-center justify-between bg-white p-4">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">活动会话锁定</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    安全锁规避多个用户同时使用同一个环境，访问相同网站所带来的检测风险。
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- 端对端加密 -->
                <div v-else-if="activeItem === 'encryption'">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">环境端对端加密</h1>

                    <!-- Security Settings -->
                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="flex items-center justify-between bg-white p-4 pb-0">
                            <div class="w-full flex items-center justify-between pb-4"
                                :class="{ 'border-b': keyVal != '' }">
                                <div>
                                    <h3 class="text-lg font-medium text-gray-900">密钥设置</h3>
                                    <p class="mt-1 text-sm text-gray-500">
                                        开启环境密钥后，可通过唯一的安全密钥对环境数据进行加密，保障数据安全
                                    </p>
                                </div>
                                <label class="relative inline-flex cursor-pointer items-center">
                                    <button @click="postLockModel = true"
                                        class="flex border py-2 px-4 shadow-lg rounded-lg hover:bg-gray-100 gap-2 items-center">
                                        <KeyIcon class="size-5" />
                                        生成密钥
                                    </button>
                                </label>
                            </div>
                        </div>
                        <div v-if="keyVal !== ''" class="bg-white px-4 py-2 space-y-3">
                            <div class="w-full flex items-center justify-between">
                                <span>密钥验证频率</span>
                                <div class="text-blue-700 font-medium">管理密钥授信设备</div>
                            </div>
                            <RadioGroup defaultValue="option-one">
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-one" id="option-one"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-one">新设备登录后验证一次 <span
                                            class="text-gray-400">&nbsp;只能通过分组维度分配给您的团队成员，团队成员拥有对应的分组环境</span></Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-two" id="option-two"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-two">每次登录后验证一次 <span
                                            class="text-gray-400">&nbsp;只能通过环境维度分配给团队成员，团队成员拥有对应的环境</span></Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-three" id="option-three"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-three">每次启动环境后验证 <span
                                            class="text-gray-400">&nbsp;API用户不推荐此项</span></Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>
                </div>

                <!-- 拓展数据保护 -->
                <div v-else-if="activeItem === 'data-protection'" class="">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">拓展数据保护</h1>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="flex items-center justify-between bg-white p-4">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">本地扩展数据保护</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    启动环境将自动加密常规扩展程序的本地数据文件，防止未经授权的访问。
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- 数据同步 -->
                <div v-else-if="activeItem === 'data-sync'" class="">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">数据同步</h1>

                    <div class="rounded-lg p-6 bg-gray-50 flex gap-6 justify-between">
                        <div class="flex items-center justify-between bg-white p-4 w-full rounded-sm">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">同步Cookie</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    同步cookie功能确保您的浏览器在不同设备间保持一致的登录状态，提升使用体验。
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                        <div class="flex items-center justify-between bg-white p-4 w-full rounded-sm">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">同步扩展程序及其数据</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    注意：环境内添加「Chrome应用商店」的扩展会同步；以「安装包形式」添加的扩展，则不会同步。。
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                    </div>
                    <div class="rounded-lg p-6 bg-gray-50 flex gap-6 justify-between">
                        <div class="flex items-center justify-between bg-white p-4 w-full rounded-sm">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">同步已保存的密码</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    同步浏览器中设置的密码
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                        <div class="flex items-center justify-between bg-white p-4 w-full rounded-sm">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">同步缓存的数据</h3>
                                <p class="mt-1 text-sm text-gray-500 flex items-center justify-between">
                                    <span class="flex items-center gap-2 mr-3">
                                        <Checkbox />标签页
                                    </span>
                                    <span class="flex items-center gap-2 mr-3">
                                        <Checkbox />历史记录
                                    </span>
                                    <span class="flex items-center gap-2 mr-3">
                                        <Checkbox />LocalStorage
                                    </span>
                                    <span class="flex items-center gap-2 mr-3">
                                        <Checkbox />IndexedDB
                                    </span>
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- 清除云端缓存 -->
                <div v-else-if="activeItem === 'cache'">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">清除云端缓存</h1>

                    <!-- Security Settings -->
                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="flex items-center justify-between bg-white p-4">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">清除云端所有缓存</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    此功能将清除所有的云端缓存数据。
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <button @click="void (0)"
                                    class="flex border py-2 px-4 shadow-lg rounded-lg hover:bg-gray-100 gap-2 items-center">
                                    <ClearIcon class="size-5" />
                                    立即清除
                                </button>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- 授权方式 -->
                <div v-else-if="activeItem === 'permissions'">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">授权方式</h1>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="bg-white px-4 py-2 space-y-3">
                            <div class="w-full flex items-center">
                                <span class="font-[600]">清除云端所有缓存</span>
                                <div class="text-[#FFAA00] font-medium text-[14px]">授权方式修改后，团队成员授权环境将全部清空，您需要重新分配环境
                                </div>
                            </div>
                            <RadioGroup defaultValue="option-one">
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-one" id="option-one"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-one" class="font-[600]">分组授权 <span
                                            class="text-gray-400">&nbsp;只能通过分组维度分配给您的团队成员，团队成员拥有对应的分组环境</span></Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-two" id="option-two"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-two" class="font-[600]">环境授权 <span
                                            class="text-gray-400">&nbsp;只能通过环境维度分配给团队成员，团队成员拥有对应的环境</span></Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>
                </div>

                <!-- 账户安全 -->
                <div v-else-if="activeItem === 'account'">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">账户安全</h1>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="bg-white px-4 py-2 space-y-3">
                            <div class="w-full flex items-center justify-between">
                                <div class="flex items-center"><span class="font-[600]">双重身份验证（2FA）</span>
                                    <div class="text-[#FFAA00] font-medium text-[14px]">
                                        异常登录时，需通过身份验证器/邮箱进行身份验证。V2.8.0及以上版本生效。</div>
                                </div>
                                <label class="relative inline-flex cursor-pointer items-center">
                                    <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                    <div
                                        class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                    </div>
                                </label>
                            </div>
                            <RadioGroup defaultValue="option-one">
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-one" id="option-one"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-one" class="font-[600]">验证等级：低 <span
                                            class="text-gray-400">&nbsp;更换设备或距上次双重验证超过90天时登录需通过双重验证</span></Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-two" id="option-two"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-two" class="font-[600] flex items-center">
                                        验证等级：中 <span class="text-gray-400">&nbsp;更换设备或距上次双重验证超过30天时登录需通过双重验证</span>
                                        <img :src="Recommend" alt="" />
                                    </Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-three" id="option-three"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-three" class="font-[600] flex items-center">验证等级：高 <span
                                            class="text-gray-400">&nbsp;每次登录需通过双重验证</span>
                                        <img :src="Recommend" alt="" />
                                    </Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>
                </div>

                <!-- IP查询渠道 -->
                <div v-else-if="activeItem === 'ip'">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-3 text-[19px]">IP查询渠道</h1>
                    <div class="mb-4">
                        <p class="text-gray-400 text-[15px]">1.设置默认的IP查询渠道，也可在代理信息中再行修改。</p>
                        <p class="text-gray-400 text-[15px]">
                            2.不同的「IP查询渠道」可能会存在一定的检测结果差异。若代理检测结果与实际信息不符，可通过调整「IP查询渠道」以确保检测结果的准确性。</p>
                    </div>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="bg-white px-4 py-2 space-y-3">
                            <div class="w-full flex items-center">
                                <span class="font-[600]">清除云端所有缓存</span>
                                <div class="text-[#FFAA00] font-medium text-[14px]">授权方式修改后，团队成员授权环境将全部清空，您需要重新分配环境
                                </div>
                            </div>
                            <RadioGroup defaultValue="option-one">
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-one" id="option-one"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-one" class="font-[600]">IP2Location</Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-two" id="option-two"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-two" class="font-[600]">MaxMind</Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-three" id="option-three"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-three" class="font-[600]">DB-IP</Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-four" id="option-four"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-four" class="font-[600]">IP-API</Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>
                </div>

                <!-- 访问控制 -->
                <div v-else-if="activeItem === 'access'" class="">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">访问控制</h1>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="flex items-center justify-between bg-white p-4">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">本地访问</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    开启后，可配置使用本地网络访问的特定网址
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- 浏览器启动页 -->
                <div v-else-if="activeItem === 'startup'">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-3 text-[19px]">浏览器启动页</h1>
                    <div class="mb-4">
                        <p class="text-gray-400 text-[15px]">设置浏览器环境启动页的预设选项，可在环境中修改</p>
                    </div>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="bg-white px-4 py-2 space-y-3">
                            <div class="w-full flex items-center">
                                <span class="font-[600]">启动后</span>
                            </div>
                            <RadioGroup defaultValue="option-one">
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-one" id="option-one"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-one" class="font-[600]">继续浏览上次打开的网页</Label>
                                </div>
                                <div class="flex items-center space-x-2">
                                    <RadioGroupItem value="option-two" id="option-two"
                                        class="text-[#5050FA] border-gray-400 data-[state=checked]:border-[#5050FA]" />
                                    <Label htmlFor="option-two" class="font-[600]">打开指定网页</Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>
                </div>

                <!-- 网页翻译 -->
                <div v-else-if="activeItem === 'translation'" class="">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-6 text-[19px]">网页翻译</h1>

                    <div class="rounded-lg p-6 bg-gray-50">
                        <div class="flex items-center justify-between bg-white p-4">
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">Google 网页翻译提示</h3>
                                <p class="mt-1 text-sm text-gray-500">
                                    开启后，访问非环境所用语言时会询问是否翻译
                                </p>
                            </div>
                            <label class="relative inline-flex cursor-pointer items-center">
                                <input type="checkbox" v-model="globeSwitch.isLocked" class="peer sr-only">
                                <div
                                    class="h-5 w-9 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-500 peer-checked:after:translate-x-full">
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <div v-else>
                    <h1 class="text-2xl font-semibold text-gray-900">{{ getCurrentTitle }}</h1>
                </div>
            </main>
        </div>
    </layout>
    <!-- 创建密钥 -->
    <Model title="创建密钥" :open="postLockModel" @close="postLockModel = false" class="min-w-[650px]">
        <!-- 使用 transition 包裹内容 -->
        <transition name="fade" mode="out-in">
            <!-- 查看密钥 -->
            <div v-if="lookOrCreateKey === 'look'" class="flex flex-col items-center justify-center py-4 px-6 space-y-2"
                key="look">
                <ContentUnavailableIcon class="size-[150px]" />
                <p class="font-[600]">独一无二的端对端加密</p>
                <p class="text-[13px] text-center">根据你的设备信息，我们会生成一个只有你知道的密钥，用来加密你在浏览器里的数据，第三方和环境模拟器都无法解密哦！</p>
            </div>

            <!-- 生成密钥 -->
            <div v-else class="flex flex-col items-center justify-center py-4 px-6 space-y-4" key="create">
                <p class="flex items-center gap-2 border shadow-xl rounded-lg p-2 w-full mb-3">
                    <InfoCircleIcon class="size-5" /> 我们不会保存你的密钥，也帮不了你找回它，所以记得自己好好保管哦！
                </p>
                <p class="font-[600] text-lg">保存您的密钥以避免无法访问加密的环境</p>
                <div
                    class="bg-[#E4E9F2] rounded-md flex flex-col items-center justify-center space-y-3 px-9 py-6 w-full">
                    <p class="font-[600]">{{ keyVal }}</p>
                    <button
                        class="px-6 py-2 border rounded-md flex items-center gap-3 bg-white text-[#5050FA] hover:bg-gray-50">
                        <CopyIcon class="size-5" />复制密钥
                    </button>
                </div>
            </div>
        </transition>

        <!-- Footer -->
        <div class="px-6 py-2 border-t flex justify-end space-x-2 items-center">
            <CancelButton @click="() => { postLockModel = false, lookOrCreateKey = 'look' }">取消</CancelButton>
            <PrimaryButton v-if="lookOrCreateKey === 'look'" @click="lookOrCreateKey = 'create'" class="h-[40px]">
                生成密钥
            </PrimaryButton>
            <PrimaryButton v-else @click="subSave" class="h-[40px]">
                我已保存
            </PrimaryButton>
        </div>
    </Model>
</template>

<script setup>
import { reactive, ref, computed } from 'vue'
import layout from './layout.vue'
import { KeyIcon } from '@/assets/icons/system-operation'
import Model from '@/components/model/model.vue'
import CancelButton from '@/components/button/cancel-button.vue'
import PrimaryButton from '@/components/button/primary-button.vue'
import { ContentUnavailableIcon, InfoCircleIcon, CopyIcon, ClearIcon, RecomIcon } from '@/assets/icons/system-operation/index'
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Checkbox } from "@/components/ui/checkbox"
import Recommend from "@/assets/icons/system-operation/recommend.png"

const menuItems = [
    { id: 'security', name: '安全锁' },
    { id: 'encryption', name: '环境端对端加密' },
    { id: 'data-protection', name: '扩展数据保护' },
    { id: 'data-sync', name: '数据同步' },
    { id: 'cache', name: '清除云端缓存' },
    { id: 'permissions', name: '授权方式' },
    { id: 'account', name: '账户安全' },
    { id: 'ip', name: 'IP查询渠道' },
    { id: 'access', name: '访问控制' },
    { id: 'startup', name: '浏览器启动页' },
    { id: 'translation', name: '网页翻译' }
]
const keyVal = ref('mC6env-49pwnN-cYWXzu-tko6fR-bKnZBD-s29MNb')
const lookOrCreateKey = ref('look')

const activeItem = ref('security')
const globeSwitch = ref({
    isLocked: true
})

const postLockModel = ref(false)

const getCurrentTitle = computed(() => {
    const item = menuItems.find(item => item.id === activeItem.value)
    return item ? item.name : ''
})

const subSave = () => {
    postLockModel.value = false
    setTimeout(() => {
        lookOrCreateKey.value = 'look'
    }, 100)
}
</script>

<style scoped>
/* 定义渐变动画 */
.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}
</style>