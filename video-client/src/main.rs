use std::io::{Read};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Remote server address (update this to your server's IP and port)
    //let remote_host = "192.168.1.10"; // Replace with the remote server's IP
    let remote_host = "192.168.1.10"; // Replace with the remote server's IP
    let remote_port = 5000;
    let remote_addr = format!("{remote_host}:{remote_port}");

    println!("Connecting to server at {}", remote_addr);

    // Establish a TCP connection to the remote server
    let mut stream = TcpStream::connect(remote_addr)?;
    println!("Connected to server!");

    // Start the libcamera-vid process to capture video
    let mut child = Command::new("libcamera-vid")
        .args([
            "-t", "0",               // Stream indefinitely
            "--width", "640",        // Video width
            "--height", "480",       // Video height
            "--codec", "h264",       // Use H.264 codec
            "-o", "-",               // Output to stdout
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
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
        stream.write_all(&buffer[..bytes_read])?;
    }

    println!("Video stream ended.");
    Ok(())
}
