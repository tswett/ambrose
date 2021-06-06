use std::error::Error;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

mod notes;

use crate::notes::FREQ_MULTIPLIER;
use crate::notes::NoteInfo;
use crate::notes::play_note_info;
use crate::notes::TICK_FREQUENCY_HZ;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut pin: OutputPin = Gpio::new()?.get(12)?.into_output();

    loop {
        println!("Playing...");
        for freq in &[294, 220, 246, 184, 197, 147, 197, 220] {
            play_note_info(&mut pin, NoteInfo {
                next_note_index: 0,
                motor_id: 0,
                enable_motor: true,
                exit: false,
                frequency: (freq * FREQ_MULTIPLIER) / 2,
                length: 12 * TICK_FREQUENCY_HZ / 10,
            })?;
        }
    }
}
