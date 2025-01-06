import { invoke as core_invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core'

export function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
    return core_invoke(cmd, args, options)
}

export const ip_info = async (): Promise<any> => {
    return await invoke('ip_info')
}

export const init_command = async (): Promise<any> => {
    return await invoke('init_command')
}

export const init_porcessor = async (): Promise<any> => {
    return await invoke('init_porcessor')
}
