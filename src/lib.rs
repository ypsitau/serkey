#![no_std]

mod win_key_codes;

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
    win_key_codes::VK_A;

}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
