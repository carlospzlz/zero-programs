use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rppal::gpio::{Gpio, Result};
use std::io::{stdout, Write};
use std::time::Duration;

// Left wheel
const LEFT_FORWARD: u8 = 27;
const LEFT_BACKWARD: u8 = 19;

// Right wheel
const RIGHT_FORWARD: u8 = 18;
const RIGHT_BACKWARD: u8 = 24;

fn main() -> Result<()> {
    let stdout = stdout();
    let mut handle = stdout.lock();

    // Enable raw mode
    // - Input will not be forwarded to screen
    // - Input will not be processed on enter press
    // - Input will not be line buffered (input sent byte-by-byte to input buffer)
    // - Special keys like backspace and CTRL+C will not be processed by terminal driver
    // - New line character will not be processed therefore println! can’t be used, use write! instead
    enable_raw_mode()?;

    write!(&mut handle, "Press 'q' to quit.\n\r")?;

    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut left_forward_pin = gpio.get(LEFT_FORWARD)?.into_output();
    let mut left_backward_pin = gpio.get(LEFT_BACKWARD)?.into_output();
    let mut right_forward_pin = gpio.get(RIGHT_FORWARD)?.into_output();
    let mut right_backward_pin = gpio.get(RIGHT_BACKWARD)?.into_output();

    loop {
        if poll(Duration::from_millis(100))? {
            // Read an event
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('i') => {
                        // Make sure no backward pins are enabled
                        left_backward_pin.set_low();
                        right_backward_pin.set_low();
                        // Forward!
                        left_forward_pin.set_high();
                        right_forward_pin.set_high();
                    }
                    KeyCode::Char('k') => {
                        // Make sure no forward pins are enabled
                        left_forward_pin.set_low();
                        right_forward_pin.set_low();
                        // Backward!
                        left_backward_pin.set_high();
                        right_backward_pin.set_high();
                    }
                    KeyCode::Char('j') => {
                        // Make sure the complementary are off
                        left_forward_pin.set_low();
                        right_backward_pin.set_low();
                        // Spin Left!
                        left_backward_pin.set_high();
                        right_forward_pin.set_high();
                    }
                    KeyCode::Char('l') => {
                        // Make sure the complementary are off
                        right_forward_pin.set_low();
                        left_backward_pin.set_low();
                        // Spin Right!
                        right_backward_pin.set_high();
                        left_forward_pin.set_high();
                    }
                    _ => write!(&mut handle, "Wubba Lubba Dub-Dub!\n\r")?,
                }
            }
        } else {
            left_forward_pin.set_low();
            left_backward_pin.set_low();
            right_forward_pin.set_low();
            right_backward_pin.set_low();
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;

    Ok(())
}
