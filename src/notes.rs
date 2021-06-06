use std::cmp::max;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::time::TimeSpec;
use nix::sys::time::TimeValLike;
use nix::time::clock_gettime;
use nix::time::ClockId;

use rppal::gpio::OutputPin;

#[derive(Copy, Clone)]
pub struct NoteInfo {
    pub next_note_index: u32,
    pub motor_id: u8,
    pub enable_motor: bool,
    pub exit: bool,
    pub frequency: u32,
    pub length: u32,
}

pub const TICK_FREQUENCY_HZ: u32 = 50000;
pub const TICK_DURATION_MCS: u32 = 1000000 / TICK_FREQUENCY_HZ;
pub const FREQ_MULTIPLIER: u32 = (4294967296u64 / TICK_FREQUENCY_HZ as u64) as u32;

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

pub fn play_note(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut last_time: TimeSpec = start_time;
    let end_time: TimeSpec = start_time + TimeSpec::seconds(1);

    while last_time < end_time {
        pulse(pin, &mut last_time, 1000000000 / frequency)?;
    }

    Ok(())
}

pub fn play_note_info(pin: &mut OutputPin, note: NoteInfo) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut next_time: TimeSpec = start_time;
    let mut ticks: u64 = 0;
    let mut phase: i32 = 0;

    pin.set_low();

    while ticks < note.length.into() {
        next_time = next_time + TimeSpec::microseconds(TICK_DURATION_MCS.into());
        wait_until(next_time)?;
        ticks += 1;

        if phase >= 0 {
            pin.set_high();
        } else {
            pin.set_low();
        }

        phase = (phase as i64 + note.frequency as i64) as i32;
    }

    Ok(())
}

pub fn play_p5(pin: &mut OutputPin, frequency: i64) -> Result<(), Box<dyn Error>> {
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
