pub mod cores;

#[test]
fn test_get_process_list() {
    use std::{
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn run_powershell_command() -> Result<String, String> {
        let log_name = format!(
            "process_list_{}.txt",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );

        let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(r#"Get-WmiObject Win32_Process | Format-Table ProcessId, CommandLine -Wrap | Out-File 'C:\Users\cgy\Desktop\ads-hubstudio\logs\{}'"#, log_name))
        .output() 
        .map_err(|e| format!("Failed to start PowerShell: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "PowerShell command failed with status: {}",
                output.status
            ));
        }

        // 将命令输出转换为字符串并返回
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    let result = run_powershell_command();

    println!("{:?}", result)
}
