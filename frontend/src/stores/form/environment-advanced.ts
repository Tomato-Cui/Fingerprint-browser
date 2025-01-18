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
    kind: string,
    host: string,
    port: string,
    username: string,
    password: string,
}

export interface AccountForm {
    platform: string,
    platform_url: string,
    platform_account: string,
    platform_password: string,
    platform_description?: string,
}

export interface settingForm {
    startedBrowserDefaultUrl: '1' | '0', // 1: 继续上次，0：打开指定
    startedBrowserAccountUrlIsOpen: boolean, // 是否打开配置账号平台设置的url
    startedBrowserStayCheckHome: boolean, // 是否停留在检测页
}

export interface FingerprintForm {
    timezone: RadioInterface<'ip' | 'custom'>;
    webRTC: RadioInterface<'real' | 'random' | 'manual' | 'disabled'>;
    geolocation: RadioInterface<boolean>;
    language: RadioInterface<'match' | 'custom'>;
    resolution: RadioInterface<'real' | 'custom'>;
    fonts: RadioInterface<'real' | 'custom'>;
    canvas: RadioInterface<'noise' | 'real'>;
    webGL: RadioInterface<'noise' | 'real'>;
    webGPU: RadioInterface<'real' | 'disabled'>;
    audioContext: RadioInterface<'noise' | 'real'>;
    mediaDevices: RadioInterface<'noise' | 'real'>;
    clientRects: RadioInterface<'noise' | 'real'>;
    speechVoices: RadioInterface<'random' | 'real'>;
    hardwareConcurrency: RadioInterface<string>;
    deviceMemory: RadioInterface<string>;
    deviceName: RadioInterface<'custom' | 'real'>;
    macAddress: RadioInterface<'custom' | 'real'>;
    doNotTrack: RadioInterface<boolean>;
    bluetooth: RadioInterface<'random' | 'real'>;
    battery: RadioInterface<'random' | 'real'>;
    portScanning: RadioInterface<boolean>;
    startupParams: RadioInterface<string>;
}

export interface FingerprintParams {
    timezone: 'ip' | 'custom';
    webRTC: 'real' | 'random' | 'manual' | 'disabled';
    geolocation: boolean;
    language: 'match' | 'custom';
    resolution: 'real' | 'custom';
    fonts: 'real' | 'custom';
    canvas: 'noise' | 'real';
    webGL: 'noise' | 'real';
    webGPU: 'real' | 'disabled';
    audioContext: 'noise' | 'real';
    mediaDevices: 'noise' | 'real';
    clientRects: 'noise' | 'real';
    speechVoices: 'random' | 'real';
    hardwareConcurrency: string;
    deviceMemory: string;
    deviceName: 'custom' | 'real';
    macAddress: 'custom' | 'real';
    doNotTrack: boolean;
    bluetooth: 'random' | 'real';
    battery: 'random' | 'real';
    portScanning: boolean;
    startupParams: string;
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
    account: AccountForm,
    fingerprint: FingerprintForm,
    cookie: string,
    setting: settingForm,
}

export const useEnvironmentAdvancedFormStore = defineStore('environment-advanced-form-store ', () => {
    const forms = ref<EnvironmentAdvancedForm>({
        name: '',
        description: '',
        browser: { value: 'chrome' },
        ua: generateRandomUserAgent(),
        os: { value: 'Windows' },
        encryPtSwitch: { value: "关闭" },
        numbers: 1,
        group: 0,
        tag: 0,
        use_proxy: false,
        proxy: {
            kind: 'NO Proxy (本地直连)',
            host: '',
            port: '',
            username: '',
            password: '',
        },
        account: {
            platform: '',
            platform_url: '',
            platform_account: '',
            platform_password: '',
            platform_description: '',
        },
        cookie: '',
        setting: {
            startedBrowserDefaultUrl: '0',
            startedBrowserAccountUrlIsOpen: false,
            startedBrowserStayCheckHome: false,
        },
        fingerprint: defaultFingerPrint as any,
    });

    const getFingerprintData = () => {
        let fingerprint = forms.value.fingerprint as any;
        const extractedValues = Object.keys(fingerprint).reduce((acc: any, key: any) => {
            acc[key] = (fingerprint[key]).value;
            return acc;
        }, {});

        return extractedValues;
    }

    const defaultFingerprintHandle = () => {
        forms.value.fingerprint = { ...defaultFingerPrint as any }
    }

    const getData = () => {
        return flattenFormsAfterCK({
            ...forms.value,
            fingerprint: getFingerprintData(),
        });
    }

    const randomUA = () => {
        forms.value.ua = generateRandomUserAgent();
    }

    return {
        forms, randomUA, getData, defaultFingerprintHandle,
    }
});

const defaultFingerPrint = {
    timezone: {
        value: 'ip', // value 的类型是 'ip' | 'custom'
    },
    webRTC: {
        value: 'random', // value 的类型是 'real' | 'random' | 'manual' | 'disabled'
    },
    geolocation: {
        value: false, // value 的类型是 boolean
    },
    language: {
        value: 'match', // value 的类型是 'match' | 'custom'
    },
    resolution: {
        value: 'real', // value 的类型是 'real' | 'custom'
    },
    fonts: {
        value: 'real', // value 的类型是 'real' | 'custom'
    },
    canvas: {
        value: 'noise', // value 的类型是 'noise' | 'real'
    },
    webGL: {
        value: 'noise',
    },
    webGPU: {
        value: 'real', // value 的类型是 'real' | 'disabled'
    },
    audioContext: {
        value: 'noise', // value 的类型是 'noise' | 'real'
    },
    mediaDevices: {
        value: 'noise', // value 的类型是 'noise' | 'real'
    },
    clientRects: {
        value: 'noise', // value 的类型是 'noise' | 'real'
    },
    speechVoices: {
        value: 'random', // value 的类型是 'random' | 'real'
    },
    hardwareConcurrency: {
        value: '1', // value 的类型是 number
    },
    deviceMemory: {
        value: '1', // value 的类型是 '8GB' | '4GB' | '2GB' | '1GB'
    },
    deviceName: {
        value: 'real', // value 的类型是 'custom' | 'real'
    },
    macAddress: {
        value: 'real', // value 的类型是 'custom' | 'real'
    },
    doNotTrack: {
        value: true, // value 的类型是 boolean
    },
    bluetooth: {
        value: 'random', // value 的类型是 'random' | 'real'
    },
    battery: {
        value: 'random', // value 的类型是 'random' | 'real'
    },
    portScanning: {
        value: true, // value 的类型是 boolean
    },
    startupParams: {
        value: '', // value 的类型是 string
    },
};

export const templaeFormtData = {
    name: { placeholder: '请输入环境名称', default: '新建环境' },
    description: { placeholder: '请输入环境备注信息', default: '该环境未添加备注信息' },
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
    platforms: [
        { id: 0, value: "Youtube", href: 'https://www.youtube.com/', avatar: 'https://cdn3.iconfinder.com/data/icons/social-network-30/512/social-06-1024.png' },
        { id: 1, value: "X", href: 'https://x.com/', avatar: 'data:image/png;base64,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' },
    ],
    cookie: { placeholder: '请输入cookie', default: '' },
    fingerprint: {
        timezones: [
            { id: 0, value: "ip", label: "匹配IP" },
            { id: 1, value: "custom", label: "自定义" },
        ],
        webRtc: [
            { id: 0, value: "manual", label: "转发" },
            { id: 1, value: "random", label: "隐私" },
            { id: 3, value: "real", label: "真实" },
            { id: 4, value: "disabled", label: "禁用" },
        ],
        geolocations: [
            { id: 0, value: true, label: "询问" },
            { id: 1, value: false, label: "禁用" },
        ],
        languages: [
            { id: 0, value: "match", label: "匹配IP" },
            { id: 1, value: "custom", label: "自定义" },
        ],
        resolutions: [
            { id: 0, value: "real", label: "真实" },
            { id: 1, value: "custom", label: "自定义" },
        ],
        fonts: [
            { id: 0, value: "real", label: "真实" },
            { id: 1, value: "custom", label: "自定义" },
        ],
        canvas: [
            { id: 0, value: "noise", label: "噪音" },
            { id: 1, value: "real", label: "真实" },
        ],
        webGL: [
            { id: 0, value: "noise", label: "噪音" },
            { id: 1, value: "real", label: "真实" },
        ],
        webGLModes: [
            { id: 0, value: "real", label: "真实" },
            { id: 1, value: "disabled", label: "关闭硬件加速" },
        ],
        webGPU: [
            { id: 0, value: "real", label: "真实" },
            { id: 1, value: "disabled", label: "禁用" },
            { id: 2, value: "webgl", label: "基于WebGL匹配" },
        ],
        audioContext: [
            { id: 0, value: "noise", label: "噪音" },
            { id: 1, value: "real", label: "真实" },
        ],
        mediaDevices: [
            { id: 0, value: "noise", label: "噪音" },
            { id: 1, value: "real", label: "真实" },
        ],
        clientRects: [
            { id: 0, value: "noise", label: "噪音" },
            { id: 1, value: "real", label: "真实" },
        ],
        speechVoices: [
            { id: 0, value: "random", label: "隐私" },
            { id: 1, value: "real", label: "真实" },
        ],
        hardwareConcurrency: [
            { id: 0, value: "1", label: "1" },
            { id: 1, value: "2", label: "2" },
            { id: 2, value: "4", label: "4" },
            { id: 3, value: "8", label: "8" },
            { id: 4, value: "16", label: "16" },
        ],
        deviceMemory: [
            { id: 0, value: "1", label: "1GB" },
            { id: 1, value: "2", label: "2GB" },
            { id: 2, value: "4", label: "4GB" },
            { id: 3, value: "8", label: "8GB" },
        ],
        deviceNames: [
            { id: 0, value: "custom", label: "自定义" },
            { id: 1, value: "real", label: "真实" },
        ],
        macAddresses: [
            { id: 0, value: "custom", label: "自定义" },
            { id: 1, value: "real", label: "真实" },
        ],
        doNotTrack: [
            { id: 0, value: true, label: "开启" },
            { id: 1, value: false, label: "关闭" },
        ],
        bluetooth: [
            { id: 0, value: "random", label: "隐私" },
            { id: 1, value: "real", label: "真实" },
        ],
        battery: [
            { id: 0, value: "random", label: "隐私" },
            { id: 1, value: "real", label: "真实" },
        ],
        portScanning: [
            { id: 0, value: true, label: "开启" },
            { id: 1, value: false, label: "关闭" },
        ],

    }
}

const valueToChineseMap = {
    ip: "匹配IP",
    custom: "自定义",
    manual: "转发",
    random: "隐私",
    real: "真实",
    disabled: "禁用",
    true: "启用",
    false: "禁用",
    noise: "噪音",
    webgl: "基于WebGL匹配",
} as any;


const keyToChineseMap = {
    name: '名称',
    description: '描述',
    browser: '浏览器',
    ua: '用户代理',
    os: '操作系统',
    encryPtSwitch: '加密开关',
    numbers: '数量',
    group: '分组',
    tag: '标签',
    use_proxy: '使用代理',
    proxy: '代理设置',
    account: '账号设置',
    cookie: 'Cookie',
    setting: '浏览器设置',
    fingerprint: '指纹设置',

    'browser.value': '浏览器',
    'os.value': '系统',
    'encryPtSwitch.value': '加密',
    'proxy.kind': '代理类型',
    'proxy.host': '代理主机',
    'proxy.port': '代理端口',
    'proxy.username': '代理用户名',
    'proxy.password': '代理密码',
    'account.platform': '平台名称',
    'account.platform_url': '平台URL',
    'account.platform_account': '平台账号',
    'account.platform_password': '平台密码',
    'account.platform_description': '平台描述',
    'setting.startedBrowserDefaultUrl': '默认URL',
    'setting.startedBrowserAccountUrlIsOpen': '账号URL开启',
    'setting.startedBrowserStayCheckHome': '停留检查页',

    'fingerprint.timezone': '时区',
    'fingerprint.webRTC': 'WebRTC',
    'fingerprint.geolocation': '地理位置',
    'fingerprint.language': '语言',
    'fingerprint.resolution': '分辨率',
    'fingerprint.fonts': '字体',
    'fingerprint.canvas': 'Canvas',
    'fingerprint.webGL': 'WebGL图像',
    'fingerprint.webGPU': 'WebGPU',
    'fingerprint.audioContext': '音频上下文',
    'fingerprint.mediaDevices': '媒体设备',
    'fingerprint.clientRects': '客户端矩形',
    'fingerprint.speechVoices': '语音合成',
    'fingerprint.hardwareConcurrency': '硬件并发数',
    'fingerprint.deviceMemory': '设备内存',
    'fingerprint.deviceName': '设备名称',
    'fingerprint.macAddress': 'MAC地址',
    'fingerprint.doNotTrack': '禁止追踪',
    'fingerprint.bluetooth': '蓝牙',
    'fingerprint.battery': '电池',
    'fingerprint.portScanning': '端口扫描保护',
    'fingerprint.startupParams': '启动参数'
} as any;

// 扁平化forms并映射key为中文
export const flattenFormsAfterCK = (obj: Record<string, any>, _prefix = ''): any => {
    const flattenForms = (obj: Record<string, any>, prefix = ''): Array<{ key: string; value: any }> => {
        return Object.keys(obj).reduce((acc, key) => {
            const fullKey = prefix ? `${prefix}.${key}` : key;
            if (typeof obj[key] === 'object' && obj[key] !== null && !Array.isArray(obj[key])) {
                // 递归处理嵌套对象
                acc.push(...flattenForms(obj[key], fullKey));
            } else {
                // 直接添加键值对
                acc.push({ key: fullKey, value: obj[key] });
            }
            return acc;
        }, [] as Array<{ key: string; value: any }>);
    };
    const flatForms = flattenForms(obj);

    let temResult = {} as any;
    flatForms.forEach((item: any) => {
        const chineseKey = keyToChineseMap[item.key] || item.key;

        if ((item.key as string).startsWith('fingerprint')) {
            const chineseValue = valueToChineseMap[item.value] || item.value;
            temResult[chineseKey] = chineseValue;
        } else {
            temResult[chineseKey] = item.value;
        }
    });

    return temResult;
};
