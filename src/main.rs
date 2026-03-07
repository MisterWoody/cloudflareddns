use std::{process};
use std::error::Error;
use cloudflareddns::run;

#[tokio::main]
#[maybe_async::maybe_async]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run().await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}