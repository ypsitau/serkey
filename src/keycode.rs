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
    pub const CONTROL_L: u8             = 1 << 0;
    pub const SHIFT_L: u8               = 1 << 1;
    pub const ALT_L: u8                 = 1 << 2;
    pub const GUI_L: u8                 = 1 << 3;
    pub const CONTROL_R: u8             = 1 << 4;
    pub const SHIFT_R: u8               = 1 << 5;
    pub const ALT_R: u8                 = 1 << 6;
    pub const GUI_R: u8                 = 1 << 7;
    pub fn control(self) -> Self        { self.control_l() }
    pub fn control_l(self) -> Self      { Self { bits: self.bits | Self::CONTROL_L, } }
    pub fn control_r(self) -> Self      { Self { bits: self.bits | Self::CONTROL_R, } }
    pub fn shift(self) -> Self          { self.shift_l() }
    pub fn shift_r(self) -> Self        { Self { bits: self.bits | Self::SHIFT_R, } }
    pub fn shift_l(self) -> Self        { Self { bits: self.bits | Self::SHIFT_L, } }
    pub fn alt(self) -> Self            { self.alt_l() }
    pub fn alt_l(self) -> Self          { Self { bits: self.bits | Self::ALT_L, } }
    pub fn alt_r(self) -> Self          { Self { bits: self.bits | Self::ALT_R, } }
    pub fn gui(self) -> Self            { self.gui_l() }
    pub fn gui_l(self) -> Self          { Self { bits: self.bits | Self::GUI_L, } }
    pub fn gui_r(self) -> Self          { Self { bits: self.bits | Self::GUI_R, } }
    pub fn is_control(&self) -> bool    { self.bits & (Self::CONTROL_L | Self::CONTROL_R) != 0 }
    pub fn is_control_l(&self) -> bool  { self.bits & Self::CONTROL_L != 0 }
    pub fn is_control_r(&self) -> bool  { self.bits & Self::CONTROL_R != 0 }
    pub fn is_shift(&self) -> bool      { self.bits & (Self::SHIFT_L | Self::SHIFT_R) != 0 }
    pub fn is_shift_l(&self) -> bool    { self.bits & Self::SHIFT_L != 0 }
    pub fn is_shift_r(&self) -> bool    { self.bits & Self::SHIFT_R != 0 }
    pub fn is_alt(&self) -> bool        { self.bits & (Self::ALT_L | Self::ALT_R) != 0 }
    pub fn is_alt_l(&self) -> bool      { self.bits & Self::ALT_L != 0 }
    pub fn is_alt_r(&self) -> bool      { self.bits & Self::ALT_R != 0 }
    pub fn is_gui(&self) -> bool        { self.bits & (Self::GUI_L | Self::GUI_R) != 0 }
    pub fn is_gui_l(&self) -> bool      { self.bits & Self::GUI_L != 0 }
    pub fn is_gui_r(&self) -> bool      { self.bits & Self::GUI_R != 0 }
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
    pub fn is_control(&self) -> bool    { self.modifier.is_control() }
    pub fn is_control_l(&self) -> bool  { self.modifier.is_control_l() }
    pub fn is_control_r(&self) -> bool  { self.modifier.is_control_r() }
    pub fn is_shift(&self) -> bool      { self.modifier.is_shift() }
    pub fn is_shift_l(&self) -> bool    { self.modifier.is_shift_l() }
    pub fn is_shift_r(&self) -> bool    { self.modifier.is_shift_r() }
    pub fn is_alt(&self) -> bool        { self.modifier.is_alt() }
    pub fn is_alt_l(&self) -> bool      { self.modifier.is_alt_l() }
    pub fn is_alt_r(&self) -> bool      { self.modifier.is_alt_r() }
    pub fn is_gui(&self) -> bool        { self.modifier.is_gui() }
    pub fn is_gui_l(&self) -> bool      { self.modifier.is_gui_l() }
    pub fn is_gui_r(&self) -> bool      { self.modifier.is_gui_r() }
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
    Leftbrace(Attr<{ KEY::LEFTBRACE }>),
    Rightbrace(Attr<{ KEY::RIGHTBRACE }>),
    Enter(Attr<{ KEY::ENTER }>),
    Leftctrl(Attr<{ KEY::LEFTCTRL }>),
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
    Leftshift(Attr<{ KEY::LEFTSHIFT }>),
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
    Rightshift(Attr<{ KEY::RIGHTSHIFT }>),
    Kpasterisk(Attr<{ KEY::KPASTERISK }>),
    Leftalt(Attr<{ KEY::LEFTALT }>),
    Space(Attr<{ KEY::SPACE }>),
    Capslock(Attr<{ KEY::CAPSLOCK }>),
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
    Numlock(Attr<{ KEY::NUMLOCK }>),
    Scrolllock(Attr<{ KEY::SCROLLLOCK }>),
    Kp7(Attr<{ KEY::KP7 }>),
    Kp8(Attr<{ KEY::KP8 }>),
    Kp9(Attr<{ KEY::KP9 }>),
    Kpminus(Attr<{ KEY::KPMINUS }>),
    Kp4(Attr<{ KEY::KP4 }>),
    Kp5(Attr<{ KEY::KP5 }>),
    Kp6(Attr<{ KEY::KP6 }>),
    Kpplus(Attr<{ KEY::KPPLUS }>),
    Kp1(Attr<{ KEY::KP1 }>),
    Kp2(Attr<{ KEY::KP2 }>),
    Kp3(Attr<{ KEY::KP3 }>),
    Kp0(Attr<{ KEY::KP0 }>),
    Kpdot(Attr<{ KEY::KPDOT }>),
    Zenkakuhankaku(Attr<{ KEY::ZENKAKUHANKAKU }>),
    Nd102(Attr<{ KEY::ND102 }>),
    F11(Attr<{ KEY::F11 }>),
    F12(Attr<{ KEY::F12 }>),
    Ro(Attr<{ KEY::RO }>),
    Katakana(Attr<{ KEY::KATAKANA }>),
    Hiragana(Attr<{ KEY::HIRAGANA }>),
    Henkan(Attr<{ KEY::HENKAN }>),
    Katakanahiragana(Attr<{ KEY::KATAKANAHIRAGANA }>),
    Muhenkan(Attr<{ KEY::MUHENKAN }>),
    Kpjpcomma(Attr<{ KEY::KPJPCOMMA }>),
    Kpenter(Attr<{ KEY::KPENTER }>),
    Rightctrl(Attr<{ KEY::RIGHTCTRL }>),
    Kpslash(Attr<{ KEY::KPSLASH }>),
    Sysrq(Attr<{ KEY::SYSRQ }>),
    Rightalt(Attr<{ KEY::RIGHTALT }>),
    Linefeed(Attr<{ KEY::LINEFEED }>),
    Home(Attr<{ KEY::HOME }>),
    Up(Attr<{ KEY::UP }>),
    Pageup(Attr<{ KEY::PAGEUP }>),
    Left(Attr<{ KEY::LEFT }>),
    Right(Attr<{ KEY::RIGHT }>),
    End(Attr<{ KEY::END }>),
    Down(Attr<{ KEY::DOWN }>),
    Pagedown(Attr<{ KEY::PAGEDOWN }>),
    Insert(Attr<{ KEY::INSERT }>),
    Delete(Attr<{ KEY::DELETE }>),
    Macro(Attr<{ KEY::MACRO }>),
    Mute(Attr<{ KEY::MUTE }>),
    Volumedown(Attr<{ KEY::VOLUMEDOWN }>),
    Volumeup(Attr<{ KEY::VOLUMEUP }>),
    Power(Attr<{ KEY::POWER }>),
    Kpequal(Attr<{ KEY::KPEQUAL }>),
    Kpplusminus(Attr<{ KEY::KPPLUSMINUS }>),
    Pause(Attr<{ KEY::PAUSE }>),
    Scale(Attr<{ KEY::SCALE }>),
    Kpcomma(Attr<{ KEY::KPCOMMA }>),
    Hangeul(Attr<{ KEY::HANGEUL }>),
    Hanguel(Attr<{ KEY::HANGUEL }>),
    Hanja(Attr<{ KEY::HANJA }>),
    Yen(Attr<{ KEY::YEN }>),
    Leftmeta(Attr<{ KEY::LEFTMETA }>),
    Rightmeta(Attr<{ KEY::RIGHTMETA }>),
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
    Wakeup(Attr<{ KEY::WAKEUP }>),
    File(Attr<{ KEY::FILE }>),
    Sendfile(Attr<{ KEY::SENDFILE }>),
    Deletefile(Attr<{ KEY::DELETEFILE }>),
    Xfer(Attr<{ KEY::XFER }>),
    Prog1(Attr<{ KEY::PROG1 }>),
    Prog2(Attr<{ KEY::PROG2 }>),
    Www(Attr<{ KEY::WWW }>),
    Msdos(Attr<{ KEY::MSDOS }>),
    Coffee(Attr<{ KEY::COFFEE }>),
    Screenlock(Attr<{ KEY::SCREENLOCK }>),
    RotateDisplay(Attr<{ KEY::ROTATE_DISPLAY }>),
    Direction(Attr<{ KEY::DIRECTION }>),
    Cyclewindows(Attr<{ KEY::CYCLEWINDOWS }>),
    Mail(Attr<{ KEY::MAIL }>),
    Bookmarks(Attr<{ KEY::BOOKMARKS }>),
    Computer(Attr<{ KEY::COMPUTER }>),
    Back(Attr<{ KEY::BACK }>),
    Forward(Attr<{ KEY::FORWARD }>),
    Closecd(Attr<{ KEY::CLOSECD }>),
    Ejectcd(Attr<{ KEY::EJECTCD }>),
    Ejectclosecd(Attr<{ KEY::EJECTCLOSECD }>),
    Nextsong(Attr<{ KEY::NEXTSONG }>),
    Playpause(Attr<{ KEY::PLAYPAUSE }>),
    Previoussong(Attr<{ KEY::PREVIOUSSONG }>),
    Stopcd(Attr<{ KEY::STOPCD }>),
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
    Scrollup(Attr<{ KEY::SCROLLUP }>),
    Scrolldown(Attr<{ KEY::SCROLLDOWN }>),
    Kpleftparen(Attr<{ KEY::KPLEFTPAREN }>),
    Kprightparen(Attr<{ KEY::KPRIGHTPAREN }>),
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
    Playcd(Attr<{ KEY::PLAYCD }>),
    Pausecd(Attr<{ KEY::PAUSECD }>),
    Prog3(Attr<{ KEY::PROG3 }>),
    Prog4(Attr<{ KEY::PROG4 }>),
    AllApplications(Attr<{ KEY::ALL_APPLICATIONS }>),
    Dashboard(Attr<{ KEY::DASHBOARD }>),
    Suspend(Attr<{ KEY::SUSPEND }>),
    Close(Attr<{ KEY::CLOSE }>),
    Play(Attr<{ KEY::PLAY }>),
    Fastforward(Attr<{ KEY::FASTFORWARD }>),
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
    Brightnessdown(Attr<{ KEY::BRIGHTNESSDOWN }>),
    Brightnessup(Attr<{ KEY::BRIGHTNESSUP }>),
    Media(Attr<{ KEY::MEDIA }>),
    Switchvideomode(Attr<{ KEY::SWITCHVIDEOMODE }>),
    Kbdillumtoggle(Attr<{ KEY::KBDILLUMTOGGLE }>),
    Kbdillumdown(Attr<{ KEY::KBDILLUMDOWN }>),
    Kbdillumup(Attr<{ KEY::KBDILLUMUP }>),
    Send(Attr<{ KEY::SEND }>),
    Reply(Attr<{ KEY::REPLY }>),
    Forwardmail(Attr<{ KEY::FORWARDMAIL }>),
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
    Micmute(Attr<{ KEY::MICMUTE }>),
    Ok(Attr<{ KEY::OK }>),
    Select(Attr<{ KEY::SELECT }>),
    Goto(Attr<{ KEY::GOTO }>),
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
    Channelup(Attr<{ KEY::CHANNELUP }>),
    Channeldown(Attr<{ KEY::CHANNELDOWN }>),
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
    Zoomin(Attr<{ KEY::ZOOMIN }>),
    Zoomout(Attr<{ KEY::ZOOMOUT }>),
    Zoomreset(Attr<{ KEY::ZOOMRESET }>),
    Wordprocessor(Attr<{ KEY::WORDPROCESSOR }>),
    Editor(Attr<{ KEY::EDITOR }>),
    Spreadsheet(Attr<{ KEY::SPREADSHEET }>),
    Graphicseditor(Attr<{ KEY::GRAPHICSEDITOR }>),
    Presentation(Attr<{ KEY::PRESENTATION }>),
    Database(Attr<{ KEY::DATABASE }>),
    News(Attr<{ KEY::NEWS }>),
    Voicemail(Attr<{ KEY::VOICEMAIL }>),
    Addressbook(Attr<{ KEY::ADDRESSBOOK }>),
    Messenger(Attr<{ KEY::MESSENGER }>),
    Displaytoggle(Attr<{ KEY::DISPLAYTOGGLE }>),
    BrightnessToggle(Attr<{ KEY::BRIGHTNESS_TOGGLE }>),
    Spellcheck(Attr<{ KEY::SPELLCHECK }>),
    Logoff(Attr<{ KEY::LOGOFF }>),
    Dollar(Attr<{ KEY::DOLLAR }>),
    Euro(Attr<{ KEY::EURO }>),
    Frameback(Attr<{ KEY::FRAMEBACK }>),
    Frameforward(Attr<{ KEY::FRAMEFORWARD }>),
    ContextMenu(Attr<{ KEY::CONTEXT_MENU }>),
    MediaRepeat(Attr<{ KEY::MEDIA_REPEAT }>),
    Tenchannelsup(Attr<{ KEY::TENCHANNELSUP }>),
    Tenchannelsdown(Attr<{ KEY::TENCHANNELSDOWN }>),
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
    Taskmanager(Attr<{ KEY::TASKMANAGER }>),
    Journal(Attr<{ KEY::JOURNAL }>),
    Controlpanel(Attr<{ KEY::CONTROLPANEL }>),
    Appselect(Attr<{ KEY::APPSELECT }>),
    Screensaver(Attr<{ KEY::SCREENSAVER }>),
    Voicecommand(Attr<{ KEY::VOICECOMMAND }>),
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
    Fastreverse(Attr<{ KEY::FASTREVERSE }>),
    Slowreverse(Attr<{ KEY::SLOWREVERSE }>),
    Data(Attr<{ KEY::DATA }>),
    OnscreenKeyboard(Attr<{ KEY::ONSCREEN_KEYBOARD }>),
    PrivacyScreenToggle(Attr<{ KEY::PRIVACY_SCREEN_TOGGLE }>),
    SelectiveScreenshot(Attr<{ KEY::SELECTIVE_SCREENSHOT }>),
    NextElement(Attr<{ KEY::NEXT_ELEMENT }>),
    PreviousElement(Attr<{ KEY::PREVIOUS_ELEMENT }>),
    AutopilotEngageToggle(Attr<{ KEY::AUTOPILOT_ENGAGE_TOGGLE }>),
    MarkWaypoint(Attr<{ KEY::MARK_WAYPOINT }>),
    Sos(Attr<{ KEY::SOS }>),
    NavChart(Attr<{ KEY::NAV_CHART }>),
    FishingChart(Attr<{ KEY::FISHING_CHART }>),
    SingleRangeRadar(Attr<{ KEY::SINGLE_RANGE_RADAR }>),
    DualRangeRadar(Attr<{ KEY::DUAL_RANGE_RADAR }>),
    RadarOverlay(Attr<{ KEY::RADAR_OVERLAY }>),
    TraditionalSonar(Attr<{ KEY::TRADITIONAL_SONAR }>),
    ClearvuSonar(Attr<{ KEY::CLEARVU_SONAR }>),
    SidevuSonar(Attr<{ KEY::SIDEVU_SONAR }>),
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















/// Represents the virtual key codes for the keyboard, as defined in the Windows API.
/// Original GitHub repository: https://github.com/microsoft/windows-rs/
/// See the file: crates/libs/windows/src/Windows/Win32/winuser/mod.rs
#[allow(non_snake_case)]
pub mod VK {
    pub const A: u16 = 65;
    pub const B: u16 = 66;
    pub const C: u16 = 67;
    pub const D: u16 = 68;
    pub const E: u16 = 69;
    pub const F: u16 = 70;
    pub const G: u16 = 71;
    pub const H: u16 = 72;
    pub const I: u16 = 73;
    pub const J: u16 = 74;
    pub const K: u16 = 75;
    pub const L: u16 = 76;
    pub const M: u16 = 77;
    pub const N: u16 = 78;
    pub const O: u16 = 79;
    pub const P: u16 = 80;
    pub const Q: u16 = 81;
    pub const R: u16 = 82;
    pub const S: u16 = 83;
    pub const T: u16 = 84;
    pub const U: u16 = 85;
    pub const V: u16 = 86;
    pub const W: u16 = 87;
    pub const X: u16 = 88;
    pub const Y: u16 = 89;
    pub const Z: u16 = 90;
    pub const NUM0: u16 = 48;
    pub const NUM1: u16 = 49;
    pub const NUM2: u16 = 50;
    pub const NUM3: u16 = 51;
    pub const NUM4: u16 = 52;
    pub const NUM5: u16 = 53;
    pub const NUM6: u16 = 54;
    pub const NUM7: u16 = 55;
    pub const NUM8: u16 = 56;
    pub const NUM9: u16 = 57;
    pub const ACCEPT: u16 = 30;
    pub const ADD: u16 = 107;
    pub const APPS: u16 = 93;
    pub const ATTN: u16 = 246;
    pub const BACK: u16 = 8;
    pub const BROWSER_BACK: u16 = 166;
    pub const BROWSER_FAVORITES: u16 = 171;
    pub const BROWSER_FORWARD: u16 = 167;
    pub const BROWSER_HOME: u16 = 172;
    pub const BROWSER_REFRESH: u16 = 168;
    pub const BROWSER_SEARCH: u16 = 170;
    pub const BROWSER_STOP: u16 = 169;
    pub const CANCEL: u16 = 3;
    pub const CAPITAL: u16 = 20;
    pub const CLEAR: u16 = 12;
    pub const CONTROL: u16 = 17;
    pub const CONVERT: u16 = 28;
    pub const CRSEL: u16 = 247;
    pub const DECIMAL: u16 = 110;
    pub const DELETE: u16 = 46;
    pub const DIVIDE: u16 = 111;
    pub const DOWN: u16 = 40;
    pub const END: u16 = 35;
    pub const EREOF: u16 = 249;
    pub const ESCAPE: u16 = 27;
    pub const EXECUTE: u16 = 43;
    pub const EXSEL: u16 = 248;
    pub const F1: u16 = 112;
    pub const F10: u16 = 121;
    pub const F11: u16 = 122;
    pub const F12: u16 = 123;
    pub const F13: u16 = 124;
    pub const F14: u16 = 125;
    pub const F15: u16 = 126;
    pub const F16: u16 = 127;
    pub const F17: u16 = 128;
    pub const F18: u16 = 129;
    pub const F19: u16 = 130;
    pub const F2: u16 = 113;
    pub const F20: u16 = 131;
    pub const F21: u16 = 132;
    pub const F22: u16 = 133;
    pub const F23: u16 = 134;
    pub const F24: u16 = 135;
    pub const F3: u16 = 114;
    pub const F4: u16 = 115;
    pub const F5: u16 = 116;
    pub const F6: u16 = 117;
    pub const F7: u16 = 118;
    pub const F8: u16 = 119;
    pub const F9: u16 = 120;
    pub const FINAL: u16 = 24;
    pub const GAMEPAD_A: u16 = 195;
    pub const GAMEPAD_B: u16 = 196;
    pub const GAMEPAD_DPAD_DOWN: u16 = 204;
    pub const GAMEPAD_DPAD_LEFT: u16 = 205;
    pub const GAMEPAD_DPAD_RIGHT: u16 = 206;
    pub const GAMEPAD_DPAD_UP: u16 = 203;
    pub const GAMEPAD_LEFT_SHOULDER: u16 = 200;
    pub const GAMEPAD_LEFT_THUMBSTICK_BUTTON: u16 = 209;
    pub const GAMEPAD_LEFT_THUMBSTICK_DOWN: u16 = 212;
    pub const GAMEPAD_LEFT_THUMBSTICK_LEFT: u16 = 214;
    pub const GAMEPAD_LEFT_THUMBSTICK_RIGHT: u16 = 213;
    pub const GAMEPAD_LEFT_THUMBSTICK_UP: u16 = 211;
    pub const GAMEPAD_LEFT_TRIGGER: u16 = 201;
    pub const GAMEPAD_MENU: u16 = 207;
    pub const GAMEPAD_RIGHT_SHOULDER: u16 = 199;
    pub const GAMEPAD_RIGHT_THUMBSTICK_BUTTON: u16 = 210;
    pub const GAMEPAD_RIGHT_THUMBSTICK_DOWN: u16 = 216;
    pub const GAMEPAD_RIGHT_THUMBSTICK_LEFT: u16 = 218;
    pub const GAMEPAD_RIGHT_THUMBSTICK_RIGHT: u16 = 217;
    pub const GAMEPAD_RIGHT_THUMBSTICK_UP: u16 = 215;
    pub const GAMEPAD_RIGHT_TRIGGER: u16 = 202;
    pub const GAMEPAD_VIEW: u16 = 208;
    pub const GAMEPAD_X: u16 = 197;
    pub const GAMEPAD_Y: u16 = 198;
    pub const HANGEUL: u16 = 21;
    pub const HANGUL: u16 = 21;
    pub const HANJA: u16 = 25;
    pub const HELP: u16 = 47;
    pub const HOME: u16 = 36;
    pub const ICO_00: u16 = 228;
    pub const ICO_CLEAR: u16 = 230;
    pub const ICO_HELP: u16 = 227;
    pub const IME_OFF: u16 = 26;
    pub const IME_ON: u16 = 22;
    pub const INSERT: u16 = 45;
    pub const JUNJA: u16 = 23;
    pub const KANA: u16 = 21;
    pub const KANJI: u16 = 25;
    pub const LAUNCH_APP1: u16 = 182;
    pub const LAUNCH_APP2: u16 = 183;
    pub const LAUNCH_MAIL: u16 = 180;
    pub const LAUNCH_MEDIA_SELECT: u16 = 181;
    pub const LBUTTON: u16 = 1;
    pub const LCONTROL: u16 = 162;
    pub const LEFT: u16 = 37;
    pub const LMENU: u16 = 164;
    pub const LSHIFT: u16 = 160;
    pub const LWIN: u16 = 91;
    pub const MBUTTON: u16 = 4;
    pub const MEDIA_NEXT_TRACK: u16 = 176;
    pub const MEDIA_PLAY_PAUSE: u16 = 179;
    pub const MEDIA_PREV_TRACK: u16 = 177;
    pub const MEDIA_STOP: u16 = 178;
    pub const MENU: u16 = 18;
    pub const MODECHANGE: u16 = 31;
    pub const MULTIPLY: u16 = 106;
    pub const NAVIGATION_ACCEPT: u16 = 142;
    pub const NAVIGATION_CANCEL: u16 = 143;
    pub const NAVIGATION_DOWN: u16 = 139;
    pub const NAVIGATION_LEFT: u16 = 140;
    pub const NAVIGATION_MENU: u16 = 137;
    pub const NAVIGATION_RIGHT: u16 = 141;
    pub const NAVIGATION_UP: u16 = 138;
    pub const NAVIGATION_VIEW: u16 = 136;
    pub const NEXT: u16 = 34;
    pub const NONAME: u16 = 252;
    pub const NONCONVERT: u16 = 29;
    pub const NUMLOCK: u16 = 144;
    pub const NUMPAD0: u16 = 96;
    pub const NUMPAD1: u16 = 97;
    pub const NUMPAD2: u16 = 98;
    pub const NUMPAD3: u16 = 99;
    pub const NUMPAD4: u16 = 100;
    pub const NUMPAD5: u16 = 101;
    pub const NUMPAD6: u16 = 102;
    pub const NUMPAD7: u16 = 103;
    pub const NUMPAD8: u16 = 104;
    pub const NUMPAD9: u16 = 105;
    pub const OEM_1: u16 = 186;
    pub const OEM_102: u16 = 226;
    pub const OEM_2: u16 = 191;
    pub const OEM_3: u16 = 192;
    pub const OEM_4: u16 = 219;
    pub const OEM_5: u16 = 220;
    pub const OEM_6: u16 = 221;
    pub const OEM_7: u16 = 222;
    pub const OEM_8: u16 = 223;
    pub const OEM_ATTN: u16 = 240;
    pub const OEM_AUTO: u16 = 243;
    pub const OEM_AX: u16 = 225;
    pub const OEM_BACKTAB: u16 = 245;
    pub const OEM_CLEAR: u16 = 254;
    pub const OEM_COMMA: u16 = 188;
    pub const OEM_COPY: u16 = 242;
    pub const OEM_CUSEL: u16 = 239;
    pub const OEM_ENLW: u16 = 244;
    pub const OEM_FINISH: u16 = 241;
    pub const OEM_FJ_JISHO: u16 = 146;
    pub const OEM_FJ_LOYA: u16 = 149;
    pub const OEM_FJ_MASSHOU: u16 = 147;
    pub const OEM_FJ_ROYA: u16 = 150;
    pub const OEM_FJ_TOUROKU: u16 = 148;
    pub const OEM_JUMP: u16 = 234;
    pub const OEM_MINUS: u16 = 189;
    pub const OEM_NEC_EQUAL: u16 = 146;
    pub const OEM_PA1: u16 = 235;
    pub const OEM_PA2: u16 = 236;
    pub const OEM_PA3: u16 = 237;
    pub const OEM_PERIOD: u16 = 190;
    pub const OEM_PLUS: u16 = 187;
    pub const OEM_RESET: u16 = 233;
    pub const OEM_WSCTRL: u16 = 238;
    pub const PA1: u16 = 253;
    pub const PACKET: u16 = 231;
    pub const PAUSE: u16 = 19;
    pub const PLAY: u16 = 250;
    pub const PRINT: u16 = 42;
    pub const PRIOR: u16 = 33;
    pub const PROCESSKEY: u16 = 229;
    pub const RBUTTON: u16 = 2;
    pub const RCONTROL: u16 = 163;
    pub const RETURN: u16 = 13;
    pub const RIGHT: u16 = 39;
    pub const RMENU: u16 = 165;
    pub const RSHIFT: u16 = 161;
    pub const RWIN: u16 = 92;
    pub const SCROLL: u16 = 145;
    pub const SELECT: u16 = 41;
    pub const SEPARATOR: u16 = 108;
    pub const SHIFT: u16 = 16;
    pub const SLEEP: u16 = 95;
    pub const SNAPSHOT: u16 = 44;
    pub const SPACE: u16 = 32;
    pub const SUBTRACT: u16 = 109;
    pub const TAB: u16 = 9;
    pub const UP: u16 = 38;
    pub const VOLUME_DOWN: u16 = 174;
    pub const VOLUME_MUTE: u16 = 173;
    pub const VOLUME_UP: u16 = 175;
    pub const XBUTTON1: u16 = 5;
    pub const XBUTTON2: u16 = 6;
    pub const ZOOM: u16 = 251;
}

/// Represents the virtual key codes as an enumeration.
#[repr(u8)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vk {
    None,
    CookedChar(char),
    CookedCtrl(u8),
    A(Attr<{ VK::A }>),
    B(Attr<{ VK::B }>),
    C(Attr<{ VK::C }>),
    D(Attr<{ VK::D }>),
    E(Attr<{ VK::E }>),
    F(Attr<{ VK::F }>),
    G(Attr<{ VK::G }>),
    H(Attr<{ VK::H }>),
    I(Attr<{ VK::I }>),
    J(Attr<{ VK::J }>),
    K(Attr<{ VK::K }>),
    L(Attr<{ VK::L }>),
    M(Attr<{ VK::M }>),
    N(Attr<{ VK::N }>),
    O(Attr<{ VK::O }>),
    P(Attr<{ VK::P }>),
    Q(Attr<{ VK::Q }>),
    R(Attr<{ VK::R }>),
    S(Attr<{ VK::S }>),
    T(Attr<{ VK::T }>),
    U(Attr<{ VK::U }>),
    V(Attr<{ VK::V }>),
    W(Attr<{ VK::W }>),
    X(Attr<{ VK::X }>),
    Y(Attr<{ VK::Y }>),
    Z(Attr<{ VK::Z }>),
    Num0(Attr<{ VK::NUM0 }>),
    Num1(Attr<{ VK::NUM1 }>),
    Num2(Attr<{ VK::NUM2 }>),
    Num3(Attr<{ VK::NUM3 }>),
    Num4(Attr<{ VK::NUM4 }>),
    Num5(Attr<{ VK::NUM5 }>),
    Num6(Attr<{ VK::NUM6 }>),
    Num7(Attr<{ VK::NUM7 }>),
    Num8(Attr<{ VK::NUM8 }>),
    Num9(Attr<{ VK::NUM9 }>),
    Accept(Attr<{ VK::ACCEPT }>),
    Add(Attr<{ VK::ADD }>),
    Apps(Attr<{ VK::APPS }>),
    Attn(Attr<{ VK::ATTN }>),
    Back(Attr<{ VK::BACK }>),
    BrowserBack(Attr<{ VK::BROWSER_BACK }>),
    BrowserFavorites(Attr<{ VK::BROWSER_FAVORITES }>),
    BrowserForward(Attr<{ VK::BROWSER_FORWARD }>),
    BrowserHome(Attr<{ VK::BROWSER_HOME }>),
    BrowserRefresh(Attr<{ VK::BROWSER_REFRESH }>),
    BrowserSearch(Attr<{ VK::BROWSER_SEARCH }>),
    BrowserStop(Attr<{ VK::BROWSER_STOP }>),
    Cancel(Attr<{ VK::CANCEL }>),
    Capital(Attr<{ VK::CAPITAL }>),
    Clear(Attr<{ VK::CLEAR }>),
    Control(Attr<{ VK::CONTROL }>),
    Convert(Attr<{ VK::CONVERT }>),
    Crsel(Attr<{ VK::CRSEL }>),
    Decimal(Attr<{ VK::DECIMAL }>),
    Delete(Attr<{ VK::DELETE }>),
    Divide(Attr<{ VK::DIVIDE }>),
    Down(Attr<{ VK::DOWN }>),
    End(Attr<{ VK::END }>),
    Ereof(Attr<{ VK::EREOF }>),
    Escape(Attr<{ VK::ESCAPE }>),
    Execute(Attr<{ VK::EXECUTE }>),
    Exsel(Attr<{ VK::EXSEL }>),
    F1(Attr<{ VK::F1 }>),
    F10(Attr<{ VK::F10 }>),
    F11(Attr<{ VK::F11 }>),
    F12(Attr<{ VK::F12 }>),
    F13(Attr<{ VK::F13 }>),
    F14(Attr<{ VK::F14 }>),
    F15(Attr<{ VK::F15 }>),
    F16(Attr<{ VK::F16 }>),
    F17(Attr<{ VK::F17 }>),
    F18(Attr<{ VK::F18 }>),
    F19(Attr<{ VK::F19 }>),
    F2(Attr<{ VK::F2 }>),
    F20(Attr<{ VK::F20 }>),
    F21(Attr<{ VK::F21 }>),
    F22(Attr<{ VK::F22 }>),
    F23(Attr<{ VK::F23 }>),
    F24(Attr<{ VK::F24 }>),
    F3(Attr<{ VK::F3 }>),
    F4(Attr<{ VK::F4 }>),
    F5(Attr<{ VK::F5 }>),
    F6(Attr<{ VK::F6 }>),
    F7(Attr<{ VK::F7 }>),
    F8(Attr<{ VK::F8 }>),
    F9(Attr<{ VK::F9 }>),
    Final(Attr<{ VK::FINAL }>),
    GamepadA(Attr<{ VK::GAMEPAD_A }>),
    GamepadB(Attr<{ VK::GAMEPAD_B }>),
    GamepadDpadDown(Attr<{ VK::GAMEPAD_DPAD_DOWN }>),
    GamepadDpadLeft(Attr<{ VK::GAMEPAD_DPAD_LEFT }>),
    GamepadDpadRight(Attr<{ VK::GAMEPAD_DPAD_RIGHT }>),
    GamepadDpadUp(Attr<{ VK::GAMEPAD_DPAD_UP }>),
    GamepadLeftShoulder(Attr<{ VK::GAMEPAD_LEFT_SHOULDER }>),
    GamepadLeftThumbstickButton(Attr<{ VK::GAMEPAD_LEFT_THUMBSTICK_BUTTON }>),
    GamepadLeftThumbstickDown(Attr<{ VK::GAMEPAD_LEFT_THUMBSTICK_DOWN }>),
    GamepadLeftThumbstickLeft(Attr<{ VK::GAMEPAD_LEFT_THUMBSTICK_LEFT }>),
    GamepadLeftThumbstickRight(Attr<{ VK::GAMEPAD_LEFT_THUMBSTICK_RIGHT }>),
    GamepadLeftThumbstickUp(Attr<{ VK::GAMEPAD_LEFT_THUMBSTICK_UP }>),
    GamepadLeftTrigger(Attr<{ VK::GAMEPAD_LEFT_TRIGGER }>),
    GamepadMenu(Attr<{ VK::GAMEPAD_MENU }>),
    GamepadRightShoulder(Attr<{ VK::GAMEPAD_RIGHT_SHOULDER }>),
    GamepadRightThumbstickButton(Attr<{ VK::GAMEPAD_RIGHT_THUMBSTICK_BUTTON }>),
    GamepadRightThumbstickDown(Attr<{ VK::GAMEPAD_RIGHT_THUMBSTICK_DOWN }>),
    GamepadRightThumbstickLeft(Attr<{ VK::GAMEPAD_RIGHT_THUMBSTICK_LEFT }>),
    GamepadRightThumbstickRight(Attr<{ VK::GAMEPAD_RIGHT_THUMBSTICK_RIGHT }>),
    GamepadRightThumbstickUp(Attr<{ VK::GAMEPAD_RIGHT_THUMBSTICK_UP }>),
    GamepadRightTrigger(Attr<{ VK::GAMEPAD_RIGHT_TRIGGER }>),
    GamepadView(Attr<{ VK::GAMEPAD_VIEW }>),
    GamepadX(Attr<{ VK::GAMEPAD_X }>),
    GamepadY(Attr<{ VK::GAMEPAD_Y }>),
    Hangeul(Attr<{ VK::HANGEUL }>),
    Hangul(Attr<{ VK::HANGUL }>),
    Hanja(Attr<{ VK::HANJA }>),
    Help(Attr<{ VK::HELP }>),
    Home(Attr<{ VK::HOME }>),
    Ico00(Attr<{ VK::ICO_00 }>),
    IcoClear(Attr<{ VK::ICO_CLEAR }>),
    IcoHelp(Attr<{ VK::ICO_HELP }>),
    ImeOff(Attr<{ VK::IME_OFF }>),
    ImeOn(Attr<{ VK::IME_ON }>),
    Insert(Attr<{ VK::INSERT }>),
    Junja(Attr<{ VK::JUNJA }>),
    Kana(Attr<{ VK::KANA }>),
    Kanji(Attr<{ VK::KANJI }>),
    LaunchApp1(Attr<{ VK::LAUNCH_APP1 }>),
    LaunchApp2(Attr<{ VK::LAUNCH_APP2 }>),
    LaunchMail(Attr<{ VK::LAUNCH_MAIL }>),
    LaunchMediaSelect(Attr<{ VK::LAUNCH_MEDIA_SELECT }>),
    LButton(Attr<{ VK::LBUTTON }>),
    LControl(Attr<{ VK::LCONTROL }>),
    Left(Attr<{ VK::LEFT }>),
    LMenu(Attr<{ VK::LMENU }>),
    LShift(Attr<{ VK::LSHIFT }>),
    LWin(Attr<{ VK::LWIN }>),
    MButton(Attr<{ VK::MBUTTON }>),
    MediaNextTrack(Attr<{ VK::MEDIA_NEXT_TRACK }>),
    MediaPlayPause(Attr<{ VK::MEDIA_PLAY_PAUSE }>),
    MediaPrevTrack(Attr<{ VK::MEDIA_PREV_TRACK }>),
    MediaStop(Attr<{ VK::MEDIA_STOP }>),
    Menu(Attr<{ VK::MENU }>),
    ModeChange(Attr<{ VK::MODECHANGE }>),
    Multiply(Attr<{ VK::MULTIPLY }>),
    NavigationAccept(Attr<{ VK::NAVIGATION_ACCEPT }>),
    NavigationCancel(Attr<{ VK::NAVIGATION_CANCEL }>),
    NavigationDown(Attr<{ VK::NAVIGATION_DOWN }>),
    NavigationLeft(Attr<{ VK::NAVIGATION_LEFT }>),
    NavigationMenu(Attr<{ VK::NAVIGATION_MENU }>),
    NavigationRight(Attr<{ VK::NAVIGATION_RIGHT }>),
    NavigationUp(Attr<{ VK::NAVIGATION_UP }>),
    NavigationView(Attr<{ VK::NAVIGATION_VIEW }>),
    Next(Attr<{ VK::NEXT }>),
    NoName(Attr<{ VK::NONAME }>),
    NonConvert(Attr<{ VK::NONCONVERT }>),
    NumLock(Attr<{ VK::NUMLOCK }>),
    Numpad0(Attr<{ VK::NUMPAD0 }>),
    Numpad1(Attr<{ VK::NUMPAD1 }>),
    Numpad2(Attr<{ VK::NUMPAD2 }>),
    Numpad3(Attr<{ VK::NUMPAD3 }>),
    Numpad4(Attr<{ VK::NUMPAD4 }>),
    Numpad5(Attr<{ VK::NUMPAD5 }>),
    Numpad6(Attr<{ VK::NUMPAD6 }>),
    Numpad7(Attr<{ VK::NUMPAD7 }>),
    Numpad8(Attr<{ VK::NUMPAD8 }>),
    Numpad9(Attr<{ VK::NUMPAD9 }>),
    Oem1(Attr<{ VK::OEM_1 }>),
    Oem102(Attr<{ VK::OEM_102 }>),
    Oem2(Attr<{ VK::OEM_2 }>),
    Oem3(Attr<{ VK::OEM_3 }>),
    Oem4(Attr<{ VK::OEM_4 }>),
    Oem5(Attr<{ VK::OEM_5 }>),
    Oem6(Attr<{ VK::OEM_6 }>),
    Oem7(Attr<{ VK::OEM_7 }>),
    Oem8(Attr<{ VK::OEM_8 }>),
    OemAttn(Attr<{ VK::OEM_ATTN }>),
    OemAuto(Attr<{ VK::OEM_AUTO }>),
    OemAx(Attr<{ VK::OEM_AX }>),
    OemBacktab(Attr<{ VK::OEM_BACKTAB }>),
    OemClear(Attr<{ VK::OEM_CLEAR }>),
    OemComma(Attr<{ VK::OEM_COMMA }>),
    OemCopy(Attr<{ VK::OEM_COPY }>),
    OemCusel(Attr<{ VK::OEM_CUSEL }>),
    OemEnlw(Attr<{ VK::OEM_ENLW }>),
    OemFinish(Attr<{ VK::OEM_FINISH }>),
    OemFjJisho(Attr<{ VK::OEM_FJ_JISHO }>),
    OemFjLoya(Attr<{ VK::OEM_FJ_LOYA }>),
    OemFjMasshou(Attr<{ VK::OEM_FJ_MASSHOU }>),
    OemFjRoya(Attr<{ VK::OEM_FJ_ROYA }>),
    OemFjTouroku(Attr<{ VK::OEM_FJ_TOUROKU }>),
    OemJump(Attr<{ VK::OEM_JUMP }>),
    OemMinus(Attr<{ VK::OEM_MINUS }>),
    OemNecEqual(Attr<{ VK::OEM_NEC_EQUAL }>),
    OemPa1(Attr<{ VK::OEM_PA1 }>),
    OemPa2(Attr<{ VK::OEM_PA2 }>),
    OemPa3(Attr<{ VK::OEM_PA3 }>),
    OemPeriod(Attr<{ VK::OEM_PERIOD }>),
    OemPlus(Attr<{ VK::OEM_PLUS }>),
    OemReset(Attr<{ VK::OEM_RESET }>),
    OemWsctrl(Attr<{ VK::OEM_WSCTRL }>),
    Pa1(Attr<{ VK::PA1 }>),
    Packet(Attr<{ VK::PACKET }>),
    Pause(Attr<{ VK::PAUSE }>),
    Play(Attr<{ VK::PLAY }>),
    Print(Attr<{ VK::PRINT }>),
    Prior(Attr<{ VK::PRIOR }>),
    Processkey(Attr<{ VK::PROCESSKEY }>),
    RButton(Attr<{ VK::RBUTTON }>),
    RControl(Attr<{ VK::RCONTROL }>),
    Return(Attr<{ VK::RETURN }>),
    Right(Attr<{ VK::RIGHT }>),
    RMenu(Attr<{ VK::RMENU }>),
    RShift(Attr<{ VK::RSHIFT }>),
    RWin(Attr<{ VK::RWIN }>),
    Scroll(Attr<{ VK::SCROLL }>),
    Select(Attr<{ VK::SELECT }>),
    Separator(Attr<{ VK::SEPARATOR }>),
    Shift(Attr<{ VK::SHIFT }>),
    Sleep(Attr<{ VK::SLEEP }>),
    Snapshot(Attr<{ VK::SNAPSHOT }>),
    Space(Attr<{ VK::SPACE }>),
    Subtract(Attr<{ VK::SUBTRACT }>),
    Tab(Attr<{ VK::TAB }>),
    Up(Attr<{ VK::UP }>),
    VolumeDown(Attr<{ VK::VOLUME_DOWN }>),
    VolumeMute(Attr<{ VK::VOLUME_MUTE }>),
    VolumeUp(Attr<{ VK::VOLUME_UP }>),
    XButton1(Attr<{ VK::XBUTTON1 }>),
    XButton2(Attr<{ VK::XBUTTON2 }>),
    Zoom(Attr<{ VK::ZOOM }>),
}

impl Vk {
    /// Creates a `Vk` instance from a virtual key code ID and a modifier.
    /// # Arguments
    /// * `id` - The virtual key code ID.
    /// * `modifier` - The modifier keys associated with the key.
    /// # Returns
    /// A `Vk` instance corresponding to the given virtual key code ID and modifier.
    pub fn from_id(id: u16, modifier: Modifier) -> Self {
        match id {
            VK::A                   => Self::A(modifier.into()),
            VK::B                   => Self::B(modifier.into()),
            VK::C                   => Self::C(modifier.into()),
            VK::D                   => Self::D(modifier.into()),
            VK::E                   => Self::E(modifier.into()),
            VK::F                   => Self::F(modifier.into()),
            VK::G                   => Self::G(modifier.into()),
            VK::H                   => Self::H(modifier.into()),
            VK::I                   => Self::I(modifier.into()),
            VK::J                   => Self::J(modifier.into()),
            VK::K                   => Self::K(modifier.into()),
            VK::L                   => Self::L(modifier.into()),
            VK::M                   => Self::M(modifier.into()),
            VK::N                   => Self::N(modifier.into()),
            VK::O                   => Self::O(modifier.into()),
            VK::P                   => Self::P(modifier.into()),
            VK::Q                   => Self::Q(modifier.into()),
            VK::R                   => Self::R(modifier.into()),
            VK::S                   => Self::S(modifier.into()),
            VK::T                   => Self::T(modifier.into()),
            VK::U                   => Self::U(modifier.into()),
            VK::V                   => Self::V(modifier.into()),
            VK::W                   => Self::W(modifier.into()),
            VK::X                   => Self::X(modifier.into()),
            VK::Y                   => Self::Y(modifier.into()),
            VK::Z                   => Self::Z(modifier.into()),
            VK::NUM0                => Self::Num0(modifier.into()),
            VK::NUM1                => Self::Num1(modifier.into()),
            VK::NUM2                => Self::Num2(modifier.into()),
            VK::NUM3                => Self::Num3(modifier.into()),
            VK::NUM4                => Self::Num4(modifier.into()),
            VK::NUM5                => Self::Num5(modifier.into()),
            VK::NUM6                => Self::Num6(modifier.into()),
            VK::NUM7                => Self::Num7(modifier.into()),
            VK::NUM8                => Self::Num8(modifier.into()),
            VK::NUM9                => Self::Num9(modifier.into()),
            VK::ACCEPT              => Self::Accept(modifier.into()),
            VK::ADD                 => Self::Add(modifier.into()),
            VK::APPS                => Self::Apps(modifier.into()),
            VK::ATTN                => Self::Attn(modifier.into()),
            VK::BACK                => Self::Back(modifier.into()),
            VK::BROWSER_BACK        => Self::BrowserBack(modifier.into()),
            VK::BROWSER_FAVORITES   => Self::BrowserFavorites(modifier.into()),
            VK::BROWSER_FORWARD     => Self::BrowserForward(modifier.into()),
            VK::BROWSER_HOME        => Self::BrowserHome(modifier.into()),
            VK::BROWSER_REFRESH     => Self::BrowserRefresh(modifier.into()),
            VK::BROWSER_SEARCH      => Self::BrowserSearch(modifier.into()),
            VK::BROWSER_STOP        => Self::BrowserStop(modifier.into()),
            VK::CANCEL              => Self::Cancel(modifier.into()),
            VK::CAPITAL             => Self::Capital(modifier.into()),
            VK::CLEAR               => Self::Clear(modifier.into()),
            VK::CONTROL             => Self::Control(modifier.into()),
            VK::CONVERT             => Self::Convert(modifier.into()),
            VK::CRSEL               => Self::Crsel(modifier.into()),
            VK::DECIMAL             => Self::Decimal(modifier.into()),
            VK::DELETE              => Self::Delete(modifier.into()),
            VK::DIVIDE              => Self::Divide(modifier.into()),
            VK::DOWN                => Self::Down(modifier.into()),
            VK::END                 => Self::End(modifier.into()),
            VK::EREOF               => Self::Ereof(modifier.into()),
            VK::ESCAPE              => Self::Escape(modifier.into()),
            VK::EXECUTE             => Self::Execute(modifier.into()),
            VK::EXSEL               => Self::Exsel(modifier.into()),
            VK::F1                  => Self::F1(modifier.into()),
            VK::F10                 => Self::F10(modifier.into()),
            VK::F11                 => Self::F11(modifier.into()),
            VK::F12                 => Self::F12(modifier.into()),
            VK::F13                 => Self::F13(modifier.into()),
            VK::F14                 => Self::F14(modifier.into()),
            VK::F15                 => Self::F15(modifier.into()),
            VK::F16                 => Self::F16(modifier.into()),
            VK::F17                 => Self::F17(modifier.into()),
            VK::F18                 => Self::F18(modifier.into()),
            VK::F19                 => Self::F19(modifier.into()),
            VK::F2                  => Self::F2(modifier.into()),
            VK::F20                 => Self::F20(modifier.into()),
            VK::F21                 => Self::F21(modifier.into()),
            VK::F22                 => Self::F22(modifier.into()),
            VK::F23                 => Self::F23(modifier.into()),
            VK::F24                 => Self::F24(modifier.into()),
            VK::F3                  => Self::F3(modifier.into()),
            VK::F4                  => Self::F4(modifier.into()),
            VK::F5                  => Self::F5(modifier.into()),
            VK::F6                  => Self::F6(modifier.into()),
            VK::F7                  => Self::F7(modifier.into()),
            VK::F8                  => Self::F8(modifier.into()),
            VK::F9                  => Self::F9(modifier.into()),
            VK::FINAL               => Self::Final(modifier.into()),
            VK::GAMEPAD_A           => Self::GamepadA(modifier.into()),
            VK::GAMEPAD_B           => Self::GamepadB(modifier.into()),
            VK::GAMEPAD_DPAD_DOWN   => Self::GamepadDpadDown(modifier.into()),
            VK::GAMEPAD_DPAD_LEFT   => Self::GamepadDpadLeft(modifier.into()),
            VK::GAMEPAD_DPAD_RIGHT  => Self::GamepadDpadRight(modifier.into()),
            VK::GAMEPAD_DPAD_UP     => Self::GamepadDpadUp(modifier.into()),
            VK::GAMEPAD_LEFT_SHOULDER           => Self::GamepadLeftShoulder(modifier.into()),
            VK::GAMEPAD_LEFT_THUMBSTICK_BUTTON  => Self::GamepadLeftThumbstickButton(modifier.into()),
            VK::GAMEPAD_LEFT_THUMBSTICK_DOWN    => Self::GamepadLeftThumbstickDown(modifier.into()),
            VK::GAMEPAD_LEFT_THUMBSTICK_LEFT    => Self::GamepadLeftThumbstickLeft(modifier.into()),
            VK::GAMEPAD_LEFT_THUMBSTICK_RIGHT   => Self::GamepadLeftThumbstickRight(modifier.into()),
            VK::GAMEPAD_LEFT_THUMBSTICK_UP      => Self::GamepadLeftThumbstickUp(modifier.into()),
            VK::GAMEPAD_LEFT_TRIGGER            => Self::GamepadLeftTrigger(modifier.into()),
            VK::GAMEPAD_MENU                    => Self::GamepadMenu(modifier.into()),
            VK::GAMEPAD_RIGHT_SHOULDER          => Self::GamepadRightShoulder(modifier.into()),
            VK::GAMEPAD_RIGHT_THUMBSTICK_BUTTON => Self::GamepadRightThumbstickButton(modifier.into()),
            VK::GAMEPAD_RIGHT_THUMBSTICK_DOWN   => Self::GamepadRightThumbstickDown(modifier.into()),
            VK::GAMEPAD_RIGHT_THUMBSTICK_LEFT   => Self::GamepadRightThumbstickLeft(modifier.into()),
            VK::GAMEPAD_RIGHT_THUMBSTICK_RIGHT  => Self::GamepadRightThumbstickRight(modifier.into()),
            VK::GAMEPAD_RIGHT_THUMBSTICK_UP     => Self::GamepadRightThumbstickUp(modifier.into()),
            VK::GAMEPAD_RIGHT_TRIGGER           => Self::GamepadRightTrigger(modifier.into()),
            VK::GAMEPAD_VIEW        => Self::GamepadView(modifier.into()),
            VK::GAMEPAD_X           => Self::GamepadX(modifier.into()),
            VK::GAMEPAD_Y           => Self::GamepadY(modifier.into()),
            //VK::HANGEUL             => Self::Hangeul(modifier.into()),
            //VK::HANGUL              => Self::Hangul(modifier.into()),
            //VK::HANJA               => Self::Hanja(modifier.into()),
            VK::HELP                => Self::Help(modifier.into()),
            VK::HOME                => Self::Home(modifier.into()),
            VK::ICO_00              => Self::Ico00(modifier.into()),
            VK::ICO_CLEAR           => Self::IcoClear(modifier.into()),
            VK::ICO_HELP            => Self::IcoHelp(modifier.into()),
            VK::IME_OFF             => Self::ImeOff(modifier.into()),
            VK::IME_ON              => Self::ImeOn(modifier.into()),
            VK::INSERT              => Self::Insert(modifier.into()),
            VK::JUNJA               => Self::Junja(modifier.into()),
            VK::KANA                => Self::Kana(modifier.into()),
            VK::KANJI               => Self::Kanji(modifier.into()),
            VK::LAUNCH_APP1         => Self::LaunchApp1(modifier.into()),
            VK::LAUNCH_APP2         => Self::LaunchApp2(modifier.into()),
            VK::LAUNCH_MAIL         => Self::LaunchMail(modifier.into()),
            VK::LAUNCH_MEDIA_SELECT => Self::LaunchMediaSelect(modifier.into()),
            VK::LBUTTON             => Self::LButton(modifier.into()),
            VK::LCONTROL            => Self::LControl(modifier.into()),
            VK::LEFT                => Self::Left(modifier.into()),
            VK::LMENU               => Self::LMenu(modifier.into()),
            VK::LSHIFT              => Self::LShift(modifier.into()),
            VK::LWIN                => Self::LWin(modifier.into()),
            VK::MBUTTON             => Self::MButton(modifier.into()),
            VK::MEDIA_NEXT_TRACK    => Self::MediaNextTrack(modifier.into()),
            VK::MEDIA_PLAY_PAUSE    => Self::MediaPlayPause(modifier.into()),
            VK::MEDIA_PREV_TRACK    => Self::MediaPrevTrack(modifier.into()),
            VK::MEDIA_STOP          => Self::MediaStop(modifier.into()),
            VK::MENU                => Self::Menu(modifier.into()),
            VK::MODECHANGE          => Self::ModeChange(modifier.into()),
            VK::MULTIPLY            => Self::Multiply(modifier.into()),
            VK::NAVIGATION_ACCEPT   => Self::NavigationAccept(modifier.into()),
            VK::NAVIGATION_CANCEL   => Self::NavigationCancel(modifier.into()),
            VK::NAVIGATION_DOWN     => Self::NavigationDown(modifier.into()),
            VK::NAVIGATION_LEFT     => Self::NavigationLeft(modifier.into()),
            VK::NAVIGATION_MENU     => Self::NavigationMenu(modifier.into()),
            VK::NAVIGATION_RIGHT    => Self::NavigationRight(modifier.into()),
            VK::NAVIGATION_UP       => Self::NavigationUp(modifier.into()),
            VK::NAVIGATION_VIEW     => Self::NavigationView(modifier.into()),
            VK::NEXT                => Self::Next(modifier.into()),
            VK::NONAME              => Self::NoName(modifier.into()),
            VK::NONCONVERT          => Self::NonConvert(modifier.into()),
            VK::NUMLOCK             => Self::NumLock(modifier.into()),
            VK::NUMPAD0             => Self::Numpad0(modifier.into()),
            VK::NUMPAD1             => Self::Numpad1(modifier.into()),
            VK::NUMPAD2             => Self::Numpad2(modifier.into()),
            VK::NUMPAD3             => Self::Numpad3(modifier.into()),
            VK::NUMPAD4             => Self::Numpad4(modifier.into()),
            VK::NUMPAD5             => Self::Numpad5(modifier.into()),
            VK::NUMPAD6             => Self::Numpad6(modifier.into()),
            VK::NUMPAD7             => Self::Numpad7(modifier.into()),
            VK::NUMPAD8             => Self::Numpad8(modifier.into()),
            VK::NUMPAD9             => Self::Numpad9(modifier.into()),
            VK::OEM_1               => Self::Oem1(modifier.into()),
            VK::OEM_102             => Self::Oem102(modifier.into()),
            VK::OEM_2               => Self::Oem2(modifier.into()),
            VK::OEM_3               => Self::Oem3(modifier.into()),
            VK::OEM_4               => Self::Oem4(modifier.into()),
            VK::OEM_5               => Self::Oem5(modifier.into()),
            VK::OEM_6               => Self::Oem6(modifier.into()),
            VK::OEM_7               => Self::Oem7(modifier.into()),
            VK::OEM_8               => Self::Oem8(modifier.into()),
            VK::OEM_ATTN            => Self::OemAttn(modifier.into()),
            VK::OEM_AUTO            => Self::OemAuto(modifier.into()),
            VK::OEM_AX              => Self::OemAx(modifier.into()),
            VK::OEM_BACKTAB         => Self::OemBacktab(modifier.into()),
            VK::OEM_CLEAR           => Self::OemClear(modifier.into()),
            VK::OEM_COMMA           => Self::OemComma(modifier.into()),
            VK::OEM_COPY            => Self::OemCopy(modifier.into()),
            VK::OEM_CUSEL           => Self::OemCusel(modifier.into()),
            VK::OEM_ENLW            => Self::OemEnlw(modifier.into()),
            VK::OEM_FINISH          => Self::OemFinish(modifier.into()),
            //VK::OEM_FJ_JISHO        => Self::OemFjJisho(modifier.into()),
            VK::OEM_FJ_LOYA         => Self::OemFjLoya(modifier.into()),
            VK::OEM_FJ_MASSHOU      => Self::OemFjMasshou(modifier.into()),
            VK::OEM_FJ_ROYA         => Self::OemFjRoya(modifier.into()),
            VK::OEM_FJ_TOUROKU      => Self::OemFjTouroku(modifier.into()),
            VK::OEM_JUMP            => Self::OemJump(modifier.into()),
            VK::OEM_MINUS           => Self::OemMinus(modifier.into()),
            VK::OEM_NEC_EQUAL       => Self::OemNecEqual(modifier.into()),
            VK::OEM_PA1             => Self::OemPa1(modifier.into()),
            VK::OEM_PA2             => Self::OemPa2(modifier.into()),
            VK::OEM_PA3             => Self::OemPa3(modifier.into()),
            VK::OEM_PERIOD          => Self::OemPeriod(modifier.into()),
            VK::OEM_PLUS            => Self::OemPlus(modifier.into()),
            VK::OEM_RESET           => Self::OemReset(modifier.into()),
            VK::OEM_WSCTRL          => Self::OemWsctrl(modifier.into()),
            VK::PA1                 => Self::Pa1(modifier.into()),
            VK::PACKET              => Self::Packet(modifier.into()),
            VK::PAUSE               => Self::Pause(modifier.into()),
            VK::PLAY                => Self::Play(modifier.into()),
            VK::PRINT               => Self::Print(modifier.into()),
            VK::PRIOR               => Self::Prior(modifier.into()),
            VK::PROCESSKEY          => Self::Processkey(modifier.into()),
            VK::RBUTTON             => Self::RButton(modifier.into()),
            VK::RCONTROL            => Self::RControl(modifier.into()),
            VK::RETURN              => Self::Return(modifier.into()),
            VK::RIGHT               => Self::Right(modifier.into()),
            VK::RMENU               => Self::RMenu(modifier.into()),
            VK::RSHIFT              => Self::RShift(modifier.into()),
            VK::RWIN                => Self::RWin(modifier.into()),
            VK::SCROLL              => Self::Scroll(modifier.into()),
            VK::SELECT              => Self::Select(modifier.into()),
            VK::SEPARATOR           => Self::Separator(modifier.into()),
            VK::SHIFT               => Self::Shift(modifier.into()),
            VK::SLEEP               => Self::Sleep(modifier.into()),
            VK::SNAPSHOT            => Self::Snapshot(modifier.into()),
            VK::SPACE               => Self::Space(modifier.into()),
            VK::SUBTRACT            => Self::Subtract(modifier.into()),
            VK::TAB                 => Self::Tab(modifier.into()),
            VK::UP                  => Self::Up(modifier.into()),
            VK::VOLUME_DOWN         => Self::VolumeDown(modifier.into()),
            VK::VOLUME_MUTE         => Self::VolumeMute(modifier.into()),
            VK::VOLUME_UP           => Self::VolumeUp(modifier.into()),
            VK::XBUTTON1            => Self::XButton1(modifier.into()),
            VK::XBUTTON2            => Self::XButton2(modifier.into()),
            VK::ZOOM                => Self::Zoom(modifier.into()),
            _                       => Self::None,
        }
    }
}
