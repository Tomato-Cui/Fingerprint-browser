use components::windows::tray;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use tauri::RunEvent;
use tauri::Window;
use tauri::WindowEvent;

pub mod command;
pub mod components;
pub mod response;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut ctx = tauri::generate_context!();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .invoke_handler(command::register_handles())
        .setup(|app| {
            register_plugins(app.app_handle());
            // regsister_update_processor(app.app_handle().clone());
            tray::menu(app)?;
            Ok(())
        })
        .on_window_event(window_event_handle)
        .build(ctx)
        .expect("error while building tauri application");

    app.run(run_event_handle);
}

fn window_event_handle(window: &Window, event: &WindowEvent) {
    match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window.hide();
        }
        _ => {}
    }
}

fn run_event_handle(_app_handle: &AppHandle, event: RunEvent) {
    match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}

fn register_plugins(app_handle: &AppHandle) {
    // Register the single instance plugin
    #[cfg(desktop)]
    app_handle
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .unwrap();

    // Register the log plugin
    #[cfg(debug_assertions)]
    app_handle
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .unwrap();

    // Register the updater plugin
    #[cfg(desktop)]
    app_handle
        .plugin(tauri_plugin_updater::Builder::new().build())
        .unwrap();

    // Register websocket plugin
    #[cfg(desktop)]
    app_handle.plugin(tauri_plugin_websocket::init()).unwrap();

    // Register process plugin
    #[cfg(desktop)]
    app_handle.plugin(tauri_plugin_process::init()).unwrap();

    #[cfg(desktop)]
    app_handle
        .plugin(tauri_plugin_updater::Builder::new().build())
        .unwrap();

    #[cfg(desktop)]
    app_handle.plugin(tauri_plugin_upload::init()).unwrap();
}

// Register the update processor
// command::updator::app_updates::PendingUpdate 作为全局状态，可以提供给install_update和fetch_update方法作为参数。
#[allow(dead_code)]
fn regsister_update_processor(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        app.manage(command::updator::app_updates::PendingUpdate(Mutex::new(
            None,
        )));
    });
}
