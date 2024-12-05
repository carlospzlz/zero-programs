use rppal::gpio::Gpio;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

// Left wheel
const LEFT_FORWARD: u8 = 27;
const LEFT_BACKWARD: u8 = 19;

// Right wheel
const RIGHT_FORWARD: u8 = 18;
const RIGHT_BACKWARD: u8 = 24;

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    loop {
        let mut line = String::new();
        print!(">>> ");

        io::stdout().flush().expect("Failed to flush prompt");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read command");
        let command = line.trim();

        match command {
            "left f" => set_one_high(LEFT_FORWARD, &gpio),
            "left b" => set_one_high(LEFT_BACKWARD, &gpio),
            "right f" => set_one_high(RIGHT_FORWARD, &gpio),
            "right b" => set_one_high(RIGHT_BACKWARD, &gpio),
            "forward" => set_two_high(LEFT_FORWARD, RIGHT_FORWARD, &gpio),
            "backward" => set_two_high(LEFT_BACKWARD, RIGHT_BACKWARD, &gpio),
            "spin r" => set_two_high(LEFT_FORWARD, RIGHT_BACKWARD, &gpio),
            "spin l" => set_two_high(LEFT_BACKWARD, RIGHT_FORWARD, &gpio),
            "" => (),
            _ => println!("Wubba Lubba Dub-Dub!"),
        }
    }
}

fn set_one_high(index: u8, gpio: &Gpio) {
    match gpio.get(index) {
        Ok(pin) => {
            let mut pin = pin.into_output();
            pin.set_high();
            sleep(Duration::from_secs(1));
        }
        Err(e) => {
            println!("Failed to access GPIO {}: {}", index, e);
        }
    }
}

fn set_two_high(index1: u8, index2: u8, gpio: &Gpio) {
    match (gpio.get(index1), gpio.get(index2)) {
        (Ok(pin1), Ok(pin2)) => {
            let mut pin1 = pin1.into_output();
            let mut pin2 = pin2.into_output();
            pin1.set_high();
            pin2.set_high();
            sleep(Duration::from_secs(1));
        }
        (Err(e), _) => {
            println!("Failed to access GPIO {}: {}", index1, e);
        }
        (_, Err(e)) => {
            println!("Failed to access GPIO {}: {}", index2, e);
        }
    }
}
