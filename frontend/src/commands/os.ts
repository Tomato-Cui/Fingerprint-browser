import { invoke } from './index'

export const platform = async (): Promise<String> => {
    return await invoke('platfrom',)
}