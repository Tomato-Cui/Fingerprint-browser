import { invoke } from './index'

export const team_query_id = async (id: number): Promise<any> => {
    return await invoke('team_query_id', { id })
};

export const team_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('team_query', { pageNum, pageSize })
};

export const team_is_leader = async (teamId: number): Promise<any> => {
    return await invoke('team_is_leader', { teamId })
};

export const team_query_name = async (teamName: String): Promise<any> => {
    return await invoke('team_query_name', { teamName })
};


export const query_current_team_info = async (): Promise<any> => {
    return await invoke('query_current_team_info', {})
};

// 拉黑
export const blocked = async (
    currentUserUuid: string,
    teamId: number,
): Promise<any> => {
    return await invoke('blocked', {
        currentUserUuid,
        teamId,
    })
};

// 恢复
export const un_blocked = async (
    currentUserUuid: string,
    teamId: number,
): Promise<any> => {
    return await invoke('un_blocked', {
        currentUserUuid,
        teamId,
    })
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


export const switch_team = async (
    id: number,
): Promise<any> => {
    return await invoke('switch_team', {
        id,
    })
};

export const remove_current_user = async (
    currentUserUuid: string,
    teamId: number,
): Promise<any> => {
    return await invoke('remove_current_user', {
        currentUserUuid,
        teamId,
    })
};

export const team_modify_team_user_info = async (
    teamId: number,
    teamGroupId: number,
    currentUserUuid: string,
    description?: string,
): Promise<any> => {
    return await invoke('team_modify_team_user_info', {
        teamId,
        teamGroupId,
        currentUserUuid,
        description
    })
};


export const team_delete = async (id: number): Promise<any> => {
    return await invoke('team_delete', { id })
};
