use evdev::{Device, InputEventKind};
use std::io::Error;

fn main() -> Result<(), Error> {
    // Path to the input device file for the controller
    // Replace this with the correct path for your system.
    let device_path = "/dev/input/event17"; // Replace 'eventX' with the actual event file

    // Open the device
    //let file = File::open(device_path)?;
    let mut device = Device::open(device_path)?;

    println!(
        "Listening for events from: {}",
        device.name().unwrap_or("Unknown Device")
    );

    // Read and process events in a loop
    loop {
        for ev in device.fetch_events().unwrap() {
            match ev.kind() {
                InputEventKind::Key(key) => {
                    println!("Key event: {:?}, value: {}", key, ev.value());
                }
                InputEventKind::AbsAxis(axis) => {
                    println!("Axis event: {:?}, value: {}", axis, ev.value());
                }
                _ => {
                    println!("Other event: {:?}, value: {}", ev.kind(), ev.value());
                }
            }
        }
    }
}
