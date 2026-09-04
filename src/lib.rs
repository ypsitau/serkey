#![no_std]

pub mod vk;

pub struct Context {
}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
    pub fn feed(&mut self, key: u8) -> Option<i32>{
        // Handle the key input here
        None
    }
}

fn sub() {
    vk::A;

}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
