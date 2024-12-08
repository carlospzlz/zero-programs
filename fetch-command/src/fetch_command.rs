use reqwest::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define the URL of the server
    let url = "http://192.168.1.143:3000/command";

    println!("Starting client to fetch updates from: {}", url);

    loop {
        // Send a GET request
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    // Read the response body as a string
                    let body = response.text().await.unwrap_or_default();
                    println!("{}", body);
                } else {
                    println!("Failed to fetch. Status: {}", response.status());
                }
            }
            Err(err) => {
                println!("Error fetching from server: {}", err);
            }
        }

        // Sleep for 1 seconds before the next request
        sleep(Duration::from_secs(1)).await;
    }
}
