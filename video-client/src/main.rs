use std::env;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <hostname:port>", args[0]);
        process::exit(0);
    }

    let server_address = &args[1];

    println!("Connecting to server at {}", server_address);

    // Establish a TCP connection to the remote server
    loop {
        match TcpStream::connect(server_address.clone()) {
            Ok(mut stream) => {
                println!("Connected to server!");

                // Start the libcamera-vid process to capture video
                let mut child = process::Command::new("libcamera-vid")
                    .args([
                        "-t", "0", // Stream indefinitely
                        "--width", "1920", // Video width
                        "--height", "1080", // Video height
                        "--codec", "h264", // Use H.264 codec
                        "-o", "-", // Output to stdout
                    ])
                    .stdout(process::Stdio::piped())
                    .stderr(process::Stdio::null())
                    .spawn()
                    .expect("Failed to start libcamera-vid");

                // Get the stdout of the libcamera process
                let mut stdout = child.stdout.take().expect("Failed to capture stdout");

                // Read frames from libcamera and send them to the server
                let mut buffer = [0u8; 1024];
                while let Ok(bytes_read) = stdout.read(&mut buffer) {
                    if bytes_read == 0 {
                        break; // EOF
                    }
                    if let Err(err) = stream.write_all(&buffer[..bytes_read]) {
                        println!("Can't write: {}", err);
                        break;
                    }
                }

                println!("Video stream ended.");
            }
            _ => {
                println!("{} is not reachable...", server_address);
                sleep(Duration::from_secs(1));
            }
        }
    }
}
