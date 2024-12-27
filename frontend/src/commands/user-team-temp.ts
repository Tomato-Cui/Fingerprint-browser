import { invoke } from '@tauri-apps/api/core'

export const user_receive_query = async (
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('user_receive_query', {
        pageNum,
        pageSize
    })
};

export const team_receive_query = async (teamId: number, pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('team_receive_query', { teamId, pageNum, pageSize })
};

export const team_send = async (
    teamId: number,
    userUuid: String,
    description: String,
): Promise<any> => {
    return await invoke('team_send', {
        teamId,
        userUuid,
        description,
    })
};

export const user_send = async (
    teamId: number,
    description: String,
): Promise<any> => {
    return await invoke('user_send', {
        teamId,
        description,
    })
};

export const team_allow = async (
    id: number,
    userUuid: String,
    teamId: number,
): Promise<any> => {
    return await invoke('team_allow', {
        id,
        userUuid,
        teamId,
    })
};

export const user_allow = async (id: number,
    teamId: number,
): Promise<any> => {
    return await invoke('user_allow', { id, teamId })
};