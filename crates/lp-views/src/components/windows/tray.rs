use tauri::menu::MenuEvent;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App,
};
use tauri::{AppHandle, Manager};

pub fn menu(app: &mut App) -> Result<TrayIcon, tauri::Error> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    let tray = TrayIconBuilder::new()
        .on_menu_event(menu_handler)
        .on_tray_icon_event(tray_handler)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .build(app)?;

    Ok(tray)
}

pub fn menu_handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "quit" => {
            println!("quit menu item was clicked");
            app.cleanup_before_exit();
            std::process::exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}

pub fn tray_handler(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                if window.is_minimized().expect("get window minimized failed") {
                    let _ = window.unminimize();
                }
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    }
}
