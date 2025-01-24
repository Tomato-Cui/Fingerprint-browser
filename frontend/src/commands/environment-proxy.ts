import { invoke } from '@tauri-apps/api/core'

export interface Proxy {
    id?: number;
    kind: string;
    host: string;
    port: string;
    username?: string;
    password?: string;
    user_uuid?: string;
    environment_group_id?: number;
}

export const environment_proxies_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_proxies_query_id', { id })
};

export const environment_proxies_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_proxies_query', { pageNum, pageSize })
};

export const environment_proxies_query_by_group = async (proxyGroupId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_proxies_query_by_group', { proxyGroupId, pageNum, pageSize })
};

export const environment_proxies_create = async (payload: Proxy): Promise<any> => {
    return await invoke('environment_proxies_create', {
        payload
    })
};

export const environment_proxies_modify = async (payload: Proxy): Promise<any> => {
    return await invoke('environment_proxies_modify', { payload })
};

export const environment_modify_proxy = async (environmentUuid: string, payload: Proxy): Promise<any> => {
    return await invoke('environment_modify_proxy', { environmentUuid, payload })
};

export const environment_proxies_delete = async (id: number): Promise<any> => {
    return await invoke('environment_proxies_delete', { id })
};

export const environment_proxies_batch_delete = async (ids: number[]) => {
    return await invoke('environment_proxies_batch_delete', { ids })
}
