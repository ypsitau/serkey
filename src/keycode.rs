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
pub struct Attr<const ID: i32> {
    modifier: Modifier,
}

impl<const ID: i32> Attr<ID> {
    pub fn id(&self) -> i32 { ID }
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

impl<const ID: i32> From<Modifier> for Attr<ID> {
    fn from(modifier: Modifier) -> Self {
        Self { modifier }
    }
}

/// Represents the virtual key codes for the keyboard, as defined in the Windows API.
/// Original GitHub repository: https://github.com/microsoft/windows-rs/
/// See the file: crates/libs/windows/src/Windows/Win32/winuser/mod.rs
#[allow(non_snake_case)]
pub mod VK {
    pub const A: i32 = 65;
    pub const B: i32 = 66;
    pub const C: i32 = 67;
    pub const D: i32 = 68;
    pub const E: i32 = 69;
    pub const F: i32 = 70;
    pub const G: i32 = 71;
    pub const H: i32 = 72;
    pub const I: i32 = 73;
    pub const J: i32 = 74;
    pub const K: i32 = 75;
    pub const L: i32 = 76;
    pub const M: i32 = 77;
    pub const N: i32 = 78;
    pub const O: i32 = 79;
    pub const P: i32 = 80;
    pub const Q: i32 = 81;
    pub const R: i32 = 82;
    pub const S: i32 = 83;
    pub const T: i32 = 84;
    pub const U: i32 = 85;
    pub const V: i32 = 86;
    pub const W: i32 = 87;
    pub const X: i32 = 88;
    pub const Y: i32 = 89;
    pub const Z: i32 = 90;
    pub const NUM0: i32 = 48;
    pub const NUM1: i32 = 49;
    pub const NUM2: i32 = 50;
    pub const NUM3: i32 = 51;
    pub const NUM4: i32 = 52;
    pub const NUM5: i32 = 53;
    pub const NUM6: i32 = 54;
    pub const NUM7: i32 = 55;
    pub const NUM8: i32 = 56;
    pub const NUM9: i32 = 57;
    pub const ACCEPT: i32 = 30;
    pub const ADD: i32 = 107;
    pub const APPS: i32 = 93;
    pub const ATTN: i32 = 246;
    pub const BACK: i32 = 8;
    pub const BROWSER_BACK: i32 = 166;
    pub const BROWSER_FAVORITES: i32 = 171;
    pub const BROWSER_FORWARD: i32 = 167;
    pub const BROWSER_HOME: i32 = 172;
    pub const BROWSER_REFRESH: i32 = 168;
    pub const BROWSER_SEARCH: i32 = 170;
    pub const BROWSER_STOP: i32 = 169;
    pub const CANCEL: i32 = 3;
    pub const CAPITAL: i32 = 20;
    pub const CLEAR: i32 = 12;
    pub const CONTROL: i32 = 17;
    pub const CONVERT: i32 = 28;
    pub const CRSEL: i32 = 247;
    pub const DECIMAL: i32 = 110;
    pub const DELETE: i32 = 46;
    pub const DIVIDE: i32 = 111;
    pub const DOWN: i32 = 40;
    pub const END: i32 = 35;
    pub const EREOF: i32 = 249;
    pub const ESCAPE: i32 = 27;
    pub const EXECUTE: i32 = 43;
    pub const EXSEL: i32 = 248;
    pub const F1: i32 = 112;
    pub const F10: i32 = 121;
    pub const F11: i32 = 122;
    pub const F12: i32 = 123;
    pub const F13: i32 = 124;
    pub const F14: i32 = 125;
    pub const F15: i32 = 126;
    pub const F16: i32 = 127;
    pub const F17: i32 = 128;
    pub const F18: i32 = 129;
    pub const F19: i32 = 130;
    pub const F2: i32 = 113;
    pub const F20: i32 = 131;
    pub const F21: i32 = 132;
    pub const F22: i32 = 133;
    pub const F23: i32 = 134;
    pub const F24: i32 = 135;
    pub const F3: i32 = 114;
    pub const F4: i32 = 115;
    pub const F5: i32 = 116;
    pub const F6: i32 = 117;
    pub const F7: i32 = 118;
    pub const F8: i32 = 119;
    pub const F9: i32 = 120;
    pub const FINAL: i32 = 24;
    pub const GAMEPAD_A: i32 = 195;
    pub const GAMEPAD_B: i32 = 196;
    pub const GAMEPAD_DPAD_DOWN: i32 = 204;
    pub const GAMEPAD_DPAD_LEFT: i32 = 205;
    pub const GAMEPAD_DPAD_RIGHT: i32 = 206;
    pub const GAMEPAD_DPAD_UP: i32 = 203;
    pub const GAMEPAD_LEFT_SHOULDER: i32 = 200;
    pub const GAMEPAD_LEFT_THUMBSTICK_BUTTON: i32 = 209;
    pub const GAMEPAD_LEFT_THUMBSTICK_DOWN: i32 = 212;
    pub const GAMEPAD_LEFT_THUMBSTICK_LEFT: i32 = 214;
    pub const GAMEPAD_LEFT_THUMBSTICK_RIGHT: i32 = 213;
    pub const GAMEPAD_LEFT_THUMBSTICK_UP: i32 = 211;
    pub const GAMEPAD_LEFT_TRIGGER: i32 = 201;
    pub const GAMEPAD_MENU: i32 = 207;
    pub const GAMEPAD_RIGHT_SHOULDER: i32 = 199;
    pub const GAMEPAD_RIGHT_THUMBSTICK_BUTTON: i32 = 210;
    pub const GAMEPAD_RIGHT_THUMBSTICK_DOWN: i32 = 216;
    pub const GAMEPAD_RIGHT_THUMBSTICK_LEFT: i32 = 218;
    pub const GAMEPAD_RIGHT_THUMBSTICK_RIGHT: i32 = 217;
    pub const GAMEPAD_RIGHT_THUMBSTICK_UP: i32 = 215;
    pub const GAMEPAD_RIGHT_TRIGGER: i32 = 202;
    pub const GAMEPAD_VIEW: i32 = 208;
    pub const GAMEPAD_X: i32 = 197;
    pub const GAMEPAD_Y: i32 = 198;
    pub const HANGEUL: i32 = 21;
    pub const HANGUL: i32 = 21;
    pub const HANJA: i32 = 25;
    pub const HELP: i32 = 47;
    pub const HOME: i32 = 36;
    pub const ICO_00: i32 = 228;
    pub const ICO_CLEAR: i32 = 230;
    pub const ICO_HELP: i32 = 227;
    pub const IME_OFF: i32 = 26;
    pub const IME_ON: i32 = 22;
    pub const INSERT: i32 = 45;
    pub const JUNJA: i32 = 23;
    pub const KANA: i32 = 21;
    pub const KANJI: i32 = 25;
    pub const LAUNCH_APP1: i32 = 182;
    pub const LAUNCH_APP2: i32 = 183;
    pub const LAUNCH_MAIL: i32 = 180;
    pub const LAUNCH_MEDIA_SELECT: i32 = 181;
    pub const LBUTTON: i32 = 1;
    pub const LCONTROL: i32 = 162;
    pub const LEFT: i32 = 37;
    pub const LMENU: i32 = 164;
    pub const LSHIFT: i32 = 160;
    pub const LWIN: i32 = 91;
    pub const MBUTTON: i32 = 4;
    pub const MEDIA_NEXT_TRACK: i32 = 176;
    pub const MEDIA_PLAY_PAUSE: i32 = 179;
    pub const MEDIA_PREV_TRACK: i32 = 177;
    pub const MEDIA_STOP: i32 = 178;
    pub const MENU: i32 = 18;
    pub const MODECHANGE: i32 = 31;
    pub const MULTIPLY: i32 = 106;
    pub const NAVIGATION_ACCEPT: i32 = 142;
    pub const NAVIGATION_CANCEL: i32 = 143;
    pub const NAVIGATION_DOWN: i32 = 139;
    pub const NAVIGATION_LEFT: i32 = 140;
    pub const NAVIGATION_MENU: i32 = 137;
    pub const NAVIGATION_RIGHT: i32 = 141;
    pub const NAVIGATION_UP: i32 = 138;
    pub const NAVIGATION_VIEW: i32 = 136;
    pub const NEXT: i32 = 34;
    pub const NONAME: i32 = 252;
    pub const NONCONVERT: i32 = 29;
    pub const NUMLOCK: i32 = 144;
    pub const NUMPAD0: i32 = 96;
    pub const NUMPAD1: i32 = 97;
    pub const NUMPAD2: i32 = 98;
    pub const NUMPAD3: i32 = 99;
    pub const NUMPAD4: i32 = 100;
    pub const NUMPAD5: i32 = 101;
    pub const NUMPAD6: i32 = 102;
    pub const NUMPAD7: i32 = 103;
    pub const NUMPAD8: i32 = 104;
    pub const NUMPAD9: i32 = 105;
    pub const OEM_1: i32 = 186;
    pub const OEM_102: i32 = 226;
    pub const OEM_2: i32 = 191;
    pub const OEM_3: i32 = 192;
    pub const OEM_4: i32 = 219;
    pub const OEM_5: i32 = 220;
    pub const OEM_6: i32 = 221;
    pub const OEM_7: i32 = 222;
    pub const OEM_8: i32 = 223;
    pub const OEM_ATTN: i32 = 240;
    pub const OEM_AUTO: i32 = 243;
    pub const OEM_AX: i32 = 225;
    pub const OEM_BACKTAB: i32 = 245;
    pub const OEM_CLEAR: i32 = 254;
    pub const OEM_COMMA: i32 = 188;
    pub const OEM_COPY: i32 = 242;
    pub const OEM_CUSEL: i32 = 239;
    pub const OEM_ENLW: i32 = 244;
    pub const OEM_FINISH: i32 = 241;
    pub const OEM_FJ_JISHO: i32 = 146;
    pub const OEM_FJ_LOYA: i32 = 149;
    pub const OEM_FJ_MASSHOU: i32 = 147;
    pub const OEM_FJ_ROYA: i32 = 150;
    pub const OEM_FJ_TOUROKU: i32 = 148;
    pub const OEM_JUMP: i32 = 234;
    pub const OEM_MINUS: i32 = 189;
    pub const OEM_NEC_EQUAL: i32 = 146;
    pub const OEM_PA1: i32 = 235;
    pub const OEM_PA2: i32 = 236;
    pub const OEM_PA3: i32 = 237;
    pub const OEM_PERIOD: i32 = 190;
    pub const OEM_PLUS: i32 = 187;
    pub const OEM_RESET: i32 = 233;
    pub const OEM_WSCTRL: i32 = 238;
    pub const PA1: i32 = 253;
    pub const PACKET: i32 = 231;
    pub const PAUSE: i32 = 19;
    pub const PLAY: i32 = 250;
    pub const PRINT: i32 = 42;
    pub const PRIOR: i32 = 33;
    pub const PROCESSKEY: i32 = 229;
    pub const RBUTTON: i32 = 2;
    pub const RCONTROL: i32 = 163;
    pub const RETURN: i32 = 13;
    pub const RIGHT: i32 = 39;
    pub const RMENU: i32 = 165;
    pub const RSHIFT: i32 = 161;
    pub const RWIN: i32 = 92;
    pub const SCROLL: i32 = 145;
    pub const SELECT: i32 = 41;
    pub const SEPARATOR: i32 = 108;
    pub const SHIFT: i32 = 16;
    pub const SLEEP: i32 = 95;
    pub const SNAPSHOT: i32 = 44;
    pub const SPACE: i32 = 32;
    pub const SUBTRACT: i32 = 109;
    pub const TAB: i32 = 9;
    pub const UP: i32 = 38;
    pub const VOLUME_DOWN: i32 = 174;
    pub const VOLUME_MUTE: i32 = 173;
    pub const VOLUME_UP: i32 = 175;
    pub const XBUTTON1: i32 = 5;
    pub const XBUTTON2: i32 = 6;
    pub const ZOOM: i32 = 251;
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
    pub fn from_id(id: i32, modifier: Modifier) -> Self {
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
