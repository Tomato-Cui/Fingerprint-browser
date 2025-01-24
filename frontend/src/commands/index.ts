import { invoke as core_invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core'

export function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
    return core_invoke(cmd, args, options)
}
export const ip_info = async (
    kind: string,
    host: string,
    port: string,
    username?: string,
    password?: string,

): Promise<any> => {
    return await invoke('ip_info',
        {
            kind, host, port, username, password
        }
    )
}

export const init_command = async (): Promise<any> => {
    return await invoke('init_command')
}

export const init_porcessor = async (): Promise<any> => {
    return await invoke('init_porcessor')
}

export const set_theme = async (theme: 'dark' | 'light' | 'auto'): Promise<any> => {
    return await invoke('plugin:theme|set_theme', { theme })
}

export const fetch_update = async (): Promise<any> => {
    return await invoke('fetch_update')
}
export const install_update = async (): Promise<any> => {
    return await invoke('install_update')
}