use chrono::{DateTime, FixedOffset, Utc};
use rppal::gpio::{Gpio, Result};
use std::collections::HashMap;
use std::env;
use std::process;
use std::time::Duration;
use tokio::time::sleep;

// Back engine
const BACK_ENGINE_FORWARD: u8 = 27;
const BACK_ENGINE_BACKWARD: u8 = 22;

// Front engine
const FRONT_ENGINE_FORWARD: u8 = 16;
const FRONT_ENGINE_BACKWARD: u8 = 20;

// Direction
const DIRECTION_LEFT: u8 = 18;
const DIRECTION_RIGHT: u8 = 17;

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

    let mut back_engine_forward_pin = gpio.get(BACK_ENGINE_FORWARD)?.into_output();
    let mut back_engine_backward_pin = gpio.get(BACK_ENGINE_BACKWARD)?.into_output();
    let mut front_engine_forward_pin = gpio.get(FRONT_ENGINE_FORWARD)?.into_output();
    let mut front_engine_backward_pin = gpio.get(FRONT_ENGINE_BACKWARD)?.into_output();
    let mut direction_left_pin = gpio.get(DIRECTION_LEFT)?.into_output();
    let mut direction_right_pin = gpio.get(DIRECTION_RIGHT)?.into_output();

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
                            back_engine_backward_pin.set_low();
                            back_engine_forward_pin.set_low();
                            front_engine_backward_pin.set_low();
                            front_engine_forward_pin.set_low();
                            direction_left_pin.set_low();
                            direction_right_pin.set_low();
                        }
                        "forward" => {
                            // Straight direction
                            direction_left_pin.set_low();
                            direction_right_pin.set_low();
                            // Make sure no backward pins are enabled
                            back_engine_backward_pin.set_low();
                            front_engine_backward_pin.set_low();
                            // Forward!
                            back_engine_forward_pin.set_high();
                            front_engine_forward_pin.set_high();
                        }
                        "backward" => {
                            // Straight direction
                            direction_left_pin.set_low();
                            direction_right_pin.set_low();
                            // Make sure no forward pins are enabled
                            back_engine_forward_pin.set_low();
                            front_engine_forward_pin.set_low();
                            // Backward!
                            back_engine_backward_pin.set_high();
                            front_engine_backward_pin.set_high();
                        }
                        "spin r" => {
                            // Make sure no forward pins are enabled
                            direction_left_pin.set_low();
                            // Right!
                            direction_right_pin.set_high();
                        }
                        "spin l" => {
                            // Make sure no forward pins are enabled
                            direction_right_pin.set_low();
                            // Left!
                            direction_left_pin.set_high();
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
