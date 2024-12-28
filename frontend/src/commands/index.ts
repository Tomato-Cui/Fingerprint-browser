import { invoke as core_invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core'

export function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
    return core_invoke(cmd, args, options)
}