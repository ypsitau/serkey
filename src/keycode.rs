const ATTR_SHIFT_L: u8 = 1 << 0;
const ATTR_SHIFT_R: u8 = 1 << 1;
const ATTR_CONTROL_L: u8 = 1 << 2;
const ATTR_CONTROL_R: u8 = 1 << 3;
const ATTR_ALT_L: u8 = 1 << 4;
const ATTR_ALT_R: u8 = 1 << 5;

#[derive(Debug, Clone, Copy, Default)]
pub struct Attr {
    modifier: u8,
}

impl Attr {
    pub fn new() -> Self {
        Self { modifier: 0 }
    }
    pub fn shift(self) -> Self {
        self.shift_l()
    }
    pub fn control(self) -> Self {
        self.control_l()
    }
    pub fn alt(self) -> Self {
        self.alt_l()
    }
    pub fn shift_r(self) -> Self {
        Self { modifier: self.modifier | ATTR_SHIFT_R, }
    }
    pub fn shift_l(self) -> Self {
        Self { modifier: self.modifier | ATTR_SHIFT_L, }
    }
    pub fn control_l(self) -> Self {
        Self { modifier: self.modifier | ATTR_CONTROL_L, }
    }
    pub fn control_r(self) -> Self {
        Self { modifier: self.modifier | ATTR_CONTROL_R, }
    }
    pub fn alt_l(self) -> Self {
        Self { modifier: self.modifier | ATTR_ALT_L, }
    }
    pub fn alt_r(self) -> Self {
        Self { modifier: self.modifier | ATTR_ALT_R, }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttrWithId<const ID: u8> {
    modifier: u8,
}

impl<const ID: u8> AttrWithId<ID> {
    pub fn id(&self) -> u8 { ID }
    pub fn is_shift(&self) -> bool {
        self.modifier & (ATTR_SHIFT_L | ATTR_SHIFT_R) != 0
    }
    pub fn is_control(&self) -> bool {
        self.modifier & (ATTR_CONTROL_L | ATTR_CONTROL_R) != 0
    }
    pub fn is_alt(&self) -> bool {
        self.modifier & (ATTR_ALT_L | ATTR_ALT_R) != 0
    }
    pub fn is_shift_l(&self) -> bool {
        self.modifier & ATTR_SHIFT_L != 0
    }
    pub fn is_shift_r(&self) -> bool {
        self.modifier & ATTR_SHIFT_R != 0
    }
    pub fn is_control_l(&self) -> bool {
        self.modifier & ATTR_CONTROL_L != 0
    }
    pub fn is_control_r(&self) -> bool {
        self.modifier & ATTR_CONTROL_R != 0
    }
    pub fn is_alt_l(&self) -> bool {
        self.modifier & ATTR_ALT_L != 0
    }
    pub fn is_alt_r(&self) -> bool {
        self.modifier & ATTR_ALT_R != 0
    }
}

impl<const ID: u8> From<Attr> for AttrWithId<ID> {
    fn from(value: Attr) -> Self {
        Self { modifier: value.modifier }
    }
}

#[cfg(test)]
mod tests {
    use super::{Attr, AttrWithId};

    #[test]
    fn attr_into_attr_with_id() {
        let value: AttrWithId<42> = Attr::new().into();
        assert_eq!(value.id(), 42);
    }
}

#[repr(u8)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vk {
    CookedChar(char),
    CookedCtrl(u8),
    A(AttrWithId<65>),
    B(AttrWithId<66>),
    C(AttrWithId<67>),
    D(AttrWithId<68>),
    E(AttrWithId<69>),
    F(AttrWithId<70>),
    G(AttrWithId<71>),
    H(AttrWithId<72>),
    I(AttrWithId<73>),
    J(AttrWithId<74>),
    K(AttrWithId<75>),
    L(AttrWithId<76>),
    M(AttrWithId<77>),
    N(AttrWithId<78>),
    O(AttrWithId<79>),
    P(AttrWithId<80>),
    Q(AttrWithId<81>),
    R(AttrWithId<82>),
    S(AttrWithId<83>),
    T(AttrWithId<84>),
    U(AttrWithId<85>),
    V(AttrWithId<86>),
    W(AttrWithId<87>),
    X(AttrWithId<88>),
    Y(AttrWithId<89>),
    Z(AttrWithId<90>),
    Accept(AttrWithId<30>),
    Add(AttrWithId<107>),
    Apps(AttrWithId<93>),
    Attn(AttrWithId<246>),
    Back(AttrWithId<8>),
    BrowserBack(AttrWithId<166>),
    BrowserFavorites(AttrWithId<171>),
    BrowserForward(AttrWithId<167>),
    BrowserHome(AttrWithId<172>),
    BrowserRefresh(AttrWithId<168>),
    BrowserSearch(AttrWithId<170>),
    BrowserStop(AttrWithId<169>),
    Cancel(AttrWithId<3>),
    Capital(AttrWithId<20>),
    Clear(AttrWithId<12>),
    Control(AttrWithId<17>),
    Convert(AttrWithId<28>),
    Crsel(AttrWithId<247>),
    Decimal(AttrWithId<110>),
    Delete(AttrWithId<46>),
    Digit0(AttrWithId<48>),
    Digit1(AttrWithId<49>),
    Digit2(AttrWithId<50>),
    Digit3(AttrWithId<51>),
    Digit4(AttrWithId<52>),
    Digit5(AttrWithId<53>),
    Digit6(AttrWithId<54>),
    Digit7(AttrWithId<55>),
    Digit8(AttrWithId<56>),
    Digit9(AttrWithId<57>),
    Divide(AttrWithId<111>),
    Down(AttrWithId<40>),
    End(AttrWithId<35>),
    Ereof(AttrWithId<249>),
    Escape(AttrWithId<27>),
    Execute(AttrWithId<43>),
    Exsel(AttrWithId<248>),
    F1(AttrWithId<112>),
    F10(AttrWithId<121>),
    F11(AttrWithId<122>),
    F12(AttrWithId<123>),
    F13(AttrWithId<124>),
    F14(AttrWithId<125>),
    F15(AttrWithId<126>),
    F16(AttrWithId<127>),
    F17(AttrWithId<128>),
    F18(AttrWithId<129>),
    F19(AttrWithId<130>),
    F2(AttrWithId<113>),
    F20(AttrWithId<131>),
    F21(AttrWithId<132>),
    F22(AttrWithId<133>),
    F23(AttrWithId<134>),
    F24(AttrWithId<135>),
    F3(AttrWithId<114>),
    F4(AttrWithId<115>),
    F5(AttrWithId<116>),
    F6(AttrWithId<117>),
    F7(AttrWithId<118>),
    F8(AttrWithId<119>),
    F9(AttrWithId<120>),
    Final(AttrWithId<24>),
    GamepadA(AttrWithId<195>),
    GamepadB(AttrWithId<196>),
    GamepadDpadDown(AttrWithId<204>),
    GamepadDpadLeft(AttrWithId<205>),
    GamepadDpadRight(AttrWithId<206>),
    GamepadDpadUp(AttrWithId<203>),
    GamepadLeftShoulder(AttrWithId<200>),
    GamepadLeftThumbstickButton(AttrWithId<209>),
    GamepadLeftThumbstickDown(AttrWithId<212>),
    GamepadLeftThumbstickLeft(AttrWithId<214>),
    GamepadLeftThumbstickRight(AttrWithId<213>),
    GamepadLeftThumbstickUp(AttrWithId<211>),
    GamepadLeftTrigger(AttrWithId<201>),
    GamepadMenu(AttrWithId<207>),
    GamepadRightShoulder(AttrWithId<199>),
    GamepadRightThumbstickButton(AttrWithId<210>),
    GamepadRightThumbstickDown(AttrWithId<216>),
    GamepadRightThumbstickLeft(AttrWithId<218>),
    GamepadRightThumbstickRight(AttrWithId<217>),
    GamepadRightThumbstickUp(AttrWithId<215>),
    GamepadRightTrigger(AttrWithId<202>),
    GamepadView(AttrWithId<208>),
    GamepadX(AttrWithId<197>),
    GamepadY(AttrWithId<198>),
    Hangeul(AttrWithId<21>),
    Hangul(AttrWithId<21>),
    Hanja(AttrWithId<25>),
    Help(AttrWithId<47>),
    Home(AttrWithId<36>),
    Ico00(AttrWithId<228>),
    IcoClear(AttrWithId<230>),
    IcoHelp(AttrWithId<227>),
    ImeOff(AttrWithId<26>),
    ImeOn(AttrWithId<22>),
    Insert(AttrWithId<45>),
    Junja(AttrWithId<23>),
    Kana(AttrWithId<21>),
    Kanji(AttrWithId<25>),
    LaunchApp1(AttrWithId<182>),
    LaunchApp2(AttrWithId<183>),
    LaunchMail(AttrWithId<180>),
    LaunchMediaSelect(AttrWithId<181>),
    LButton(AttrWithId<1>),
    LControl(AttrWithId<162>),
    Left(AttrWithId<37>),
    LMenu(AttrWithId<164>),
    LShift(AttrWithId<160>),
    LWin(AttrWithId<91>),
    MButton(AttrWithId<4>),
    MediaNextTrack(AttrWithId<176>),
    MediaPlayPause(AttrWithId<179>),
    MediaPrevTrack(AttrWithId<177>),
    MediaStop(AttrWithId<178>),
    Menu(AttrWithId<18>),
    Modechange(AttrWithId<31>),
    Multiply(AttrWithId<106>),
    NavigationAccept(AttrWithId<142>),
    NavigationCancel(AttrWithId<143>),
    NavigationDown(AttrWithId<139>),
    NavigationLeft(AttrWithId<140>),
    NavigationMenu(AttrWithId<137>),
    NavigationRight(AttrWithId<141>),
    NavigationUp(AttrWithId<138>),
    NavigationView(AttrWithId<136>),
    Next(AttrWithId<34>),
    Noname(AttrWithId<252>),
    Nonconvert(AttrWithId<29>),
    Numlock(AttrWithId<144>),
    Numpad0(AttrWithId<96>),
    Numpad1(AttrWithId<97>),
    Numpad2(AttrWithId<98>),
    Numpad3(AttrWithId<99>),
    Numpad4(AttrWithId<100>),
    Numpad5(AttrWithId<101>),
    Numpad6(AttrWithId<102>),
    Numpad7(AttrWithId<103>),
    Numpad8(AttrWithId<104>),
    Numpad9(AttrWithId<105>),
    Oem1(AttrWithId<186>),
    Oem102(AttrWithId<226>),
    Oem2(AttrWithId<191>),
    Oem3(AttrWithId<192>),
    Oem4(AttrWithId<219>),
    Oem5(AttrWithId<220>),
    Oem6(AttrWithId<221>),
    Oem7(AttrWithId<222>),
    Oem8(AttrWithId<223>),
    OemAttn(AttrWithId<240>),
    OemAuto(AttrWithId<243>),
    OemAx(AttrWithId<225>),
    OemBacktab(AttrWithId<245>),
    OemClear(AttrWithId<254>),
    OemComma(AttrWithId<188>),
    OemCopy(AttrWithId<242>),
    OemCusel(AttrWithId<239>),
    OemEnlw(AttrWithId<244>),
    OemFinish(AttrWithId<241>),
    OemFjJisho(AttrWithId<146>),
    OemFjLoya(AttrWithId<149>),
    OemFjMasshou(AttrWithId<147>),
    OemFjRoya(AttrWithId<150>),
    OemFjTouroku(AttrWithId<148>),
    OemJump(AttrWithId<234>),
    OemMinus(AttrWithId<189>),
    OemNecEqual(AttrWithId<146>),
    OemPa1(AttrWithId<235>),
    OemPa2(AttrWithId<236>),
    OemPa3(AttrWithId<237>),
    OemPeriod(AttrWithId<190>),
    OemPlus(AttrWithId<187>),
    OemReset(AttrWithId<233>),
    OemWsctrl(AttrWithId<238>),
    Pa1(AttrWithId<253>),
    Packet(AttrWithId<231>),
    Pause(AttrWithId<19>),
    Play(AttrWithId<250>),
    Print(AttrWithId<42>),
    Prior(AttrWithId<33>),
    Processkey(AttrWithId<229>),
    RButton(AttrWithId<2>),
    RControl(AttrWithId<163>),
    Return(AttrWithId<13>),
    Right(AttrWithId<39>),
    RMenu(AttrWithId<165>),
    RShift(AttrWithId<161>),
    RWin(AttrWithId<92>),
    Scroll(AttrWithId<145>),
    Select(AttrWithId<41>),
    Separator(AttrWithId<108>),
    Shift(AttrWithId<16>),
    Sleep(AttrWithId<95>),
    Snapshot(AttrWithId<44>),
    Space(AttrWithId<32>),
    Subtract(AttrWithId<109>),
    Tab(AttrWithId<9>),
    Up(AttrWithId<38>),
    VolumeDown(AttrWithId<174>),
    VolumeMute(AttrWithId<173>),
    VolumeUp(AttrWithId<175>),
    XButton1(AttrWithId<5>),
    XButton2(AttrWithId<6>),
    Zoom(AttrWithId<251>),
}
