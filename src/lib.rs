mod cloudflare_dns;
mod config;
mod external_ip;
mod monitor;

use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{signal};
use monitor::check_ips_and_update_dns;
use crate::config::Config;

pub async fn run() -> Result<(), Box<dyn Error>> {

    let mut conf = Config::new();

    let api_token = conf.api_token();
    let record_types = (conf).record_types();
    let repeat_interval = (conf).repeat_interval();

    let record_type_values = record_types.split(';').collect::<Vec<_>>();
    let ipv4 = record_type_values.contains(&"A");
    let ipv6 = record_type_values.contains(&"AAAA");

    // host and zones are parallel arrays with elements at the same index expected to go together
    let hosts = (conf).hosts();
    let zones = (conf).zones();
    // Split the hosts and zones strings on the semicolon character into vectors.
    let hosts_vec = hosts.split(';').collect::<Vec<_>>();
    let zones_vec = zones.split(';').collect::<Vec<_>>();

    // If the lengths of hosts and zones not equal, return an error.
    if hosts_vec.len() != zones_vec.len() {
        let length_mismatch: Box<dyn Error> = String::from("Error: hosts and zones have different lengths. These need to match").into();
        return Err(length_mismatch);
    }
    // If either hosts or zones are empty then error
    if hosts_vec.is_empty() || zones_vec.is_empty() {
        let empty_values: Box<dyn Error> = String::from("Error: hosts and zones must both not be empty.").into();
        return Err(empty_values);
    }

    if repeat_interval == 0 {
         match check_ips_and_update_dns(&api_token, &hosts_vec, &zones_vec, ipv4, ipv6).await {
            Ok(()) => {},
            Err(e) => eprintln!("{}", e)
        }
    }
    else {
        loop {
            match check_ips_and_update_dns(&api_token, &hosts_vec, &zones_vec, ipv4, ipv6).await {
                Ok(()) => {},
                Err(e) => eprintln!("{}", e)
            }
            println!("DNS updated. Sleeping for {repeat_interval} seconds.");
            sleep(Duration::from_secs(repeat_interval)).await;
        }
    }

    // shutdown_signal().await;

    Ok(())
}

// todo add graceful shutdown to fix issue with ctrl-c
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}