use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut pin = Gpio::new()?.get(12)?.into_output();

    loop {
        println!("Turning pin 12 on...");
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        println!("Turning pin 12 off...");
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }
}
