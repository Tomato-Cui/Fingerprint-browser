
import { invoke } from '@tauri-apps/api/core';

interface Environment {
    ID?: number | null;                   // 自增ID, 可选的数字，可能为null
    name: string;                         // 环境名称
    description?: string | null;          // 环境描述，可能为null
    owner_id: string;                     // 所有者ID
    domain_name: string;                  // 账号平台的域名
    open_urls?: string | null;            // 其他URL，可能为null
    repeat_config?: string | null;        // 去重配置，可能为null
    username: string;                     // 账号
    password: string;                     // 密码
    fakey: string;                        // 2FA密钥
    cookie?: string | null;               // Cookie，可能为null
    ignore_cookie_error?: number | null;  // 校验Cookie失败时的行为，可能为null
    group_id?: number | null;             // 分组ID，可能为null
    fp_info_id?: number | null;           // 指纹信息ID，可能为null
    ua: string;                           // 用户代理
    os: string;                           // 操作系统
    country?: string | null;              // 国家/地区，可能为null
    region?: string | null;               // 省/州，可能为null
    city?: string | null;                 // 城市，可能为null
    remark?: string | null;               // 备注，可能为null
    ipchecker: string;                    // IP查询渠道
    sys_app_cate_id: string;              // 应用分类ID
    user_proxy_config?: string | null;    // 环境代理配置，可能为null
    proxy?: string | null;                // 代理IP，可能为null
    proxy_enable: number;                 // 代理启用
    is_tz: number;                        // 是否启用时区
    is_pos: number;                       // 是否启用地理位置
    user_data_file: string;               // 用户数据文件路径
    driver_location?: string | null;      // 浏览器驱动位置，可能为null
    status: number;                       // 浏览器状态
    created_at?: string | null;           // 创建时间，可能为null
    updated_at?: string | null;           // 更新时间，可能为null
    lasted_at?: string | null;            // 最近时间，可能为null
    deleted_at?: string | null;           // 删除时间，可能为null
}

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

interface Browser {
    environment: Environment,
    fingerprint: Fingerprint,
}


export const starts = async (browsers: Array<Browser>): Promise<any> => {
    let response = invoke && await invoke('starts', { browsers });
    return response
}

export const stops = async (browserIds: Array<number>): Promise<any> => {
    let response = invoke && await invoke('stops', { browserIds });
    return response
}

export const status = async (): Promise<any> => {
    let response = invoke && await invoke('status');
    return response
}

export default {
    starts,
    stops,
    status,
}
