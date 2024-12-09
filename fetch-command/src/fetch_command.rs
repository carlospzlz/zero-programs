use reqwest::Error;
use rppal::gpio::Gpio;
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
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <URL>", args[0]);
        process::exit(0);
    }

    let url = &format!("{}/command", args[1]);

    println!("Starting client to fetch updates from: {}", url);

    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    loop {
        // Send a GET request
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    // Read the response body as a string
                    let command = response.text().await.unwrap_or_default();
                    match command.as_str() {
                        "left f" => set_one_high(LEFT_FORWARD, &gpio).await,
                        "left b" => set_one_high(LEFT_BACKWARD, &gpio).await,
                        "right f" => set_one_high(RIGHT_FORWARD, &gpio).await,
                        "right b" => set_one_high(RIGHT_BACKWARD, &gpio).await,
                        "forward" => set_two_high(LEFT_FORWARD, RIGHT_FORWARD, &gpio).await,
                        "backward" => set_two_high(LEFT_BACKWARD, RIGHT_BACKWARD, &gpio).await,
                        "spin r" => set_two_high(LEFT_FORWARD, RIGHT_BACKWARD, &gpio).await,
                        "spin l" => set_two_high(LEFT_BACKWARD, RIGHT_FORWARD, &gpio).await,
                        _ => {
                            // Sleep for 1 seconds before the next request
                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                } else {
                    println!("Failed to fetch. Status: {}", response.status());
                }
            }
            Err(err) => {
                println!("Error fetching from server: {}", err);
            }
        }
    }
}

async fn set_one_high(index: u8, gpio: &Gpio) {
    match gpio.get(index) {
        Ok(pin) => {
            let mut pin = pin.into_output();
            pin.set_high();
            sleep(Duration::from_secs(1)).await;
        }
        Err(e) => {
            println!("Failed to access GPIO {}: {}", index, e);
        }
    }
}

async fn set_two_high(index1: u8, index2: u8, gpio: &Gpio) {
    match (gpio.get(index1), gpio.get(index2)) {
        (Ok(pin1), Ok(pin2)) => {
            let mut pin1 = pin1.into_output();
            let mut pin2 = pin2.into_output();
            pin1.set_high();
            pin2.set_high();
            sleep(Duration::from_secs(1)).await;
        }
        (Err(e), _) => {
            println!("Failed to access GPIO {}: {}", index1, e);
        }
        (_, Err(e)) => {
            println!("Failed to access GPIO {}: {}", index2, e);
        }
    }
}
