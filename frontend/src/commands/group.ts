import { invoke } from '@tauri-apps/api/core'

export const group_query_id = async (id: number): Promise<any> => {
    return await invoke('group_query_id', { id })
};

export const group_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('group_query', { pageNum, pageSize })
};

export const group_create = async (name: string, description?: string): Promise<any> => {
    return await invoke('group_create', { name, description })
};

export const group_modify = async (id: number, name: string, description?: string): Promise<any> => {
    return await invoke('group_modify', { id, name, description })
};

export const group_grant_user = async (group_id: number): Promise<any> => {
    return await invoke('group_grant_user', { id: group_id })
};

export const group_delete = async (group_id: number): Promise<any> => {
    return await invoke('group_delete', { id: group_id })
};