use tokio::process::Command;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const  KILLED_BY_SIGNAL_STATUS: i32 = -10000;

#[derive(Debug)]
pub struct MonitorResults {
    pub service_name: String,
    pub is_good: bool,
    pub message: String,
}

pub struct ParsedConsoleOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32
}

async fn execute_cmd(cmd: &str) -> Result<ParsedConsoleOutput> {
    let c = Command::new("/bin/bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .await?;

    let exit_code = match c.status.code() {
        Some(c) => c,
        None => KILLED_BY_SIGNAL_STATUS
    };

    let out = ParsedConsoleOutput{
        stdout: String::from_utf8_lossy(&c.stdout).trim().to_string(),
        stderr: String::from_utf8_lossy(&c.stderr).trim().to_string(),
        exit_code
    };

    Ok(out)
}

pub async fn check_net_device(net_dev_id: &str) -> Result<MonitorResults> {
    let output = execute_cmd(&format!("ip address | grep \": {}:\"", net_dev_id)).await?;
    let mut s_name = String::from("NetDev ");
    s_name.push_str(net_dev_id);
    let res = if output.stdout.len() != 0 {
        MonitorResults {
            service_name: s_name,
            is_good: true,
            message: output.stdout,
        }
    } else {
        MonitorResults {
            service_name: s_name,
            is_good: false,
            message: output.stderr,
        }
    };

    Ok(res)
}