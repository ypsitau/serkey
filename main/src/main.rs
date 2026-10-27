#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{
            GetStdHandle, GetConsoleMode, SetConsoleMode,
            ENABLE_VIRTUAL_TERMINAL_INPUT, STD_INPUT_HANDLE,
        };
        unsafe {
            let handle = GetStdHandle(STD_INPUT_HANDLE);
            let mut mode = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_INPUT);
            }
        }
    }
    let mut stdin = embedded_io_adapters::tokio_1::FromTokio::new(tokio::io::stdin());
    //let mut stdin = embedded_io_adapters::std::FromStd::new(std::io::stdin());
    let mut serkey_reader = serkey::Reader::new();
    print!("Ctrl-C to exit\r\n");
    let mut sbuf: heapless::String<64> = heapless::String::new();
    loop {
        if let Some(key) = serkey_reader.read_key(&mut stdin).await {
            sbuf.clear();
            key_to_string(&mut sbuf, &key);
            print!("{}\r\n", sbuf);
            if let serkey::Key::CookedCtrl(0x03) = key {
                break;
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn key_to_string(sbuf: &mut impl core::fmt::Write, key: &serkey::Key) {
    use serkey::{Key, Modifier};
    fn write_key(sbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(sbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_ctrl() { " + Ctrl" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" }).ok();
    }
    match key {
        Key::CookedChar(ch)      => { write!(sbuf, "CookedChar: {}", ch).ok(); }
        Key::CookedCtrl(n)       => { write!(sbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
        Key::Tab(attr)           => { write_key(sbuf, "Tab", attr.modifier()); }
        Key::Enter(attr)         => { write_key(sbuf, "Enter", attr.modifier()); }
        Key::Left(attr)          => { write_key(sbuf, "Left", attr.modifier()); }
        Key::Right(attr)         => { write_key(sbuf, "Right", attr.modifier()); }
        Key::Up(attr)            => { write_key(sbuf, "Up", attr.modifier()); }
        Key::Down(attr)          => { write_key(sbuf, "Down", attr.modifier()); }
        Key::Home(attr)          => { write_key(sbuf, "Home", attr.modifier()); }
        Key::End(attr)           => { write_key(sbuf, "End", attr.modifier()); }
        Key::PageUp(attr)        => { write_key(sbuf, "PageUp", attr.modifier()); }
        Key::PageDown(attr)      => { write_key(sbuf, "PageDown", attr.modifier()); }
        Key::Backspace(attr)     => { write_key(sbuf, "Backspace", attr.modifier()); }
        Key::Delete(attr)        => { write_key(sbuf, "Delete", attr.modifier()); }
        Key::Insert(attr)        => { write_key(sbuf, "Insert", attr.modifier()); }
        Key::Esc(attr)           => { write_key(sbuf, "Esc", attr.modifier()); }
        Key::F1(attr)            => { write_key(sbuf, "F1", attr.modifier()); }
        Key::F2(attr)            => { write_key(sbuf, "F2", attr.modifier()); }
        Key::F3(attr)            => { write_key(sbuf, "F3", attr.modifier()); }
        Key::F4(attr)            => { write_key(sbuf, "F4", attr.modifier()); }
        Key::F5(attr)            => { write_key(sbuf, "F5", attr.modifier()); }
        Key::F6(attr)            => { write_key(sbuf, "F6", attr.modifier()); }
        Key::F7(attr)            => { write_key(sbuf, "F7", attr.modifier()); }
        Key::F8(attr)            => { write_key(sbuf, "F8", attr.modifier()); }
        Key::F9(attr)            => { write_key(sbuf, "F9", attr.modifier()); }
        Key::F10(attr)           => { write_key(sbuf, "F10", attr.modifier()); }
        Key::F11(attr)           => { write_key(sbuf, "F11", attr.modifier()); }
        Key::F12(attr)           => { write_key(sbuf, "F12", attr.modifier()); }
        _ => ()
    }
}
