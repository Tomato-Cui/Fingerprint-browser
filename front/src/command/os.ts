import { platform, type Platform } from '@tauri-apps/plugin-os';


// 获取系统类型
export const platform_str = platform();
