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
    pub fn shift(self) -> Self { self.shift_l() }
    pub fn control(self) -> Self { self.control_l() }
    pub fn alt(self) -> Self { self.alt_l() }
    pub fn shift_r(self) -> Self { Self { bits: self.bits | ATTR_SHIFT_R, } }
    pub fn shift_l(self) -> Self { Self { bits: self.bits | ATTR_SHIFT_L, } }
    pub fn control_l(self) -> Self { Self { bits: self.bits | ATTR_CONTROL_L, } }
    pub fn control_r(self) -> Self { Self { bits: self.bits | ATTR_CONTROL_R, } }
    pub fn alt_l(self) -> Self { Self { bits: self.bits | ATTR_ALT_L, } }
    pub fn alt_r(self) -> Self { Self { bits: self.bits | ATTR_ALT_R, } }
    pub fn is_shift(&self) -> bool { self.bits & (ATTR_SHIFT_L | ATTR_SHIFT_R) != 0 }
    pub fn is_control(&self) -> bool { self.bits & (ATTR_CONTROL_L | ATTR_CONTROL_R) != 0 }
    pub fn is_alt(&self) -> bool { self.bits & (ATTR_ALT_L | ATTR_ALT_R) != 0 }
    pub fn is_shift_l(&self) -> bool { self.bits & ATTR_SHIFT_L != 0 }
    pub fn is_shift_r(&self) -> bool { self.bits & ATTR_SHIFT_R != 0 }
    pub fn is_control_l(&self) -> bool { self.bits & ATTR_CONTROL_L != 0 }
    pub fn is_control_r(&self) -> bool { self.bits & ATTR_CONTROL_R != 0 }
    pub fn is_alt_l(&self) -> bool { self.bits & ATTR_ALT_L != 0 }
    pub fn is_alt_r(&self) -> bool { self.bits & ATTR_ALT_R != 0 }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attr<const ID: u8> {
    modifier: Modifier,
}

impl<const ID: u8> Attr<ID> {
    pub fn id(&self) -> u8 { ID }
    pub fn modifier(&self) -> Modifier { self.modifier }
    pub fn is_shift(&self) -> bool { self.modifier.is_shift() }
    pub fn is_control(&self) -> bool { self.modifier.is_control() }
    pub fn is_alt(&self) -> bool { self.modifier.is_alt() }
    pub fn is_shift_l(&self) -> bool { self.modifier.is_shift_l() }
    pub fn is_shift_r(&self) -> bool { self.modifier.is_shift_r() }
    pub fn is_control_l(&self) -> bool { self.modifier.is_control_l() }
    pub fn is_control_r(&self) -> bool { self.modifier.is_control_r() }
    pub fn is_alt_l(&self) -> bool { self.modifier.is_alt_l() }
    pub fn is_alt_r(&self) -> bool { self.modifier.is_alt_r() }
}

impl<const ID: u8> From<Modifier> for Attr<ID> {
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
    Digit0(Attr<48>),
    Digit1(Attr<49>),
    Digit2(Attr<50>),
    Digit3(Attr<51>),
    Digit4(Attr<52>),
    Digit5(Attr<53>),
    Digit6(Attr<54>),
    Digit7(Attr<55>),
    Digit8(Attr<56>),
    Digit9(Attr<57>),
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
