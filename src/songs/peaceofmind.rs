use std::error::Error;

/*
use crate::motor::GpioMotor;
use crate::motor::gpio_motor;
*/

use crate::notes::NoteInfo;
use crate::notes::play_note_info_array;
use crate::notes::Voice;
use crate::notes::voice;

pub struct SongBuilder {
    pub notes: Vec<NoteInfo>,
    pub voices: Vec<VoiceInfo>,
}

pub struct VoiceInfo {
    pub first_note_index: u32,
    last_note_index: u32,
}

impl SongBuilder {
    fn new() -> Self {
        SongBuilder { notes: Vec::<NoteInfo>::new(), voices: Vec::<VoiceInfo>::new() }
    }

    fn add(&mut self, voice: u8, note: NoteInfo) {
        let new_note_index: u32 = self.notes.len() as u32;

        let new_voice: bool = voice as usize == self.voices.len();

        if new_voice {
            let new_voice = VoiceInfo {
                first_note_index: new_note_index,
                last_note_index: new_note_index,
            };

            self.voices.push(new_voice);
        }

        let last_note_index: u32 = self.voices[voice as usize].last_note_index;

        if !new_voice {
            self.notes[last_note_index as usize].next_note_index = new_note_index;
        }

        self.notes.push(NoteInfo { motor_id: voice, ..note });

        self.voices[voice as usize].last_note_index = new_note_index;
    }
}

const BEAT_DURATION: u64 = 117188;

fn note(octave: i16, pitch: i16, duration: u64) -> NoteInfo {
    let pitch_from_a440: i16 = octave * 12 + pitch - 57;
    let frequency_hz: f64 = 440.0 * (2.0_f64.powf(pitch_from_a440 as f64 / 12.0));
    let frequency_mchz: u64 = (frequency_hz * 1_000_000.0).round() as u64;

    NoteInfo {
        next_note_index: 0,
        motor_id: 0,
        exit: false,
        frequency_mchz,
        length_mcs: duration * BEAT_DURATION,
        rearticulate: true,
    }
}

impl NoteInfo {
    fn slur(self) -> Self {
        NoteInfo { rearticulate: false, ..self }
    }

    fn kick(self) -> Self {
        NoteInfo { rearticulate: true, ..self }
    }

    fn rest(self) -> Self {
        NoteInfo { frequency_mchz: 0, ..self }
    }

    fn exit(self) -> Self {
        NoteInfo { exit: true, ..self }
    }
}

/*
fn main() -> Result<(), Box<dyn Error>> {
    let builder: SongBuilder = build_song();

    let pins: Vec<GpioMotor> = vec![
        gpio_motor(15)?,
        gpio_motor(14)?,
    ];

    let voices: Vec<Voice> = builder.voices.into_iter().map(|v| voice(v.first_note_index)).collect();

    let notes: Vec<NoteInfo> = builder.notes;

    println!("Playing...");
    play_note_info_array(pins, notes, voices)?;
    Ok(())
}
*/

pub fn build_song() -> SongBuilder {
    let mut b: SongBuilder = SongBuilder::new();

    b.add(0, note(2,  1,  4));           b.add(1, note(4,  8,  2));        // I
                                         b.add(1, note(4,  8,  2).kick()); // un-

    b.add(0, note(2,  4,  2));           b.add(1, note(4,  8,  2).kick()); // der-
    b.add(0, note(1,  9,  2));           b.add(1, note(4,  8,  2).kick()); // stand

    b.add(0, note(1,  9,  2).slur());    b.add(1, note(4,  8,  2).slur());
    b.add(0, note(1,  9,  2).kick());    b.add(1, note(4,  6,  2));        // a-

    b.add(0, note(1, 11,  2));           b.add(1, note(4,  4,  4));        // bout
    b.add(0, note(2,  3,  2));



    b.add(0, note(2,  4,  4));           b.add(1, note(4,  8,  2));        // in-
                                         b.add(1, note(4,  8,  2).kick()); // de-

    b.add(0, note(2,  6,  2));           b.add(1, note(4,  8,  2).slur());
    b.add(0, note(1, 11,  2));           b.add(1, note(4,  6,  2));        // ci-

    b.add(0, note(1, 11,  2).slur());    b.add(1, note(4,  6,  2).slur());
    b.add(0, note(2,  8,  2));           b.add(1, note(4,  3,  1));        // sion
                                         b.add(1, note(4,  1,  1).slur());

    b.add(0, note(2,  6,  4));           b.add(1, note(3, 11,  2).slur());
                                         b.add(1, note(3, 11,  2).kick()); // but



    b.add(0, note(2,  1,  4));           b.add(1, note(4,  8,  2));        // I      
                                         b.add(1, note(4,  8,  2).kick()); // don't  
                                                                                 
    b.add(0, note(2,  4,  2));           b.add(1, note(4,  8,  2).slur());
    b.add(0, note(1,  9,  2));           b.add(1, note(4,  8,  2).kick()); // care  
                                                                                 
    b.add(0, note(1,  9,  2).slur());    b.add(1, note(4,  8,  2).slur());           
    b.add(0, note(1,  8,  2));           b.add(1, note(4,  6,  2));        // if

    b.add(0, note(1,  8,  2).slur());    b.add(1, note(4,  4,  4));        // I   
    b.add(0, note(1,  6,  2));



    b.add(0, note(1,  4,  4));           b.add(1, note(4,  8,  2));        // get
                                         b.add(1, note(4,  8,  2).kick()); // be-

    b.add(0, note(1,  8,  2));           b.add(1, note(4,  9,  2).slur());             
    b.add(0, note(1, 11,  2));           b.add(1, note(4,  6,  2));        // hind

    b.add(0, note(1, 11,  2).slur());    b.add(1, note(4,  6,  4).slur());             
    b.add(0, note(2,  3,  2));           

    b.add(0, note(2,  4,  1));           b.add(1, note(0,  0,  4).rest());
    b.add(0, note(2,  3,  1));
    b.add(0, note(1, 11,  2));



    b.add(0, note(2,  1,  4));           b.add(1, note(4,  8,  2));        // peo-
                                         b.add(1, note(4,  8,  2).kick()); // ple

    b.add(0, note(2,  4,  2));           b.add(1, note(4,  8,  2).slur());
    b.add(0, note(1,  9,  2));           b.add(1, note(4,  9,  2));        // liv-

    b.add(0, note(1,  9,  2).slur());    b.add(1, note(4,  9,  2).slur());
    b.add(0, note(2,  1,  2));           b.add(1, note(4,  9,  2).kick()); // ing

    b.add(0, note(2,  1,  2).slur());    b.add(1, note(4,  9,  4).kick()); // in
    b.add(0, note(2,  3,  2));



    b.add(0, note(2,  4,  4));           b.add(1, note(4,  8,  2));        // com-
                                         b.add(1, note(4,  8,  2).kick()); // pe-

    b.add(0, note(2,  6,  2));           b.add(1, note(4,  8,  2).slur());
    b.add(0, note(1, 11,  2));           b.add(1, note(4,  8,  2).kick()); // ti-

    b.add(0, note(1, 11,  2).slur());    b.add(1, note(4,  8,  2).slur());
    b.add(0, note(2,  8,  2));           b.add(1, note(4,  6,  2));        // tion

    b.add(0, note(2,  6,  4));           b.add(1, note(4,  6,  2).slur());
                                         b.add(1, note(0,  0,  2).rest());



    b.add(0, note(2,  1,  4));           b.add(1, note(4, 11,  2));        // all
                                         b.add(1, note(4, 11,  2).kick()); // I

    b.add(0, note(2,  4,  2));           b.add(1, note(4, 11,  2).slur());
    b.add(0, note(1,  9,  2));           b.add(1, note(4,  6,  2).kick()); // want

    b.add(0, note(1,  9,  2).slur());    b.add(1, note(4,  6,  2).slur());
    b.add(0, note(1,  9,  2).kick());    b.add(1, note(4,  4,  2));        // is

    b.add(0, note(2,  1,  2));           b.add(1, note(4,  4,  4));        // to
    b.add(0, note(2,  3,  2));



    b.add(0, note(2,  4,  4));           b.add(1, note(4,  8,  2));        // have
                                         b.add(1, note(4,  8,  2).kick()); // my

    b.add(0, note(2,  6,  2));           b.add(1, note(4,  8,  2).slur());
    b.add(0, note(1, 11,  2));           b.add(1, note(4,  6,  2));        // peace

    b.add(0, note(1, 11,  2).slur());    b.add(1, note(4,  6,  2).slur());
    b.add(0, note(2,  4,  2));           b.add(1, note(4,  8,  2));        // of

    b.add(0, note(2,  3,  2));           b.add(1, note(4,  6,  2).slur());
    b.add(0, note(1, 11,  2));           b.add(1, note(4,  4,  2).slur());



    b.add(0, note(1,  9, 18));           b.add(1, note(4,  4, 18).kick()); // mind
    b.add(0, note(1,  9, 1).exit());

    b
}
