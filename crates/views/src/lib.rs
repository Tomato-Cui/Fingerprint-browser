use std::sync::Arc;

use tauri::Manager;

pub mod components;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    let app_handle = Arc::new(app.handle().clone());

    tauri::async_runtime::spawn({
        let handle = app_handle.clone();
        async move {
            let cache_dir = handle.path().cache_dir().unwrap();
            cores::state::init_state(cores::state::State {
                app_cache_location: cache_dir,
            })
            .await;
        }
    });

    // 如果在调试模式下，初始化日志插件
    if cfg!(debug_assertions) {
        app_handle
            .plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )
            .unwrap();
    }

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
