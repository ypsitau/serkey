use embedded_io_async as io_async;

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
    let stdin = embedded_io_adapters::tokio_1::FromTokio::new(tokio::io::stdin());
    //let stdin = embedded_io_adapters::std::FromStd::new(std::io::stdin());
    run(stdin).await;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

async fn run(mut reader: impl io_async::Read) {
    let mut serkey_reader = serkey::Reader::new();
    print!("Ctrl-C to exit\r\n");
    //let mut sbuf: heapless::String<64> = heapless::String::new();
    loop {
        if let Some(key) = serkey_reader.read_key(&mut reader).await {
            println!("{:?}", key);
            if let serkey::Key::CookedCtrl(0x03) = key { break; }
            //sbuf.clear();
            //format_key(&mut sbuf, &key);
            //print!("{}\r\n", sbuf);
            //if let serkey::Key::CookedCtrl(0x03) = key {
            //    break;
            //}
        }
    }
}

fn format_key(w: &mut impl core::fmt::Write, key: &serkey::Key) {
    use serkey::{Key, Modifier};
    fn sub(w: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(w, "{}", text).ok();
        if modifier.is_shift() { write!(w, " + Shift").ok(); }
        if modifier.is_ctrl() { write!(w, " + Ctrl").ok(); }
        if modifier.is_alt() { write!(w, " + Alt").ok(); }
    }
    match key {
        Key::CookedChar(ch)      => { write!(w, "CookedChar: {}", ch).ok(); }
        Key::CookedCtrl(n)       => { write!(w, "CookedCtrl: 0x{:02x}", n).ok(); }
        Key::Tab(attr)           => { sub(w, "Tab", attr.modifier()); }
        Key::Enter(attr)         => { sub(w, "Enter", attr.modifier()); }
        Key::Left(attr)          => { sub(w, "Left", attr.modifier()); }
        Key::Right(attr)         => { sub(w, "Right", attr.modifier()); }
        Key::Up(attr)            => { sub(w, "Up", attr.modifier()); }
        Key::Down(attr)          => { sub(w, "Down", attr.modifier()); }
        Key::Home(attr)          => { sub(w, "Home", attr.modifier()); }
        Key::End(attr)           => { sub(w, "End", attr.modifier()); }
        Key::PageUp(attr)        => { sub(w, "PageUp", attr.modifier()); }
        Key::PageDown(attr)      => { sub(w, "PageDown", attr.modifier()); }
        Key::Backspace(attr)     => { sub(w, "Backspace", attr.modifier()); }
        Key::Delete(attr)        => { sub(w, "Delete", attr.modifier()); }
        Key::Insert(attr)        => { sub(w, "Insert", attr.modifier()); }
        Key::Esc(attr)           => { sub(w, "Esc", attr.modifier()); }
        Key::F1(attr)            => { sub(w, "F1", attr.modifier()); }
        Key::F2(attr)            => { sub(w, "F2", attr.modifier()); }
        Key::F3(attr)            => { sub(w, "F3", attr.modifier()); }
        Key::F4(attr)            => { sub(w, "F4", attr.modifier()); }
        Key::F5(attr)            => { sub(w, "F5", attr.modifier()); }
        Key::F6(attr)            => { sub(w, "F6", attr.modifier()); }
        Key::F7(attr)            => { sub(w, "F7", attr.modifier()); }
        Key::F8(attr)            => { sub(w, "F8", attr.modifier()); }
        Key::F9(attr)            => { sub(w, "F9", attr.modifier()); }
        Key::F10(attr)           => { sub(w, "F10", attr.modifier()); }
        Key::F11(attr)           => { sub(w, "F11", attr.modifier()); }
        Key::F12(attr)           => { sub(w, "F12", attr.modifier()); }
        _ => ()
    }
}
