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

        let fundamental: i64 = 100;
        println!("Playing notes...");
        play_note(&mut pin, fundamental * 2)?;
        play_note(&mut pin, fundamental * 3)?;
        println!("Playing a perfect fifth (maybe)...");
        play_p5(&mut pin, fundamental)?;
    }
}

fn now() -> Result<TimeSpec, nix::Error> {
    // return clock_gettime(ClockId::CLOCK_PROCESS_CPUTIME_ID);
    clock_gettime(ClockId::CLOCK_MONOTONIC)
}

fn wait_until(target_time: TimeSpec) -> Result<(), Box<dyn Error>> {
    Ok(sleep(Duration::from(max(target_time - now()?, TimeSpec::seconds(0)))))
}

fn pulse(pin: &mut OutputPin, control_time: &mut TimeSpec, delay_ns: i64) -> Result<(), Box<dyn Error>> {
    *control_time = *control_time + TimeSpec::nanoseconds(delay_ns);
    wait_until(*control_time)?;
    pin.set_high();
    sleep(Duration::from_micros(2));
    pin.set_low();
    Ok(())
}

fn play_note(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency)?;
    }

    Ok(())
}

fn play_p5(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency / 4)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 12)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 12)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 4)?;
        pulse(pin, &mut last_time, 0)?;
    }

    Ok(())
}

fn play_p5_v2(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
        pulse(pin, &mut last_time, 0)?;
        pulse(pin, &mut last_time, 1000000000 / frequency * 2 / 3)?;
        pulse(pin, &mut last_time, 0)?;
    }

    Ok(())
}

fn play_p5_v3(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
        pulse(pin, &mut last_time, 0)?;
    }

    Ok(())
}

fn play_p5_v4(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 6)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 6)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 3)?;
    }

    Ok(())
}

fn play_maj3(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency / 5)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 20)?;
        pulse(pin, &mut last_time, 1000000000 / frequency * 3 / 20)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 10)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 10)?;
        pulse(pin, &mut last_time, 1000000000 / frequency * 3 / 20)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 20)?;
        pulse(pin, &mut last_time, 1000000000 / frequency / 5)?;
    }

    Ok(())
}
