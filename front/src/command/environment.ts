
import { invoke } from '@tauri-apps/api/core';

interface Fingerprint {
    id?: number | null;                    // 自增ID, 可选，可能为null
    ua_version: number;                    // UA版本范围102~124
    ua: string;                             // 自定义UA
    language_type: number;                 // 语言类型 0-跟随IP，1-自定义，2-跟随电脑
    languages: string;                     // 渲染语言
    gmt: string;                           // 时区
    geography: string;                     // 地理
    geo_tips: number;                      // 地理位置请求行为
    geo_rule: number;                      // 地理位置规则
    longitude?: string | null;             // 自定义经度，可能为null
    latitude?: string | null;              // 自定义纬度，可能为null
    radius?: number | null;                // 自定义半径，可能为null
    height?: number | null;                // 分辨率高，可能为null
    width?: number | null;                 // 分辨率宽，可能为null
    fonts_type: number;                    // 字体列表保护 0-隐私，1-真实
    fonts?: string | null;                 // 字体列表，可能为null
    font_fingerprint: number;              // 字体指纹
    web_rtc: number;                       // WebRTC配置
    web_rtc_local_ip?: string | null;      // 内网IP，可能为null
    canvas: number;                        // Canvas隐私保护
    webgl: number;                         // WebGL隐私保护
    hardware_acceleration: number;         // 硬件加速
    webgl_info: number;                    // WebGL信息
    audio_context: number;                 // AudioContext隐私保护
    speech_voices: number;                 // SpeechVoices
    media: number;                         // 媒体设备隐私保护
    cpu: number;                           // CPU
    memory: number;                        // 内存
    do_not_track: number;                  // 追踪设置
    battery: number;                       // 电池隐私保护
    port_scan: number;                     // 本地端口扫码
    white_list?: string | null;            // 端口白名单，可能为null
    created_at?: string | null;            // 创建时间，可能为null
    updated_at?: string | null;            // 更新时间，可能为null
    deleted_at?: string | null;            // 删除时间，可能为null
}

export interface Browser {
    environment_id: Number,
    fingerprint: Fingerprint,
}

export const default_fingerprint = {
    "id": 1,
    "ua_version": 105,
    "ua": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36",
    "language_type": 1,
    "languages": "en-US,en;q=0.9",
    "gmt": "GMT+8",
    "geography": "Asia/Shanghai",
    "geo_tips": 1,
    "geo_rule": 0,
    "longitude": "121.4737",
    "latitude": "31.2304",
    "radius": 50,
    "height": 1080,
    "width": 1920,
    "fonts_type": 1,
    "fonts": "Arial, Helvetica, sans-serif",
    "font_fingerprint": 1,
    "web_rtc": 3,
    "web_rtc_local_ip": "192.168.1.1",
    "canvas": 0,
    "webgl": 1,
    "hardware_acceleration": 1,
    "webgl_info": 0,
    "audio_context": 0,
    "speech_voices": 1,
    "media": 0,
    "cpu": 8,
    "memory": 8,
    "do_not_track": 1,
    "battery": 1,
    "port_scan": 0,
    "white_list": "192.168.1.1, 192.168.1.2",
    "created_at": "2024-01-01T10:00:00Z",
    "updated_at": "2024-01-10T12:00:00Z",
    "deleted_at": null
};

export const start = async (environment_id: Number, fingerprint: Fingerprint): Promise<any> => {
    let response = invoke && await invoke('starts', {
        environments: [
            {
                environment_id,
                fingerprint
            }
        ]
    });
    return response
}
export const starts = async (browsers: Browser[]): Promise<any> => {
    let response = invoke && await invoke('starts', {
        environments: browsers
    });
    return response
}

export const stops = async (envIds: Array<number>): Promise<any> => {
    let response = invoke && await invoke('stops', { envIds });
    return response
}

export const status = async (): Promise<any> => {
    let response = invoke && await invoke('status');
    return response
}
