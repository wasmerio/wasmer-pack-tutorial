use std::sync::Mutex;
use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("resources.wai");

pub struct Resources;
impl resources::Resources for Resources {}

pub struct Calculator(Mutex<f32>);

impl Calculator {
    fn new(initial_value: f32) -> Self {
        Calculator(Mutex::new(initial_value))
    }
}

impl resources::Calculator for Calculator {
    fn new(initial_value: f32) -> Handle<Calculator> {
        Handle::new(Calculator::new(initial_value))
    }

    fn current_value(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    fn add(&self, value: f32) {
        *self.0.lock().unwrap() += value;
    }

    fn multiply(&self, value: f32) {
        *self.0.lock().unwrap() *= value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::Calculator as _;

    #[test]
    fn check_add() {
        let calulator = Calculator::new(0.5);
        calulator.add(1.0);
        calulator.add(10.0);
        let result = calulator.current_value();
        let answer = 0.5 + (1.0 + 10.0);
        assert_eq!(result, answer, "The add function is not working properly");
    }

    #[test]
    fn check_multiply() {
        let calulator = Calculator::new(1.0);
        calulator.multiply(3.0);
        calulator.multiply(4.0);
        let result = calulator.current_value();
        let answer = 3.0 * 4.0;
        assert_eq!(
            result, answer,
            "The multiply function is not working properly"
        );
    }
    #[test]
    fn check_factorial() {
        let calulator = Calculator::new(1.0);
        let num = 6;

        for n in 1..=num {
            calulator.multiply(n as f32);
        }

        let answer: i32 = (1..=6).product();
        let result = calulator.current_value();
        assert_eq!(
            result, answer as f32,
            "The multiply function is not working properly"
        );
    }
}
