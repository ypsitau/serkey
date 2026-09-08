//! A simple keycode parser for handling UTF-8 and VT100 terminal control sequences.
//!
//! # Example
//! 
//! Create a new parser instance using `serkey::Parser::new()`.
//!
//! ```rust
//! let mut parser = serkey::Parser::new();
//! ```
//! Call `serkey::Parser::push()` to feed bytes into the parser and `serkey::Parser::next_keycode()` to retrieve parsed keycodes.
//! In the following example, assume that `buf` contains the bytes read from the serial device such as UART, USB CDC, and network sockets.
//!
//! ```rust
//! fn feed_parser(parser: &mut serkey::Parser, buf: &[u8]) {
//!     for &byte in buf {
//!         parser.push(byte);
//!         while let Some(keycode) = parser.next_keycode() {
//!             match keycode {
//!                 serkey::Vk::CookedChar(ch) => { info!("CookedChar: {}", ch); }
//!                 serkey::Vk::CookedCtrl(n) => { info!("CookedCtrl: 0x{:02x}", n); }
//!                 serkey::Vk::Backspace => { info!("Backspace"); }
//!                 serkey::Vk::Enter => { info!("Enter"); }
//!                 serkey::Vk::Left => { info!("Left"); }
//!                 serkey::Vk::Right => { info!("Right"); }
//!                 serkey::Vk::Up => { info!("Up"); }
//!                 serkey::Vk::Down => { info!("Down"); }
//!                 serkey::Vk::Home => { info!("Home"); }
//!                 serkey::Vk::End => { info!("End"); }
//!                 serkey::Vk::PageUp => { info!("PageUp"); }
//!                 serkey::Vk::PageDown => { info!("PageDown"); }
//!                 serkey::Vk::Tab => { info!("Tab"); }
//!                 serkey::Vk::BackTab => { info!("BackTab"); }
//!                 serkey::Vk::Delete => { info!("Delete"); }
//!                 serkey::Vk::Insert => { info!("Insert"); }
//!                 serkey::Vk::F(n) => { info!("F{}", n); }
//!                 serkey::Vk::Null => { info!("Null"); }
//!                 serkey::Vk::Esc => { info!("Esc"); }
//!                 serkey::Vk::ShiftLeft => { info!("ShiftLeft"); }
//!                 serkey::Vk::ShiftRight => { info!("ShiftRight"); }
//!                 serkey::Vk::ShiftUp => { info!("ShiftUp"); }
//!                 serkey::Vk::ShiftDown => { info!("ShiftDown"); }
//!                 serkey::Vk::ShiftHome => { info!("ShiftHome"); }
//!                 serkey::Vk::ShiftEnd => { info!("ShiftEnd"); }
//!                 serkey::Vk::ShiftPageUp => { info!("ShiftPageUp"); }
//!                 serkey::Vk::ShiftPageDown => { info!("ShiftPageDown"); }
//!                 serkey::Vk::ShiftDelete => { info!("ShiftDelete"); }
//!                 serkey::Vk::ShiftInsert => { info!("ShiftInsert"); }
//!                 serkey::Vk::ShiftF(n) => { info!("ShiftF{}", n); }
//!             }
//!         }
//!     }
//! }
//! ```
#![no_std]
mod keycode;
use heapless::Vec;
use heapless::spsc::Queue;
pub use keycode::{Vk, Modifier};

#[derive(Debug, PartialEq, Eq)]
enum Stat {
    FirstByte,
    Utf8Follow,
    AfterLF,
    AfterCR,
    Escape,
    SS2,
    SS3,
    DCS,
    CsiParameterTop,
    CsiParameter,
    CsiIntermediate,
    CsiFinal,
    ST,
    OSC,
    SOS,
    PM,
    APC,
}

pub struct Parser {
    queue: Queue<Vk, 8>,
    stat: Stat,
    param_accum: u16,
    params: Vec::<u16, 4>,
    buf_intermediate: Vec::<u8, 32>,
    utf8_accum: u32,
    utf8_remain: u8,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            queue: Queue::new(),
            stat: Stat::FirstByte,
            param_accum: 0,
            params: Vec::new(),
            buf_intermediate: Vec::new(),
            utf8_accum: 0,
            utf8_remain: 0,
        }
    }
    pub fn next_keycode(&mut self) -> Option<Vk> {
        self.queue.dequeue()
    }
    fn gen_keycode(&mut self, vk: Vk) {
        self.queue.enqueue(vk).ok();
    }
    pub fn push(&mut self, byte: u8) {
        let mut cont_flag = true;
        while cont_flag {
            cont_flag = false;
            match self.stat {
                Stat::FirstByte => {
                    let attr = Modifier::default();
                    self.stat = match byte {
                        0x08 => {
                            self.gen_keycode(Vk::Back(attr.into()));
                            Stat::FirstByte
                        },
                        0x09 => {
                            self.gen_keycode(Vk::Tab(attr.into()));
                            Stat::FirstByte
                        },
                        0x0a => {
                            self.gen_keycode(Vk::Return(attr.into()));
                            Stat::AfterLF
                        },
                        0x0d => {
                            self.gen_keycode(Vk::Return(attr.into()));
                            Stat::AfterCR
                        },
                        0x1b => Stat::Escape,
                        0x7f => {
                            self.gen_keycode(Vk::Delete(attr.into()));
                            Stat::FirstByte
                        },
                        byte => {
                            if byte < 0x20 {
                                self.gen_keycode(Vk::CookedCtrl(byte));
                                Stat::FirstByte
                            } else if byte < 0x80 {
                                self.gen_keycode(Vk::CookedChar(char::from(byte)));
                                Stat::FirstByte
                            } else if byte & 0xe0 == 0xc0 {
                                // Start of a 2-byte UTF-8 sequence
                                self.utf8_accum = (byte & 0x1f) as u32;
                                self.utf8_remain = 1;
                                Stat::Utf8Follow
                            } else if byte & 0xf0 == 0xe0 {
                                // Start of a 3-byte UTF-8 sequence
                                self.utf8_accum = (byte & 0x0f) as u32;
                                self.utf8_remain = 2;
                                Stat::Utf8Follow
                            } else if byte & 0xf8 == 0xf0 {
                                // Start of a 4-byte UTF-8 sequence
                                self.utf8_accum = (byte & 0x07) as u32;
                                self.utf8_remain = 3;
                                Stat::Utf8Follow
                            } else {
                                Stat::FirstByte
                            }
                        }
                    };
                }
                Stat::Utf8Follow => {
                    self.stat = if byte & 0xc0 == 0x80 {
                        self.utf8_accum = (self.utf8_accum << 6) | (byte & 0x3f) as u32;
                        self.utf8_remain -= 1;
                        if self.utf8_remain == 0 {
                            if let Some(ch) = char::from_u32(self.utf8_accum) {
                                self.gen_keycode(Vk::CookedChar(ch));
                            }
                            Stat::FirstByte
                        } else {
                            Stat::Utf8Follow
                        }
                    } else {
                        // Invalid UTF-8 continuation byte
                        Stat::FirstByte
                    };
                }
                Stat::AfterLF => {
                    self.stat = match byte {
                        0x0d => Stat::FirstByte,
                        _ => {
                            cont_flag = true;
                            Stat::FirstByte
                        },
                    };
                }
                Stat::AfterCR => {
                    self.stat = match byte {
                        0x0a => Stat::FirstByte,
                        _ => {
                            cont_flag = true;
                            Stat::FirstByte
                        },
                    };
                }
                Stat::Escape => {
                    let attr = Modifier::default();
                    self.stat = match byte {
                        0x1b => {
                            self.gen_keycode(Vk::Escape(attr.into()));
                            Stat::FirstByte
                        },
                        b'N' => Stat::SS2,                          // 0x4e
                        b'O' => Stat::SS3,                          // 0x4f
                        b'P' => Stat::DCS,                          // 0x50
                        b'X' => Stat::SOS,                          // 0x58
                        b'[' => {                                   // 0x5b
                            self.param_accum = 0;
                            self.params.clear();
                            self.buf_intermediate.clear();
                            Stat::CsiParameterTop
                        },
                        b'\\' => Stat::ST,                          // 0x5c
                        b']' => Stat::OSC,                          // 0x5d
                        b'^' => Stat::PM,                           // 0x5e
                        b'_' => Stat::APC,                          // 0x5f
                        _ => Stat::FirstByte,
                    };
                }
                Stat::SS2 => {
                    self.stat = Stat::FirstByte;
                }
                Stat::SS3 => {
                    let attr = Modifier::default();
                    match byte {
                        b'P' => self.gen_keycode(Vk::F1(attr.into())),    // 0x50
                        b'Q' => self.gen_keycode(Vk::F2(attr.into())),    // 0x51
                        b'R' => self.gen_keycode(Vk::F3(attr.into())),    // 0x52
                        b'S' => self.gen_keycode(Vk::F4(attr.into())),    // 0x53
                        b'T' => self.gen_keycode(Vk::F5(attr.into())),    // 0x54
                        _ => (),
                    }
                    self.stat = Stat::FirstByte;
                }
                Stat::DCS => {
                    self.stat = Stat::FirstByte;
                }
                Stat::SOS => {
                    self.stat = Stat::FirstByte;
                }
                Stat::CsiParameterTop => {
                    self.stat = match byte {
                        b'0'..=b'9' => {
                            cont_flag = true;
                            Stat::CsiParameter
                        }
                        _ => {
                            cont_flag = true;
                            Stat::CsiIntermediate
                        }
                    };
                }
                Stat::CsiParameter => {
                    self.stat = match byte {
                        b'0'..=b'9' => {
                            // ignore the overflow
                            self.param_accum = self.param_accum * 10 + (byte - b'0') as u16;
                            Stat::CsiParameter
                        }
                        b';' => {
                            self.params.push(self.param_accum).ok();
                            self.param_accum = 0;
                            Stat::CsiParameter
                        }
                        _ => {
                            self.params.push(self.param_accum).ok();
                            cont_flag = true;
                            Stat::CsiIntermediate
                        }
                    };
                }
                Stat::CsiIntermediate => {
                    self.stat = match byte {
                        b' '..=b'/' => {
                            self.buf_intermediate.push(byte).ok();
                            Stat::CsiIntermediate
                        }
                        _ => {
                            cont_flag = true;
                            Stat::CsiFinal
                        }
                    };
                }
                Stat::CsiFinal => {
                    let attr = Modifier::default();
                    let params = self.params.as_slice();
                    match byte {
                        b'A' => match params {                          // 0x41
                            &[1, 2] => self.gen_keycode(Vk::Up(attr.shift().into())),
                            _ => self.gen_keycode(Vk::Up(attr.into())),
                        }
                        b'B' => match params {                          // 0x42
                            &[1, 2] => self.gen_keycode(Vk::Down(attr.shift().into())),
                            _ => self.gen_keycode(Vk::Down(attr.into())),
                        }
                        b'C' => match params {                          // 0x43
                            &[1, 2] => self.gen_keycode(Vk::Right(attr.shift().into())),
                            _ => self.gen_keycode(Vk::Right(attr.into())),
                        }
                        b'D' => match params {                          // 0x44
                            &[1, 2] => self.gen_keycode(Vk::Left(attr.shift().into())),
                            _ => self.gen_keycode(Vk::Left(attr.into())),
                        }
                        b'F' => match params {                          // 0x46
                            &[1, 2] => self.gen_keycode(Vk::End(attr.shift().into())),
                            _ => self.gen_keycode(Vk::End(attr.into())),
                        }
                        b'H' => match params {                          // 0x48
                            &[1, 2] => self.gen_keycode(Vk::Home(attr.shift().into())),
                            _ => self.gen_keycode(Vk::Home(attr.into())),
                        }
                        b'P' => match params {                          // 0x50
                            &[1, 2] => self.gen_keycode(Vk::F1(attr.shift().into())),
                            _ => self.gen_keycode(Vk::F1(attr.into())),
                        }
                        b'Q' => match params {                          // 0x51
                            &[1, 2] => self.gen_keycode(Vk::F2(attr.shift().into())),
                            _ => self.gen_keycode(Vk::F2(attr.into())),
                        }
                        b'R' => match params {                          // 0x52
                            &[1, 2] => self.gen_keycode(Vk::F3(attr.shift().into())),
                            _ => self.gen_keycode(Vk::F3(attr.into())),
                        }
                        b'S' => match params {                          // 0x53
                            &[1, 2] => self.gen_keycode(Vk::F4(attr.shift().into())),
                            _ => self.gen_keycode(Vk::F4(attr.into())),
                        }
                        b'Z' => match params {                          // 0x5a
                            _ => self.gen_keycode(Vk::OemBacktab(attr.into())),
                        }
                        b'~' => match params {                          // 0x7e
                            &[1] => self.gen_keycode(Vk::Home(attr.into())),
                            &[2] => self.gen_keycode(Vk::Insert(attr.into())),
                            &[3] => self.gen_keycode(Vk::Delete(attr.into())),
                            &[4] => self.gen_keycode(Vk::End(attr.into())),
                            &[5] => self.gen_keycode(Vk::Prior(attr.into())),
                            &[6] => self.gen_keycode(Vk::Next(attr.into())),
                            &[15] => self.gen_keycode(Vk::F5(attr.into())),
                            &[17] => self.gen_keycode(Vk::F6(attr.into())),
                            &[18] => self.gen_keycode(Vk::F7(attr.into())),
                            &[19] => self.gen_keycode(Vk::F8(attr.into())),
                            &[20] => self.gen_keycode(Vk::F9(attr.into())),
                            &[21] => self.gen_keycode(Vk::F10(attr.into())),
                            &[23] => self.gen_keycode(Vk::F11(attr.into())),
                            &[24] => self.gen_keycode(Vk::F12(attr.into())),
                            &[1, 2] => self.gen_keycode(Vk::Home(attr.shift().into())),
                            &[2, 2] => self.gen_keycode(Vk::Insert(attr.shift().into())),
                            &[3, 2] => self.gen_keycode(Vk::Delete(attr.shift().into())),
                            &[4, 2] => self.gen_keycode(Vk::End(attr.shift().into())),
                            &[5, 2] => self.gen_keycode(Vk::Prior(attr.shift().into())),
                            &[6, 2] => self.gen_keycode(Vk::Next(attr.shift().into())),
                            &[15, 2] => self.gen_keycode(Vk::F5(attr.shift().into())),
                            &[17, 2] => self.gen_keycode(Vk::F6(attr.shift().into())),
                            &[18, 2] => self.gen_keycode(Vk::F7(attr.shift().into())),
                            &[19, 2] => self.gen_keycode(Vk::F8(attr.shift().into())),
                            &[20, 2] => self.gen_keycode(Vk::F9(attr.shift().into())),
                            &[21, 2] => self.gen_keycode(Vk::F10(attr.shift().into())),
                            &[23, 2] => self.gen_keycode(Vk::F11(attr.shift().into())),
                            &[24, 2] => self.gen_keycode(Vk::F12(attr.shift().into())),
                            _ => (), // Unrecognized CSI parameter, ignore it
                        }
                        _ => (),     // Unrecognized CSI final byte, ignore it
                    }
                    self.stat = Stat::FirstByte;
                }
                Stat::ST => {
                    self.stat = Stat::FirstByte;
                }
                Stat::OSC => {
                    self.stat = Stat::FirstByte;
                }
                Stat::PM => {
                    self.stat = Stat::FirstByte;
                }
                Stat::APC => {
                    self.stat = Stat::FirstByte;
                }
            }
        }
    }
}
