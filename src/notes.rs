use std::error::Error;

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

pub trait Timer {
    /// Tell the timer to wait the given number of microseconds.
    /// The timer is expected to keep track of a "target time," and this
    /// function is expected to advance the target time by the given number of
    /// microseconds, then wait until the target time. This way, if, for
    /// example, we call
    ///
    ///   timer.wait_microseconds(10_000);
    ///
    /// 360,000 times, then all of the calls combined will take almost exactly
    /// 3,600,000,000 microseconds, which is to say, one hour.
    fn wait_microseconds(&mut self, duration: u64);
}

pub struct DummyTimer { }

impl Timer for DummyTimer {
    fn wait_microseconds(&mut self, _duration: u64) { }
}

pub fn play_note_info_array<M: Motor, T: Timer>(
    mut pins: Vec<M>,
    notes: Vec<NoteInfo>,
    mut voices: Vec<Voice>,
    timer: &mut T
) -> Result<(), Box<dyn Error>> {
    for pin in &mut *pins { pin.reset(); }

    loop {
        timer.wait_microseconds(TICK_DURATION_MCS);

        for voice in &mut *voices {
            // println!("playing note {}", voice.note_index);
            let note: NoteInfo = notes[voice.note_index as usize];

            if note.exit {
                return Ok(());
            }

            let increment: u64 = note.frequency_mchz * TICK_DURATION_MCS;
            voice.phase = (voice.phase + increment) % 1_000_000_000_000;

            let motor: &mut M = &mut pins[note.motor_id as usize];

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

                // println!("moving on to note {}, frequency {}", voice.note_index, notes[voice.note_index as usize].frequency_mchz);
            }
        }
    }
}
