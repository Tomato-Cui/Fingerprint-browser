import { invoke } from '@tauri-apps/api/core'

export interface EnvironmentGroup {
    name: string;
    description?: string;
}

export const environment_group_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_group_query_id', { id })
};

export const environment_group_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_group_query', { pageNum, pageSize })
};

export const environment_group_create = async (
    name: string,
    description?: string
): Promise<any> => {
    return await invoke('environment_group_create', { name, description })
};

export const environment_group_modify = async (
    id: number,
    name: string,
    description?: string
): Promise<any> => {
    return await invoke('environment_group_modify', { id, name, description })
};

export const environment_group_delete = async (id: number): Promise<any> => {
    return await invoke('environment_group_delete', { id })
};
