import { invoke } from './index'

export const team_group_query_id = async (id: number): Promise<any> => {
    return await invoke('team_group_query_id', { id })
};

export const team_group_query_all = async (teamId: number): Promise<any> => {
    return await invoke('team_group_query_all', { teamId })
};
;