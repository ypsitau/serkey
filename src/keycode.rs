//! This crate provides functionality for handling keyboard input, including key codes, enumerations, 
//! and modifiers.

/// Represents the modifier keys (Shift, Control, Alt) on the keyboard.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Modifier {
    bits: u8,
}

impl Modifier {
    /// Each bit arrangement is the same as the corresponding modifier key bits in the USB HID specification.
    pub const LEFT_CTRL: u8             = 1 << 0;
    pub const LEFT_SHIFT: u8            = 1 << 1;
    pub const LEFT_ALT: u8              = 1 << 2;
    pub const LEFT_META: u8             = 1 << 3;
    pub const RIGHT_CTRL: u8            = 1 << 4;
    pub const RIGHT_SHIFT: u8           = 1 << 5;
    pub const RIGHT_ALT: u8             = 1 << 6;
    pub const RIGHT_META: u8            = 1 << 7;
    pub fn ctrl(self) -> Self           { self.ctrl_l() }
    pub fn ctrl_l(self) -> Self         { Self { bits: self.bits | Self::LEFT_CTRL, } }
    pub fn ctrl_r(self) -> Self         { Self { bits: self.bits | Self::RIGHT_CTRL, } }
    pub fn shift(self) -> Self          { self.shift_l() }
    pub fn shift_r(self) -> Self        { Self { bits: self.bits | Self::RIGHT_SHIFT, } }
    pub fn shift_l(self) -> Self        { Self { bits: self.bits | Self::LEFT_SHIFT, } }
    pub fn alt(self) -> Self            { self.alt_l() }
    pub fn alt_l(self) -> Self          { Self { bits: self.bits | Self::LEFT_ALT, } }
    pub fn alt_r(self) -> Self          { Self { bits: self.bits | Self::RIGHT_ALT, } }
    pub fn meta(self) -> Self           { self.meta_l() }
    pub fn meta_l(self) -> Self         { Self { bits: self.bits | Self::LEFT_META, } }
    pub fn meta_r(self) -> Self         { Self { bits: self.bits | Self::RIGHT_META, } }
    pub fn is_ctrl(&self) -> bool       { self.bits & (Self::LEFT_CTRL | Self::RIGHT_CTRL) != 0 }
    pub fn is_ctrl_l(&self) -> bool     { self.bits & Self::LEFT_CTRL != 0 }
    pub fn is_ctrl_r(&self) -> bool     { self.bits & Self::RIGHT_CTRL != 0 }
    pub fn is_shift(&self) -> bool      { self.bits & (Self::LEFT_SHIFT | Self::RIGHT_SHIFT) != 0 }
    pub fn is_shift_l(&self) -> bool    { self.bits & Self::LEFT_SHIFT != 0 }
    pub fn is_shift_r(&self) -> bool    { self.bits & Self::RIGHT_SHIFT != 0 }
    pub fn is_alt(&self) -> bool        { self.bits & (Self::LEFT_ALT | Self::RIGHT_ALT) != 0 }
    pub fn is_alt_l(&self) -> bool      { self.bits & Self::LEFT_ALT != 0 }
    pub fn is_alt_r(&self) -> bool      { self.bits & Self::RIGHT_ALT != 0 }
    pub fn is_meta(&self) -> bool       { self.bits & (Self::LEFT_META | Self::RIGHT_META) != 0 }
    pub fn is_meta_l(&self) -> bool     { self.bits & Self::LEFT_META != 0 }
    pub fn is_meta_r(&self) -> bool     { self.bits & Self::RIGHT_META != 0 }
}

/// Represents a keyboard attribute with a specific ID and associated modifier keys.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attr<const ID: u16> {
    modifier: Modifier,
}

impl<const ID: u16> Attr<ID> {
    pub fn id(&self) -> u16 { ID }
    pub fn modifier(&self) -> Modifier  { self.modifier }
    pub fn is_ctrl(&self) -> bool       { self.modifier.is_ctrl() }
    pub fn is_ctrl_l(&self) -> bool     { self.modifier.is_ctrl_l() }
    pub fn is_ctrl_r(&self) -> bool     { self.modifier.is_ctrl_r() }
    pub fn is_shift(&self) -> bool      { self.modifier.is_shift() }
    pub fn is_shift_l(&self) -> bool    { self.modifier.is_shift_l() }
    pub fn is_shift_r(&self) -> bool    { self.modifier.is_shift_r() }
    pub fn is_alt(&self) -> bool        { self.modifier.is_alt() }
    pub fn is_alt_l(&self) -> bool      { self.modifier.is_alt_l() }
    pub fn is_alt_r(&self) -> bool      { self.modifier.is_alt_r() }
    pub fn is_meta(&self) -> bool       { self.modifier.is_meta() }
    pub fn is_meta_l(&self) -> bool     { self.modifier.is_meta_l() }
    pub fn is_meta_r(&self) -> bool     { self.modifier.is_meta_r() }
}

impl<const ID: u16> From<Modifier> for Attr<ID> {
    fn from(modifier: Modifier) -> Self {
        Self { modifier }
    }
}

#[allow(non_snake_case)]
pub mod KEY {
    pub const RESERVED: u16 =                   0;
    pub const ESC: u16 =                        1;
    pub const NUM1: u16 =                       2;
    pub const NUM2: u16 =                       3;
    pub const NUM3: u16 =                       4;
    pub const NUM4: u16 =                       5;
    pub const NUM5: u16 =                       6;
    pub const NUM6: u16 =                       7;
    pub const NUM7: u16 =                       8;
    pub const NUM8: u16 =                       9;
    pub const NUM9: u16 =                       10;
    pub const NUM0: u16 =                       11;
    pub const MINUS: u16 =                      12;
    pub const EQUAL: u16 =                      13;
    pub const BACKSPACE: u16 =                  14;
    pub const TAB: u16 =                        15;
    pub const Q: u16 =                          16;
    pub const W: u16 =                          17;
    pub const E: u16 =                          18;
    pub const R: u16 =                          19;
    pub const T: u16 =                          20;
    pub const Y: u16 =                          21;
    pub const U: u16 =                          22;
    pub const I: u16 =                          23;
    pub const O: u16 =                          24;
    pub const P: u16 =                          25;
    pub const LEFTBRACE: u16 =                  26;
    pub const RIGHTBRACE: u16 =                 27;
    pub const ENTER: u16 =                      28;
    pub const LEFTCTRL: u16 =                   29;
    pub const A: u16 =                          30;
    pub const S: u16 =                          31;
    pub const D: u16 =                          32;
    pub const F: u16 =                          33;
    pub const G: u16 =                          34;
    pub const H: u16 =                          35;
    pub const J: u16 =                          36;
    pub const K: u16 =                          37;
    pub const L: u16 =                          38;
    pub const SEMICOLON: u16 =                  39;
    pub const APOSTROPHE: u16 =                 40;
    pub const GRAVE: u16 =                      41;
    pub const LEFTSHIFT: u16 =                  42;
    pub const BACKSLASH: u16 =                  43;
    pub const Z: u16 =                          44;
    pub const X: u16 =                          45;
    pub const C: u16 =                          46;
    pub const V: u16 =                          47;
    pub const B: u16 =                          48;
    pub const N: u16 =                          49;
    pub const M: u16 =                          50;
    pub const COMMA: u16 =                      51;
    pub const DOT: u16 =                        52;
    pub const SLASH: u16 =                      53;
    pub const RIGHTSHIFT: u16 =                 54;
    pub const KPASTERISK: u16 =                 55;
    pub const LEFTALT: u16 =                    56;
    pub const SPACE: u16 =                      57;
    pub const CAPSLOCK: u16 =                   58;
    pub const F1: u16 =                         59;
    pub const F2: u16 =                         60;
    pub const F3: u16 =                         61;
    pub const F4: u16 =                         62;
    pub const F5: u16 =                         63;
    pub const F6: u16 =                         64;
    pub const F7: u16 =                         65;
    pub const F8: u16 =                         66;
    pub const F9: u16 =                         67;
    pub const F10: u16 =                        68;
    pub const NUMLOCK: u16 =                    69;
    pub const SCROLLLOCK: u16 =                 70;
    pub const KP7: u16 =                        71;
    pub const KP8: u16 =                        72;
    pub const KP9: u16 =                        73;
    pub const KPMINUS: u16 =                    74;
    pub const KP4: u16 =                        75;
    pub const KP5: u16 =                        76;
    pub const KP6: u16 =                        77;
    pub const KPPLUS: u16 =                     78;
    pub const KP1: u16 =                        79;
    pub const KP2: u16 =                        80;
    pub const KP3: u16 =                        81;
    pub const KP0: u16 =                        82;
    pub const KPDOT: u16 =                      83;
    pub const ZENKAKUHANKAKU: u16 =             85;
    pub const ND102: u16 =                      86;
    pub const F11: u16 =                        87;
    pub const F12: u16 =                        88;
    pub const RO: u16 =                         89;
    pub const KATAKANA: u16 =                   90;
    pub const HIRAGANA: u16 =                   91;
    pub const HENKAN: u16 =                     92;
    pub const KATAKANAHIRAGANA: u16 =           93;
    pub const MUHENKAN: u16 =                   94;
    pub const KPJPCOMMA: u16 =                  95;
    pub const KPENTER: u16 =                    96;
    pub const RIGHTCTRL: u16 =                  97;
    pub const KPSLASH: u16 =                    98;
    pub const SYSRQ: u16 =                      99;
    pub const RIGHTALT: u16 =                   100;
    pub const LINEFEED: u16 =                   101;
    pub const HOME: u16 =                       102;
    pub const UP: u16 =                         103;
    pub const PAGEUP: u16 =                     104;
    pub const LEFT: u16 =                       105;
    pub const RIGHT: u16 =                      106;
    pub const END: u16 =                        107;
    pub const DOWN: u16 =                       108;
    pub const PAGEDOWN: u16 =                   109;
    pub const INSERT: u16 =                     110;
    pub const DELETE: u16 =                     111;
    pub const MACRO: u16 =                      112;
    pub const MUTE: u16 =                       113;
    pub const VOLUMEDOWN: u16 =                 114;
    pub const VOLUMEUP: u16 =                   115;
    pub const POWER: u16 =                      116;
    pub const KPEQUAL: u16 =                    117;
    pub const KPPLUSMINUS: u16 =                118;
    pub const PAUSE: u16 =                      119;
    pub const SCALE: u16 =                      120;
    pub const KPCOMMA: u16 =                    121;
    pub const HANGEUL: u16 =                    122;
    pub const HANGUEL: u16 =                    HANGEUL;
    pub const HANJA: u16 =                      123;
    pub const YEN: u16 =                        124;
    pub const LEFTMETA: u16 =                   125;
    pub const RIGHTMETA: u16 =                  126;
    pub const COMPOSE: u16 =                    127;
    pub const STOP: u16 =                       128;
    pub const AGAIN: u16 =                      129;
    pub const PROPS: u16 =                      130;
    pub const UNDO: u16 =                       131;
    pub const FRONT: u16 =                      132;
    pub const COPY: u16 =                       133;
    pub const OPEN: u16 =                       134;
    pub const PASTE: u16 =                      135;
    pub const FIND: u16 =                       136;
    pub const CUT: u16 =                        137;
    pub const HELP: u16 =                       138;
    pub const MENU: u16 =                       139;
    pub const CALC: u16 =                       140;
    pub const SETUP: u16 =                      141;
    pub const SLEEP: u16 =                      142;
    pub const WAKEUP: u16 =                     143;
    pub const FILE: u16 =                       144;
    pub const SENDFILE: u16 =                   145;
    pub const DELETEFILE: u16 =                 146;
    pub const XFER: u16 =                       147;
    pub const PROG1: u16 =                      148;
    pub const PROG2: u16 =                      149;
    pub const WWW: u16 =                        150;
    pub const MSDOS: u16 =                      151;
    pub const COFFEE: u16 =                     152;
    pub const SCREENLOCK: u16 =                 COFFEE;
    pub const ROTATE_DISPLAY: u16 =             153;
    pub const DIRECTION: u16 =                  ROTATE_DISPLAY;
    pub const CYCLEWINDOWS: u16 =               154;
    pub const MAIL: u16 =                       155;
    pub const BOOKMARKS: u16 =                  156;
    pub const COMPUTER: u16 =                   157;
    pub const BACK: u16 =                       158;
    pub const FORWARD: u16 =                    159;
    pub const CLOSECD: u16 =                    160;
    pub const EJECTCD: u16 =                    161;
    pub const EJECTCLOSECD: u16 =               162;
    pub const NEXTSONG: u16 =                   163;
    pub const PLAYPAUSE: u16 =                  164;
    pub const PREVIOUSSONG: u16 =               165;
    pub const STOPCD: u16 =                     166;
    pub const RECORD: u16 =                     167;
    pub const REWIND: u16 =                     168;
    pub const PHONE: u16 =                      169;
    pub const ISO: u16 =                        170;
    pub const CONFIG: u16 =                     171;
    pub const HOMEPAGE: u16 =                   172;
    pub const REFRESH: u16 =                    173;
    pub const EXIT: u16 =                       174;
    pub const MOVE: u16 =                       175;
    pub const EDIT: u16 =                       176;
    pub const SCROLLUP: u16 =                   177;
    pub const SCROLLDOWN: u16 =                 178;
    pub const KPLEFTPAREN: u16 =                179;
    pub const KPRIGHTPAREN: u16 =               180;
    pub const NEW: u16 =                        181;
    pub const REDO: u16 =                       182;
    pub const F13: u16 =                        183;
    pub const F14: u16 =                        184;
    pub const F15: u16 =                        185;
    pub const F16: u16 =                        186;
    pub const F17: u16 =                        187;
    pub const F18: u16 =                        188;
    pub const F19: u16 =                        189;
    pub const F20: u16 =                        190;
    pub const F21: u16 =                        191;
    pub const F22: u16 =                        192;
    pub const F23: u16 =                        193;
    pub const F24: u16 =                        194;
    pub const PLAYCD: u16 =                     200;
    pub const PAUSECD: u16 =                    201;
    pub const PROG3: u16 =                      202;
    pub const PROG4: u16 =                      203;
    pub const ALL_APPLICATIONS: u16 =           204;
    pub const DASHBOARD: u16 =                  ALL_APPLICATIONS;
    pub const SUSPEND: u16 =                    205;
    pub const CLOSE: u16 =                      206;
    pub const PLAY: u16 =                       207;
    pub const FASTFORWARD: u16 =                208;
    pub const BASSBOOST: u16 =                  209;
    pub const PRINT: u16 =                      210;
    pub const HP: u16 =                         211;
    pub const CAMERA: u16 =                     212;
    pub const SOUND: u16 =                      213;
    pub const QUESTION: u16 =                   214;
    pub const EMAIL: u16 =                      215;
    pub const CHAT: u16 =                       216;
    pub const SEARCH: u16 =                     217;
    pub const CONNECT: u16 =                    218;
    pub const FINANCE: u16 =                    219;
    pub const SPORT: u16 =                      220;
    pub const SHOP: u16 =                       221;
    pub const ALTERASE: u16 =                   222;
    pub const CANCEL: u16 =                     223;
    pub const BRIGHTNESSDOWN: u16 =             224;
    pub const BRIGHTNESSUP: u16 =               225;
    pub const MEDIA: u16 =                      226;
    pub const SWITCHVIDEOMODE: u16 =            227;
    pub const KBDILLUMTOGGLE: u16 =             228;
    pub const KBDILLUMDOWN: u16 =               229;
    pub const KBDILLUMUP: u16 =                 230;
    pub const SEND: u16 =                       231;
    pub const REPLY: u16 =                      232;
    pub const FORWARDMAIL: u16 =                233;
    pub const SAVE: u16 =                       234;
    pub const DOCUMENTS: u16 =                  235;
    pub const BATTERY: u16 =                    236;
    pub const BLUETOOTH: u16 =                  237;
    pub const WLAN: u16 =                       238;
    pub const UWB: u16 =                        239;
    pub const UNKNOWN: u16 =                    240;
    pub const VIDEO_NEXT: u16 =                 241;
    pub const VIDEO_PREV: u16 =                 242;
    pub const BRIGHTNESS_CYCLE: u16 =           243;
    pub const BRIGHTNESS_AUTO: u16 =            244;
    pub const BRIGHTNESS_ZERO: u16 =            BRIGHTNESS_AUTO;
    pub const DISPLAY_OFF: u16 =                245;
    pub const WWAN: u16 =                       246;
    pub const WIMAX: u16 =                      WWAN;
    pub const RFKILL: u16 =                     247;
    pub const MICMUTE: u16 =                    248;
    pub const OK: u16 =                         0x160;
    pub const SELECT: u16 =                     0x161;
    pub const GOTO: u16 =                       0x162;
    pub const CLEAR: u16 =                      0x163;
    pub const POWER2: u16 =                     0x164;
    pub const OPTION: u16 =                     0x165;
    pub const INFO: u16 =                       0x166;
    pub const TIME: u16 =                       0x167;
    pub const VENDOR: u16 =                     0x168;
    pub const ARCHIVE: u16 =                    0x169;
    pub const PROGRAM: u16 =                    0x16a;
    pub const CHANNEL: u16 =                    0x16b;
    pub const FAVORITES: u16 =                  0x16c;
    pub const EPG: u16 =                        0x16d;
    pub const PVR: u16 =                        0x16e;
    pub const MHP: u16 =                        0x16f;
    pub const LANGUAGE: u16 =                   0x170;
    pub const TITLE: u16 =                      0x171;
    pub const SUBTITLE: u16 =                   0x172;
    pub const ANGLE: u16 =                      0x173;
    pub const FULL_SCREEN: u16 =                0x174;
    pub const ZOOM: u16 =                       FULL_SCREEN;
    pub const MODE: u16 =                       0x175;
    pub const KEYBOARD: u16 =                   0x176;
    pub const ASPECT_RATIO: u16 =               0x177;
    pub const SCREEN: u16 =                     ASPECT_RATIO;
    pub const PC: u16 =                         0x178;
    pub const TV: u16 =                         0x179;
    pub const TV2: u16 =                        0x17a;
    pub const VCR: u16 =                        0x17b;
    pub const VCR2: u16 =                       0x17c;
    pub const SAT: u16 =                        0x17d;
    pub const SAT2: u16 =                       0x17e;
    pub const CD: u16 =                         0x17f;
    pub const TAPE: u16 =                       0x180;
    pub const RADIO: u16 =                      0x181;
    pub const TUNER: u16 =                      0x182;
    pub const PLAYER: u16 =                     0x183;
    pub const TEXT: u16 =                       0x184;
    pub const DVD: u16 =                        0x185;
    pub const AUX: u16 =                        0x186;
    pub const MP3: u16 =                        0x187;
    pub const AUDIO: u16 =                      0x188;
    pub const VIDEO: u16 =                      0x189;
    pub const DIRECTORY: u16 =                  0x18a;
    pub const LIST: u16 =                       0x18b;
    pub const MEMO: u16 =                       0x18c;
    pub const CALENDAR: u16 =                   0x18d;
    pub const RED: u16 =                        0x18e;
    pub const GREEN: u16 =                      0x18f;
    pub const YELLOW: u16 =                     0x190;
    pub const BLUE: u16 =                       0x191;
    pub const CHANNELUP: u16 =                  0x192;
    pub const CHANNELDOWN: u16 =                0x193;
    pub const FIRST: u16 =                      0x194;
    pub const LAST: u16 =                       0x195;
    pub const AB: u16 =                         0x196;
    pub const NEXT: u16 =                       0x197;
    pub const RESTART: u16 =                    0x198;
    pub const SLOW: u16 =                       0x199;
    pub const SHUFFLE: u16 =                    0x19a;
    pub const BREAK: u16 =                      0x19b;
    pub const PREVIOUS: u16 =                   0x19c;
    pub const DIGITS: u16 =                     0x19d;
    pub const TEEN: u16 =                       0x19e;
    pub const TWEN: u16 =                       0x19f;
    pub const VIDEOPHONE: u16 =                 0x1a0;
    pub const GAMES: u16 =                      0x1a1;
    pub const ZOOMIN: u16 =                     0x1a2;
    pub const ZOOMOUT: u16 =                    0x1a3;
    pub const ZOOMRESET: u16 =                  0x1a4;
    pub const WORDPROCESSOR: u16 =              0x1a5;
    pub const EDITOR: u16 =                     0x1a6;
    pub const SPREADSHEET: u16 =                0x1a7;
    pub const GRAPHICSEDITOR: u16 =             0x1a8;
    pub const PRESENTATION: u16 =               0x1a9;
    pub const DATABASE: u16 =                   0x1aa;
    pub const NEWS: u16 =                       0x1ab;
    pub const VOICEMAIL: u16 =                  0x1ac;
    pub const ADDRESSBOOK: u16 =                0x1ad;
    pub const MESSENGER: u16 =                  0x1ae;
    pub const DISPLAYTOGGLE: u16 =              0x1af;
    pub const BRIGHTNESS_TOGGLE: u16 =          DISPLAYTOGGLE;
    pub const SPELLCHECK: u16 =                 0x1b0;
    pub const LOGOFF: u16 =                     0x1b1;
    pub const DOLLAR: u16 =                     0x1b2;
    pub const EURO: u16 =                       0x1b3;
    pub const FRAMEBACK: u16 =                  0x1b4;
    pub const FRAMEFORWARD: u16 =               0x1b5;
    pub const CONTEXT_MENU: u16 =               0x1b6;
    pub const MEDIA_REPEAT: u16 =               0x1b7;
    pub const TENCHANNELSUP: u16 =              0x1b8;
    pub const TENCHANNELSDOWN: u16 =            0x1b9;
    pub const IMAGES: u16 =                     0x1ba;
    pub const NOTIFICATION_CENTER: u16 =        0x1bc;
    pub const PICKUP_PHONE: u16 =               0x1bd;
    pub const HANGUP_PHONE: u16 =               0x1be;
    pub const LINK_PHONE: u16 =                 0x1bf;
    pub const DEL_EOL: u16 =                    0x1c0;
    pub const DEL_EOS: u16 =                    0x1c1;
    pub const INS_LINE: u16 =                   0x1c2;
    pub const DEL_LINE: u16 =                   0x1c3;
    pub const FN: u16 =                         0x1d0;
    pub const FN_ESC: u16 =                     0x1d1;
    pub const FN_F1: u16 =                      0x1d2;
    pub const FN_F2: u16 =                      0x1d3;
    pub const FN_F3: u16 =                      0x1d4;
    pub const FN_F4: u16 =                      0x1d5;
    pub const FN_F5: u16 =                      0x1d6;
    pub const FN_F6: u16 =                      0x1d7;
    pub const FN_F7: u16 =                      0x1d8;
    pub const FN_F8: u16 =                      0x1d9;
    pub const FN_F9: u16 =                      0x1da;
    pub const FN_F10: u16 =                     0x1db;
    pub const FN_F11: u16 =                     0x1dc;
    pub const FN_F12: u16 =                     0x1dd;
    pub const FN_1: u16 =                       0x1de;
    pub const FN_2: u16 =                       0x1df;
    pub const FN_D: u16 =                       0x1e0;
    pub const FN_E: u16 =                       0x1e1;
    pub const FN_F: u16 =                       0x1e2;
    pub const FN_S: u16 =                       0x1e3;
    pub const FN_B: u16 =                       0x1e4;
    pub const FN_RIGHT_SHIFT: u16 =             0x1e5;
    pub const BRL_DOT1: u16 =                   0x1f1;
    pub const BRL_DOT2: u16 =                   0x1f2;
    pub const BRL_DOT3: u16 =                   0x1f3;
    pub const BRL_DOT4: u16 =                   0x1f4;
    pub const BRL_DOT5: u16 =                   0x1f5;
    pub const BRL_DOT6: u16 =                   0x1f6;
    pub const BRL_DOT7: u16 =                   0x1f7;
    pub const BRL_DOT8: u16 =                   0x1f8;
    pub const BRL_DOT9: u16 =                   0x1f9;
    pub const BRL_DOT10: u16 =                  0x1fa;
    pub const NUMERIC_0: u16 =                  0x200;
    pub const NUMERIC_1: u16 =                  0x201;
    pub const NUMERIC_2: u16 =                  0x202;
    pub const NUMERIC_3: u16 =                  0x203;
    pub const NUMERIC_4: u16 =                  0x204;
    pub const NUMERIC_5: u16 =                  0x205;
    pub const NUMERIC_6: u16 =                  0x206;
    pub const NUMERIC_7: u16 =                  0x207;
    pub const NUMERIC_8: u16 =                  0x208;
    pub const NUMERIC_9: u16 =                  0x209;
    pub const NUMERIC_STAR: u16 =               0x20a;
    pub const NUMERIC_POUND: u16 =              0x20b;
    pub const NUMERIC_A: u16 =                  0x20c;
    pub const NUMERIC_B: u16 =                  0x20d;
    pub const NUMERIC_C: u16 =                  0x20e;
    pub const NUMERIC_D: u16 =                  0x20f;
    pub const CAMERA_FOCUS: u16 =               0x210;
    pub const WPS_BUTTON: u16 =                 0x211;
    pub const TOUCHPAD_TOGGLE: u16 =            0x212;
    pub const TOUCHPAD_ON: u16 =                0x213;
    pub const TOUCHPAD_OFF: u16 =               0x214;
    pub const CAMERA_ZOOMIN: u16 =              0x215;
    pub const CAMERA_ZOOMOUT: u16 =             0x216;
    pub const CAMERA_UP: u16 =                  0x217;
    pub const CAMERA_DOWN: u16 =                0x218;
    pub const CAMERA_LEFT: u16 =                0x219;
    pub const CAMERA_RIGHT: u16 =               0x21a;
    pub const ATTENDANT_ON: u16 =               0x21b;
    pub const ATTENDANT_OFF: u16 =              0x21c;
    pub const ATTENDANT_TOGGLE: u16 =           0x21d;
    pub const LIGHTS_TOGGLE: u16 =              0x21e;
    pub const ALS_TOGGLE: u16 =                 0x230;
    pub const ROTATE_LOCK_TOGGLE: u16 =         0x231;
    pub const REFRESH_RATE_TOGGLE: u16 =        0x232;
    pub const BUTTONCONFIG: u16 =               0x240;
    pub const TASKMANAGER: u16 =                0x241;
    pub const JOURNAL: u16 =                    0x242;
    pub const CONTROLPANEL: u16 =               0x243;
    pub const APPSELECT: u16 =                  0x244;
    pub const SCREENSAVER: u16 =                0x245;
    pub const VOICECOMMAND: u16 =               0x246;
    pub const ASSISTANT: u16 =                  0x247;
    pub const KBD_LAYOUT_NEXT: u16 =            0x248;
    pub const EMOJI_PICKER: u16 =               0x249;
    pub const DICTATE: u16 =                    0x24a;
    pub const CAMERA_ACCESS_ENABLE: u16 =       0x24b;
    pub const CAMERA_ACCESS_DISABLE: u16 =      0x24c;
    pub const CAMERA_ACCESS_TOGGLE: u16 =       0x24d;
    pub const ACCESSIBILITY: u16 =              0x24e;
    pub const DO_NOT_DISTURB: u16 =             0x24f;
    pub const BRIGHTNESS_MIN: u16 =             0x250;
    pub const BRIGHTNESS_MAX: u16 =             0x251;
    pub const EPRIVACY_SCREEN_ON: u16 =         0x252;
    pub const EPRIVACY_SCREEN_OFF: u16 =        0x253;
    pub const ACTION_ON_SELECTION: u16 =        0x254;
    pub const CONTEXTUAL_INSERT: u16 =          0x255;
    pub const CONTEXTUAL_QUERY: u16 =           0x256;
    pub const KBDINPUTASSIST_PREV: u16 =        0x260;
    pub const KBDINPUTASSIST_NEXT: u16 =        0x261;
    pub const KBDINPUTASSIST_PREVGROUP: u16 =   0x262;
    pub const KBDINPUTASSIST_NEXTGROUP: u16 =   0x263;
    pub const KBDINPUTASSIST_ACCEPT: u16 =      0x264;
    pub const KBDINPUTASSIST_CANCEL: u16 =      0x265;
    pub const RIGHT_UP: u16 =                   0x266;
    pub const RIGHT_DOWN: u16 =                 0x267;
    pub const LEFT_UP: u16 =                    0x268;
    pub const LEFT_DOWN: u16 =                  0x269;
    pub const ROOT_MENU: u16 =                  0x26a;
    pub const MEDIA_TOP_MENU: u16 =             0x26b;
    pub const NUMERIC_11: u16 =                 0x26c;
    pub const NUMERIC_12: u16 =                 0x26d;
    pub const AUDIO_DESC: u16 =                 0x26e;
    pub const MODE_3D: u16 =                    0x26f;
    pub const NEXT_FAVORITE: u16 =              0x270;
    pub const STOP_RECORD: u16 =                0x271;
    pub const PAUSE_RECORD: u16 =               0x272;
    pub const VOD: u16 =                        0x273;
    pub const UNMUTE: u16 =                     0x274;
    pub const FASTREVERSE: u16 =                0x275;
    pub const SLOWREVERSE: u16 =                0x276;
    pub const DATA: u16 =                       0x277;
    pub const ONSCREEN_KEYBOARD: u16 =          0x278;
    pub const PRIVACY_SCREEN_TOGGLE: u16 =      0x279;
    pub const SELECTIVE_SCREENSHOT: u16 =       0x27a;
    pub const NEXT_ELEMENT: u16 =               0x27b;
    pub const PREVIOUS_ELEMENT: u16 =           0x27c;
    pub const AUTOPILOT_ENGAGE_TOGGLE: u16 =    0x27d;
    pub const MARK_WAYPOINT: u16 =              0x27e;
    pub const SOS: u16 =                        0x27f;
    pub const NAV_CHART: u16 =                  0x280;
    pub const FISHING_CHART: u16 =              0x281;
    pub const SINGLE_RANGE_RADAR: u16 =         0x282;
    pub const DUAL_RANGE_RADAR: u16 =           0x283;
    pub const RADAR_OVERLAY: u16 =              0x284;
    pub const TRADITIONAL_SONAR: u16 =          0x285;
    pub const CLEARVU_SONAR: u16 =              0x286;
    pub const SIDEVU_SONAR: u16 =               0x287;
    pub const NAV_INFO: u16 =                   0x288;
    pub const BRIGHTNESS_MENU: u16 =            0x289;
    pub const MACRO1: u16 =                     0x290;
    pub const MACRO2: u16 =                     0x291;
    pub const MACRO3: u16 =                     0x292;
    pub const MACRO4: u16 =                     0x293;
    pub const MACRO5: u16 =                     0x294;
    pub const MACRO6: u16 =                     0x295;
    pub const MACRO7: u16 =                     0x296;
    pub const MACRO8: u16 =                     0x297;
    pub const MACRO9: u16 =                     0x298;
    pub const MACRO10: u16 =                    0x299;
    pub const MACRO11: u16 =                    0x29a;
    pub const MACRO12: u16 =                    0x29b;
    pub const MACRO13: u16 =                    0x29c;
    pub const MACRO14: u16 =                    0x29d;
    pub const MACRO15: u16 =                    0x29e;
    pub const MACRO16: u16 =                    0x29f;
    pub const MACRO17: u16 =                    0x2a0;
    pub const MACRO18: u16 =                    0x2a1;
    pub const MACRO19: u16 =                    0x2a2;
    pub const MACRO20: u16 =                    0x2a3;
    pub const MACRO21: u16 =                    0x2a4;
    pub const MACRO22: u16 =                    0x2a5;
    pub const MACRO23: u16 =                    0x2a6;
    pub const MACRO24: u16 =                    0x2a7;
    pub const MACRO25: u16 =                    0x2a8;
    pub const MACRO26: u16 =                    0x2a9;
    pub const MACRO27: u16 =                    0x2aa;
    pub const MACRO28: u16 =                    0x2ab;
    pub const MACRO29: u16 =                    0x2ac;
    pub const MACRO30: u16 =                    0x2ad;
    pub const MACRO_RECORD_START: u16 =         0x2b0;
    pub const MACRO_RECORD_STOP: u16 =          0x2b1;
    pub const MACRO_PRESET_CYCLE: u16 =         0x2b2;
    pub const MACRO_PRESET1: u16 =              0x2b3;
    pub const MACRO_PRESET2: u16 =              0x2b4;
    pub const MACRO_PRESET3: u16 =              0x2b5;
    pub const KBD_LCD_MENU1: u16 =              0x2b8;
    pub const KBD_LCD_MENU2: u16 =              0x2b9;
    pub const KBD_LCD_MENU3: u16 =              0x2ba;
    pub const KBD_LCD_MENU4: u16 =              0x2bb;
    pub const KBD_LCD_MENU5: u16 =              0x2bc;
    pub const PERFORMANCE: u16 =                0x2bd;
    pub const MIN_INTERESTING: u16 =            MUTE;
    pub const MAX: u16 =                        0x2ff;
}

/// Represents the virtual key codes as an enumeration.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Key {
    None,
    CookedChar(char),
    CookedCtrl(u8),
    Reserved(Attr<{ KEY::RESERVED }>),
    Esc(Attr<{ KEY::ESC }>),
    Num1(Attr<{ KEY::NUM1 }>),
    Num2(Attr<{ KEY::NUM2 }>),
    Num3(Attr<{ KEY::NUM3 }>),
    Num4(Attr<{ KEY::NUM4 }>),
    Num5(Attr<{ KEY::NUM5 }>),
    Num6(Attr<{ KEY::NUM6 }>),
    Num7(Attr<{ KEY::NUM7 }>),
    Num8(Attr<{ KEY::NUM8 }>),
    Num9(Attr<{ KEY::NUM9 }>),
    Num0(Attr<{ KEY::NUM0 }>),
    Minus(Attr<{ KEY::MINUS }>),
    Equal(Attr<{ KEY::EQUAL }>),
    Backspace(Attr<{ KEY::BACKSPACE }>),
    Tab(Attr<{ KEY::TAB }>),
    Q(Attr<{ KEY::Q }>),
    W(Attr<{ KEY::W }>),
    E(Attr<{ KEY::E }>),
    R(Attr<{ KEY::R }>),
    T(Attr<{ KEY::T }>),
    Y(Attr<{ KEY::Y }>),
    U(Attr<{ KEY::U }>),
    I(Attr<{ KEY::I }>),
    O(Attr<{ KEY::O }>),
    P(Attr<{ KEY::P }>),
    LeftBrace(Attr<{ KEY::LEFTBRACE }>),
    RightBrace(Attr<{ KEY::RIGHTBRACE }>),
    Enter(Attr<{ KEY::ENTER }>),
    LeftCtrl(Attr<{ KEY::LEFTCTRL }>),
    A(Attr<{ KEY::A }>),
    S(Attr<{ KEY::S }>),
    D(Attr<{ KEY::D }>),
    F(Attr<{ KEY::F }>),
    G(Attr<{ KEY::G }>),
    H(Attr<{ KEY::H }>),
    J(Attr<{ KEY::J }>),
    K(Attr<{ KEY::K }>),
    L(Attr<{ KEY::L }>),
    Semicolon(Attr<{ KEY::SEMICOLON }>),
    Apostrophe(Attr<{ KEY::APOSTROPHE }>),
    Grave(Attr<{ KEY::GRAVE }>),
    LeftShift(Attr<{ KEY::LEFTSHIFT }>),
    Backslash(Attr<{ KEY::BACKSLASH }>),
    Z(Attr<{ KEY::Z }>),
    X(Attr<{ KEY::X }>),
    C(Attr<{ KEY::C }>),
    V(Attr<{ KEY::V }>),
    B(Attr<{ KEY::B }>),
    N(Attr<{ KEY::N }>),
    M(Attr<{ KEY::M }>),
    Comma(Attr<{ KEY::COMMA }>),
    Dot(Attr<{ KEY::DOT }>),
    Slash(Attr<{ KEY::SLASH }>),
    RightShift(Attr<{ KEY::RIGHTSHIFT }>),
    KpAsterisk(Attr<{ KEY::KPASTERISK }>),
    LeftAlt(Attr<{ KEY::LEFTALT }>),
    Space(Attr<{ KEY::SPACE }>),
    CapsLock(Attr<{ KEY::CAPSLOCK }>),
    F1(Attr<{ KEY::F1 }>),
    F2(Attr<{ KEY::F2 }>),
    F3(Attr<{ KEY::F3 }>),
    F4(Attr<{ KEY::F4 }>),
    F5(Attr<{ KEY::F5 }>),
    F6(Attr<{ KEY::F6 }>),
    F7(Attr<{ KEY::F7 }>),
    F8(Attr<{ KEY::F8 }>),
    F9(Attr<{ KEY::F9 }>),
    F10(Attr<{ KEY::F10 }>),
    NumLock(Attr<{ KEY::NUMLOCK }>),
    ScrollLock(Attr<{ KEY::SCROLLLOCK }>),
    Kp7(Attr<{ KEY::KP7 }>),
    Kp8(Attr<{ KEY::KP8 }>),
    Kp9(Attr<{ KEY::KP9 }>),
    KpMinus(Attr<{ KEY::KPMINUS }>),
    Kp4(Attr<{ KEY::KP4 }>),
    Kp5(Attr<{ KEY::KP5 }>),
    Kp6(Attr<{ KEY::KP6 }>),
    KpPlus(Attr<{ KEY::KPPLUS }>),
    Kp1(Attr<{ KEY::KP1 }>),
    Kp2(Attr<{ KEY::KP2 }>),
    Kp3(Attr<{ KEY::KP3 }>),
    Kp0(Attr<{ KEY::KP0 }>),
    KpDot(Attr<{ KEY::KPDOT }>),
    ZenkakuHankaku(Attr<{ KEY::ZENKAKUHANKAKU }>),
    Nd102(Attr<{ KEY::ND102 }>),
    F11(Attr<{ KEY::F11 }>),
    F12(Attr<{ KEY::F12 }>),
    Ro(Attr<{ KEY::RO }>),
    Katakana(Attr<{ KEY::KATAKANA }>),
    Hiragana(Attr<{ KEY::HIRAGANA }>),
    Henkan(Attr<{ KEY::HENKAN }>),
    KatakanaHiragana(Attr<{ KEY::KATAKANAHIRAGANA }>),
    Muhenkan(Attr<{ KEY::MUHENKAN }>),
    KpJpComma(Attr<{ KEY::KPJPCOMMA }>),
    KpEnter(Attr<{ KEY::KPENTER }>),
    RightCtrl(Attr<{ KEY::RIGHTCTRL }>),
    KpSlash(Attr<{ KEY::KPSLASH }>),
    Sysrq(Attr<{ KEY::SYSRQ }>),
    RightAlt(Attr<{ KEY::RIGHTALT }>),
    Linefeed(Attr<{ KEY::LINEFEED }>),
    Home(Attr<{ KEY::HOME }>),
    Up(Attr<{ KEY::UP }>),
    PageUp(Attr<{ KEY::PAGEUP }>),
    Left(Attr<{ KEY::LEFT }>),
    Right(Attr<{ KEY::RIGHT }>),
    End(Attr<{ KEY::END }>),
    Down(Attr<{ KEY::DOWN }>),
    PageDown(Attr<{ KEY::PAGEDOWN }>),
    Insert(Attr<{ KEY::INSERT }>),
    Delete(Attr<{ KEY::DELETE }>),
    Macro(Attr<{ KEY::MACRO }>),
    Mute(Attr<{ KEY::MUTE }>),
    VolumeDown(Attr<{ KEY::VOLUMEDOWN }>),
    VolumeUp(Attr<{ KEY::VOLUMEUP }>),
    Power(Attr<{ KEY::POWER }>),
    KpEqual(Attr<{ KEY::KPEQUAL }>),
    KpPlusMinus(Attr<{ KEY::KPPLUSMINUS }>),
    Pause(Attr<{ KEY::PAUSE }>),
    Scale(Attr<{ KEY::SCALE }>),
    KpComma(Attr<{ KEY::KPCOMMA }>),
    Hangeul(Attr<{ KEY::HANGEUL }>),
    Hanguel(Attr<{ KEY::HANGUEL }>),
    Hanja(Attr<{ KEY::HANJA }>),
    Yen(Attr<{ KEY::YEN }>),
    LeftMeta(Attr<{ KEY::LEFTMETA }>),
    RightMeta(Attr<{ KEY::RIGHTMETA }>),
    Compose(Attr<{ KEY::COMPOSE }>),
    Stop(Attr<{ KEY::STOP }>),
    Again(Attr<{ KEY::AGAIN }>),
    Props(Attr<{ KEY::PROPS }>),
    Undo(Attr<{ KEY::UNDO }>),
    Front(Attr<{ KEY::FRONT }>),
    Copy(Attr<{ KEY::COPY }>),
    Open(Attr<{ KEY::OPEN }>),
    Paste(Attr<{ KEY::PASTE }>),
    Find(Attr<{ KEY::FIND }>),
    Cut(Attr<{ KEY::CUT }>),
    Help(Attr<{ KEY::HELP }>),
    Menu(Attr<{ KEY::MENU }>),
    Calc(Attr<{ KEY::CALC }>),
    Setup(Attr<{ KEY::SETUP }>),
    Sleep(Attr<{ KEY::SLEEP }>),
    WakeUp(Attr<{ KEY::WAKEUP }>),
    File(Attr<{ KEY::FILE }>),
    SendFile(Attr<{ KEY::SENDFILE }>),
    DeleteFile(Attr<{ KEY::DELETEFILE }>),
    Xfer(Attr<{ KEY::XFER }>),
    Prog1(Attr<{ KEY::PROG1 }>),
    Prog2(Attr<{ KEY::PROG2 }>),
    Www(Attr<{ KEY::WWW }>),
    Msdos(Attr<{ KEY::MSDOS }>),
    Coffee(Attr<{ KEY::COFFEE }>),
    ScreenLock(Attr<{ KEY::SCREENLOCK }>),
    RotateDisplay(Attr<{ KEY::ROTATE_DISPLAY }>),
    Direction(Attr<{ KEY::DIRECTION }>),
    CycleWindows(Attr<{ KEY::CYCLEWINDOWS }>),
    Mail(Attr<{ KEY::MAIL }>),
    Bookmarks(Attr<{ KEY::BOOKMARKS }>),
    Computer(Attr<{ KEY::COMPUTER }>),
    Back(Attr<{ KEY::BACK }>),
    Forward(Attr<{ KEY::FORWARD }>),
    CloseCd(Attr<{ KEY::CLOSECD }>),
    EjectCd(Attr<{ KEY::EJECTCD }>),
    EjectCloseCd(Attr<{ KEY::EJECTCLOSECD }>),
    NextSong(Attr<{ KEY::NEXTSONG }>),
    PlayPause(Attr<{ KEY::PLAYPAUSE }>),
    PreviousSong(Attr<{ KEY::PREVIOUSSONG }>),
    StopCd(Attr<{ KEY::STOPCD }>),
    Record(Attr<{ KEY::RECORD }>),
    Rewind(Attr<{ KEY::REWIND }>),
    Phone(Attr<{ KEY::PHONE }>),
    Iso(Attr<{ KEY::ISO }>),
    Config(Attr<{ KEY::CONFIG }>),
    Homepage(Attr<{ KEY::HOMEPAGE }>),
    Refresh(Attr<{ KEY::REFRESH }>),
    Exit(Attr<{ KEY::EXIT }>),
    Move(Attr<{ KEY::MOVE }>),
    Edit(Attr<{ KEY::EDIT }>),
    ScrollUp(Attr<{ KEY::SCROLLUP }>),
    ScrollDown(Attr<{ KEY::SCROLLDOWN }>),
    KpLeftParen(Attr<{ KEY::KPLEFTPAREN }>),
    KpRightParen(Attr<{ KEY::KPRIGHTPAREN }>),
    New(Attr<{ KEY::NEW }>),
    Redo(Attr<{ KEY::REDO }>),
    F13(Attr<{ KEY::F13 }>),
    F14(Attr<{ KEY::F14 }>),
    F15(Attr<{ KEY::F15 }>),
    F16(Attr<{ KEY::F16 }>),
    F17(Attr<{ KEY::F17 }>),
    F18(Attr<{ KEY::F18 }>),
    F19(Attr<{ KEY::F19 }>),
    F20(Attr<{ KEY::F20 }>),
    F21(Attr<{ KEY::F21 }>),
    F22(Attr<{ KEY::F22 }>),
    F23(Attr<{ KEY::F23 }>),
    F24(Attr<{ KEY::F24 }>),
    PlayCd(Attr<{ KEY::PLAYCD }>),
    PauseCd(Attr<{ KEY::PAUSECD }>),
    Prog3(Attr<{ KEY::PROG3 }>),
    Prog4(Attr<{ KEY::PROG4 }>),
    AllApplications(Attr<{ KEY::ALL_APPLICATIONS }>),
    Dashboard(Attr<{ KEY::DASHBOARD }>),
    Suspend(Attr<{ KEY::SUSPEND }>),
    Close(Attr<{ KEY::CLOSE }>),
    Play(Attr<{ KEY::PLAY }>),
    FastForward(Attr<{ KEY::FASTFORWARD }>),
    Bassboost(Attr<{ KEY::BASSBOOST }>),
    Print(Attr<{ KEY::PRINT }>),
    Hp(Attr<{ KEY::HP }>),
    Camera(Attr<{ KEY::CAMERA }>),
    Sound(Attr<{ KEY::SOUND }>),
    Question(Attr<{ KEY::QUESTION }>),
    Email(Attr<{ KEY::EMAIL }>),
    Chat(Attr<{ KEY::CHAT }>),
    Search(Attr<{ KEY::SEARCH }>),
    Connect(Attr<{ KEY::CONNECT }>),
    Finance(Attr<{ KEY::FINANCE }>),
    Sport(Attr<{ KEY::SPORT }>),
    Shop(Attr<{ KEY::SHOP }>),
    Alterase(Attr<{ KEY::ALTERASE }>),
    Cancel(Attr<{ KEY::CANCEL }>),
    BrightnessDown(Attr<{ KEY::BRIGHTNESSDOWN }>),
    BrightnessUp(Attr<{ KEY::BRIGHTNESSUP }>),
    Media(Attr<{ KEY::MEDIA }>),
    Switchvideomode(Attr<{ KEY::SWITCHVIDEOMODE }>),
    KbdIllumToggle(Attr<{ KEY::KBDILLUMTOGGLE }>),
    KbdIllumDown(Attr<{ KEY::KBDILLUMDOWN }>),
    KbdIllumUp(Attr<{ KEY::KBDILLUMUP }>),
    Send(Attr<{ KEY::SEND }>),
    Reply(Attr<{ KEY::REPLY }>),
    ForwardMail(Attr<{ KEY::FORWARDMAIL }>),
    Save(Attr<{ KEY::SAVE }>),
    Documents(Attr<{ KEY::DOCUMENTS }>),
    Battery(Attr<{ KEY::BATTERY }>),
    Bluetooth(Attr<{ KEY::BLUETOOTH }>),
    Wlan(Attr<{ KEY::WLAN }>),
    Uwb(Attr<{ KEY::UWB }>),
    Unknown(Attr<{ KEY::UNKNOWN }>),
    VideoNext(Attr<{ KEY::VIDEO_NEXT }>),
    VideoPrev(Attr<{ KEY::VIDEO_PREV }>),
    BrightnessCycle(Attr<{ KEY::BRIGHTNESS_CYCLE }>),
    BrightnessAuto(Attr<{ KEY::BRIGHTNESS_AUTO }>),
    BrightnessZero(Attr<{ KEY::BRIGHTNESS_ZERO }>),
    DisplayOff(Attr<{ KEY::DISPLAY_OFF }>),
    Wwan(Attr<{ KEY::WWAN }>),
    Wimax(Attr<{ KEY::WIMAX }>),
    Rfkill(Attr<{ KEY::RFKILL }>),
    MicMute(Attr<{ KEY::MICMUTE }>),
    Ok(Attr<{ KEY::OK }>),
    Select(Attr<{ KEY::SELECT }>),
    GoTo(Attr<{ KEY::GOTO }>),
    Clear(Attr<{ KEY::CLEAR }>),
    Power2(Attr<{ KEY::POWER2 }>),
    Option(Attr<{ KEY::OPTION }>),
    Info(Attr<{ KEY::INFO }>),
    Time(Attr<{ KEY::TIME }>),
    Vendor(Attr<{ KEY::VENDOR }>),
    Archive(Attr<{ KEY::ARCHIVE }>),
    Program(Attr<{ KEY::PROGRAM }>),
    Channel(Attr<{ KEY::CHANNEL }>),
    Favorites(Attr<{ KEY::FAVORITES }>),
    Epg(Attr<{ KEY::EPG }>),
    Pvr(Attr<{ KEY::PVR }>),
    Mhp(Attr<{ KEY::MHP }>),
    Language(Attr<{ KEY::LANGUAGE }>),
    Title(Attr<{ KEY::TITLE }>),
    Subtitle(Attr<{ KEY::SUBTITLE }>),
    Angle(Attr<{ KEY::ANGLE }>),
    FullScreen(Attr<{ KEY::FULL_SCREEN }>),
    Zoom(Attr<{ KEY::ZOOM }>),
    Mode(Attr<{ KEY::MODE }>),
    Keyboard(Attr<{ KEY::KEYBOARD }>),
    AspectRatio(Attr<{ KEY::ASPECT_RATIO }>),
    Screen(Attr<{ KEY::SCREEN }>),
    Pc(Attr<{ KEY::PC }>),
    Tv(Attr<{ KEY::TV }>),
    Tv2(Attr<{ KEY::TV2 }>),
    Vcr(Attr<{ KEY::VCR }>),
    Vcr2(Attr<{ KEY::VCR2 }>),
    Sat(Attr<{ KEY::SAT }>),
    Sat2(Attr<{ KEY::SAT2 }>),
    Cd(Attr<{ KEY::CD }>),
    Tape(Attr<{ KEY::TAPE }>),
    Radio(Attr<{ KEY::RADIO }>),
    Tuner(Attr<{ KEY::TUNER }>),
    Player(Attr<{ KEY::PLAYER }>),
    Text(Attr<{ KEY::TEXT }>),
    Dvd(Attr<{ KEY::DVD }>),
    Aux(Attr<{ KEY::AUX }>),
    Mp3(Attr<{ KEY::MP3 }>),
    Audio(Attr<{ KEY::AUDIO }>),
    Video(Attr<{ KEY::VIDEO }>),
    Directory(Attr<{ KEY::DIRECTORY }>),
    List(Attr<{ KEY::LIST }>),
    Memo(Attr<{ KEY::MEMO }>),
    Calendar(Attr<{ KEY::CALENDAR }>),
    Red(Attr<{ KEY::RED }>),
    Green(Attr<{ KEY::GREEN }>),
    Yellow(Attr<{ KEY::YELLOW }>),
    Blue(Attr<{ KEY::BLUE }>),
    ChannelUp(Attr<{ KEY::CHANNELUP }>),
    ChannelDown(Attr<{ KEY::CHANNELDOWN }>),
    First(Attr<{ KEY::FIRST }>),
    Last(Attr<{ KEY::LAST }>),
    Ab(Attr<{ KEY::AB }>),
    Next(Attr<{ KEY::NEXT }>),
    Restart(Attr<{ KEY::RESTART }>),
    Slow(Attr<{ KEY::SLOW }>),
    Shuffle(Attr<{ KEY::SHUFFLE }>),
    Break(Attr<{ KEY::BREAK }>),
    Previous(Attr<{ KEY::PREVIOUS }>),
    Digits(Attr<{ KEY::DIGITS }>),
    Teen(Attr<{ KEY::TEEN }>),
    Twen(Attr<{ KEY::TWEN }>),
    Videophone(Attr<{ KEY::VIDEOPHONE }>),
    Games(Attr<{ KEY::GAMES }>),
    ZoomIn(Attr<{ KEY::ZOOMIN }>),
    ZoomOut(Attr<{ KEY::ZOOMOUT }>),
    ZoomReset(Attr<{ KEY::ZOOMRESET }>),
    WordProcessor(Attr<{ KEY::WORDPROCESSOR }>),
    Editor(Attr<{ KEY::EDITOR }>),
    Spreadsheet(Attr<{ KEY::SPREADSHEET }>),
    GraphicsEditor(Attr<{ KEY::GRAPHICSEDITOR }>),
    Presentation(Attr<{ KEY::PRESENTATION }>),
    Database(Attr<{ KEY::DATABASE }>),
    News(Attr<{ KEY::NEWS }>),
    VoiceMail(Attr<{ KEY::VOICEMAIL }>),
    AddressBook(Attr<{ KEY::ADDRESSBOOK }>),
    Messenger(Attr<{ KEY::MESSENGER }>),
    DisplayToggle(Attr<{ KEY::DISPLAYTOGGLE }>),
    BrightnessToggle(Attr<{ KEY::BRIGHTNESS_TOGGLE }>),
    Spellcheck(Attr<{ KEY::SPELLCHECK }>),
    Logoff(Attr<{ KEY::LOGOFF }>),
    Dollar(Attr<{ KEY::DOLLAR }>),
    Euro(Attr<{ KEY::EURO }>),
    FrameBack(Attr<{ KEY::FRAMEBACK }>),
    FrameForward(Attr<{ KEY::FRAMEFORWARD }>),
    ContextMenu(Attr<{ KEY::CONTEXT_MENU }>),
    MediaRepeat(Attr<{ KEY::MEDIA_REPEAT }>),
    TenChannelsUp(Attr<{ KEY::TENCHANNELSUP }>),
    TenChannelsDown(Attr<{ KEY::TENCHANNELSDOWN }>),
    Images(Attr<{ KEY::IMAGES }>),
    NotificationCenter(Attr<{ KEY::NOTIFICATION_CENTER }>),
    PickupPhone(Attr<{ KEY::PICKUP_PHONE }>),
    HangupPhone(Attr<{ KEY::HANGUP_PHONE }>),
    LinkPhone(Attr<{ KEY::LINK_PHONE }>),
    DelEol(Attr<{ KEY::DEL_EOL }>),
    DelEos(Attr<{ KEY::DEL_EOS }>),
    InsLine(Attr<{ KEY::INS_LINE }>),
    DelLine(Attr<{ KEY::DEL_LINE }>),
    Fn(Attr<{ KEY::FN }>),
    FnEsc(Attr<{ KEY::FN_ESC }>),
    FnF1(Attr<{ KEY::FN_F1 }>),
    FnF2(Attr<{ KEY::FN_F2 }>),
    FnF3(Attr<{ KEY::FN_F3 }>),
    FnF4(Attr<{ KEY::FN_F4 }>),
    FnF5(Attr<{ KEY::FN_F5 }>),
    FnF6(Attr<{ KEY::FN_F6 }>),
    FnF7(Attr<{ KEY::FN_F7 }>),
    FnF8(Attr<{ KEY::FN_F8 }>),
    FnF9(Attr<{ KEY::FN_F9 }>),
    FnF10(Attr<{ KEY::FN_F10 }>),
    FnF11(Attr<{ KEY::FN_F11 }>),
    FnF12(Attr<{ KEY::FN_F12 }>),
    Fn1(Attr<{ KEY::FN_1 }>),
    Fn2(Attr<{ KEY::FN_2 }>),
    FnD(Attr<{ KEY::FN_D }>),
    FnE(Attr<{ KEY::FN_E }>),
    FnF(Attr<{ KEY::FN_F }>),
    FnS(Attr<{ KEY::FN_S }>),
    FnB(Attr<{ KEY::FN_B }>),
    FnRightShift(Attr<{ KEY::FN_RIGHT_SHIFT }>),
    BrlDot1(Attr<{ KEY::BRL_DOT1 }>),
    BrlDot2(Attr<{ KEY::BRL_DOT2 }>),
    BrlDot3(Attr<{ KEY::BRL_DOT3 }>),
    BrlDot4(Attr<{ KEY::BRL_DOT4 }>),
    BrlDot5(Attr<{ KEY::BRL_DOT5 }>),
    BrlDot6(Attr<{ KEY::BRL_DOT6 }>),
    BrlDot7(Attr<{ KEY::BRL_DOT7 }>),
    BrlDot8(Attr<{ KEY::BRL_DOT8 }>),
    BrlDot9(Attr<{ KEY::BRL_DOT9 }>),
    BrlDot10(Attr<{ KEY::BRL_DOT10 }>),
    Numeric0(Attr<{ KEY::NUMERIC_0 }>),
    Numeric1(Attr<{ KEY::NUMERIC_1 }>),
    Numeric2(Attr<{ KEY::NUMERIC_2 }>),
    Numeric3(Attr<{ KEY::NUMERIC_3 }>),
    Numeric4(Attr<{ KEY::NUMERIC_4 }>),
    Numeric5(Attr<{ KEY::NUMERIC_5 }>),
    Numeric6(Attr<{ KEY::NUMERIC_6 }>),
    Numeric7(Attr<{ KEY::NUMERIC_7 }>),
    Numeric8(Attr<{ KEY::NUMERIC_8 }>),
    Numeric9(Attr<{ KEY::NUMERIC_9 }>),
    NumericStar(Attr<{ KEY::NUMERIC_STAR }>),
    NumericPound(Attr<{ KEY::NUMERIC_POUND }>),
    NumericA(Attr<{ KEY::NUMERIC_A }>),
    NumericB(Attr<{ KEY::NUMERIC_B }>),
    NumericC(Attr<{ KEY::NUMERIC_C }>),
    NumericD(Attr<{ KEY::NUMERIC_D }>),
    CameraFocus(Attr<{ KEY::CAMERA_FOCUS }>),
    WpsButton(Attr<{ KEY::WPS_BUTTON }>),
    TouchpadToggle(Attr<{ KEY::TOUCHPAD_TOGGLE }>),
    TouchpadOn(Attr<{ KEY::TOUCHPAD_ON }>),
    TouchpadOff(Attr<{ KEY::TOUCHPAD_OFF }>),
    CameraZoomin(Attr<{ KEY::CAMERA_ZOOMIN }>),
    CameraZoomout(Attr<{ KEY::CAMERA_ZOOMOUT }>),
    CameraUp(Attr<{ KEY::CAMERA_UP }>),
    CameraDown(Attr<{ KEY::CAMERA_DOWN }>),
    CameraLeft(Attr<{ KEY::CAMERA_LEFT }>),
    CameraRight(Attr<{ KEY::CAMERA_RIGHT }>),
    AttendantOn(Attr<{ KEY::ATTENDANT_ON }>),
    AttendantOff(Attr<{ KEY::ATTENDANT_OFF }>),
    AttendantToggle(Attr<{ KEY::ATTENDANT_TOGGLE }>),
    LightsToggle(Attr<{ KEY::LIGHTS_TOGGLE }>),
    AlsToggle(Attr<{ KEY::ALS_TOGGLE }>),
    RotateLockToggle(Attr<{ KEY::ROTATE_LOCK_TOGGLE }>),
    RefreshRateToggle(Attr<{ KEY::REFRESH_RATE_TOGGLE }>),
    Buttonconfig(Attr<{ KEY::BUTTONCONFIG }>),
    TaskManager(Attr<{ KEY::TASKMANAGER }>),
    Journal(Attr<{ KEY::JOURNAL }>),
    ControlPanel(Attr<{ KEY::CONTROLPANEL }>),
    AppSelect(Attr<{ KEY::APPSELECT }>),
    ScreenSaver(Attr<{ KEY::SCREENSAVER }>),
    VoiceCommand(Attr<{ KEY::VOICECOMMAND }>),
    Assistant(Attr<{ KEY::ASSISTANT }>),
    KbdLayoutNext(Attr<{ KEY::KBD_LAYOUT_NEXT }>),
    EmojiPicker(Attr<{ KEY::EMOJI_PICKER }>),
    Dictate(Attr<{ KEY::DICTATE }>),
    CameraAccessEnable(Attr<{ KEY::CAMERA_ACCESS_ENABLE }>),
    CameraAccessDisable(Attr<{ KEY::CAMERA_ACCESS_DISABLE }>),
    CameraAccessToggle(Attr<{ KEY::CAMERA_ACCESS_TOGGLE }>),
    Accessibility(Attr<{ KEY::ACCESSIBILITY }>),
    DoNotDisturb(Attr<{ KEY::DO_NOT_DISTURB }>),
    BrightnessMin(Attr<{ KEY::BRIGHTNESS_MIN }>),
    BrightnessMax(Attr<{ KEY::BRIGHTNESS_MAX }>),
    EprivacyScreenOn(Attr<{ KEY::EPRIVACY_SCREEN_ON }>),
    EprivacyScreenOff(Attr<{ KEY::EPRIVACY_SCREEN_OFF }>),
    ActionOnSelection(Attr<{ KEY::ACTION_ON_SELECTION }>),
    ContextualInsert(Attr<{ KEY::CONTEXTUAL_INSERT }>),
    ContextualQuery(Attr<{ KEY::CONTEXTUAL_QUERY }>),
    KbdinputassistPrev(Attr<{ KEY::KBDINPUTASSIST_PREV }>),
    KbdinputassistNext(Attr<{ KEY::KBDINPUTASSIST_NEXT }>),
    KbdinputassistPrevgroup(Attr<{ KEY::KBDINPUTASSIST_PREVGROUP }>),
    KbdinputassistNextgroup(Attr<{ KEY::KBDINPUTASSIST_NEXTGROUP }>),
    KbdinputassistAccept(Attr<{ KEY::KBDINPUTASSIST_ACCEPT }>),
    KbdinputassistCancel(Attr<{ KEY::KBDINPUTASSIST_CANCEL }>),
    RightUp(Attr<{ KEY::RIGHT_UP }>),
    RightDown(Attr<{ KEY::RIGHT_DOWN }>),
    LeftUp(Attr<{ KEY::LEFT_UP }>),
    LeftDown(Attr<{ KEY::LEFT_DOWN }>),
    RootMenu(Attr<{ KEY::ROOT_MENU }>),
    MediaTopMenu(Attr<{ KEY::MEDIA_TOP_MENU }>),
    Numeric11(Attr<{ KEY::NUMERIC_11 }>),
    Numeric12(Attr<{ KEY::NUMERIC_12 }>),
    AudioDesc(Attr<{ KEY::AUDIO_DESC }>),
    Mode3d(Attr<{ KEY::MODE_3D }>),
    NextFavorite(Attr<{ KEY::NEXT_FAVORITE }>),
    StopRecord(Attr<{ KEY::STOP_RECORD }>),
    PauseRecord(Attr<{ KEY::PAUSE_RECORD }>),
    Vod(Attr<{ KEY::VOD }>),
    Unmute(Attr<{ KEY::UNMUTE }>),
    FastReverse(Attr<{ KEY::FASTREVERSE }>),
    SlowReverse(Attr<{ KEY::SLOWREVERSE }>),
    Data(Attr<{ KEY::DATA }>),
    OnScreenKeyboard(Attr<{ KEY::ONSCREEN_KEYBOARD }>),
    PrivacyScreenToggle(Attr<{ KEY::PRIVACY_SCREEN_TOGGLE }>),
    SelectiveScreenshot(Attr<{ KEY::SELECTIVE_SCREENSHOT }>),
    NextElement(Attr<{ KEY::NEXT_ELEMENT }>),
    PreviousElement(Attr<{ KEY::PREVIOUS_ELEMENT }>),
    AutoPilotEngageToggle(Attr<{ KEY::AUTOPILOT_ENGAGE_TOGGLE }>),
    MarkWayPoint(Attr<{ KEY::MARK_WAYPOINT }>),
    Sos(Attr<{ KEY::SOS }>),
    NavChart(Attr<{ KEY::NAV_CHART }>),
    FishingChart(Attr<{ KEY::FISHING_CHART }>),
    SingleRangeRadar(Attr<{ KEY::SINGLE_RANGE_RADAR }>),
    DualRangeRadar(Attr<{ KEY::DUAL_RANGE_RADAR }>),
    RadarOverlay(Attr<{ KEY::RADAR_OVERLAY }>),
    TraditionalSonar(Attr<{ KEY::TRADITIONAL_SONAR }>),
    ClearVuSonar(Attr<{ KEY::CLEARVU_SONAR }>),
    SideVuSonar(Attr<{ KEY::SIDEVU_SONAR }>),
    NavInfo(Attr<{ KEY::NAV_INFO }>),
    BrightnessMenu(Attr<{ KEY::BRIGHTNESS_MENU }>),
    Macro1(Attr<{ KEY::MACRO1 }>),
    Macro2(Attr<{ KEY::MACRO2 }>),
    Macro3(Attr<{ KEY::MACRO3 }>),
    Macro4(Attr<{ KEY::MACRO4 }>),
    Macro5(Attr<{ KEY::MACRO5 }>),
    Macro6(Attr<{ KEY::MACRO6 }>),
    Macro7(Attr<{ KEY::MACRO7 }>),
    Macro8(Attr<{ KEY::MACRO8 }>),
    Macro9(Attr<{ KEY::MACRO9 }>),
    Macro10(Attr<{ KEY::MACRO10 }>),
    Macro11(Attr<{ KEY::MACRO11 }>),
    Macro12(Attr<{ KEY::MACRO12 }>),
    Macro13(Attr<{ KEY::MACRO13 }>),
    Macro14(Attr<{ KEY::MACRO14 }>),
    Macro15(Attr<{ KEY::MACRO15 }>),
    Macro16(Attr<{ KEY::MACRO16 }>),
    Macro17(Attr<{ KEY::MACRO17 }>),
    Macro18(Attr<{ KEY::MACRO18 }>),
    Macro19(Attr<{ KEY::MACRO19 }>),
    Macro20(Attr<{ KEY::MACRO20 }>),
    Macro21(Attr<{ KEY::MACRO21 }>),
    Macro22(Attr<{ KEY::MACRO22 }>),
    Macro23(Attr<{ KEY::MACRO23 }>),
    Macro24(Attr<{ KEY::MACRO24 }>),
    Macro25(Attr<{ KEY::MACRO25 }>),
    Macro26(Attr<{ KEY::MACRO26 }>),
    Macro27(Attr<{ KEY::MACRO27 }>),
    Macro28(Attr<{ KEY::MACRO28 }>),
    Macro29(Attr<{ KEY::MACRO29 }>),
    Macro30(Attr<{ KEY::MACRO30 }>),
    MacroRecordStart(Attr<{ KEY::MACRO_RECORD_START }>),
    MacroRecordStop(Attr<{ KEY::MACRO_RECORD_STOP }>),
    MacroPresetCycle(Attr<{ KEY::MACRO_PRESET_CYCLE }>),
    MacroPreset1(Attr<{ KEY::MACRO_PRESET1 }>),
    MacroPreset2(Attr<{ KEY::MACRO_PRESET2 }>),
    MacroPreset3(Attr<{ KEY::MACRO_PRESET3 }>),
    KbdLcdMenu1(Attr<{ KEY::KBD_LCD_MENU1 }>),
    KbdLcdMenu2(Attr<{ KEY::KBD_LCD_MENU2 }>),
    KbdLcdMenu3(Attr<{ KEY::KBD_LCD_MENU3 }>),
    KbdLcdMenu4(Attr<{ KEY::KBD_LCD_MENU4 }>),
    KbdLcdMenu5(Attr<{ KEY::KBD_LCD_MENU5 }>),
    Performance(Attr<{ KEY::PERFORMANCE }>),
    MinInteresting(Attr<{ KEY::MIN_INTERESTING }>),
    Max(Attr<{ KEY::MAX }>),
}
