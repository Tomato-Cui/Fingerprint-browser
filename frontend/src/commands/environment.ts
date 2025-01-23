import { invoke } from '@tauri-apps/api/core'

export interface Environment {
    name: string;                    // 环境名称
    description?: string;            // 环境描述
    default_urls?: string;           // 默认打开网页
    proxy_enable: number;            // 代理启用
    team_id?: number;                 // 团队ID
    proxy_id?: number;               // 代理ID
    fp_info_id?: number;             // 指纹信息ID
    group_id?: number;               // 分组ID
}

export interface EnvironmentInfo {
    id: number;
    uuid?: string;
    user_uuid?: string;
    team_id?: number;
    proxy?: Proxy;
    fp_info: EnvironmentFingerprint;
    name: string;
    description?: string;
    default_urls?: string;
    proxy_enable: number;
}

export interface Proxy {
    id: number;
    kind: string;
    host: string;
    port: string;
    username?: string;
    password?: string;
    user_uuid?: string;
    environment_group_id?: number;
    created_at?: string;
    updated_at?: string;
    deleted_at?: string;
}

export interface EnvironmentFingerprint {
    id: number;
    user_uuid?: string;
    browser: string;
    ua: string;
    os: string;
    country?: string;
    region?: string;
    city?: string;
    language_type: number;
    languages: string;
    gmt: string;
    geography: string;
    geo_tips: number;
    geo_rule: number;
    longitude?: number;
    latitude?: number;
    radius?: number;
    height?: number;
    width?: number;
    fonts_type: number;
    fonts?: string;
    font_fingerprint: number;
    web_rtc: number;
    web_rtc_local_ip?: string;
    canvas: number;
    webgl: number;
    hardware_acceleration: number;
    webgl_info: number;
    audio_context: number;
    speech_voices: number;
    media: number;
    cpu: number;
    memory: number;
    do_not_track: number;
    battery: number;
    port_scan: number;
    white_list?: string;
    created_at?: string;
    updated_at?: string;
    deleted_at?: string;
}

export interface EnvironmentDetailWithAdvanceCreateRequest {
    uuid?: string;
    team_id?: number;
    proxy_id?: number;
    fp_info_id?: number;
    group_id?: number;
    tag_id?: number;
    name: string;
    description?: string;
    default_urls?: string;

    browser: string;                  // 浏览器信息
    ua: string;                       // 用户代理（User-Agent）
    os: string;                       // 操作系统信息
    country?: string;                 // 国家
    region?: string;                  // 地区
    city?: string;                    // 城市
    language?: string;                // 语言类型
    languages?: string;               // 语言列表
    timezone?: string;                // 时区
    geography?: string;               // 地理位置信息
    geo_tips?: string;                // 地理位置提示
    geo_rule?: string;                // 地理位置规则
    longitude?: number;               // 经度（数值类型，保留6位小数）
    latitude?: number;                // 纬度（数值类型，保留6位小数）
    radius?: number;                  // 半径（数值类型）
    height?: number;                  // 屏幕高度（整数类型）
    width?: number;                   // 屏幕宽度（整数类型）
    fonts?: string;                   // 字体列表
    web_rtc: boolean;                 // WebRTC 配置（布尔类型）
    web_rtc_local_ip?: string;        // WebRTC 本地IP（IP地址类型）
    canvas?: string;                  // Canvas 指纹
    webgl: boolean;                   // WebGL 配置（布尔类型）
    hardware_acceleration: boolean;   // 硬件加速（布尔类型）
    webgl_info?: string;              // WebGL 信息（JSON格式文本）
    audio_context: boolean;           // 音频上下文（布尔类型）
    speech_voices: boolean;           // 语音支持（布尔类型）
    media: boolean;                   // 媒体支持（布尔类型）
    cpu: number;                      // CPU 核心数（整数类型）
    memory: number;                   // 内存大小（整数类型，单位：GB）
    do_not_track: boolean;            // 是否启用 Do Not Track（布尔类型）
    battery: boolean;                 // 电池状态（布尔类型）
    port_scan: boolean;               // 端口扫描（布尔类型）

    proxy_kind?: string;
    proxy_host?: string;
    proxy_port?: string;
    proxy_username?: string;
    proxy_password?: string;
    proxy_user_uuid?: string;
    proxy_environment_group_id?: number;
}

export const environment_query_id = async (environmentUuid: string): Promise<any> => {
    return await invoke('environment_query_id', { environmentUuid })
};

export const environment_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query', { pageNum, pageSize })
};

export const environment_query_by_group = async (id: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_group', { id, pageNum, pageSize })
};

export const environment_query_by_team = async (id: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_team', { id, pageNum, pageSize })
};


export const environment_query_by_extension = async (extensionUuid: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_extension', { extensionUuid, pageNum, pageSize })
};

export const environment_detail_create = async (_payload: EnvironmentInfo): Promise<any> => {
};

export const environment_simple_create = async (
    browserType: string,
    osType: string,
    numbers: number,
    useEncrypt: boolean,
    groupId?: number,
): Promise<any> => {
    return await invoke('environment_simple_create', {
        browserType,
        osType,
        numbers,
        useEncrypt,
        groupId
    })
};

export const environment_advanced_create = async (
    numbers: number,
    useEncrypt: boolean,
    environment: EnvironmentDetailWithAdvanceCreateRequest,
): Promise<any> => {
    return await invoke('environment_advanced_create', {
        numbers, useEncrypt, environment
    })
};


export const environment_batch_create = async (_environmentNames: Array<String>): Promise<any> => {
}

export const environment_modify_info = async (
    environmentUuid: String, payload: EnvironmentInfo): Promise<any> => {
    return await invoke('environment_modify_info', { environmentUuid, payload })
};


export const environment_advanced_modify = async (environmentUuid: string, payload: EnvironmentDetailWithAdvanceCreateRequest): Promise<any> => {
    return await invoke('environment_advanced_modify', { environmentUuid, payload })
};

export const environment_modify_basic_info = async (environmentUuid: string, payload: Environment): Promise<any> => {
    return await invoke('environment_modify_basic_info', { environmentUuid, payload })
};

export const environment_move_to_group = async (
    environmentUuid: string,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_move_to_group', { environmentUuid, groupId })
};

export const environment_batch_move_to_group = async (
    environmentUuids: Array<string>,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_batch_move_to_group', {
        environmentUuids, groupId
    })
};

export const environment_batch_move_to_tag = async (
    environmentUuids: Array<string>,
    tagId: number,
): Promise<any> => {
    return await invoke('environment_batch_move_to_tag', {
        environmentUuids, tagId
    })
};

export const environment_delete = async (environmentUuid: String,): Promise<any> => {
    return await invoke('environment_delete', { environmentUuid })
};

export const environment_batch_delete = async (
    environmentUuids: Array<string>,
): Promise<any> => {
    return await invoke('environment_batch_delete', { environmentUuids })
};