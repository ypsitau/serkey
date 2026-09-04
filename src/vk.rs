//! # Windows key codes
//!
//! `win_key_codes` is a translation of all available windows virtual
//! key codes in Rust
//!
//! Original repository: https://github.com/DucaRii/win_key_codes

/// Left mouse button
pub const LBUTTON: u8 = 0x01;
/// Right mouse button
pub const RBUTTON: u8 = 0x02;
/// Control-break processing
pub const CANCEL: u8 = 0x03;
/// Middle mouse button (three-button mouse)
pub const MBUTTON: u8 = 0x04;
/// X1 mouse button
pub const XBUTTON1: u8 = 0x05;
/// X2 mouse button
pub const XBUTTON2: u8 = 0x06;
/// BACKSPACE key
pub const BACK: u8 = 0x08;
/// TAB key
pub const TAB: u8 = 0x09;
/// CLEAR key
pub const CLEAR: u8 = 0x0C;
/// ENTER key
pub const RETURN: u8 = 0x0D;
/// SHIFT key
pub const SHIFT: u8 = 0x10;
/// CTRL key
pub const CONTROL: u8 = 0x11;
/// ALT key
pub const MENU: u8 = 0x12;
/// PAUSE key
pub const PAUSE: u8 = 0x13;
/// CAPS LOCK key
pub const CAPITAL: u8 = 0x14;
/// IME Kana mode
pub const KANA: u8 = 0x15;
/// IME Hanguel mode (maintained for compatibility; use VK_HANGUL)
pub const HANGUEL: u8 = 0x15;
/// IME Hangul mode
pub const HANGUL: u8 = 0x15;
/// IME Junja mode
pub const JUNJA: u8 = 0x17;
/// IME final mode
pub const FINAL: u8 = 0x18;
/// IME Hanja mode
pub const HANJA: u8 = 0x19;
/// IME Kanji mode
pub const KANJI: u8 = 0x19;
/// ESC key
pub const ESCAPE: u8 = 0x1B;
/// IME convert
pub const CONVERT: u8 = 0x1C;
/// IME nonconvert
pub const NONCONVERT: u8 = 0x1D;
/// IME accept
pub const ACCEPT: u8 = 0x1E;
/// IME mode change request
pub const MODECHANGE: u8 = 0x1F;
/// SPACEBAR
pub const SPACE: u8 = 0x20;
/// PAGE UP key
pub const PRIOR: u8 = 0x21;
/// PAGE DOWN key
pub const NEXT: u8 = 0x22;
/// END key
pub const END: u8 = 0x23;
/// HOME key
pub const HOME: u8 = 0x24;
/// LEFT ARROW key
pub const LEFT: u8 = 0x25;
/// UP ARROW key
pub const UP: u8 = 0x26;
/// RIGHT ARROW key
pub const RIGHT: u8 = 0x27;
/// DOWN ARROW key
pub const DOWN: u8 = 0x28;
/// SELECT key
pub const SELECT: u8 = 0x29;
/// PRINT key
pub const PRINT: u8 = 0x2A;
/// EXECUTE key
pub const EXECUTE: u8 = 0x2B;
/// PRINT SCREEN key
pub const SNAPSHOT: u8 = 0x2C;
/// INS key
pub const INSERT: u8 = 0x2D;
/// DEL key
pub const DELETE: u8 = 0x2E;
/// HELP key
pub const HELP: u8 = 0x2F;
/// 0 key
pub const NUM0: u8 = 0x30;
/// 1 key
pub const NUM1: u8 = 0x31;
/// 2 key
pub const NUM2: u8 = 0x32;
/// 3 key
pub const NUM3: u8 = 0x33;
/// 4 key
pub const NUM4: u8 = 0x34;
/// 5 key
pub const NUM5: u8 = 0x35;
/// 6 key
pub const NUM6: u8 = 0x36;
/// 7 key
pub const NUM7: u8 = 0x37;
/// 8 key
pub const NUM8: u8 = 0x38;
/// 9 key
pub const NUM9: u8 = 0x39;
/// A key
pub const A: u8 = 0x41;
/// B key
pub const B: u8 = 0x42;
/// C key
pub const C: u8 = 0x43;
/// D key
pub const D: u8 = 0x44;
/// E key
pub const E: u8 = 0x45;
/// F key
pub const F: u8 = 0x46;
/// G key
pub const G: u8 = 0x47;
/// H key
pub const H: u8 = 0x48;
/// I key
pub const I: u8 = 0x49;
/// J key
pub const J: u8 = 0x4A;
/// K key
pub const K: u8 = 0x4B;
/// L key
pub const L: u8 = 0x4C;
/// M key
pub const M: u8 = 0x4D;
/// N key
pub const N: u8 = 0x4E;
/// O key
pub const O: u8 = 0x4F;
/// P key
pub const P: u8 = 0x50;
/// Q key
pub const Q: u8 = 0x51;
/// R key
pub const R: u8 = 0x52;
/// S key
pub const S: u8 = 0x53;
/// T key
pub const T: u8 = 0x54;
/// U key
pub const U: u8 = 0x55;
/// V key
pub const V: u8 = 0x56;
/// W key
pub const W: u8 = 0x57;
/// X key
pub const X: u8 = 0x58;
/// Y key
pub const Y: u8 = 0x59;
/// Z key
pub const Z: u8 = 0x5A;
/// Left Windows key (Natural keyboard)
pub const LWIN: u8 = 0x5B;
/// Right Windows key (Natural keyboard)
pub const RWIN: u8 = 0x5C;
/// Applications key (Natural keyboard)
pub const APPS: u8 = 0x5D;
/// Computer Sleep key
pub const SLEEP: u8 = 0x5F;
/// Numeric keypad 0 key
pub const NUMPAD0: u8 = 0x60;
/// Numeric keypad 1 key
pub const NUMPAD1: u8 = 0x61;
/// Numeric keypad 2 key
pub const NUMPAD2: u8 = 0x62;
/// Numeric keypad 3 key
pub const NUMPAD3: u8 = 0x63;
/// Numeric keypad 4 key
pub const NUMPAD4: u8 = 0x64;
/// Numeric keypad 5 key
pub const NUMPAD5: u8 = 0x65;
/// Numeric keypad 6 key
pub const NUMPAD6: u8 = 0x66;
/// Numeric keypad 7 key
pub const NUMPAD7: u8 = 0x67;
/// Numeric keypad 8 key
pub const NUMPAD8: u8 = 0x68;
/// Numeric keypad 9 key
pub const NUMPAD9: u8 = 0x69;
/// Multiply key
pub const MULTIPLY: u8 = 0x6A;
/// Add key
pub const ADD: u8 = 0x6B;
/// Separator key
pub const SEPARATOR: u8 = 0x6C;
/// Subtract key
pub const SUBTRACT: u8 = 0x6D;
/// Decimal key
pub const DECIMAL: u8 = 0x6E;
/// Divide key
pub const DIVIDE: u8 = 0x6F;
/// F1 key
pub const F1: u8 = 0x70;
/// F2 key
pub const F2: u8 = 0x71;
/// F3 key
pub const F3: u8 = 0x72;
/// F4 key
pub const F4: u8 = 0x73;
/// F5 key
pub const F5: u8 = 0x74;
/// F6 key
pub const F6: u8 = 0x75;
/// F7 key
pub const F7: u8 = 0x76;
/// F8 key
pub const F8: u8 = 0x77;
/// F9 key
pub const F9: u8 = 0x78;
/// F10 key
pub const F10: u8 = 0x79;
/// F11 key
pub const F11: u8 = 0x7A;
/// F12 key
pub const F12: u8 = 0x7B;
/// F13 key
pub const F13: u8 = 0x7C;
/// F14 key
pub const F14: u8 = 0x7D;
/// F15 key
pub const F15: u8 = 0x7E;
/// F16 key
pub const F16: u8 = 0x7F;
/// F17 key
pub const F17: u8 = 0x80;
/// F18 key
pub const F18: u8 = 0x81;
/// F19 key
pub const F19: u8 = 0x82;
/// F20 key
pub const F20: u8 = 0x83;
/// F21 key
pub const F21: u8 = 0x84;
/// F22 key
pub const F22: u8 = 0x85;
/// F23 key
pub const F23: u8 = 0x86;
/// F24 key
pub const F24: u8 = 0x87;
/// NUM LOCK key
pub const NUMLOCK: u8 = 0x90;
/// SCROLL LOCK key
pub const SCROLL: u8 = 0x91;
/// Left SHIFT key
pub const LSHIFT: u8 = 0xA0;
/// Right SHIFT key
pub const RSHIFT: u8 = 0xA1;
/// Left CONTROL key
pub const LCONTROL: u8 = 0xA2;
/// Right CONTROL key
pub const RCONTROL: u8 = 0xA3;
/// Left MENU key
pub const LMENU: u8 = 0xA4;
/// Right MENU key
pub const RMENU: u8 = 0xA5;
/// Browser Back key
pub const BROWSER_BACK: u8 = 0xA6;
/// Browser Forward key
pub const BROWSER_FORWARD: u8 = 0xA7;
/// Browser Refresh key
pub const BROWSER_REFRESH: u8 = 0xA8;
/// Browser Stop key
pub const BROWSER_STOP: u8 = 0xA9;
/// Browser Search key
pub const BROWSER_SEARCH: u8 = 0xAA;
/// Browser Favorites key
pub const BROWSER_FAVORITES: u8 = 0xAB;
/// Browser Start and Home key
pub const BROWSER_HOME: u8 = 0xAC;
/// Volume Mute key
pub const VOLUME_MUTE: u8 = 0xAD;
/// Volume Down key
pub const VOLUME_DOWN: u8 = 0xAE;
/// Volume Up key
pub const VOLUME_UP: u8 = 0xAF;
/// Next Track key
pub const MEDIA_NEXT_TRACK: u8 = 0xB0;
/// Previous Track key
pub const MEDIA_PREV_TRACK: u8 = 0xB1;
/// Stop Media key
pub const MEDIA_STOP: u8 = 0xB2;
/// Play/Pause Media key
pub const MEDIA_PLAY_PAUSE: u8 = 0xB3;
/// Start Mail key
pub const LAUNCH_MAIL: u8 = 0xB4;
/// Select Media key
pub const LAUNCH_MEDIA_SELECT: u8 = 0xB5;
/// Start Application 1 key
pub const LAUNCH_APP1: u8 = 0xB6;
/// Start Application 2 key
pub const LAUNCH_APP2: u8 = 0xB7;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_1: u8 = 0xBA;
/// For any country/region, the '+' key
pub const OEM_PLUS: u8 = 0xBB;
/// For any country/region, the ',' key
pub const OEM_COMMA: u8 = 0xBC;
/// For any country/region, the '-' key
pub const OEM_MINUS: u8 = 0xBD;
/// For any country/region, the '.' key
pub const OEM_PERIOD: u8 = 0xBE;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_2: u8 = 0xBF;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_3: u8 = 0xC0;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_4: u8 = 0xDB;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_5: u8 = 0xDC;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_6: u8 = 0xDD;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_7: u8 = 0xDE;
/// Used for miscellaneous characters; it can vary by keyboard.
pub const OEM_8: u8 = 0xDF;
/// Either the angle bracket key or the backslash key on the RT 102-key keyboard
pub const OEM_102: u8 = 0xE2;
/// IME PROCESS key
pub const PROCESSKEY: u8 = 0xE5;
/// Used to pass Unicode characters as if they were keystrokes.
pub const PACKET: u8 = 0xE7;
/// Attn key
pub const ATTN: u8 = 0xF6;
/// CrSel key
pub const CRSEL: u8 = 0xF7;
/// ExSel key
pub const EXSEL: u8 = 0xF8;
/// Erase EOF key
pub const EREOF: u8 = 0xF9;
/// Play key
pub const PLAY: u8 = 0xFA;
/// Zoom key
pub const ZOOM: u8 = 0xFB;
/// PA1 key
pub const PA1: u8 = 0xFD;
/// Clear key
pub const OEM_CLEAR: u8 = 0xFE;

