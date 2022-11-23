pub mod sysmon;
use colored::*;
use futures::future;
use sysmon::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let can0 = sysmon::check_net_device("can0");
    let kanto_cm0 = sysmon::check_net_device("kanto-cm0");
    let res = future::join_all(vec![can0, kanto_cm0]).await;
    println!("{0: <30} | {1: <30} | {2: <30}", "Service name", "Status", "Message");
    println!("-------------------------------------------------------------------------------");
    for r in &res {
        if let Ok(r) = r {
            if r.is_good {
                println!("{0: <30} | {1: <30} | {2: <30}", r.service_name, "[OK]".green(), r.message.green());
            } else {
                println!("{0: <30} | {1: <30} | {2: <30}", r.service_name, "[FAIL]".red(), r.message.red());
            }
        }
    }
    Ok(())
}
