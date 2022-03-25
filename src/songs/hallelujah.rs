use crate::notes::NoteInfo;
use crate::songbuilder::SongBuilder;

const BEAT_DURATION: u64 = 62500;

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

pub fn build_song() -> SongBuilder {
    let mut b: SongBuilder = SongBuilder::new();

    b.add(0, note(1, 11, 20));           b.add(1, note(3,  6,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  6,  4));
                                         b.add(1, note(4,  3,  4));
    b.add(0, note(1, 10,  4));           b.add(1, note(3, 11,  4));

    b.add(0, note(1,  8, 14));           b.add(1, note(3,  8,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  8,  4));
    b.add(0, note(1,  8,  2).rest());
    b.add(0, note(1,  8,  4));           b.add(1, note(4,  3,  4));
    b.add(0, note(1, 10,  4));           b.add(1, note(3, 11,  4));



    b.add(0, note(1, 11, 14));           b.add(1, note(3,  6,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  6,  4));
    b.add(0, note(1, 11,  2).rest());    
    b.add(0, note(1, 11,  4));           b.add(1, note(4,  3,  4));
    b.add(0, note(1, 10,  4));           b.add(1, note(3, 11,  4));
                                         
    b.add(0, note(1,  8, 18));           b.add(1, note(3,  8,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  8,  4));
                                         b.add(1, note(4,  3,  4));
    b.add(0, note(1,  8,  2).rest());
    b.add(0, note(1,  8,  4));           b.add(1, note(3, 11,  4));



    b.add(0, note(2,  1, 18));           b.add(1, note(3,  8,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  4,  4));
                                         b.add(1, note(4,  8,  4));
                                         b.add(1, note(4,  4,  4));
    b.add(0, note(2,  1,  2).rest());    
    b.add(0, note(2,  1,  4));           b.add(1, note(3, 11,  4));

    b.add(0, note(1,  6, 18));           b.add(1, note(3,  6,  4));
                                         b.add(1, note(3, 10,  4));
                                         b.add(1, note(4,  1,  4));
                                         b.add(1, note(4,  6,  4));
                                         b.add(1, note(4,  1,  4));
    b.add(0, note(1,  6,  2).rest());    
    b.add(0, note(1,  6,  4));           b.add(1, note(3, 10,  4));



    b.add(0, note(1, 11, 16));           b.add(1, note(3,  6,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  6,  4));
    b.add(0, note(1,  3,  2));           b.add(1, note(4,  3,  4));
    b.add(0, note(1,  3,  2).rest());
    b.add(0, note(1,  4,  2));           b.add(1, note(3, 11,  4));
    b.add(0, note(1,  4,  2).rest());

    b.add(0, note(1,  6, 10));           b.add(1, note(3,  6,  4));
                                         b.add(1, note(3, 10,  4));
                                         b.add(1, note(4,  1,  4));
    b.add(0, note(1,  6,  2).rest());
    b.add(0, note(1,  6, 12));           b.add(1, note(4,  6,  4));
                                         b.add(1, note(4,  1,  4));
                                         b.add(1, note(3, 10,  4));







    b.add(0, note(1, 11, 24));           b.add(1, note(3,  6,  4));
                                         b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  6,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(3, 11,  4));

    b.add(0, note(1,  4, 12));           b.add(1, note(3, 11,  4).kick());
                                         b.add(1, note(4,  4,  4));
                                         b.add(1, note(4,  8,  4));
    b.add(0, note(1,  6, 12));           b.add(1, note(4,  1,  4));
                                         b.add(1, note(4,  6,  4));
                                         b.add(1, note(4, 10,  4));



    b.add(0, note(1,  8, 14));           b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  8,  4));
                                         b.add(1, note(4, 11,  4));
                                         b.add(1, note(4, 11,  4).kick());
    b.add(0, note(1,  8,  2).rest());
    b.add(0, note(1,  8,  2));           b.add(1, note(4,  8,  4));
    b.add(0, note(1,  8,  2).rest());
    b.add(0, note(1,  6,  2));           b.add(1, note(4,  3,  4));
    b.add(0, note(1,  6,  2).rest());

    b.add(0, note(1,  4, 18));           b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  4,  4));
                                         b.add(1, note(4,  8,  4));
                                         b.add(1, note(4, 11,  4));
                                         b.add(1, note(4,  8,  4));
    b.add(0, note(1,  4,  2).rest());
    b.add(0, note(1,  4,  4));           b.add(1, note(4,  4,  4));



    b.add(0, note(1,  6, 20));           b.add(1, note(4,  1,  4));
                                         b.add(1, note(4,  6,  4));
                                         b.add(1, note(4, 10,  4));
                                         b.add(1, note(5,  1,  4));
                                         b.add(1, note(4, 10,  4));
    b.add(0, note(1,  6,  4).rest());    b.add(1, note(4,  6,  4));

    b.add(0, note(1,  3,  8));           b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  7,  4));
    b.add(0, note(1,  3,  4).rest());    b.add(1, note(4, 10,  4));
    b.add(0, note(1,  3,  8));           b.add(1, note(5,  3,  4));
    b.add(0, note(1,  3,  4).rest());    b.add(1, note(4, 10,  4));
                                         b.add(1, note(4,  7,  4));



    b.add(0, note(1,  8, 48));           b.add(1, note(3, 11,  4));
                                         b.add(1, note(4,  3,  4));
                                         b.add(1, note(4,  8,  4));
                                         b.add(1, note(4, 11,  4));
                                         b.add(1, note(4,  8,  4));
                                         b.add(1, note(4,  3,  4));

                                         b.add(1, note(4, 11, 24));

    b.add(0, note(1,  8, 24).exit());

    b
}
