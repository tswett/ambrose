pub trait Motor {
    /// Command the motor to advance one step. 
    fn advance(&mut self);

    /// Prepare the motor to advance another step later.
    fn reset(&mut self);
}

pub struct TestMotor {
    count: u64,
}

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
    fn can_create_test_motor() {
        let mut motor: TestMotor = test_motor();

        motor.advance();
        motor.reset();
    }
}
