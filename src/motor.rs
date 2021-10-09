use std::error::Error;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

pub trait Motor {
    /// Command the motor to advance one step. 
    fn advance(&mut self);

    /// Prepare the motor to advance another step later.
    fn reset(&mut self);
}

pub struct GpioMotor {
    output_pin: OutputPin,
}

pub fn gpio_motor(pin_number: u8) -> Result<GpioMotor, Box<dyn Error>> {
    let output_pin: OutputPin = Gpio::new()?.get(pin_number)?.into_output();
    Ok(GpioMotor { output_pin })
}

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

#[cfg(test)]
mod tests {
    use crate::motor::*;

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
