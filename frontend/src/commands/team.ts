import { invoke } from '@tauri-apps/api/core'

export const team_query_id = async (id: number): Promise<any> => {
    return await invoke('team_query_id', { id })
};

export const team_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('team_query', { pageNum, pageSize })
};

export const team_create = async (name: string, description: string): Promise<any> => {
    return await invoke('team_create', { name, description })
};

export const team_modify = async (
    id: number,
    name: String,
    description: String,
): Promise<any> => {
    return await invoke('team_modify', {
        id,
        name,
        description
    })
};

export const team_delete = async (id: number): Promise<any> => {
    return await invoke('team_delete', { id })
};
