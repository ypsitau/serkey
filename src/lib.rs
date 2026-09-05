#![no_std]
use heapless::Vec;

pub mod vk;

use vk::VK;

#[derive(Debug, PartialEq, Eq)]
enum Stat {
    First,
    Escape,
    SS2,
    SS3,
    DCS,
    CsiParameter,
    CsiIntermediate,
    CsiFinal,
    ST,
    OSC,
    SOS,
    PM,
    APC,
}

pub struct Context {
    stat: Stat,
    buf_parameter: Vec::<u8, 32>,
    buf_intermediate: Vec::<u8, 32>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            stat: Stat::First,
            buf_parameter: Vec::new(),
            buf_intermediate: Vec::new(),
        }
    }
    pub fn feed(&mut self, byte: u8) -> VK {
        let mut cont_flag = true;
        let mut vk = VK::None;
        while cont_flag {
            cont_flag = false;
            match self.stat {
                Stat::First => {
                    self.stat = match byte {
                        0x08 => Stat::First,
                        0x1b => Stat::Escape,
                        0x7f => Stat::First,
                        _ => Stat::First,
                    };
                }
                Stat::Escape => {
                    self.stat = match byte {
                        0x1b => Stat::First,
                        0x4e => Stat::SS2,           // 'N'
                        0x4f => Stat::SS3,           // 'O'
                        0x50 => Stat::DCS,           // 'P'
                        0x58 => Stat::SOS,           // 'X'
                        0x5b => {
                            self.buf_parameter.clear();
                            Stat::CsiParameter
                        },                           // '['
                        0x5c => Stat::ST,            // '\'
                        0x5d => Stat::OSC,           // ']'
                        0x5e => Stat::PM,            // '^'
                        0x5f => Stat::APC,           // '_'
                        _ => Stat::First,
                    };
                    // Handle the Escape state
                }
                Stat::SS2 => {
                    self.stat = Stat::First;
                }
                Stat::SS3 => {
                    self.stat = Stat::First;
                }
                Stat::DCS => {
                    self.stat = Stat::First;
                }
                Stat::SOS => {
                }
                Stat::CsiParameter => {
                    self.stat = if 0x30 <= byte && byte <= 0x39 {
                        self.buf_parameter.push(byte).ok();
                        Stat::CsiParameter
                    } else {
                        cont_flag = true;
                        Stat::CsiIntermediate
                    }
                }
                Stat::CsiIntermediate => {
                    self.stat = if 0x20 <= byte && byte <= 0x2f {
                        self.buf_intermediate.push(byte).ok();
                        Stat::CsiIntermediate
                    } else {
                        cont_flag = true;
                        Stat::CsiFinal
                    }
                }
                Stat::CsiFinal => {
                    match byte {
                        0x41 => {           // 'A'
                            vk = VK::UP;
                        }
                        0x42 => {           // 'B'
                            vk = VK::DOWN;
                        }
                        0x43 => {           // 'C'
                            vk = VK::RIGHT;
                        }
                        0x44 => {           // 'D'
                            vk = VK::LEFT;
                        }
                        0x7e => {           // '~'
                            if self.buf_parameter.eq(&[0x31]) {         // '1'
                                vk = VK::HOME;
                            } else if self.buf_parameter.eq(&[0x34]) {  // '4'
                                vk = VK::END;
                            } else if self.buf_parameter.eq(&[0x35]) {  // '5'
                                vk = VK::PRIOR;
                            } else if self.buf_parameter.eq(&[0x36]) {  // '6'
                                vk = VK::NEXT;
                            }
                        }
                        _ => {}
                    }
                    self.stat = Stat::First;
                }
                Stat::ST => {
                    self.stat = Stat::First;
                }
                Stat::OSC => {
                    self.stat = Stat::First;
                }
                Stat::PM => {
                    self.stat = Stat::First;
                }
                Stat::APC => {
                    self.stat = Stat::First;
                }
            }
        }
        // Handle the key input here
        vk
    }
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
