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
    let print_key = |text: &str, modifier: serkey::Modifier| {
        info!("{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_control() { " + Control" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" });
    };
    for &byte in buf {
        parser.push(byte);
        while let Some(keycode) = parser.next_keycode() {
            match keycode {
                serkey::Vk::CookedChar(ch)      => { info!("CookedChar: {}", ch); }
                serkey::Vk::CookedCtrl(n)       => { info!("CookedCtrl: 0x{:02x}", n); }
                serkey::Vk::Back(attr)          => { print_key("Back", attr.modifier()); }
                serkey::Vk::Return(attr)        => { print_key("Return", attr.modifier()); }
                serkey::Vk::Left(attr)          => { print_key("Left", attr.modifier()); }
                serkey::Vk::Right(attr)         => { print_key("Right", attr.modifier()); }
                serkey::Vk::Up(attr)            => { print_key("Up", attr.modifier()); }
                serkey::Vk::Down(attr)          => { print_key("Down", attr.modifier()); }
                serkey::Vk::Home(attr)          => { print_key("Home", attr.modifier()); }
                serkey::Vk::End(attr)           => { print_key("End", attr.modifier()); }
                serkey::Vk::Prior(attr)         => { print_key("Prior", attr.modifier()); }
                serkey::Vk::Next(attr)          => { print_key("Next", attr.modifier()); }
                serkey::Vk::Tab(attr)           => { print_key("Tab", attr.modifier()); }
                serkey::Vk::OemBacktab(attr)    => { print_key("OemBacktab", attr.modifier()); }
                serkey::Vk::Delete(attr)        => { print_key("Delete", attr.modifier()); }
                serkey::Vk::Insert(attr)        => { print_key("Insert", attr.modifier()); }
                serkey::Vk::Escape(attr)        => { print_key("Esc", attr.modifier()); }
                serkey::Vk::F1(attr)            => { print_key("F1", attr.modifier()); }
                serkey::Vk::F2(attr)            => { print_key("F2", attr.modifier()); }
                serkey::Vk::F3(attr)            => { print_key("F3", attr.modifier()); }
                serkey::Vk::F4(attr)            => { print_key("F4", attr.modifier()); }
                serkey::Vk::F5(attr)            => { print_key("F5", attr.modifier()); }
                serkey::Vk::F6(attr)            => { print_key("F6", attr.modifier()); }
                serkey::Vk::F7(attr)            => { print_key("F7", attr.modifier()); }
                serkey::Vk::F8(attr)            => { print_key("F8", attr.modifier()); }
                serkey::Vk::F9(attr)            => { print_key("F9", attr.modifier()); }
                serkey::Vk::F10(attr)           => { print_key("F10", attr.modifier()); }
                serkey::Vk::F11(attr)           => { print_key("F11", attr.modifier()); }
                serkey::Vk::F12(attr)           => { print_key("F12", attr.modifier()); }
                _ => {}
            }
        }
    }
}
