import { invoke } from '@tauri-apps/api/core'

export const opeartion_query_by_team = async (
    teamId: number,
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('opeartion_query_by_team', {
        teamId,
        pageNum,
        pageSize
    })
};

export const opeartion_query = async (
    pageNum: number,
    pageSize: number,
): Promise<any> => {
    return await invoke('opeartion_query', {
        pageNum,
        pageSize
    })
};
