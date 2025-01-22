
import { invoke } from '@tauri-apps/api/core'

export interface EnvironmentFingerprint {
    browser: string;              // 浏览器
    ua: string;                   // 自定义UA
    os: string;                   // 操作系统
    country?: string;             // 国家
    region?: string;              // 省/州
    city?: string;                // 城市
    language_type: number;        // 语言类型 0-跟随IP，1-自定义，2-跟随电脑
    languages: string;            // 渲染语言
    gmt: string;                  // 时区
    geography: string;            // 地理
    geo_tips: number;             // 地理位置请求行为
    geo_rule: number;             // 地理位置规则
    longitude?: string;           // 自定义经度
    latitude?: string;            // 自定义纬度
    radius?: number;              // 自定义半径
    height?: number;              // 分辨率高
    width?: number;               // 分辨率宽
    fonts_type: number;           // 字体列表保护 0-隐私，1-真实
    fonts?: string;               // 字体列表
    font_fingerprint: number;     // 字体指纹
    web_rtc: number;              // WebRTC配置
    web_rtc_local_ip?: string;    // 内网IP
    canvas: number;               // Canvas隐私保护
    webgl: number;                // WebGL隐私保护
    hardware_acceleration: number;// 硬件加速
    webgl_info: number;           // WebGL信息
    audio_context: number;        // AudioContext隐私保护
    speech_voices: number;        // SpeechVoices
    media: number;                // 媒体设备隐私保护
    cpu: number;                  // CPU
    memory: number;               // 内存
    do_not_track: number;         // 追踪设置
    battery: number;              // 电池隐私保护
    port_scan: number;            // 本地端口扫码
    white_list?: string;          // 端口白名单
}

export const environment_fingerprint_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_fingerprint_query_id', { id })
};

export const environment_fingerprint_query = async (
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('environment_fingerprint_query ', {
        pageNum,
        pageSize
    })
};

export const environment_fingerprint_create = async (payload:
    EnvironmentFingerprint
): Promise<any> => {
    return await invoke('environment_fingerprint_create', { payload })
};

export const environment_fingerprint_modify = async (id: number, payload: EnvironmentFingerprint): Promise<any> => {
    return await invoke('environment_fingerprint_modify', { id, payload })
};

export const environment_fingerprint_modify_ua = async (id: number, ua: string): Promise<any> => {
    return await invoke('environment_fingerprint_modify_ua', { id, ua })
};

export const environment_fingerprint_modify_by_colname = async (id: number, colName: string, colValue: string): Promise<any> => {
    return await invoke('environment_fingerprint_modify_by_colname', { id, colName, colValue })
};

export const environment_fingerprint_delete = async (id: number): Promise<any> => {
    return await invoke('environment_fingerprint_delete', { id })
};