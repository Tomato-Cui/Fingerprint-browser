
export default {
    install: async (_app: any) => {
        localStorage.removeItem('browser-status');
        document.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            return false;
        }, { capture: true });

        document.addEventListener('keydown', (e) => {
            if (e.key === 'F12') {
                e.preventDefault();
                return false;
            }

            if (e.ctrlKey && e.shiftKey && e.key === 'I') {
                e.preventDefault();
                return false;
            }
        }, { capture: true });
    }
}
