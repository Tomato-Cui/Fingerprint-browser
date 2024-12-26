import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const teams = ref([
    { id: 1, name: '管理员', description: '123' },
    { id: 2, name: '编辑权限', description: '123'},
    { id: 3, name: '查看权限', description: '123' },
]);


export const team_create = async (name: string, description: string): Promise<any> => {
    let last_team = teams.value[teams.value.length - 1];
    teams.value.push({ id: last_team.id, name, description })
};

export const team_query_id = async (id: number): Promise<any> => {
    return teams.value.find(item => item.id = id)
};

export const team_query = async (pageNum: number, pageSize: number): Promise<any> => {
    // console.log("team_query:", teams.value.splice(0, 5));
    
    return teams.value.splice(0, 5)
};

export const team_update = async (id: number, name: string, description: string): Promise<any> => {
    console.log("team_update:", id, name, description);
    
    teams.value[id] = { id, name, description }
};

export const team_delete = async (id: number): Promise<any> => {
    teams.value = teams.value.filter(item => item.id != id)
    // return await invoke('environment_delete', { id })
};
