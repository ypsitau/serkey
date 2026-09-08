use println as info;
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
    use serkey::Vk;
    let print_key = |text: &str, modifier: serkey::Modifier| {
        info!("{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_control() { " + Control" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" });
    };
    for &byte in buf {
        parser.push(byte);
        while let Some(vk) = parser.next_vk() {
            match vk {
                Vk::CookedChar(ch)      => { info!("CookedChar: {}", ch); }
                Vk::CookedCtrl(n)       => { info!("CookedCtrl: 0x{:02x}", n); }
                Vk::Back(attr)          => { print_key("Back", attr.modifier()); }
                Vk::Return(attr)        => { print_key("Return", attr.modifier()); }
                Vk::Left(attr)          => { print_key("Left", attr.modifier()); }
                Vk::Right(attr)         => { print_key("Right", attr.modifier()); }
                Vk::Up(attr)            => { print_key("Up", attr.modifier()); }
                Vk::Down(attr)          => { print_key("Down", attr.modifier()); }
                Vk::Home(attr)          => { print_key("Home", attr.modifier()); }
                Vk::End(attr)           => { print_key("End", attr.modifier()); }
                Vk::Prior(attr)         => { print_key("Prior", attr.modifier()); }
                Vk::Next(attr)          => { print_key("Next", attr.modifier()); }
                Vk::Tab(attr)           => { print_key("Tab", attr.modifier()); }
                Vk::OemBacktab(attr)    => { print_key("OemBacktab", attr.modifier()); }
                Vk::Delete(attr)        => { print_key("Delete", attr.modifier()); }
                Vk::Insert(attr)        => { print_key("Insert", attr.modifier()); }
                Vk::Escape(attr)        => { print_key("Esc", attr.modifier()); }
                Vk::F1(attr)            => { print_key("F1", attr.modifier()); }
                Vk::F2(attr)            => { print_key("F2", attr.modifier()); }
                Vk::F3(attr)            => { print_key("F3", attr.modifier()); }
                Vk::F4(attr)            => { print_key("F4", attr.modifier()); }
                Vk::F5(attr)            => { print_key("F5", attr.modifier()); }
                Vk::F6(attr)            => { print_key("F6", attr.modifier()); }
                Vk::F7(attr)            => { print_key("F7", attr.modifier()); }
                Vk::F8(attr)            => { print_key("F8", attr.modifier()); }
                Vk::F9(attr)            => { print_key("F9", attr.modifier()); }
                Vk::F10(attr)           => { print_key("F10", attr.modifier()); }
                Vk::F11(attr)           => { print_key("F11", attr.modifier()); }
                Vk::F12(attr)           => { print_key("F12", attr.modifier()); }
                _ => {}
            }
        }
    }
}
