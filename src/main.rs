use std::cmp::max;
use std::error::Error;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::time::TimeSpec;
use nix::sys::time::TimeValLike;
use nix::time::clock_gettime;
use nix::time::ClockId;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut pin: OutputPin = Gpio::new()?.get(12)?.into_output();

    loop {
        println!("Turning pin 12 on...");
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        println!("Turning pin 12 off...");
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
        println!("Playing an A...");
        play_a110(&mut pin)?;
    }
}

fn now() -> Result<TimeSpec, nix::Error> {
    // return clock_gettime(ClockId::CLOCK_PROCESS_CPUTIME_ID);
    clock_gettime(ClockId::CLOCK_MONOTONIC)
}

fn wait_until(target_time: TimeSpec) -> Result<(), Box<dyn Error>> {
    Ok(sleep(Duration::from(max(target_time - now()?, TimeSpec::seconds(0)))))
}

fn play_a110(pin: &mut OutputPin) -> Result<(), Box<dyn Error>> {
    let frequency = 12800;
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        // println!("last_time: {}", last_time);
        // println!("now: {}", now()?);
        let mut next_time: TimeSpec = last_time + TimeSpec::nanoseconds(1000000000 / frequency / 2);
        wait_until(next_time)?;
        pin.set_high();
        next_time = next_time + TimeSpec::nanoseconds(1000000000 / frequency / 2);
        wait_until(next_time)?;
        pin.set_low();
        last_time = next_time;
    }

    Ok(())
}
