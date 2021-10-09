use std::cmp::max;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::time::TimeSpec;
use nix::sys::time::TimeValLike;
use nix::time::clock_gettime;
use nix::time::ClockId;

use crate::motor::Motor;

#[derive(Copy, Clone)]
pub struct NoteInfo {
    pub next_note_index: u32,
    pub motor_id: u8,
    pub enable_motor: bool,
    pub exit: bool,
    pub frequency: u32,
    pub length: u32,
}

pub struct Voice {
    note_index: u32,
    ticks: u64,
    phase: i32,
}

pub fn voice(note_index: u32) -> Voice {
    Voice { note_index, ticks: 0, phase: 0 }
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

pub fn play_note_info_array<M: Motor>(
    pins: &mut [&mut M],
    notes: &mut [NoteInfo],
    voices: &mut [&mut Voice]
) -> Result<(), Box<dyn Error>> {
    let start_time: TimeSpec = now()?;
    let mut next_time: TimeSpec = start_time;

    for pin in &mut *pins { pin.reset(); }

    loop {
        next_time = next_time + TimeSpec::microseconds(TICK_DURATION_MCS.into());
        wait_until(next_time)?;

        for voice in &mut *voices {
            let note: NoteInfo = notes[voice.note_index as usize];

            if note.exit {
                return Ok(());
            }

            if voice.phase >= 0 {
                pins[note.motor_id as usize].advance();
            } else {
                pins[note.motor_id as usize].reset();
            }

            voice.phase = (voice.phase as i64 + note.frequency as i64) as i32;

            voice.ticks += 1;
            if voice.ticks >= note.length as u64 {
                voice.note_index = note.next_note_index;
                voice.ticks = 0;
            }
        }
    }
}
