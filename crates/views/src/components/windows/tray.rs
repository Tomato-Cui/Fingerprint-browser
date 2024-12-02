// use tauri::{
//     AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
//     SystemTrayMenuItem,
// };

// // 托盘菜单
// pub fn menu() -> SystemTray {
//     let quit = CustomMenuItem::new("quit".to_string(), "退出");
//     let show = CustomMenuItem::new("show".to_string(), "显示");
//     let hide = CustomMenuItem::new("hide".to_string(), "隐藏");

//     let tray_menu = SystemTrayMenu::new()
//         .add_item(hide)
//         .add_item(show)
//         .add_native_item(SystemTrayMenuItem::Separator)
//         .add_item(quit);

//     SystemTray::new().with_menu(tray_menu)
// }

// // 托盘事件
// pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
//     match event {
//         SystemTrayEvent::LeftClick {
//             position: _,
//             size: _,
//             ..
//         } => {
//             println!("点击左键");
//         }
//         SystemTrayEvent::RightClick {
//             position: _,
//             size: _,
//             ..
//         } => {
//             println!("点击右键");
//         }
//         SystemTrayEvent::DoubleClick {
//             position: _,
//             size: _,
//             ..
//         } => {
//             println!("双击");
//         }
//         SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
//             "hide" => {
//                 let window = app.get_window("main").unwrap();
//                 window.hide().unwrap();
//             }
//             "show" => {
//                 let window = app.get_window("main").unwrap();
//                 window.show().unwrap();
//             }
//             "quit" => {
//                 std::process::exit(0);
//             }
//             _ => {}
//         },
//         _ => {}
//     }
// }
