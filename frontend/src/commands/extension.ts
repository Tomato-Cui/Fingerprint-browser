import { invoke } from './index'

export interface Extension {
    uuid: string;                        // String => string
    name: string;                        // String => string
    description?: string | null;         // Option<String> => string or null
    avatar_url?: string | null;           // Option<String> => string or null
    release_url?: string | null;         // Option<String> => string or null
    size?: number | null;                // Option<i32> => number or null
    all_can_use?: number | null;         // Option<i32> => number or null
}

export const extension_info_by_chrome_store_url = async (
    url: string,
): Promise<any> => {
    return await invoke('extension_info_by_chrome_store_url', { url })
};

export const extension_user_create = async (
    extension: Extension,
): Promise<any> => {
    return await invoke('extension_user_create', { extension })
};

export const extension_team_create = async (
    teamId: String,
    extension: Extension,
): Promise<any> => {
    return await invoke('extension_team_create', {
        teamId,
        extension
    })
};

export const extension_query_by_team = async (
    teamId: number,
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('extension_query_by_team', {
        teamId,
        pageNum,
        pageSize

    })
};

export const extension_query_by_user = async (
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('extension_query_by_user', {
        pageNum,
        pageSize,
    })
};

export const extension_query_by_environment = async (
    environmnetUuid: String,
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('extension_query_by_environment', {
        environmnetUuid,
        pageNum,
        pageSize,
    })
};

export const extension_query = async (
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('extension_query', {
        pageNum,
        pageSize
    })
};

export const extension_environmnet_use_extension = async (
    extensionUuid: String,
    environmentUuid: String,
): Promise<any> => {
    return await invoke('extension_environmnet_use_extension', {
        extensionUuid,
        environmentUuid
    })
};

export const extension_update = async (
    extensionUuid: string,
    extension: Extension
): Promise<any> => {
    return await invoke('extension_update', {
        extensionUuid,
        extension
    })
};

export const extension_delete_by_uuid = async (
    extensionUuid: string,
): Promise<any> => {
    return await invoke('extension_delete_by_uuid', {
        extensionUuid,
    })
};