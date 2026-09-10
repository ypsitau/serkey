//! A simple keycode parser for handling UTF-8 and VT100 terminal control sequences.
//!
//! # Example
//! 
//! Create a new parser instance using `serkey::Parser::new()`.
//!
//! ```rust
//! let mut parser = serkey::Parser::new();
//! ```
//! Call `serkey::Parser::push()` to feed bytes into the parser and `serkey::Parser::next_key()` to retrieve parsed keycodes.
//! In the following example, assume that `buf` contains the bytes read from the serial device such as UART, USB CDC, and network sockets.
//!
//! ```rust
//! fn feed_parser(parser: &mut serkey::Parser, buf: &[u8]) {
//!     use core::fmt::Write as _;
//!     use serkey::{Key, Modifier};
//!     fn write_key(strbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
//!         write!(strbuf, "{}{}{}{}", text,
//!             if modifier.is_shift() { " + Shift" } else { "" },
//!             if modifier.is_ctrl() { " + Ctrl" } else { "" },
//!             if modifier.is_alt() { " + Alt" } else { "" }).ok();
//!     }
//!     let mut strbuf: heapless::String<64> = heapless::String::new();
//!     for &byte in buf {
//!         parser.push(byte);
//!         while let Some(key) = parser.next_key() {
//!             match key {
//!                 Key::CookedChar(ch)      => { write!(strbuf, "CookedChar: {}", ch).ok(); }
//!                 Key::CookedCtrl(n)       => { write!(strbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
//!                 Key::Back(attr)          => { write_key(&mut strbuf, "Back", attr.modifier()); }
//!                 Key::Enter(attr)         => { write_key(&mut strbuf, "Enter", attr.modifier()); }
//!                 Key::Left(attr)          => { write_key(&mut strbuf, "Left", attr.modifier()); }
//!                 Key::Right(attr)         => { write_key(&mut strbuf, "Right", attr.modifier()); }
//!                 Key::Up(attr)            => { write_key(&mut strbuf, "Up", attr.modifier()); }
//!                 Key::Down(attr)          => { write_key(&mut strbuf, "Down", attr.modifier()); }
//!                 Key::Home(attr)          => { write_key(&mut strbuf, "Home", attr.modifier()); }
//!                 Key::End(attr)           => { write_key(&mut strbuf, "End", attr.modifier()); }
//!                 Key::PageUp(attr)        => { write_key(&mut strbuf, "PageUp", attr.modifier()); }
//!                 Key::PageDown(attr)      => { write_key(&mut strbuf, "PageDown", attr.modifier()); }
//!                 Key::Tab(attr)           => { write_key(&mut strbuf, "Tab", attr.modifier()); }
//!                 Key::Delete(attr)        => { write_key(&mut strbuf, "Delete", attr.modifier()); }
//!                 Key::Insert(attr)        => { write_key(&mut strbuf, "Insert", attr.modifier()); }
//!                 Key::Esc(attr)           => { write_key(&mut strbuf, "Esc", attr.modifier()); }
//!                 Key::F1(attr)            => { write_key(&mut strbuf, "F1", attr.modifier()); }
//!                 Key::F2(attr)            => { write_key(&mut strbuf, "F2", attr.modifier()); }
//!                 Key::F3(attr)            => { write_key(&mut strbuf, "F3", attr.modifier()); }
//!                 Key::F4(attr)            => { write_key(&mut strbuf, "F4", attr.modifier()); }
//!                 Key::F5(attr)            => { write_key(&mut strbuf, "F5", attr.modifier()); }
//!                 Key::F6(attr)            => { write_key(&mut strbuf, "F6", attr.modifier()); }
//!                 Key::F7(attr)            => { write_key(&mut strbuf, "F7", attr.modifier()); }
//!                 Key::F8(attr)            => { write_key(&mut strbuf, "F8", attr.modifier()); }
//!                 Key::F9(attr)            => { write_key(&mut strbuf, "F9", attr.modifier()); }
//!                 Key::F10(attr)           => { write_key(&mut strbuf, "F10", attr.modifier()); }
//!                 Key::F11(attr)           => { write_key(&mut strbuf, "F11", attr.modifier()); }
//!                 Key::F12(attr)           => { write_key(&mut strbuf, "F12", attr.modifier()); }
//!                 _ => { continue; }
//!             }
//!             info!("{}", strbuf.as_str());
//!         }
//!     }
//! }
//! ```
#![no_std]
mod keycode;
use heapless::Vec;
use heapless::spsc::Queue;
pub use keycode::{Key, Modifier};

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
    queue: Queue<Key, 8>,
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
    pub fn next_key(&mut self) -> Option<Key> {
        self.queue.dequeue()
    }
    fn gen_key(&mut self, key: Key) {
        self.queue.enqueue(key).ok();
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
                            self.gen_key(Key::Back(attr.into()));
                            Stat::FirstByte
                        },
                        0x09 => {
                            self.gen_key(Key::Tab(attr.into()));
                            Stat::FirstByte
                        },
                        0x0a => {
                            self.gen_key(Key::Enter(attr.into()));
                            Stat::AfterLF
                        },
                        0x0d => {
                            self.gen_key(Key::Enter(attr.into()));
                            Stat::AfterCR
                        },
                        0x1b => Stat::Escape,
                        0x7f => {
                            self.gen_key(Key::Delete(attr.into()));
                            Stat::FirstByte
                        },
                        byte => {
                            if byte < 0x20 {
                                self.gen_key(Key::CookedCtrl(byte));
                                Stat::FirstByte
                            } else if byte < 0x80 {
                                self.gen_key(Key::CookedChar(char::from(byte)));
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
                                self.gen_key(Key::CookedChar(ch));
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
                            self.gen_key(Key::Esc(attr.into()));
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
                        b'P' => self.gen_key(Key::F1(attr.into())),    // 0x50
                        b'Q' => self.gen_key(Key::F2(attr.into())),    // 0x51
                        b'R' => self.gen_key(Key::F3(attr.into())),    // 0x52
                        b'S' => self.gen_key(Key::F4(attr.into())),    // 0x53
                        b'T' => self.gen_key(Key::F5(attr.into())),    // 0x54
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
                    let mut attr = Modifier::default();
                    let params = self.params.as_slice();
                    attr = match params {
                        &[_, 2] => attr.shift(),
                        &[_, 3] => attr.alt(),
                        &[_, 4] => attr.shift().alt(),
                        &[_, 5] => attr.control(),
                        &[_, 6] => attr.shift().control(),
                        &[_, 7] => attr.alt().control(),
                        &[_, 8] => attr.shift().alt().control(),
                        _ => attr,
                    };
                    match byte {
                        b'A' => {                          // 0x41
                            self.gen_key(Key::Up(attr.into()));
                        }
                        b'B' => {                          // 0x42
                            self.gen_key(Key::Down(attr.into()));
                        }
                        b'C' => {                          // 0x43
                            self.gen_key(Key::Right(attr.into()));
                        }
                        b'D' => {                          // 0x44
                            self.gen_key(Key::Left(attr.into()));
                        }
                        b'F' => {                          // 0x46
                            self.gen_key(Key::End(attr.into()));
                        }
                        b'H' => {                          // 0x48
                            self.gen_key(Key::Home(attr.into()));
                        }
                        b'P' => {                          // 0x50
                            self.gen_key(Key::F1(attr.into()));
                        }
                        b'Q' => {                          // 0x51
                            self.gen_key(Key::F2(attr.into()));
                        }
                        b'R' => {                          // 0x52
                            self.gen_key(Key::F3(attr.into()));
                        }
                        b'S' => {                          // 0x53
                            self.gen_key(Key::F4(attr.into()));
                        }
                        b'Z' => {                          // 0x5a
                            self.gen_key(Key::Tab(attr.shift().into()));
                        }
                        b'~' => match params {        // 0x7e
                            [1, ..] => self.gen_key(Key::Home(attr.into())),
                            [2, ..] => self.gen_key(Key::Insert(attr.into())),
                            [3, ..] => self.gen_key(Key::Delete(attr.into())),
                            [4, ..] => self.gen_key(Key::End(attr.into())),
                            [5, ..] => self.gen_key(Key::PageUp(attr.into())),
                            [6, ..] => self.gen_key(Key::PageDown(attr.into())),
                            [15, ..] => self.gen_key(Key::F5(attr.into())),
                            [17, ..] => self.gen_key(Key::F6(attr.into())),
                            [18, ..] => self.gen_key(Key::F7(attr.into())),
                            [19, ..] => self.gen_key(Key::F8(attr.into())),
                            [20, ..] => self.gen_key(Key::F9(attr.into())),
                            [21, ..] => self.gen_key(Key::F10(attr.into())),
                            [23, ..] => self.gen_key(Key::F11(attr.into())),
                            [24, ..] => self.gen_key(Key::F12(attr.into())),
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
