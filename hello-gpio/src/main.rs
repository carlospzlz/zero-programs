use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut pins = Vec::new();

    for i in 1..41 {
        match gpio.get(i) {
            Ok(pin) => {
                let mut pin = pin.into_output();
                println!("Pin {}: {}", i, pin.is_set_high());
                pin.set_high();
                pins.push(pin);
            }
            Err(e) => {
                println!("Failed to access GPIO {}: {}", i, e);
            }
        }
        sleep(Duration::from_secs(1));
    }
}

