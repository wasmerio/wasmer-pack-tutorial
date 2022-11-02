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
        let calulator = Calculator::new(5.0);
        let num1 = 5;
        let num2 = 10;
        calulator.add(num1 as f32);
        calulator.add(num2 as f32);
        let result = calulator.current_value();
        let answer = 5.0 + (num1 + num2) as f32;
        assert_eq!(result, answer, "The add function is not working properly");
    }

    #[test]
    fn check_multiply() {
        let calulator = Calculator::new(1.0);
        let num1 = 3;
        let num2 = 4;
        calulator.multiply(num1 as f32);
        calulator.multiply(num2 as f32);
        let result = calulator.current_value();
        let answer = (num1 * num2) as f32;
        assert_eq!(
            result, answer,
            "The multiply function is not working properly"
        );
    }
    #[test]
    fn check_factorial() {
        let calulator = Calculator::new(1.0);
        let num = 6;
        let mut answer = 1;
        for i in 1..=num {
            calulator.multiply(i as f32);
            answer *= i;
        }
        let answer = answer as f32;
        let result = calulator.current_value();
        assert_eq!(
            result, answer,
            "The multiply function is not working properly"
        );
    }
}
