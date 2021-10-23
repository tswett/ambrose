use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Clone)]
pub struct SimpleAudioMotor {
    pub is_high: Rc<RefCell<bool>>,
}

impl SimpleAudioMotor {
    pub fn new() -> Self {
        SimpleAudioMotor { is_high: Rc::new(RefCell::new(false)) }
    }
}

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

    #[test]
    fn can_create_test_motor() {
        let mut motor: TestMotor = test_motor();

        motor.advance();
        motor.reset();
    }
}
