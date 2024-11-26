use std::{
    env::current_dir,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    use core::models::ua::Ua;
    use core::utils::command::start_browser;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    // use core::utils::command::stop_browser;
    // use std::{thread, time::Duration};

    let mut childs = Vec::new();

    println!("{:?}", current_dir());

    for i in 0..10 {
        let ua = Ua {
            id: 0, // id 在反序列化时会被忽略
            os_name: String::from("Windows 10"),
            os_ver: String::from("10.0.19043"),
            browser_ver: String::from("Chrome 92.0.4515.159"),
        };

        let child = start_browser(
            ua,
            "",
            "",
            "",
            "",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
            1,
            &format!(
                "{}.windows.{}",
                i,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros()
            ),
        )
        .await
        .unwrap();

        childs.push(child);

        // thread::sleep(Duration::from_secs(5));
        // let exit_status = stop_browser(&mut child, 1).unwrap();
        // println!("{:?}", exit_status);
    }

    let mut tasks = Vec::new();

    // 使用 Mutex 包裹的计数器来控制输出顺序
    let output_lock = Arc::new(Mutex::new(Vec::new()));

    // 遍历所有的子进程
    for mut child in childs {
        // 使用 tokio::spawn 启动异步任务
        let output_lock = Arc::clone(&output_lock);

        let task = tokio::spawn(async move {
            // 使用 block_in_place 来执行阻塞的 child.wait()
            let exit_status = tokio::task::block_in_place(|| {
                // 阻塞式地等待子进程结束
                child.wait()
            });

            // 在 Mutex 锁保护下输出信息，确保只有一个任务输出
            let mut output = output_lock.lock().unwrap();

            if exit_status.await.unwrap().success() {
                output.push("Browser exited successfully.".to_string());
            } else {
                output.push(format!(
                    "Browser exited with error. Exit code: {:?}",
                    exit_status.await.unwrap().code()
                ));
            }
        });

        tasks.push(task);
    }

    // 等待所有任务完成
    for task in tasks {
        task.await.unwrap();
    }
}
