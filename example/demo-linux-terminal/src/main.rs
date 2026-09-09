use std::println as info;
use std::io::Read as _;
use std::os::unix::io::AsRawFd as _;

fn main() -> std::io::Result<()> {
    let fd_stdin = std::io::stdin().as_raw_fd();
    let termios_org = unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(fd_stdin, &mut termios);
        termios
    };
    let mut termios_raw = termios_org;
    unsafe {
        libc::cfmakeraw(&mut termios_raw);
        termios_raw.c_oflag = termios_org.c_oflag;
        libc::tcsetattr(fd_stdin, libc::TCSANOW, &termios_raw);
    }
    let mut parser = serkey::Parser::new();
    let mut buf = [0u8; 1];
    println!("Ctrl-C to exit");
    loop {
        std::io::stdin().read_exact(&mut buf)?;
        //info!("Read byte: {:02x}", buf[0]);
        feed_parser(&mut parser, &buf);
        if buf[0] == 3 { break; }
    }
    unsafe {
        libc::tcsetattr(fd_stdin, libc::TCSANOW, &termios_org);
    }
    Ok(())
}

fn feed_parser(parser: &mut serkey::Parser, buf: &[u8]) {
    use core::fmt::Write as _;
    use serkey::{Vk, Modifier};
    fn write_key(strbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(strbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_control() { " + Control" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" }).ok();
    }
    let mut strbuf: heapless::String<64> = heapless::String::new();
    for &byte in buf {
        parser.push(byte);
        while let Some(vk) = parser.next_vk() {
            match vk {
                Vk::CookedChar(ch)      => { write!(strbuf, "CookedChar: {}", ch).ok(); }
                Vk::CookedCtrl(n)       => { write!(strbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
                Vk::Back(attr)          => { write_key(&mut strbuf, "Back", attr.modifier()); }
                Vk::Return(attr)        => { write_key(&mut strbuf, "Return", attr.modifier()); }
                Vk::Left(attr)          => { write_key(&mut strbuf, "Left", attr.modifier()); }
                Vk::Right(attr)         => { write_key(&mut strbuf, "Right", attr.modifier()); }
                Vk::Up(attr)            => { write_key(&mut strbuf, "Up", attr.modifier()); }
                Vk::Down(attr)          => { write_key(&mut strbuf, "Down", attr.modifier()); }
                Vk::Home(attr)          => { write_key(&mut strbuf, "Home", attr.modifier()); }
                Vk::End(attr)           => { write_key(&mut strbuf, "End", attr.modifier()); }
                Vk::Prior(attr)         => { write_key(&mut strbuf, "Prior", attr.modifier()); }
                Vk::Next(attr)          => { write_key(&mut strbuf, "Next", attr.modifier()); }
                Vk::Tab(attr)           => { write_key(&mut strbuf, "Tab", attr.modifier()); }
                Vk::OemBacktab(attr)    => { write_key(&mut strbuf, "OemBacktab", attr.modifier()); }
                Vk::Delete(attr)        => { write_key(&mut strbuf, "Delete", attr.modifier()); }
                Vk::Insert(attr)        => { write_key(&mut strbuf, "Insert", attr.modifier()); }
                Vk::Escape(attr)        => { write_key(&mut strbuf, "Esc", attr.modifier()); }
                Vk::F1(attr)            => { write_key(&mut strbuf, "F1", attr.modifier()); }
                Vk::F2(attr)            => { write_key(&mut strbuf, "F2", attr.modifier()); }
                Vk::F3(attr)            => { write_key(&mut strbuf, "F3", attr.modifier()); }
                Vk::F4(attr)            => { write_key(&mut strbuf, "F4", attr.modifier()); }
                Vk::F5(attr)            => { write_key(&mut strbuf, "F5", attr.modifier()); }
                Vk::F6(attr)            => { write_key(&mut strbuf, "F6", attr.modifier()); }
                Vk::F7(attr)            => { write_key(&mut strbuf, "F7", attr.modifier()); }
                Vk::F8(attr)            => { write_key(&mut strbuf, "F8", attr.modifier()); }
                Vk::F9(attr)            => { write_key(&mut strbuf, "F9", attr.modifier()); }
                Vk::F10(attr)           => { write_key(&mut strbuf, "F10", attr.modifier()); }
                Vk::F11(attr)           => { write_key(&mut strbuf, "F11", attr.modifier()); }
                Vk::F12(attr)           => { write_key(&mut strbuf, "F12", attr.modifier()); }
                _ => { continue; }
            }
            info!("{}", strbuf);
        }
    }
}
