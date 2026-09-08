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
//!                 serkey::KeyCode::CookedChar(ch) => { info!("CookedChar: {}", ch); }
//!                 serkey::KeyCode::CookedCtrl(n) => { info!("CookedCtrl: 0x{:02x}", n); }
//!                 serkey::KeyCode::Backspace => { info!("Backspace"); }
//!                 serkey::KeyCode::Enter => { info!("Enter"); }
//!                 serkey::KeyCode::Left => { info!("Left"); }
//!                 serkey::KeyCode::Right => { info!("Right"); }
//!                 serkey::KeyCode::Up => { info!("Up"); }
//!                 serkey::KeyCode::Down => { info!("Down"); }
//!                 serkey::KeyCode::Home => { info!("Home"); }
//!                 serkey::KeyCode::End => { info!("End"); }
//!                 serkey::KeyCode::PageUp => { info!("PageUp"); }
//!                 serkey::KeyCode::PageDown => { info!("PageDown"); }
//!                 serkey::KeyCode::Tab => { info!("Tab"); }
//!                 serkey::KeyCode::BackTab => { info!("BackTab"); }
//!                 serkey::KeyCode::Delete => { info!("Delete"); }
//!                 serkey::KeyCode::Insert => { info!("Insert"); }
//!                 serkey::KeyCode::F(n) => { info!("F{}", n); }
//!                 serkey::KeyCode::Null => { info!("Null"); }
//!                 serkey::KeyCode::Esc => { info!("Esc"); }
//!                 serkey::KeyCode::ShiftLeft => { info!("ShiftLeft"); }
//!                 serkey::KeyCode::ShiftRight => { info!("ShiftRight"); }
//!                 serkey::KeyCode::ShiftUp => { info!("ShiftUp"); }
//!                 serkey::KeyCode::ShiftDown => { info!("ShiftDown"); }
//!                 serkey::KeyCode::ShiftHome => { info!("ShiftHome"); }
//!                 serkey::KeyCode::ShiftEnd => { info!("ShiftEnd"); }
//!                 serkey::KeyCode::ShiftPageUp => { info!("ShiftPageUp"); }
//!                 serkey::KeyCode::ShiftPageDown => { info!("ShiftPageDown"); }
//!                 serkey::KeyCode::ShiftDelete => { info!("ShiftDelete"); }
//!                 serkey::KeyCode::ShiftInsert => { info!("ShiftInsert"); }
//!                 serkey::KeyCode::ShiftF(n) => { info!("ShiftF{}", n); }
//!             }
//!         }
//!     }
//! }
//! ```
#![no_std]
use heapless::Vec;
use heapless::spsc::Queue;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyCode {
    CookedChar(char),
    CookedCtrl(u8),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Null,
    Esc,
    ShiftLeft,
    ShiftRight,
    ShiftUp,
    ShiftDown,
    ShiftHome,
    ShiftEnd,
    ShiftPageUp,
    ShiftPageDown,
    ShiftDelete,
    ShiftInsert,
    ShiftF(u8),
}

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
    queue: Queue<KeyCode, 8>,
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
    pub fn next_keycode(&mut self) -> Option<KeyCode> {
        self.queue.dequeue()
    }
    fn gen_keycode(&mut self, vk: KeyCode) {
        self.queue.enqueue(vk).ok();
    }
    pub fn push(&mut self, byte: u8) {
        let mut cont_flag = true;
        while cont_flag {
            cont_flag = false;
            match self.stat {
                Stat::FirstByte => {
                    self.stat = match byte {
                        0x08 => {
                            self.gen_keycode(KeyCode::Backspace);
                            Stat::FirstByte
                        },
                        0x09 => {
                            self.gen_keycode(KeyCode::Tab);
                            Stat::FirstByte
                        },
                        0x0a => {
                            self.gen_keycode(KeyCode::Enter);
                            Stat::AfterLF
                        },
                        0x0d => {
                            self.gen_keycode(KeyCode::Enter);
                            Stat::AfterCR
                        },
                        0x1b => Stat::Escape,
                        0x7f => {
                            self.gen_keycode(KeyCode::Delete);
                            Stat::FirstByte
                        },
                        byte => {
                            if byte < 0x20 {
                                self.gen_keycode(KeyCode::CookedCtrl(byte));
                                Stat::FirstByte
                            } else if byte < 0x80 {
                                self.gen_keycode(KeyCode::CookedChar(char::from(byte)));
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
                                self.gen_keycode(KeyCode::CookedChar(ch));
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
                    self.stat = match byte {
                        0x1b => {
                            self.gen_keycode(KeyCode::Esc);
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
                    match byte {
                        b'P' => self.gen_keycode(KeyCode::F(1)),    // 0x50
                        b'Q' => self.gen_keycode(KeyCode::F(2)),    // 0x51
                        b'R' => self.gen_keycode(KeyCode::F(3)),    // 0x52
                        b'S' => self.gen_keycode(KeyCode::F(4)),    // 0x53
                        b'T' => self.gen_keycode(KeyCode::F(5)),    // 0x54
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
                    let params = self.params.as_slice();
                    match byte {
                        b'A' => match params {                          // 0x41
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftUp),
                            _ => self.gen_keycode(KeyCode::Up),
                        }
                        b'B' => match params {                          // 0x42
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftDown),
                            _ => self.gen_keycode(KeyCode::Down),
                        }
                        b'C' => match params {                          // 0x43
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftRight),
                            _ => self.gen_keycode(KeyCode::Right),
                        }
                        b'D' => match params {                          // 0x44
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftLeft),
                            _ => self.gen_keycode(KeyCode::Left),
                        }
                        b'F' => match params {                          // 0x46
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftEnd),
                            _ => self.gen_keycode(KeyCode::End),
                        }
                        b'H' => match params {                          // 0x48
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftHome),
                            _ => self.gen_keycode(KeyCode::Home),
                        }
                        b'P' => match params {                          // 0x50
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftF(1)),
                            _ => self.gen_keycode(KeyCode::F(1)),
                        }
                        b'Q' => match params {                          // 0x51
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftF(2)),
                            _ => self.gen_keycode(KeyCode::F(2)),
                        }
                        b'R' => match params {                          // 0x52
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftF(3)),
                            _ => self.gen_keycode(KeyCode::F(3)),
                        }
                        b'S' => match params {                          // 0x53
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftF(4)),
                            _ => self.gen_keycode(KeyCode::F(4)),
                        }
                        b'Z' => match params {                          // 0x5a
                            _ => self.gen_keycode(KeyCode::BackTab),
                        }
                        b'~' => match params {                          // 0x7e
                            &[1] => self.gen_keycode(KeyCode::Home),
                            &[2] => self.gen_keycode(KeyCode::Insert),
                            &[3] => self.gen_keycode(KeyCode::Delete),
                            &[4] => self.gen_keycode(KeyCode::End),
                            &[5] => self.gen_keycode(KeyCode::PageUp),
                            &[6] => self.gen_keycode(KeyCode::PageDown),
                            &[15] => self.gen_keycode(KeyCode::F(5)),
                            &[17] => self.gen_keycode(KeyCode::F(6)),
                            &[18] => self.gen_keycode(KeyCode::F(7)),
                            &[19] => self.gen_keycode(KeyCode::F(8)),
                            &[20] => self.gen_keycode(KeyCode::F(9)),
                            &[21] => self.gen_keycode(KeyCode::F(10)),
                            &[23] => self.gen_keycode(KeyCode::F(11)),
                            &[24] => self.gen_keycode(KeyCode::F(12)),
                            &[1, 2] => self.gen_keycode(KeyCode::ShiftHome),
                            &[2, 2] => self.gen_keycode(KeyCode::ShiftInsert),
                            &[3, 2] => self.gen_keycode(KeyCode::ShiftDelete),
                            &[4, 2] => self.gen_keycode(KeyCode::ShiftEnd),
                            &[5, 2] => self.gen_keycode(KeyCode::ShiftPageUp),
                            &[6, 2] => self.gen_keycode(KeyCode::ShiftPageDown),
                            &[15, 2] => self.gen_keycode(KeyCode::ShiftF(5)),
                            &[17, 2] => self.gen_keycode(KeyCode::ShiftF(6)),
                            &[18, 2] => self.gen_keycode(KeyCode::ShiftF(7)),
                            &[19, 2] => self.gen_keycode(KeyCode::ShiftF(8)),
                            &[20, 2] => self.gen_keycode(KeyCode::ShiftF(9)),
                            &[21, 2] => self.gen_keycode(KeyCode::ShiftF(10)),
                            &[23, 2] => self.gen_keycode(KeyCode::ShiftF(11)),
                            &[24, 2] => self.gen_keycode(KeyCode::ShiftF(12)),
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
