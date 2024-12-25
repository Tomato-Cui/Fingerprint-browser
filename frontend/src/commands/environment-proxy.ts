import { invoke } from '@tauri-apps/api/core'

export const environment_proxies_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_proxies_query_id', { id })
};

export const environment_proxies_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_proxies_query', { pageNum, pageSize })
};

export const environment_proxies_create = async (
    kind: String,
    host: String,
    port: String,
    username?: String,
    password?: String,
): Promise<any> => {
    return await invoke('environment_proxies_create', {
        kind,
        host,
        port,
        username,
        password,
    })
};

export const environment_proxies_modify = async (
    kind: String,
    host: String,
    port: String,
    username?: String,
    password?: String,
): Promise<any> => {
    return await invoke('environment_proxies_modify', {
        kind,
        host,
        port,
        username,
        password,
    })
};

export const environment_proxies_delete = async (id: number): Promise<any> => {
    return await invoke('environment_proxies_delete', { id })
};