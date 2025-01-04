import { listen } from '@tauri-apps/api/event'
import { init_command, init_porcessor } from '@/commands/index'
import { useBrowserStatusStore } from '@/stores/browser'

export default {
    install: async (_app: any) => {
        const browserStore = useBrowserStatusStore();

        init_command().then(res => console.log(res));
        init_porcessor().then(res => console.log(res));

        await listen('init_command', (res) => console.log(res));
        await listen('environment_close', (res) => {
            let environment_uuid = res.payload as string;
            browserStore.updateStatus(environment_uuid, false)
        });

        // document.addEventListener('contextmenu', (e) => {
        //     e.preventDefault();
        //     return false;
        // }, { capture: true });

        // document.addEventListener('keydown', (e) => {
        //     if (e.key === 'F12') {
        //         e.preventDefault();
        //         return false;
        //     }
        //     if (e.key === 'F5') {
        //         e.preventDefault();
        //         return false;
        //     }

        //     if (e.ctrlKey && e.shiftKey && e.key === 'I') {
        //         e.preventDefault();
        //         return false;
        //     }
        // }, { capture: true });
    }
}
