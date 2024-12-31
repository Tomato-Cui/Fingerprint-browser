import { listen } from '@tauri-apps/api/event'

export default {
    install: async (_app: any) => {
        await listen('init', (_event) => {
            localStorage.removeItem('browser-status');
        })
    }
}
