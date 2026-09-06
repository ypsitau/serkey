#![no_std]
use heapless::Vec;
use heapless::spsc::Queue;
pub mod vk;

use vk::VK;

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

pub struct Context {
    queue: Queue<VK, 8>,
    stat: Stat,
    parameter_accum: u32,
    buf_intermediate: Vec::<u8, 32>,
    utf8_accum: u32,
    utf8_remain: u8,
}

impl Context {
    pub fn new() -> Self {
        Context {
            queue: Queue::new(),
            stat: Stat::FirstByte,
            parameter_accum: 0,
            buf_intermediate: Vec::new(),
            utf8_accum: 0,
            utf8_remain: 0,
        }
    }
    pub fn vk(&mut self) -> VK {
        self.queue.dequeue().unwrap_or(VK::None)
    }
    fn gen_vk(&mut self, vk: VK) {
        self.queue.enqueue(vk).ok();
    }
    pub fn feed(&mut self, byte: u8) {
        let mut cont_flag = true;
        while cont_flag {
            cont_flag = false;
            match self.stat {
                Stat::FirstByte => {
                    self.stat = match byte {
                        0x08 => {
                            self.gen_vk(VK::BACK);
                            Stat::FirstByte
                        },
                        0x09 => {
                            self.gen_vk(VK::TAB);
                            Stat::FirstByte
                        },
                        0x0a => {
                            self.gen_vk(VK::RETURN);
                            Stat::AfterLF
                        },
                        0x0d => {
                            self.gen_vk(VK::RETURN);
                            Stat::AfterCR
                        },
                        0x1b => Stat::Escape,
                        0x7f => {
                            self.gen_vk(VK::DELETE);
                            Stat::FirstByte
                        },
                        byte => {
                            if byte & 0x80 == 0 {
                                self.gen_vk(VK::Char(char::from(byte)));
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
                                self.gen_vk(VK::Char(ch));
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
                            self.queue.enqueue(VK::ESCAPE).ok();
                            Stat::FirstByte
                        },
                        b'N' => Stat::SS2,
                        b'O' => Stat::SS3,
                        b'P' => Stat::DCS,
                        b'X' => Stat::SOS,
                        b'[' => {
                            self.parameter_accum = 0;
                            Stat::CsiParameter
                        },
                        b'\\' => Stat::ST,
                        b']' => Stat::OSC,
                        b'^' => Stat::PM,
                        b'_' => Stat::APC,
                        _ => Stat::FirstByte,
                    };
                }
                Stat::SS2 => {
                    self.stat = Stat::FirstByte;
                }
                Stat::SS3 => {
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
                        b'A' => self.gen_vk(VK::UP),
                        b'B' => self.gen_vk(VK::DOWN),
                        b'C' => self.gen_vk(VK::RIGHT),
                        b'D' => self.gen_vk(VK::LEFT),
                        b'~' => match self.parameter_accum {
                            1 => self.gen_vk(VK::HOME),
                            4 => self.gen_vk(VK::END),
                            5 => self.gen_vk(VK::PRIOR),
                            6 => self.gen_vk(VK::NEXT),
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
