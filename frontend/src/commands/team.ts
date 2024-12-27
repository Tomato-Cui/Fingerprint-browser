import { invoke } from '@tauri-apps/api/core'

export const team_query_id = async (id: number): Promise<any> => {
    return await invoke('team_query_id', { id })
};

export const team_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('team_query', { pageNum, pageSize })
};


export const query_current_team_info = async (): Promise<any> => {
    return await invoke('query_current_team_info', {})
};

export const query_team_all_user = async (teamId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('query_team_all_user', { teamId, pageNum, pageSize })
};

export const query_team_all_blocked_user = async (teamId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('query_team_all_blocked_user', { teamId, pageNum, pageSize })
};

export const query_team_group_all_user = async (teamId: number, teamGroupId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('query_team_group_all_user', { teamId, teamGroupId, pageNum, pageSize })
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
