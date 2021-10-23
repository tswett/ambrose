use std::iter::repeat;

use crate::motor::SimpleAudioMotor;

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

pub struct SimpleAudioTimer {
    sample_rate: u32,
    motors: Vec<SimpleAudioMotor>,
    time_mcs: u64,
    pub data: Vec<f32>,
}

impl SimpleAudioTimer {
    pub fn new(sample_rate: u32, motors: &Vec<SimpleAudioMotor>) -> Self {
        SimpleAudioTimer {
            sample_rate,
            motors: motors.clone(),
            time_mcs: 0,
            data: vec![]
        }
    }
}

impl Timer for SimpleAudioTimer {
    fn wait_microseconds(&mut self, duration: u64) {
        let old_sample_count: u64 =
            (self.sample_rate as u64) * self.time_mcs / 1_000_000;
        self.time_mcs += duration;
        let new_sample_count: u64 =
            (self.sample_rate as u64) * self.time_mcs / 1_000_000;

        let samples_to_add: u64 = new_sample_count - old_sample_count;

        let mut new_amplitude: f32 = 0.0;

        for motor in self.motors.iter() {
            if *motor.is_high.borrow() {
                new_amplitude += 0.1;
            } else {
                new_amplitude -= 0.1;
            }
        }

        self.data.extend(repeat(new_amplitude).take(samples_to_add as usize));
    }
}
