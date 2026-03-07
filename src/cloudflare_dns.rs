use serde_json::json;

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
    let client = reqwest::Client::new();

    // First we get all DNS records in the zone matching the name and type
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}&type={}",
        zone_id, record_name, record_type
    );
    println!("Url for POST request: {}", url);

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_token))
        .header("Content-Type", "application/json")
        .send().await?;

    if res.status().is_success() {
        // Read the response body as a JSON value
        let json = res.json::<serde_json::Value>().await?;
        let records = json["result"].as_array().unwrap();
        if !records.is_empty() && records[0]["content"] == ip {
            println!(
                "The record is already correct. No need to do anything here!\n{}",
                records[0]
            );
            Ok(())
        } else if records.is_empty() {
            // We need to create the record
            let client = reqwest::Client::new();
            let post_url = format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                zone_id
            );
            let body = serde_json::json!({
                "type": record_type,
                "name": record_name,
                "content": ip,
                "ttl": 1,
                "proxied": true
            });

            println!("POST URL: {}\nPOST body: {}", post_url, body);

            let res = client
                .post(&post_url)
                .header("Authorization", format!("Bearer {}", api_token))
                .header("Content-Type", "application/json")
                .json(&body)
                .send().await?;

            if res.status().is_success() {
                println!("Created a new record\n{}", res.text().await?);
                Ok(())
            } else {
                println!("Failed to create record.");
                Err(res.error_for_status().unwrap_err())
            }
        } else {
            // We need to put a new value in the record
            let patch_url = format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                zone_id,
                records[0]["id"].as_str().unwrap()
            );

            // Send a PATCH request to the API endpoint
            let body = json!({
                "type": record_type,
                "name": record_name,
                "content": ip,
                "ttl": 1,
                "proxied": true
            });

            println!("PATCH URL: {}\nPATCH body: {}", patch_url, body);

            let res = client
                .patch(&patch_url)
                .header("Authorization", format!("Bearer {}", api_token))
                .header("Content-Type", "application/json")
                .json(&body)
                .send().await?;

            println!("{}", res.text().await?);
            Ok(())
        }
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}