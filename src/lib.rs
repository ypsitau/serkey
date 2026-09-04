#![no_std]

pub struct Context {
}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}


#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
