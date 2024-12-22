import { invoke } from '@tauri-apps/api/core'

export const group_query_id = async (): Promise<any> => {
    return await invoke('group_query_id', {})
};

export const group_query = async (): Promise<any> => {
    return await invoke('group_query', {})
};

export const group_create = async (): Promise<any> => {
    return await invoke('group_create', {})
};

export const group_modify = async (): Promise<any> => {
    return await invoke('group_modify', {})
};

export const group_grant_user = async (): Promise<any> => {
    return await invoke('group_grant_user', {})
};

export const group_delete = async (): Promise<any> => {
    return await invoke('group_delete', {})
};