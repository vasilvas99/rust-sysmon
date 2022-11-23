pub mod sysmon;
use futures::future;
use sysmon::{Result};
use colored::*;
#[tokio::main]
async fn main() -> Result<()> {
    let can0 = sysmon::check_net_device("cam0");
    let res = future::join_all(vec![can0]).await;

    if let Ok(r) = &res[0] {
        if r.is_good {
            println!("{}. Name: {}", "Service is good".green(), r.service_name);
            println!("{}", r.message)
        } else {
            println!("{}. Name {}", "Service is bad".red(), r.service_name);
        }
    }
    Ok(())
}
