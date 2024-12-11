use chrono::{DateTime, FixedOffset, Utc};
use rppal::gpio::{Gpio, Result};
use std::collections::HashMap;
use std::env;
use std::process;
use std::time::Duration;
use tokio::time::sleep;

// Left wheel
const LEFT_FORWARD: u8 = 27;
const LEFT_BACKWARD: u8 = 19;

// Right wheel
const RIGHT_FORWARD: u8 = 18;
const RIGHT_BACKWARD: u8 = 24;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <URL>", args[0]);
        process::exit(0);
    }

    let url = &format!("{}/command", args[1]);

    // For timestamp
    let utc_plus_one = FixedOffset::east_opt(3600).unwrap(); // 3600 seconds = 1 hour.

    println!("Starting client to fetch updates from: {}", url);

    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut left_forward_pin = gpio.get(LEFT_FORWARD)?.into_output();
    let mut left_backward_pin = gpio.get(LEFT_BACKWARD)?.into_output();
    let mut right_forward_pin = gpio.get(RIGHT_FORWARD)?.into_output();
    let mut right_backward_pin = gpio.get(RIGHT_BACKWARD)?.into_output();

    loop {
        // Send a GET request
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse the response body as JSON
                    let data = response
                        .json::<HashMap<String, String>>()
                        .await
                        .expect("Failed to parse JSON");
                    let command = &data["command"];
                    let timestamp = DateTime::parse_from_rfc3339(&data["timestamp"])
                        .expect("Failed to parse timestamp");
                    let latency = Utc::now().with_timezone(&utc_plus_one) - timestamp;
                    println!("Command: {} ({} ms)", command, latency.num_milliseconds());
                    match command.as_str() {
                        "idle" => {
                            // All low
                            left_backward_pin.set_low();
                            right_backward_pin.set_low();
                            left_forward_pin.set_low();
                            right_forward_pin.set_low();
                        }
                        "forward" => {
                            // Make sure no backward pins are enabled
                            left_backward_pin.set_low();
                            right_backward_pin.set_low();
                            // Forward!
                            left_forward_pin.set_high();
                            right_forward_pin.set_high();
                        }
                        "backward" => {
                            // Make sure no forward pins are enabled
                            left_forward_pin.set_low();
                            right_forward_pin.set_low();
                            // Backward!
                            left_backward_pin.set_high();
                            right_backward_pin.set_high();
                        }
                        "spin r" => {
                            // Make sure the complementary are off
                            right_forward_pin.set_low();
                            left_backward_pin.set_low();
                            // Spin Right!
                            right_backward_pin.set_high();
                            left_forward_pin.set_high();
                        }
                        "spin l" => {
                            // Make sure the complementary are off
                            left_forward_pin.set_low();
                            right_backward_pin.set_low();
                            // Spin Left!
                            left_backward_pin.set_high();
                            right_forward_pin.set_high();
                        }
                        _ => {
                            sleep(Duration::from_millis(20)).await;
                        }
                    }
                } else {
                    println!("Failed to fetch. Status: {}", response.status());
                }
            }
            Err(err) => {
                println!("Error fetching from server: {} Is server up?", err);
            }
        }
    }
}
