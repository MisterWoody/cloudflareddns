//! Cloudflare DNS API specific related functions
use reqwest::Response;
use serde_json::json;
use chrono::Local;

pub async fn get_zone_id(api_token: &str, zone_name: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones?name={}",
        zone_name
    );
    println!("Url for GET request: {}", url);

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .send().await?;

    if res.status().is_success() {
        let json = res.json::<serde_json::Value>().await?;
        let zones = json["result"].as_array().expect("Expected array of zones");
        let zone = &zones[0];
        let zone_id = zone["id"]
            .as_str()
            .expect("Expected zone ID to be a string");
        Ok(zone_id.to_string())
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

pub async fn create_or_update_record(
    api_token: &str,
    ip: &str,
    record_name: &str,
    record_type: &str,
    zone_id: &str,
) -> Result<(), reqwest::Error> {

    let res = dns_records(api_token, zone_id, record_name, record_type).await?;
    if res.status().is_success() {
        // Read the response body as a JSON value
        let json = res.json::<serde_json::Value>().await?;
        let records = json["result"].as_array().unwrap();
        if !records.is_empty() && records[0]["content"] == ip {
            let the_time = Local::now();
            println!("{} The record is already correct.\n{}", the_time.format("%Y-%m-%d %H:%M:%S%z"), records[0]);
            Ok(())
        } else if records.is_empty() {
            let res = create_dns_record(api_token, ip, zone_id, record_name, record_type).await?;
            let the_time = Local::now();
            if res.status().is_success() {
                println!("{} Created a new record\n{}", the_time.format("%Y-%m-%d %H:%M:%S%z"), res.text().await?);
                Ok(())
            } else {
                println!("{} Failed to create record.", the_time.format("%Y-%m-%d %H:%M:%S%z"));
                Err(res.error_for_status().unwrap_err())
            }
        } else {
            let record_id = records[0]["id"].as_str().unwrap();
            let res = update_dns_record(api_token, ip, zone_id, record_name, record_type, record_id).await?;
            if res.status().is_success() {
                println!("{}", res.text().await?);
                Ok(())
            }
            else {
                let the_time = Local::now();
                println!("{} Failed to update record.", the_time.format("%Y-%m-%d %H:%M:%S%z"));
                Err(res.error_for_status().unwrap_err())
            }
        }
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

/// Get all DNS records in the zone matching the name and type
async fn dns_records(api_token: &str, zone_id: &str, record_name: &str, record_type: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}&type={}",
        zone_id, record_name, record_type
    );
    let the_time = Local::now();
    println!("{} Url for GET request: {}", the_time.format("%Y-%m-%d %H:%M:%S%z"), url);

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .send().await?;

    if res.status().is_success() {
        Ok(res)
    }
    else {
        Err(res.error_for_status().unwrap_err())
    }
}

/// Create a new DNS record
async fn create_dns_record(api_token: &str, ip: &str, zone_id: &str, record_name: &str, record_type: &str ) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let post_url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
        zone_id
        );
    let body = json!({
        "type": record_type,
        "name": record_name,
        "content": ip,
        "ttl": 1,
        "proxied": true
    });
    let the_time = Local::now();
    println!("{} POST URL: {}\nPOST body: {}", the_time.format("%Y-%m-%d %H:%M:%S%z"), post_url, body);

    let res = client
        .post(&post_url)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await?;
    if res.status().is_success() {
        Ok(res)
    }
    else {
        Err(res.error_for_status().unwrap_err())
    }
}

/// Update the existing DNS record
async fn update_dns_record(api_token: &str, ip: &str, zone_id: &str, record_name: &str, record_type: &str, record_id: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let patch_url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id,
        record_id
    );

    // Send a PATCH request to the API endpoint
    let body = json!({
                "type": record_type,
                "name": record_name,
                "content": ip,
                "ttl": 1,
                "proxied": true
            });
    let the_time = Local::now();
    println!("{} PATCH URL: {}\nPATCH body: {}", the_time.format("%Y-%m-%d %H:%M:%S%z"), patch_url, body);

    let res = client
        .patch(&patch_url)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await?;
    if res.status().is_success() {
        Ok(res)
    }
    else {
        Err(res.error_for_status().unwrap_err())
    }
}