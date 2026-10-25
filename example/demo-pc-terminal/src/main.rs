use std::println as info;
use std::io::Read as _;

fn main() -> std::io::Result<()> {
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
    let mut stdin = std::io::stdin();
    let mut buf = [0u8; 32];
    let mut parser = serkey::Parser::new();
    info!("Ctrl-C to exit");
    loop {
        let buf_read = {
            let bytes_read = stdin.read(&mut buf)?;
            &buf[..bytes_read]
        };
        if buf_read.is_empty() || buf_read.contains(&0x03) { break; }
        info!("{:02X?}", buf_read);
        feed_parser(&mut parser, &buf_read);
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn feed_parser(parser: &mut serkey::Parser, buf: &[u8]) {
    use core::fmt::Write as _;
    use serkey::{Key, Modifier};
    fn write_key(strbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(strbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_ctrl() { " + Ctrl" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" }).ok();
    }
    let mut strbuf: heapless::String<64> = heapless::String::new();
    for &byte in buf {
        parser.push(byte);
        while let Some(key) = parser.next_key() {
            match key {
                Key::CookedChar(ch)      => { write!(strbuf, "CookedChar: {}", ch).ok(); }
                Key::CookedCtrl(n)       => { write!(strbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
                Key::Tab(attr)           => { write_key(&mut strbuf, "Tab", attr.modifier()); }
                Key::Enter(attr)         => { write_key(&mut strbuf, "Enter", attr.modifier()); }
                Key::Left(attr)          => { write_key(&mut strbuf, "Left", attr.modifier()); }
                Key::Right(attr)         => { write_key(&mut strbuf, "Right", attr.modifier()); }
                Key::Up(attr)            => { write_key(&mut strbuf, "Up", attr.modifier()); }
                Key::Down(attr)          => { write_key(&mut strbuf, "Down", attr.modifier()); }
                Key::Home(attr)          => { write_key(&mut strbuf, "Home", attr.modifier()); }
                Key::End(attr)           => { write_key(&mut strbuf, "End", attr.modifier()); }
                Key::PageUp(attr)        => { write_key(&mut strbuf, "PageUp", attr.modifier()); }
                Key::PageDown(attr)      => { write_key(&mut strbuf, "PageDown", attr.modifier()); }
                Key::Backspace(attr)     => { write_key(&mut strbuf, "Backspace", attr.modifier()); }
                Key::Delete(attr)        => { write_key(&mut strbuf, "Delete", attr.modifier()); }
                Key::Insert(attr)        => { write_key(&mut strbuf, "Insert", attr.modifier()); }
                Key::Esc(attr)           => { write_key(&mut strbuf, "Esc", attr.modifier()); }
                Key::F1(attr)            => { write_key(&mut strbuf, "F1", attr.modifier()); }
                Key::F2(attr)            => { write_key(&mut strbuf, "F2", attr.modifier()); }
                Key::F3(attr)            => { write_key(&mut strbuf, "F3", attr.modifier()); }
                Key::F4(attr)            => { write_key(&mut strbuf, "F4", attr.modifier()); }
                Key::F5(attr)            => { write_key(&mut strbuf, "F5", attr.modifier()); }
                Key::F6(attr)            => { write_key(&mut strbuf, "F6", attr.modifier()); }
                Key::F7(attr)            => { write_key(&mut strbuf, "F7", attr.modifier()); }
                Key::F8(attr)            => { write_key(&mut strbuf, "F8", attr.modifier()); }
                Key::F9(attr)            => { write_key(&mut strbuf, "F9", attr.modifier()); }
                Key::F10(attr)           => { write_key(&mut strbuf, "F10", attr.modifier()); }
                Key::F11(attr)           => { write_key(&mut strbuf, "F11", attr.modifier()); }
                Key::F12(attr)           => { write_key(&mut strbuf, "F12", attr.modifier()); }
                _ => { continue; }
            }
            info!("{}", strbuf.as_str());
        }
    }
}
