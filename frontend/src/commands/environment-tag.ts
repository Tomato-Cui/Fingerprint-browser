import { invoke } from '@tauri-apps/api/core';

// 定义环境标签接口
export interface EnvironmentTag {
    name: string;
    description?: string;
}

// 根据 ID 查询环境标签
export const environment_tag_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_tag_query_id', { id });
};

// 分页查询环境标签
export const environment_tag_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_tag_query', { pageNum, pageSize });
};

// 创建环境标签
export const environment_tag_create = async (
    name: string,
    description?: string
): Promise<any> => {
    return await invoke('environment_tag_create', { name, description });
};

// 修改环境标签
export const environment_tag_modify = async (
    id: number,
    name: string,
    description?: string
): Promise<any> => {
    return await invoke('environment_tag_modify', { id, name, description });
};

// 删除环境标签
export const environment_tag_delete = async (id: number): Promise<any> => {
    return await invoke('environment_tag_delete', { id });
};