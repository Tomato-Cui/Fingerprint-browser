import { invoke } from '@tauri-apps/api/core'

// 根据ID查询分组信息
export const group_query_id = async (): Promise<any> => {
    return await invoke('group_query_id', {})
};

// 分页查询分组列表
export const group_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('group_query', { pageNum, pageSize })
};

// 创建新分组
export const group_create = async (): Promise<any> => {
    return await invoke('group_create', {})
};

// 修改分组信息
export const group_modify = async (): Promise<any> => {
    return await invoke('group_modify', {})
};

// 授权用户访问分组
export const group_grant_user = async (): Promise<any> => {
    return await invoke('group_grant_user', {})
};

// 删除分组
export const group_delete = async (): Promise<any> => {
    return await invoke('group_delete', {})
};