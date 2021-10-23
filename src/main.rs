use std::error::Error;
use std::iter;

use rodio::buffer::SamplesBuffer;
use rodio::OutputStream;
use rodio::Sink;

mod motor;
mod notes;

use crate::motor::TestMotor;
use crate::motor::test_motor;

use crate::notes::DummyTimer;
use crate::notes::NoteInfo;
use crate::notes::play_note_info_array;
use crate::notes::Voice;
use crate::notes::voice;

fn note(next_note_index: u32, motor_id: u8, frequency: u32, length: u32) -> NoteInfo {
    NoteInfo {
        next_note_index,
        motor_id,
        exit: false,
        frequency_mchz: (frequency as u64) * 10000,
        length_mcs: (length as u64) * 10000,
        rearticulate: false,
    }
}

fn play_pachelbel() -> Result<(), Box<dyn Error>> {
    let pins: Vec<TestMotor> = vec![
        test_motor(),
        test_motor(),
    ];

    let voices: Vec<Voice> = vec![voice(0), voice(8)];

    let notes: Vec<NoteInfo> = vec![
        note( 1, 0, 14712, 120), // d
        note( 2, 0, 11000, 120), // a
        note( 3, 0, 12298, 120), // b
        note( 4, 0,  9195, 120), // f# (thanks, Rob Paravonian)
        note( 5, 0,  9839, 120), // g
        note( 6, 0,  7356, 120), // d
        note( 7, 0,  9839, 120), // g
        note( 0, 0, 11000, 120), // a


        note( 9, 1, 73561, 120), // f#
        note(10, 1, 65795, 120), // e
        note(11, 1, 58849, 120), // d
        note(12, 1, 55000, 120), // c#
        note(13, 1, 49193, 120), // b
        note(14, 1, 44000, 120), // a
        note(15, 1, 49193, 120), // b
        note(16, 1, 55000, 120), // c#

        note(17, 1, 58849, 120), // d
        note(18, 1, 55000, 120), // c#
        note(19, 1, 49193, 120), // b
        note(20, 1, 44000, 120), // a
        note(21, 1, 39355, 120), // g
        note(22, 1, 36781, 120), // f#
        note(23, 1, 39355, 120), // g
        note(24, 1, 32898, 120), // e


        note(25, 1, 29425,  60), // d
        note(26, 1, 36781,  60), // f#
        note(27, 1, 44000,  60), // a
        note(28, 1, 39355,  60), // g
        note(29, 1, 36781,  60), // f#
        note(30, 1, 29425,  60), // d
        note(31, 1, 36781,  60), // f
        note(32, 1, 32898,  60), // e

        note(33, 1, 29425,  60), // d
        note(34, 1, 24597,  60), // b
        note(35, 1, 29425,  60), // d
        note(36, 1, 44000,  60), // a
        note(37, 1, 39355,  60), // g
        note(38, 1, 49193,  60), // b
        note(39, 1, 44000,  60), // a
        note(40, 1, 39355,  60), // g


        note(41, 1, 36781,  60), // f#
        note(42, 1, 29425,  60), // d
        note(43, 1, 32898,  60), // e
        note(44, 1, 55000,  60), // c#
        note(45, 1, 58849,  60), // d
        note(46, 1, 73561,  60), // f#
        note(47, 1, 88000,  60), // a
        note(48, 1, 44000,  60), // a

        note(49, 1, 49193,  60), // b
        note(50, 1, 39355,  60), // g
        note(51, 1, 44000,  60), // a
        note(52, 1, 36781,  60), // f#
        note(53, 1, 29425,  60), // d
        note(54, 1, 58849,  60), // d
        // note(55, 1, 58849,  90), // d
        NoteInfo {
            next_note_index: 55,
            motor_id: 1,
            exit: false,
            frequency_mchz: 588_490_000,
            length_mcs: 900_000,
            rearticulate: true,
        },
        note(56, 1, 55000,  30), // c#


        note(57, 1, 58849, 120), // d

        NoteInfo {
            next_note_index: 0,
            motor_id: 1,
            exit: true,
            frequency_mchz: 0,
            length_mcs: 1_000_000,
            rearticulate: false,
        },
    ];

    let mut timer: DummyTimer = DummyTimer { };

    println!("Playing...");
    play_note_info_array(pins, notes, voices, &mut timer)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let wave: Vec<f32> =
        iter::repeat(0.1)
        .take(50)
        .chain(
            iter::repeat(-0.1)
            .take(50))
        .collect();

    let sound: Vec<f32> = wave.iter().copied().cycle().take(44100).collect();

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink: Sink = Sink::try_new(&stream_handle)?;

    let buffer: SamplesBuffer<f32> = SamplesBuffer::new(1, 44100, sound);

    println!("Making a horrible noise...");

    sink.append(buffer);

    sink.sleep_until_end();

    Ok(())
}
