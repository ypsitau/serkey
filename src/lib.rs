#![no_std]
use heapless::Vec;
use heapless::spsc::Queue;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyCode {
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
    Char(char),
    Null,
    Esc,
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
    parameter_accum: u32,
    buf_intermediate: Vec::<u8, 32>,
    utf8_accum: u32,
    utf8_remain: u8,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            queue: Queue::new(),
            stat: Stat::FirstByte,
            parameter_accum: 0,
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
                            if byte & 0x80 == 0 {
                                self.gen_keycode(KeyCode::Char(char::from(byte)));
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
                                self.gen_keycode(KeyCode::Char(ch));
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
                            self.parameter_accum = 0;
                            Stat::CsiParameter
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
                Stat::CsiParameter => {
                    self.stat = match byte {
                        b'0'..=b'9' => {
                            // ignore the overflow
                            self.parameter_accum = self.parameter_accum * 10 + (byte - b'0') as u32;
                            Stat::CsiParameter
                        }
                        _ => {
                            self.buf_intermediate.clear();
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
                    match byte {
                        b'A' => self.gen_keycode(KeyCode::Up),          // 0x41
                        b'B' => self.gen_keycode(KeyCode::Down),        // 0x42
                        b'C' => self.gen_keycode(KeyCode::Right),       // 0x43
                        b'D' => self.gen_keycode(KeyCode::Left),        // 0x44
                        b'Z' => self.gen_keycode(KeyCode::BackTab),     // 0x5a
                        b'~' => match self.parameter_accum {            // 0x7e
                            1 => self.gen_keycode(KeyCode::Home),
                            4 => self.gen_keycode(KeyCode::End),
                            5 => self.gen_keycode(KeyCode::PageUp),
                            6 => self.gen_keycode(KeyCode::PageDown),
                            15 => self.gen_keycode(KeyCode::F(5)),
                            17 => self.gen_keycode(KeyCode::F(6)),
                            18 => self.gen_keycode(KeyCode::F(7)),
                            19 => self.gen_keycode(KeyCode::F(8)),
                            20 => self.gen_keycode(KeyCode::F(9)),
                            21 => self.gen_keycode(KeyCode::F(10)),
                            23 => self.gen_keycode(KeyCode::F(11)),
                            24 => self.gen_keycode(KeyCode::F(12)),
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
