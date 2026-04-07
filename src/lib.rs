pub mod cloudflare_dns;
pub mod config;
pub mod external_ip;
pub mod monitor;
pub mod shutdown;
pub mod log_util;

use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use monitor::check_ips_and_update_dns;
use config::Config;
use shutdown::shutdown_signal;

pub async fn run() -> Result<(), Box<dyn Error>> {

    let mut conf = Config::new();

    let api_token = conf.api_token();
    let record_types = conf.record_types();
    let repeat_interval = conf.repeat_interval();

    let record_type_values = record_types.split(';').collect::<Vec<_>>();
    let ipv4 = record_type_values.contains(&"A");
    let ipv6 = record_type_values.contains(&"AAAA");

    // host and zones are parallel arrays with elements at the same index expected to go together
    let hosts = conf.hosts();
    let zones = conf.zones();
    // Split the hosts and zones strings on the semicolon character into vectors.
    let hosts = hosts.split(';').collect::<Vec<_>>();
    let zones = zones.split(';').collect::<Vec<_>>();

    // If the lengths of hosts and zones not equal, return an error.
    if hosts.len() != zones.len() {
        let length_mismatch: Box<dyn Error> = String::from("Error: hosts and zones have different lengths. These need to match").into();
        return Err(length_mismatch);
    }
    // If either hosts or zones are empty then error
    if hosts.is_empty() || zones.is_empty() {
        let empty_values: Box<dyn Error> = String::from("Error: hosts and zones must both not be empty.").into();
        return Err(empty_values);
    }

    if repeat_interval == 0 {
         match check_ips_and_update_dns(&api_token, &hosts, &zones, ipv4, ipv6).await {
            Ok(()) => {},
            Err(e) => eprintln!("{}", e)
        }
    }
    else {
        let mut end:bool = false;
        loop {
            match check_ips_and_update_dns(&api_token, &hosts, &zones, ipv4, ipv6).await {
                Ok(()) => {},
                Err(e) => eprintln!("{}", e)
            }
            println!("DNS updated. Sleeping for {repeat_interval} seconds.");
            // See rust in a month of lunches select! in section 19.33.4
            tokio::select! {
                _ = sleep(Duration::from_secs(repeat_interval)) => {},
                _ = shutdown_signal() => {
                    end = true;
                }
            }
            if end {
                break;
            }
        }
    }

    Ok(())
}