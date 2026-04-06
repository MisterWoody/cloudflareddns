use std::error::Error;
use crate::cloudflare_dns::{create_or_update_record, get_zone_id};
use crate::external_ip::{get_external_ipv4, get_external_ipv6};
use chrono::Local;

pub async fn check_ips_and_update_dns(
    api_token: &str,
    hosts_vec: &[&str],
    zones_vec: &[&str],
    ipv4: bool,
    ipv6: bool,
) -> Result<(), Box<dyn Error>> {
    let external_ipv4 = if ipv4 {
        get_external_ipv4().await?
    } else {
        let unused_ipv4: Box<dyn Error> = String::from("IPv4 is unused").into();
        return Err(unused_ipv4);
    };
    let the_time = Local::now();
    println!("{} External IPv4 address: {}", the_time.format("%Y-%m-%d %H:%M:%S%:z"), external_ipv4);

    let external_ipv6 = if ipv6 {
        get_external_ipv6().await?
    } else {
        String::from("IPv6 is unused")
    //     let unused_ipv6: Box<dyn Error> = String::from("IPv6 is unused").into();
    //     return Err(unused_ipv6);
    };
    let the_time = Local::now();
    println!("{} External IPv6 address: {}", the_time.format("%Y-%m-%d %H:%M:%S%:z"), external_ipv6);

    // Iterate over an enumerated value of a tuple of the matching host and zone
    for (host, zone) in hosts_vec.iter().zip(zones_vec.iter()) {
        // Call the get_zone_id function to get the zone ID for the current host.
        let zone_id = get_zone_id(api_token, zone).await?;
        println!("{} Zone ID for zone {}: {}", the_time.format("%Y-%m-%d %H:%M:%S%:z"), zone, zone_id);
        // Need host and zone to query DNS
        let record_name = format!("{}.{}", host, zone);
        //
        if ipv4 {
            match create_or_update_record(api_token, &external_ipv4, &record_name, "A", &zone_id).await {
                Ok(_) => {
                    let the_time = Local::now();
                    println!(
                    "{} Successfully updated A record for {}, zone {} in CloudFlare to {}",
                    the_time.format("%Y-%m-%d %H:%M:%S%:z"),
                    host, zone, external_ipv4
                    )
                },
                Err(e) => println!("Failed to create or update record: {}", e),
            }
        }
        //
        //         if ipv6 {
        //             match create_or_update_record(api_token, &external_ipv6, &record_name, "AAAA", &zone_id).await {
        //                 Ok(_) => println!(
        //                     "Successfully updated AAAA record for {}, zone {} in CloudFlare to {}",
        //                     host, zone, external_ipv6
        //                 ),
        //                 Err(e) => println!("Failed to create or update record: {}", e),
        //             }
        //         }
    }
    //
    Ok(())
}