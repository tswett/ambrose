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
    pub exit: bool,
    pub frequency_mchz: u64,
    pub length_mcs: u64,
    pub rearticulate: bool,
}

pub struct Voice {
    note_index: u32,
    microseconds: u64,
    phase: u64,
}

pub fn voice(note_index: u32) -> Voice {
    Voice { note_index, microseconds: 0, phase: 500_000_000_000 }
}

const TICK_FREQUENCY_HZ: u64 = 50000;
const TICK_DURATION_MCS: u64 = 1000000 / TICK_FREQUENCY_HZ;

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
        next_time = next_time + TimeSpec::microseconds(TICK_DURATION_MCS as i64);
        wait_until(next_time)?;

        for voice in &mut *voices {
            let note: NoteInfo = notes[voice.note_index as usize];

            if note.exit {
                return Ok(());
            }

            let increment: u64 = note.frequency_mchz * TICK_DURATION_MCS;
            voice.phase = (voice.phase + increment) % 1_000_000_000_000;

            let motor: &mut M = pins[note.motor_id as usize];

            if voice.phase < 500_000_000_000 {
                motor.advance();
            } else {
                motor.reset();
            }

            voice.microseconds += TICK_DURATION_MCS;
            if voice.microseconds >= note.length_mcs {
                voice.note_index = note.next_note_index;
                voice.microseconds -= note.length_mcs;

                if notes[voice.note_index as usize].rearticulate {
                    voice.phase = (voice.phase + 500_000_000_000) % 1_000_000_000_000;
                }
            }
        }
    }
}
