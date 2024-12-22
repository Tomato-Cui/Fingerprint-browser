import { invoke } from '@tauri-apps/api/core'

export const proxies_query_id = async (): Promise<any> => {
    return await invoke('proxies_query_id', {})
};

export const proxies_query = async (): Promise<any> => {
    return await invoke('proxies_query', {})
};

export const proxies_create = async (): Promise<any> => {
    return await invoke('proxies_create', {})
};

export const proxies_modify = async (): Promise<any> => {
    return await invoke('proxies_modify', {})
};

export const proxies_delete = async (): Promise<any> => {
    return await invoke('proxies_delete', {})
};