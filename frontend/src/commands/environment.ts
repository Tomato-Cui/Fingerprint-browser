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
    user_uuid?: number;
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
    longitude?: string;
    latitude?: string;
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

export const environment_detail_create = async (payload: EnvironmentInfo): Promise<any> => {
    return await invoke('environment_detail_create', { payload })
};

export const environment_create = async (environmentName: String): Promise<any> => {
    console.log(environmentName);

    return await invoke('environment_create', { environmentName })
};

export const environment_batch_create = async (environmentNames: Array<String>): Promise<any> => {
    return await invoke('environment_batch_create', { environmentNames })
}

export const environment_modify_info = async (
    envirunmentUuid: String, payload: EnvironmentInfo): Promise<any> => {
    return await invoke('environment_modify_info ', { envirunmentUuid, payload })
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

export const environment_delete = async (environmentUuid: String,): Promise<any> => {
    return await invoke('environment_delete', { environmentUuid })
};

export const environment_batch_delete = async (
    environmentUuids: Array<string>,
): Promise<any> => {
    return await invoke('environment_batch_delete', { environmentUuids })
};