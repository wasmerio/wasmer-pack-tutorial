// src/lib.rs

wai_bindgen_rust::export!("hello-world.wai");

pub struct HelloWorld;

impl hello_world::HelloWorld for HelloWorld {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use hello_world::HelloWorld as _;

    #[test]
    fn add_two_numbers() {
        let result = HelloWorld::add(2, 2);
        assert_eq!(result, 4);
    }
}
