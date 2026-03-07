use std::env;
use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use reqwest::Client;

pub async fn get_external_ip(api_endpoint: &str) -> Result<std::string::String, Box<dyn Error>> {
    let client = Client::new();

    let res = client.get(api_endpoint).send().await?;

    if res.status().is_success() {
        let body = res.text().await?;
        let ip = IpAddr::from_str(&body);

        if let Ok(_ip) = ip {
            // If parsing succeeded, return the IP address
            Ok(body)
        } else {
            // If parsing failed, return an error
            eprintln!("Error: {} is not a valid IP address.", &body);
            let invalid_ip: Box<dyn Error> = String::from(format!("IP address {} is invalid", body)).into();
            Err(invalid_ip)
        }
    } else {
        let endpoint_fail: Box<dyn Error> =  String::from(format!("Retrieving the IP address API endpoint failed: {}", res.error_for_status().unwrap_err())).into();
        Err(endpoint_fail)
    }
}

pub async fn get_external_ipv6() -> Result<std::string::String, Box<dyn Error>> {
    // Allows users to optionally configure which endpoints are used, with a sensible default.
    let api_endpoint = env::var("CLOUDFLAREDDNS_IPV6_API_ENDPOINT")
        .unwrap_or_else(|_| "https://api6.ipify.org".to_string());
    get_external_ip(&api_endpoint).await
}

pub async fn get_external_ipv4() -> Result<std::string::String, Box<dyn Error>> {
    // Allows users to optionally configure which endpoints are used, with a sensible default.
    let api_endpoint = env::var("CLOUDFLAREDDNS_IPV4_API_ENDPOINT")
        .unwrap_or_else(|_| "https://api.ipify.org".to_string());
    get_external_ip(&api_endpoint).await
}