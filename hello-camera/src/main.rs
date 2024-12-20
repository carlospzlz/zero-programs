use futures::StreamExt;
use rscam::{Camera, Config};
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a channel to share frames
    let (frame_tx, frame_rx) = watch::channel(Vec::new());

    // Spawn a task to capture frames
    tokio::spawn(async move {
        capture_frames(frame_tx).await;
    });

    // Start the video server
    let video_route = warp::path!("video").map(move || {
        let rx = frame_rx.clone();
        warp::sse::reply(warp::sse::keep_alive().stream(rx_stream(rx)))
    });

    // Enable CORS for all origins
    let cors = warp::cors()
        .allow_any_origin() // This allows any origin to access the server
        .allow_methods(vec!["GET", "POST"]) // Allow specific HTTP methods
        .allow_headers(vec!["Content-Type"]); // Allow specific headers

    // Combine CORS with the SSE handler
    let routes = video_route.with(cors);

    println!("Starting video server on http://0.0.0.0:5000/video");
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}

async fn capture_frames(tx: watch::Sender<Vec<u8>>) {
    let mut camera = Camera::new("/dev/video0").expect("Failed to open camera");

    camera
        .start(&Config {
            interval: (1, 30), // 30 fps
            resolution: (640, 480),
            format: b"MJPG",
            ..Default::default()
        })
        .expect("Failed to configure camera");

    loop {
        match camera.capture() {
            Ok(frame) => {
                if tx.send(frame.to_vec()).is_err() {
                    break; // Stop if no subscribers
                }
            }
            Err(e) => eprintln!("Error capturing frame: {}", e),
        }

        tokio::time::sleep(Duration::from_millis(33)).await; // Approx. 30 FPS
    }
}

fn rx_stream(
    rx: watch::Receiver<Vec<u8>>,
) -> impl futures::Stream<Item = Result<warp::sse::Event, warp::Error>> {
    // Convert the watch receiver into a stream and handle Result from WatchStream
    WatchStream::new(rx).filter_map(|frame| {
        async {
            Some(Ok(warp::sse::Event::default()
                .id("frame")
                .data(base64::encode(frame)))) // Encode frame as Base64
        }
    })
}
