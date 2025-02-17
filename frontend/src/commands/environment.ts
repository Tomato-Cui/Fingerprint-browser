import { invoke } from '@tauri-apps/api/core'

// 环境基础信息接口
export interface Environment {
    name: string;                    // 环境名称
    description?: string;            // 环境描述
    default_urls?: string;           // 默认打开网页
    proxy_enable: number;            // 代理启用
    team_id?: number;                // 团队ID
    proxy_id?: number;               // 代理ID
    fp_info_id?: number;             // 指纹信息ID
    group_id?: number;               // 分组ID
}

// 环境详细信息接口
export interface EnvironmentInfo {
    id: number;                      // 环境ID
    uuid?: string;                   // 环境UUID
    user_uuid?: string;              // 用户UUID
    team_id?: number;                // 团队ID
    proxy?: Proxy;                   // 代理信息
    fp_info: EnvironmentFingerprint; // 指纹信息
    name: string;                    // 环境名称
    description?: string;            // 环境描述
    default_urls?: string;           // 默认打开网页
    proxy_enable: number;            // 代理启用状态
}

// 代理信息接口
export interface Proxy {
    id: number;                      // 代理ID
    kind: string;                    // 代理类型
    host: string;                    // 代理主机
    port: string;                    // 代理端口
    username?: string;               // 代理用户名
    password?: string;               // 代理密码
    user_uuid?: string;              // 用户UUID
    environment_group_id?: number;    // 环境组ID
    created_at?: string;             // 创建时间
    updated_at?: string;             // 更新时间
    deleted_at?: string;             // 删除时间
}

// 环境指纹信息接口
export interface EnvironmentFingerprint {
    id: number;                      // 指纹ID
    user_uuid?: string;              // 用户UUID
    browser: string;                 // 浏览器信息
    ua: string;                      // 用户代理
    os: string;                      // 操作系统
    country?: string;                // 国家
    region?: string;                 // 地区
    city?: string;                   // 城市
    language_type: number;           // 语言类型
    languages: string;               // 语言列表
    gmt: string;                     // GMT时区
    geography: string;               // 地理位置
    geo_tips: number;                // 地理位置提示
    geo_rule: number;                // 地理位置规则
    longitude?: number;              // 经度
    latitude?: number;               // 纬度
    radius?: number;                 // 半径
    height?: number;                 // 屏幕高度
    width?: number;                  // 屏幕宽度
    fonts_type: number;              // 字体类型
    fonts?: string;                  // 字体列表
    font_fingerprint: number;        // 字体指纹
    web_rtc: number;                 // WebRTC配置
    web_rtc_local_ip?: string;       // WebRTC本地IP
    canvas: number;                  // Canvas指纹
    webgl: number;                   // WebGL配置
    hardware_acceleration: number;    // 硬件加速
    webgl_info: number;              // WebGL信息
    audio_context: number;           // 音频上下文
    speech_voices: number;           // 语音支持
    media: number;                   // 媒体支持
    cpu: number;                     // CPU核心数
    memory: number;                  // 内存大小
    do_not_track: number;            // Do Not Track
    battery: number;                 // 电池状态
    port_scan: number;               // 端口扫描
    white_list?: string;             // 白名单
    created_at?: string;             // 创建时间
    updated_at?: string;             // 更新时间
    deleted_at?: string;             // 删除时间
}

// 高级创建环境请求接口
export interface EnvironmentDetailWithAdvanceCreateRequest {
    id?: number;                     // 环境ID
    uuid?: string;                   // 环境UUID
    team_id?: number;                // 团队ID
    proxy_id?: number;               // 代理ID
    fp_info_id?: number;             // 指纹信息ID
    group_id?: number;               // 分组ID
    tag_id?: number;                 // 标签ID
    name: string;                    // 环境名称
    description?: string;            // 环境描述
    default_urls?: string;           // 默认打开网页

    browser: string;                 // 浏览器信息
    ua: string;                      // 用户代理
    os: string;                      // 操作系统
    country?: string;                // 国家
    region?: string;                 // 地区
    city?: string;                   // 城市
    language?: string;               // 语言类型
    languages?: string;              // 语言列表
    timezone?: string;               // 时区
    geography?: string;              // 地理位置信息
    geo_tips?: string;               // 地理位置提示
    geo_rule?: string;               // 地理位置规则
    longitude?: number;              // 经度
    latitude?: number;               // 纬度
    radius?: number;                 // 半径
    height?: number;                 // 屏幕高度
    width?: number;                  // 屏幕宽度
    fonts?: string;                  // 字体列表
    web_rtc: boolean;                // WebRTC配置
    web_rtc_local_ip?: string;       // WebRTC本地IP
    canvas?: string;                 // Canvas指纹
    webgl: boolean;                  // WebGL配置
    hardware_acceleration: boolean;   // 硬件加速
    webgl_info?: string;             // WebGL信息
    audio_context: boolean;          // 音频上下文
    speech_voices: boolean;          // 语音支持
    media: boolean;                  // 媒体支持
    cpu: number;                     // CPU核心数
    memory: number;                  // 内存大小
    do_not_track: boolean;           // Do Not Track
    battery: boolean;                // 电池状态
    port_scan: boolean;              // 端口扫描

    proxy_kind?: string;             // 代理类型
    proxy_host?: string;             // 代理主机
    proxy_port?: string;             // 代理端口
    proxy_username?: string;         // 代理用户名
    proxy_password?: string;         // 代理密码
    proxy_user_uuid?: string;        // 代理用户UUID
    proxy_environment_group_id?: number; // 代理环境组ID
}

// 根据环境UUID查询环境信息
export const environment_query_id = async (environmentUuid: string): Promise<any> => {
    return await invoke('environment_query_id', { environmentUuid })
};

// 分页查询环境列表
export const environment_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query', { pageNum, pageSize })
};

// 根据分组ID分页查询环境列表
export const environment_query_by_group = async (id: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_group', { id, pageNum, pageSize })
};

// 根据团队ID分页查询环境列表
export const environment_query_by_team = async (id: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_team', { id, pageNum, pageSize })
};

// 根据扩展UUID分页查询环境列表
export const environment_query_by_extension = async (extensionUuid: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_query_by_extension', { extensionUuid, pageNum, pageSize })
};

// 创建详细环境信息
export const environment_detail_create = async (_payload: EnvironmentInfo): Promise<any> => {
};

// 简单创建环境
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

// 高级创建环境
export const environment_advanced_create = async (
    numbers: number,
    useEncrypt: boolean,
    environment: EnvironmentDetailWithAdvanceCreateRequest,
): Promise<any> => {
    return await invoke('environment_advanced_create', {
        numbers, useEncrypt, environment
    })
};

// 批量创建环境
export const environment_batch_create = async (_environmentNames: Array<String>): Promise<any> => {
}

// 修改环境信息
export const environment_modify_info = async (
    environmentUuid: String, payload: EnvironmentInfo): Promise<any> => {
    return await invoke('environment_modify_info', { environmentUuid, payload })
};

// 高级修改环境
export const environment_advanced_modify = async (environmentUuid: string, payload: EnvironmentDetailWithAdvanceCreateRequest): Promise<any> => {
    return await invoke('environment_advanced_modify', { environmentUuid, payload })
};

// 修改环境基本信息
export const environment_modify_basic_info = async (environmentUuid: string, payload: Environment): Promise<any> => {
    return await invoke('environment_modify_basic_info', { environmentUuid, payload })
};

// 修改环境默认URL
export const environment_modify_default_url = async (environmentUuid: string, defaultUrls: String): Promise<any> => {
    return await invoke('environment_modify_default_url', { environmentUuid, defaultUrls })
};

// 移动环境到指定分组
export const environment_move_to_group = async (
    environmentUuid: string,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_move_to_group', { environmentUuid, groupId })
};

// 批量移动环境到指定分组
export const environment_batch_move_to_group = async (
    environmentUuids: Array<string>,
    groupId: number,
): Promise<any> => {
    return await invoke('environment_batch_move_to_group', {
        environmentUuids, groupId
    })
};

// 批量移动环境到指定标签
export const environment_batch_move_to_tag = async (
    environmentUuids: Array<string>,
    tagId: number,
): Promise<any> => {
    return await invoke('environment_batch_move_to_tag', {
        environmentUuids, tagId
    })
};

// 删除环境
export const environment_delete = async (environmentUuid: String,): Promise<any> => {
    return await invoke('environment_delete', { environmentUuid })
};

// 批量删除环境
export const environment_batch_delete = async (
    environmentUuids: Array<string>,
): Promise<any> => {
    return await invoke('environment_batch_delete', { environmentUuids })
};