// use std::env::current_dir;

// #[tokio::main]
// async fn main() {
//     use cores::models::ua::Ua;
//     use cores::utils::command::start_browser;
//     use std::time::SystemTime;
//     use std::time::UNIX_EPOCH;
//     // use core::utils::command::stop_browser;
//     // use std::{thread, time::Duration};

//     let mut childs = Vec::new();

//     println!("{:?}", current_dir());

//     for i in 0..10 {
//         let ua = Ua {
//             id: 0, // id 在反序列化时会被忽略
//             os_name: String::from("Windows 10"),
//             os_ver: String::from("10.0.19043"),
//             browser_ver: String::from("Chrome 92.0.4515.159"),
//         };

//         let child = start_browser(
//             ua,
//             "",
//             "",
//             "",
//             "",
//             "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
//             1,
//             &format!(
//                 "{}.windows.{}",
//                 i,
//                 SystemTime::now()
//                     .duration_since(UNIX_EPOCH)
//                     .unwrap()
//                     .as_micros()
//             ),
//         )
//         .await
//         .unwrap();

//         childs.push(child);

//         // thread::sleep(Duration::from_secs(5));
//         // let exit_status = stop_browser(&mut child, 1).unwrap();
//         // println!("{:?}", exit_status);
//     }
// }

fn main() {}
