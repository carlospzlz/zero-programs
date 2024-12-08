use axum::{routing::get, Router};
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    // Shared state: A watch channel to broadcast the current message
    let (tx, rx) = watch::channel(String::from("Hello, World!"));
    let rx = Arc::new(Mutex::new(rx));

    println!("Server running at http://0.0.0.0:3000/");

    // Spawn a task to listen for keyboard input and update the message
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut buffer = String::new();

        loop {
            print!(">>> ");
            io::stdout().flush().expect("Failed to flush prompt");
            buffer.clear();
            tokio::io::AsyncBufReadExt::read_line(
                &mut tokio::io::BufReader::new(tokio::io::stdin()),
                &mut buffer,
            )
            .await
            .expect("Failed to read command");

            // Update the shared message (trim to remove newline)
            let command = buffer.trim();
            if !command.is_empty() {
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
                async move { rx.lock().unwrap().borrow().clone() }
            }
        }),
    );

    // Serve the app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}
