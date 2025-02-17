import { invoke } from '@tauri-apps/api/core'

// 根据环境UUID查询回收站中的环境
export const environment_trash_query_id = async (environmentUuid: String): Promise<any> => {
    return await invoke('environment_trash_query_id', { environmentUuid })
};

// 分页查询回收站中的环境列表
export const environment_trash_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_trash_query', { pageNum, pageSize })
};

// 恢复单个环境
export const environment_trash_recover = async (environmentUuid: string): Promise<any> => {
    return await invoke('environment_trash_recover', { environmentUuid })
};

// 批量恢复多个环境
export const environment_trash_recovers = async (environmentUuids: String[]): Promise<any> => {
    return await invoke('environment_trash_recovers', { environmentUuids })
};

// 恢复回收站中的所有环境
export const environment_trash_recover_all = async (): Promise<any> => {
    return await invoke('environment_trash_recover_all', {})
};

// 批量删除回收站中的环境
export const environment_trash_delete_batch = async (environmentUuids: String[]): Promise<any> => {
    return await invoke('environment_trash_delete_batch', { environmentUuids })
};

// 清空回收站
export const environment_trash_clean = async (
): Promise<any> => {
    return await invoke('environment_trash_clean', {})
};