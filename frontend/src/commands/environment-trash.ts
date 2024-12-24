import { invoke } from '@tauri-apps/api/core'


export const environment_trash_query_id = async (id: number): Promise<any> => {
    return await invoke('environment_trash_query_id', { id })
};

export const environment_trash_query = async (pageNum: number, pageSize: number): Promise<any> => {
    return await invoke('environment_trash_query', { pageNum, pageSize })
};

export const environment_trash_recover = async (id: number): Promise<any> => {
    return await invoke('environment_trash_recover', { id })
};

export const environment_trash_recovers = async (environmentIds: number[]): Promise<any> => {
    return await invoke('environment_trash_recovers', { environmentIds })
};

export const environment_trash_recover_all = async (): Promise<any> => {
    return await invoke('environment_trash_recover_all', {})
};

export const environment_trash_delete_again = async (id: number): Promise<any> => {
    return await invoke('environment_trash_delete_again', { id })
<<<<<<< HEAD
=======
};

export const environment_trash_batch_delete_again = async (ids: number[]): Promise<any> => {
    return await invoke('environment_trash_batch_delete_again', { ids })
>>>>>>> c9947a3b90c9729c1c60118e94b6d6b3292c17d7
};

export const environment_trash_clean = async (
): Promise<any> => {
    return await invoke('environment_trash_clean', {})
};