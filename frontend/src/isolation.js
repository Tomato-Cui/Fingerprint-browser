
window.addEventListener('DOMContentLoaded', () => {
    console.log('Isolation application loaded');

    console.log(window.__TAURI__);
    
    window.__TAURI__.invoke = (cmd, args) => {
        console.log(`Intercepted command: ${cmd}`, args);

        // 在这里可以修改命令或参数
        // 例如，阻止某些命令的执行
        if (cmd === 'someSensitiveCommand') {
            console.warn('Blocked sensitive command');
            return Promise.reject('Blocked sensitive command');
        }

        // 调用原始的 Tauri API
        return window.__TAURI_ORIGINAL__.invoke(cmd, args);
    };
});