use axum::{routing::get, Json, Router};
use chrono::{FixedOffset, Utc};
use evdev::{AbsoluteAxisType, Device, InputEventKind};
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    // Shared state: A watch channel to broadcast the current message
    let (tx, rx) = watch::channel(String::from("idle"));
    let rx = Arc::new(Mutex::new(rx));

    let device_path = "/dev/input/event17";
    let mut device = Device::open(device_path).expect("Failed to open device");

    // For timestamp
    let utc_plus_one = FixedOffset::east_opt(3600).unwrap(); // 3600 seconds = 1 hour.

    println!("Server running at http://0.0.0.0:3000/");

    // Spawn a task to listen for keyboard input and update the message
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut x_value = 128;
        let mut y_value = 128;
        loop {
            for ev in device.fetch_events().unwrap() {
                match ev.kind() {
                    InputEventKind::AbsAxis(axis) => match axis {
                        AbsoluteAxisType::ABS_RX => x_value = ev.value(),
                        AbsoluteAxisType::ABS_Y => y_value = ev.value(),
                        _ => (),
                    },
                    _ => (),
                }
                let x_displacement = (x_value - 128).abs();
                let y_displacement = (y_value - 128).abs();
                let command = if x_displacement > y_displacement {
                    if x_value < 128 {
                        "spin l"
                    } else {
                        "spin r"
                    }
                } else if y_displacement > x_displacement {
                    if y_value < 128 {
                        "forward"
                    } else {
                        "backward"
                    }
                } else {
                    "idle"
                };
                // Send to channel
                tx_clone
                    .send(command.to_string())
                    .expect("Failed to send command");
            }
        }
    });

    // Create the Axum app with the route
    let app = Router::new().route(
        "/command",
        get({
            move || {
                let rx = rx.clone();
                // Create JSON
                let dict = serde_json::json!({
                    "command": rx.lock().unwrap().borrow().clone(),
                    "timestamp": Utc::now().with_timezone(&utc_plus_one).to_rfc3339(),
                });
                async move { Json(dict.clone()) }
            }
        }),
    );

    // Serve the app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}
