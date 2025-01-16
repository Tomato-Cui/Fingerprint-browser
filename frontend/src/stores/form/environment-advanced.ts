import { defineStore } from "pinia";
import { generateRandomUserAgent } from '@/util/lib'
import { type RadioInterface } from '@/views/environment-creates/components/radio.vue'
import { ref } from "vue";
import {
    IconChrome,
    IconFrame,
    IconFrame1,
    IconFrame2,
    IconFrame3,
    IconFrame4,
} from "@/assets/icons/environment-creates/simple-create/index";


export interface ProxyForm {

}

export interface FingerprintForm {

}


export interface EnvironmentAdvancedForm {
    name: string;
    description: string;
    browser: RadioInterface;
    ua: string;
    os: RadioInterface;
    encryPtSwitch: RadioInterface,
    numbers: number,
    group: any,
    tag: any,
    use_proxy: boolean,
    proxy: ProxyForm,
    fingerprint: FingerprintForm,
}

export const useEnvironmentAdvancedFormStore = defineStore('environment-advanced-form-store ', () => {
    const forms = ref<EnvironmentAdvancedForm>({
        name: '新建环境',
        description: '该环境未添加备注',
        browser: { value: 'chrome' },
        ua: generateRandomUserAgent(),
        os: { value: 'Windows' },
        encryPtSwitch: { value: "关闭" },
        numbers: 1,
        group: 0,
        tag: 0,
        use_proxy: false,
        proxy: {},
        fingerprint: {},
    })

    return {
        forms
    }
});

export const templaeFormtData = {
    browserTypes: [
        { value: "chrome", icon: IconChrome },
        { value: "firefox", icon: IconFrame },
    ] as any[],
    osTypes: [
        { value: "Windows", icon: IconFrame1 },
        { value: "MacOs", icon: IconFrame2 },
        { value: "Android", icon: IconFrame3 },
        { value: "IOS", icon: IconFrame4 },
    ] as any[],
    encrypts: [{ value: "开启" }, { value: "关闭" }] as any[],
    groups: [{ id: 0, value: "未知分组" }] as any[],
    tags: [{ id: 0, value: "未知标签" }] as any[],
    proxy_methods: [
        { id: 0, value: "新代理" },
        { id: 1, value: "使用现有代理" },
    ],
    timezones: [
        { id: 0, value: "匹配IP" },
        { id: 1, value: "自定义" },
    ],
    webRtcs: [
        { id: 0, value: "转发" },
        { id: 1, value: "隐私" },
        { id: 2, value: "替换" },
        { id: 3, value: "真实" },
        { id: 4, value: "禁用" },
    ],
    geolocations: [
        { id: 0, value: "询问" },
        { id: 1, value: "禁用" },
    ],
    languages: [
        { id: 0, value: "匹配IP" },
        { id: 1, value: "自定义" },
    ],
    resolution: [
        { id: 0, value: "真实" },
        { id: 1, value: "自定义" },
    ],
    font: [
        { id: 0, value: "真实" },
        { id: 1, value: "自定义" },
    ],
    canvas: [
        { id: 0, value: "噪音" },
        { id: 1, value: "真实" },
    ],
    webGLI: [
        { id: 0, value: "噪音" },
        { id: 1, value: "真实" },
    ],
    webGLM: [
        { id: 0, value: "真实" },
        { id: 1, value: "关闭硬件加速" },
    ],
    webGPU: [
        { id: 0, value: "真实" },
        { id: 1, value: "禁用" },
        { id: 2, value: "基于WebGL匹配" },
    ],
    zz: [
        { id: 0, value: "噪音" },
        { id: 1, value: "真实" },
    ],
    yz: [
        { id: 0, value: "隐私" },
        { id: 1, value: "真实" },
    ],
    kg: [
        { id: 0, value: "开启" },
        { id: 1, value: "关闭" },
    ],
}
