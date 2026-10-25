use tokio::io::AsyncReadExt as _;

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
    let mut stdin = tokio::io::stdin();
    let mut buf = [0u8; 32];
    let mut parser = serkey::Parser::new();
    println!("Ctrl-C to exit");
    loop {
        let buf_read = {
            let bytes_read = stdin.read(&mut buf).await?;
            &buf[..bytes_read]
        };
        if buf_read.is_empty() || buf_read.contains(&0x03) { break; }
        println!("{:02X?}", buf_read);
        feed_parser(&mut parser, &buf_read);
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn feed_parser(parser: &mut serkey::Parser, buf: &[u8]) {
    use core::fmt::Write as _;
    use serkey::{Key, Modifier};
    fn write_key(sbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(sbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_ctrl() { " + Ctrl" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" }).ok();
    }
    let mut sbuf: heapless::String<64> = heapless::String::new();
    for &byte in buf {
        parser.push(byte);
        while let Some(key) = parser.next_key() {
            match key {
                Key::CookedChar(ch)      => { write!(sbuf, "CookedChar: {}", ch).ok(); }
                Key::CookedCtrl(n)       => { write!(sbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
                Key::Tab(attr)           => { write_key(&mut sbuf, "Tab", attr.modifier()); }
                Key::Enter(attr)         => { write_key(&mut sbuf, "Enter", attr.modifier()); }
                Key::Left(attr)          => { write_key(&mut sbuf, "Left", attr.modifier()); }
                Key::Right(attr)         => { write_key(&mut sbuf, "Right", attr.modifier()); }
                Key::Up(attr)            => { write_key(&mut sbuf, "Up", attr.modifier()); }
                Key::Down(attr)          => { write_key(&mut sbuf, "Down", attr.modifier()); }
                Key::Home(attr)          => { write_key(&mut sbuf, "Home", attr.modifier()); }
                Key::End(attr)           => { write_key(&mut sbuf, "End", attr.modifier()); }
                Key::PageUp(attr)        => { write_key(&mut sbuf, "PageUp", attr.modifier()); }
                Key::PageDown(attr)      => { write_key(&mut sbuf, "PageDown", attr.modifier()); }
                Key::Backspace(attr)     => { write_key(&mut sbuf, "Backspace", attr.modifier()); }
                Key::Delete(attr)        => { write_key(&mut sbuf, "Delete", attr.modifier()); }
                Key::Insert(attr)        => { write_key(&mut sbuf, "Insert", attr.modifier()); }
                Key::Esc(attr)           => { write_key(&mut sbuf, "Esc", attr.modifier()); }
                Key::F1(attr)            => { write_key(&mut sbuf, "F1", attr.modifier()); }
                Key::F2(attr)            => { write_key(&mut sbuf, "F2", attr.modifier()); }
                Key::F3(attr)            => { write_key(&mut sbuf, "F3", attr.modifier()); }
                Key::F4(attr)            => { write_key(&mut sbuf, "F4", attr.modifier()); }
                Key::F5(attr)            => { write_key(&mut sbuf, "F5", attr.modifier()); }
                Key::F6(attr)            => { write_key(&mut sbuf, "F6", attr.modifier()); }
                Key::F7(attr)            => { write_key(&mut sbuf, "F7", attr.modifier()); }
                Key::F8(attr)            => { write_key(&mut sbuf, "F8", attr.modifier()); }
                Key::F9(attr)            => { write_key(&mut sbuf, "F9", attr.modifier()); }
                Key::F10(attr)           => { write_key(&mut sbuf, "F10", attr.modifier()); }
                Key::F11(attr)           => { write_key(&mut sbuf, "F11", attr.modifier()); }
                Key::F12(attr)           => { write_key(&mut sbuf, "F12", attr.modifier()); }
                _ => { continue; }
            }
            println!("{}", sbuf.as_str());
        }
    }
}
