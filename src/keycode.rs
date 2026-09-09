const ATTR_SHIFT_L: u8 = 1 << 0;
const ATTR_SHIFT_R: u8 = 1 << 1;
const ATTR_CONTROL_L: u8 = 1 << 2;
const ATTR_CONTROL_R: u8 = 1 << 3;
const ATTR_ALT_L: u8 = 1 << 4;
const ATTR_ALT_R: u8 = 1 << 5;

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Modifier {
    bits: u8,
}

impl Modifier {
    pub fn shift(self) -> Self          { self.shift_l() }
    pub fn control(self) -> Self        { self.control_l() }
    pub fn alt(self) -> Self            { self.alt_l() }
    pub fn shift_r(self) -> Self        { Self { bits: self.bits | ATTR_SHIFT_R, } }
    pub fn shift_l(self) -> Self        { Self { bits: self.bits | ATTR_SHIFT_L, } }
    pub fn control_l(self) -> Self      { Self { bits: self.bits | ATTR_CONTROL_L, } }
    pub fn control_r(self) -> Self      { Self { bits: self.bits | ATTR_CONTROL_R, } }
    pub fn alt_l(self) -> Self          { Self { bits: self.bits | ATTR_ALT_L, } }
    pub fn alt_r(self) -> Self          { Self { bits: self.bits | ATTR_ALT_R, } }
    pub fn is_shift(&self) -> bool      { self.bits & (ATTR_SHIFT_L | ATTR_SHIFT_R) != 0 }
    pub fn is_control(&self) -> bool    { self.bits & (ATTR_CONTROL_L | ATTR_CONTROL_R) != 0 }
    pub fn is_alt(&self) -> bool        { self.bits & (ATTR_ALT_L | ATTR_ALT_R) != 0 }
    pub fn is_shift_l(&self) -> bool    { self.bits & ATTR_SHIFT_L != 0 }
    pub fn is_shift_r(&self) -> bool    { self.bits & ATTR_SHIFT_R != 0 }
    pub fn is_control_l(&self) -> bool  { self.bits & ATTR_CONTROL_L != 0 }
    pub fn is_control_r(&self) -> bool  { self.bits & ATTR_CONTROL_R != 0 }
    pub fn is_alt_l(&self) -> bool      { self.bits & ATTR_ALT_L != 0 }
    pub fn is_alt_r(&self) -> bool      { self.bits & ATTR_ALT_R != 0 }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attr<const ID: i32> {
    modifier: Modifier,
}

impl<const ID: i32> Attr<ID> {
    pub fn id(&self) -> i32 { ID }
    pub fn modifier(&self) -> Modifier  { self.modifier }
    pub fn is_shift(&self) -> bool      { self.modifier.is_shift() }
    pub fn is_control(&self) -> bool    { self.modifier.is_control() }
    pub fn is_alt(&self) -> bool        { self.modifier.is_alt() }
    pub fn is_shift_l(&self) -> bool    { self.modifier.is_shift_l() }
    pub fn is_shift_r(&self) -> bool    { self.modifier.is_shift_r() }
    pub fn is_control_l(&self) -> bool  { self.modifier.is_control_l() }
    pub fn is_control_r(&self) -> bool  { self.modifier.is_control_r() }
    pub fn is_alt_l(&self) -> bool      { self.modifier.is_alt_l() }
    pub fn is_alt_r(&self) -> bool      { self.modifier.is_alt_r() }
}

impl<const ID: i32> From<Modifier> for Attr<ID> {
    fn from(modifier: Modifier) -> Self {
        Self { modifier }
    }
}

#[cfg(test)]
mod tests {
    use super::{Modifier, Attr};

    #[test]
    fn attr_into_attr_with_id() {
        let value: Attr<42> = Modifier::default().into();
        assert_eq!(value.id(), 42);
    }
}

// Original source: https://github.com/microsoft/windows-rs/
// crates/libs/windows/src/Windows/Win32/winuser/mod.rs
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

#[repr(u8)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vk {
    CookedChar(char),
    CookedCtrl(u8),
    A(Attr<65>),
    B(Attr<66>),
    C(Attr<67>),
    D(Attr<68>),
    E(Attr<69>),
    F(Attr<70>),
    G(Attr<71>),
    H(Attr<72>),
    I(Attr<73>),
    J(Attr<74>),
    K(Attr<75>),
    L(Attr<76>),
    M(Attr<77>),
    N(Attr<78>),
    O(Attr<79>),
    P(Attr<80>),
    Q(Attr<81>),
    R(Attr<82>),
    S(Attr<83>),
    T(Attr<84>),
    U(Attr<85>),
    V(Attr<86>),
    W(Attr<87>),
    X(Attr<88>),
    Y(Attr<89>),
    Z(Attr<90>),
    Num0(Attr<48>),
    Num1(Attr<49>),
    Num2(Attr<50>),
    Num3(Attr<51>),
    Num4(Attr<52>),
    Num5(Attr<53>),
    Num6(Attr<54>),
    Num7(Attr<55>),
    Num8(Attr<56>),
    Num9(Attr<57>),
    Accept(Attr<30>),
    Add(Attr<107>),
    Apps(Attr<93>),
    Attn(Attr<246>),
    Back(Attr<8>),
    BrowserBack(Attr<166>),
    BrowserFavorites(Attr<171>),
    BrowserForward(Attr<167>),
    BrowserHome(Attr<172>),
    BrowserRefresh(Attr<168>),
    BrowserSearch(Attr<170>),
    BrowserStop(Attr<169>),
    Cancel(Attr<3>),
    Capital(Attr<20>),
    Clear(Attr<12>),
    Control(Attr<17>),
    Convert(Attr<28>),
    Crsel(Attr<247>),
    Decimal(Attr<110>),
    Delete(Attr<46>),
    Divide(Attr<111>),
    Down(Attr<40>),
    End(Attr<35>),
    Ereof(Attr<249>),
    Escape(Attr<27>),
    Execute(Attr<43>),
    Exsel(Attr<248>),
    F1(Attr<112>),
    F10(Attr<121>),
    F11(Attr<122>),
    F12(Attr<123>),
    F13(Attr<124>),
    F14(Attr<125>),
    F15(Attr<126>),
    F16(Attr<127>),
    F17(Attr<128>),
    F18(Attr<129>),
    F19(Attr<130>),
    F2(Attr<113>),
    F20(Attr<131>),
    F21(Attr<132>),
    F22(Attr<133>),
    F23(Attr<134>),
    F24(Attr<135>),
    F3(Attr<114>),
    F4(Attr<115>),
    F5(Attr<116>),
    F6(Attr<117>),
    F7(Attr<118>),
    F8(Attr<119>),
    F9(Attr<120>),
    Final(Attr<24>),
    GamepadA(Attr<195>),
    GamepadB(Attr<196>),
    GamepadDpadDown(Attr<204>),
    GamepadDpadLeft(Attr<205>),
    GamepadDpadRight(Attr<206>),
    GamepadDpadUp(Attr<203>),
    GamepadLeftShoulder(Attr<200>),
    GamepadLeftThumbstickButton(Attr<209>),
    GamepadLeftThumbstickDown(Attr<212>),
    GamepadLeftThumbstickLeft(Attr<214>),
    GamepadLeftThumbstickRight(Attr<213>),
    GamepadLeftThumbstickUp(Attr<211>),
    GamepadLeftTrigger(Attr<201>),
    GamepadMenu(Attr<207>),
    GamepadRightShoulder(Attr<199>),
    GamepadRightThumbstickButton(Attr<210>),
    GamepadRightThumbstickDown(Attr<216>),
    GamepadRightThumbstickLeft(Attr<218>),
    GamepadRightThumbstickRight(Attr<217>),
    GamepadRightThumbstickUp(Attr<215>),
    GamepadRightTrigger(Attr<202>),
    GamepadView(Attr<208>),
    GamepadX(Attr<197>),
    GamepadY(Attr<198>),
    Hangeul(Attr<21>),
    Hangul(Attr<21>),
    Hanja(Attr<25>),
    Help(Attr<47>),
    Home(Attr<36>),
    Ico00(Attr<228>),
    IcoClear(Attr<230>),
    IcoHelp(Attr<227>),
    ImeOff(Attr<26>),
    ImeOn(Attr<22>),
    Insert(Attr<45>),
    Junja(Attr<23>),
    Kana(Attr<21>),
    Kanji(Attr<25>),
    LaunchApp1(Attr<182>),
    LaunchApp2(Attr<183>),
    LaunchMail(Attr<180>),
    LaunchMediaSelect(Attr<181>),
    LButton(Attr<1>),
    LControl(Attr<162>),
    Left(Attr<37>),
    LMenu(Attr<164>),
    LShift(Attr<160>),
    LWin(Attr<91>),
    MButton(Attr<4>),
    MediaNextTrack(Attr<176>),
    MediaPlayPause(Attr<179>),
    MediaPrevTrack(Attr<177>),
    MediaStop(Attr<178>),
    Menu(Attr<18>),
    Modechange(Attr<31>),
    Multiply(Attr<106>),
    NavigationAccept(Attr<142>),
    NavigationCancel(Attr<143>),
    NavigationDown(Attr<139>),
    NavigationLeft(Attr<140>),
    NavigationMenu(Attr<137>),
    NavigationRight(Attr<141>),
    NavigationUp(Attr<138>),
    NavigationView(Attr<136>),
    Next(Attr<34>),
    Noname(Attr<252>),
    Nonconvert(Attr<29>),
    Numlock(Attr<144>),
    Numpad0(Attr<96>),
    Numpad1(Attr<97>),
    Numpad2(Attr<98>),
    Numpad3(Attr<99>),
    Numpad4(Attr<100>),
    Numpad5(Attr<101>),
    Numpad6(Attr<102>),
    Numpad7(Attr<103>),
    Numpad8(Attr<104>),
    Numpad9(Attr<105>),
    Oem1(Attr<186>),
    Oem102(Attr<226>),
    Oem2(Attr<191>),
    Oem3(Attr<192>),
    Oem4(Attr<219>),
    Oem5(Attr<220>),
    Oem6(Attr<221>),
    Oem7(Attr<222>),
    Oem8(Attr<223>),
    OemAttn(Attr<240>),
    OemAuto(Attr<243>),
    OemAx(Attr<225>),
    OemBacktab(Attr<245>),
    OemClear(Attr<254>),
    OemComma(Attr<188>),
    OemCopy(Attr<242>),
    OemCusel(Attr<239>),
    OemEnlw(Attr<244>),
    OemFinish(Attr<241>),
    OemFjJisho(Attr<146>),
    OemFjLoya(Attr<149>),
    OemFjMasshou(Attr<147>),
    OemFjRoya(Attr<150>),
    OemFjTouroku(Attr<148>),
    OemJump(Attr<234>),
    OemMinus(Attr<189>),
    OemNecEqual(Attr<146>),
    OemPa1(Attr<235>),
    OemPa2(Attr<236>),
    OemPa3(Attr<237>),
    OemPeriod(Attr<190>),
    OemPlus(Attr<187>),
    OemReset(Attr<233>),
    OemWsctrl(Attr<238>),
    Pa1(Attr<253>),
    Packet(Attr<231>),
    Pause(Attr<19>),
    Play(Attr<250>),
    Print(Attr<42>),
    Prior(Attr<33>),
    Processkey(Attr<229>),
    RButton(Attr<2>),
    RControl(Attr<163>),
    Return(Attr<13>),
    Right(Attr<39>),
    RMenu(Attr<165>),
    RShift(Attr<161>),
    RWin(Attr<92>),
    Scroll(Attr<145>),
    Select(Attr<41>),
    Separator(Attr<108>),
    Shift(Attr<16>),
    Sleep(Attr<95>),
    Snapshot(Attr<44>),
    Space(Attr<32>),
    Subtract(Attr<109>),
    Tab(Attr<9>),
    Up(Attr<38>),
    VolumeDown(Attr<174>),
    VolumeMute(Attr<173>),
    VolumeUp(Attr<175>),
    XButton1(Attr<5>),
    XButton2(Attr<6>),
    Zoom(Attr<251>),
}
