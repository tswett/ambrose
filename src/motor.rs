#[cfg(not(feature = "raspi"))]
use std::{
    cell::RefCell,
    rc::Rc,
};
#[cfg(feature = "raspi")]
use std::error::Error;

#[cfg(feature = "raspi")]
use rppal::{
    gpio::Gpio,
    gpio::OutputPin,
};

pub trait Motor {
    /// Command the motor to advance one step. 
    fn advance(&mut self);

    /// Prepare the motor to advance another step later.
    fn reset(&mut self);
}

#[cfg(feature = "raspi")]
pub struct GpioMotor {
    output_pin: OutputPin,
}

#[cfg(feature = "raspi")]
pub fn gpio_motor(pin_number: u8) -> Result<GpioMotor, Box<dyn Error>> {
    let output_pin: OutputPin = Gpio::new()?.get(pin_number)?.into_output();
    Ok(GpioMotor { output_pin })
}

#[cfg(feature = "raspi")]
impl Motor for GpioMotor {
    fn advance(&mut self) {
        self.output_pin.set_high();
    }

    fn reset(&mut self) {
        self.output_pin.set_low();
    }
}

pub struct TestMotor {
    count: u64,
}

#[cfg(test)]
pub fn test_motor() -> TestMotor {
    TestMotor { count: 0 }
}

impl Motor for TestMotor {
    fn advance(&mut self) {
        self.count += 1;
    }

    fn reset(&mut self) { }
}

#[cfg(not(feature = "raspi"))]
#[derive(Clone)]
pub struct SimpleAudioMotor {
    pub is_high: Rc<RefCell<bool>>,
}

#[cfg(not(feature = "raspi"))]
impl SimpleAudioMotor {
    pub fn new() -> Self {
        SimpleAudioMotor { is_high: Rc::new(RefCell::new(false)) }
    }
}

#[cfg(not(feature = "raspi"))]
impl Motor for SimpleAudioMotor {
    fn advance(&mut self) {
        *(self.is_high.borrow_mut()) = true;
    }

    fn reset(&mut self) {
        *(self.is_high.borrow_mut()) = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::motor::*;

    #[cfg(feature = "raspi")]
    #[test]
    fn can_create_gpio_motor() -> Result<(), Box<dyn Error>> {
        let _motor: GpioMotor = gpio_motor(14)?;
        Ok(())
    }

    #[test]
    fn can_create_test_motor() {
        let mut motor: TestMotor = test_motor();

        motor.advance();
        motor.reset();
    }
}
