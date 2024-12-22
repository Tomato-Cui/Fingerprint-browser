import { invoke } from '@tauri-apps/api/core'

export const fingerprint_query_id = async (): Promise<any> => {
    return await invoke('fingerprint_query_id', {})
};

export const fingerprint_query = async (): Promise<any> => {
    return await invoke('fingerprint_query', {})
};

export const fingerprint_create = async (): Promise<any> => {
    return await invoke('fingerprint_create', {})
};

export const fingerprint_modify = async (): Promise<any> => {
    return await invoke('fingerprint_modify', {})
};

export const fingerprint_delete = async (): Promise<any> => {
    return await invoke('fingerprint_delete', {})
};