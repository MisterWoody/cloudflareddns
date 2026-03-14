use std::env;

// TODO only invoke the dotenv call once to merge file variables into the environment

pub struct Config {
    merged: bool,
}

impl Config {
    pub fn new() -> Self {
        Config { merged: false }
    }

    pub fn api_token(&mut self) -> String {
        let api_token = env::var("CLOUDFLAREDDNS_APITOKEN").unwrap_or_else(|_| {
            dotenv::dotenv().expect("Unable to load environment variable from .env file");
            self.merged = true;
            env::var("CLOUDFLAREDDNS_APITOKEN").expect("CLOUDFLAREDDNS_APITOKEN environment variable not set")
        });
        // dbg!(&api_token);
        api_token
    }

    pub fn record_types(&mut self) -> String {
        let record_types = env::var("CLOUDFLAREDDNS_RECORDTYPES").unwrap_or_else(|_| {

            if !self.merged {
                dotenv::dotenv().expect("Unable to load environment variable from .env file");
                self.merged = true;
            }
            env::var("CLOUDFLAREDDNS_RECORDTYPES").expect("CLOUDFLAREDDNS_RECORDTYPES environment variable not set")
        });
        // dbg!(&record_types);
        record_types
    }

    // Get repeat interval, defaults to 0 if not specified, which runs only once.
    pub fn repeat_interval(&mut self) -> u64 {
        let repeat_interval = env::var("CLOUDFLAREDDNS_REPEAT_INTERVAL").unwrap_or_else(|_| {
            if !self.merged {
                dotenv::dotenv().expect("Unable to load environment variable from .env file");
                self.merged = true;
            }
            env::var("CLOUDFLAREDDNS_REPEAT_INTERVAL").unwrap_or_else(|_| "0".to_owned())
        });
        // Parse this string value into a 64-bit unsigned integer.
        let repeat_interval: u64 = repeat_interval.parse().unwrap_or(0);
        // dbg!(&repeat_interval);
        repeat_interval
    }

    pub fn hosts() -> String {
        let hosts = std::env::var("CLOUDFLAREDDNS_HOSTS")
            .expect("CLOUDFLAREDDNS_HOSTS environment variable not set");
        // dbg!(&hosts);
        hosts
    }

    pub fn zones() -> String {
        let zones = std::env::var("CLOUDFLAREDDNS_ZONES")
            .expect("CLOUDFLAREDDNS_ZONES environment variable not set");
        // dbg!(&zones);
        zones
    }
}