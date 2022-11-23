pub mod sysmon;
use colored::*;
use futures::future;
use sysmon::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let can0 = sysmon::check_net_device("can0");
    let kanto_cm0 = sysmon::check_net_device("kanto-cm0");
    let res = future::join_all(vec![can0, kanto_cm0]).await;

    println!("Service name\tMessage\tStatus");
    for r in &res {
        if let Ok(r) = r {
            if r.is_good {
                println!("{}\t{}\t{}", r.service_name, r.message, "[OK]".green());
            } else {
                println!("{}\t{}\t{}", r.service_name, r.message, "[FAIL]".red());
            }
        }
    }
    Ok(())
}
