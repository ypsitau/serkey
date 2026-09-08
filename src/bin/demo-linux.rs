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
    let print_key = |text: &str, is_shift: bool| {
        info!("{}{}", text, if is_shift { " + Shift" } else { "" });
    };
    for &byte in buf {
        parser.push(byte);
        while let Some(keycode) = parser.next_keycode() {
            match keycode {
                serkey::Vk::CookedChar(ch) => { info!("CookedChar: {}", ch); }
                serkey::Vk::CookedCtrl(n) => { info!("CookedCtrl: 0x{:02x}", n); }
                serkey::Vk::Back(attr) => { print_key("Back", attr.is_shift()); }
                serkey::Vk::Return(attr) => { print_key("Return", attr.is_shift()); }
                serkey::Vk::Left(attr) => { print_key("Left", attr.is_shift()); }
                serkey::Vk::Right(attr) => { print_key("Right", attr.is_shift()); }
                serkey::Vk::Up(attr) => { print_key("Up", attr.is_shift()); }
                serkey::Vk::Down(attr) => { print_key("Down", attr.is_shift()); }
                serkey::Vk::Home(attr) => { print_key("Home", attr.is_shift()); }
                serkey::Vk::End(attr) => { print_key("End", attr.is_shift()); }
                serkey::Vk::Prior(attr) => { print_key("Prior", attr.is_shift()); }
                serkey::Vk::Next(attr) => { print_key("Next", attr.is_shift()); }
                serkey::Vk::Tab(attr) => { print_key("Tab", attr.is_shift()); }
                serkey::Vk::OemBacktab(attr) => { print_key("OemBacktab", attr.is_shift()); }
                serkey::Vk::Delete(attr) => { print_key("Delete", attr.is_shift()); }
                serkey::Vk::Insert(attr) => { print_key("Insert", attr.is_shift()); }
                serkey::Vk::Escape(attr) => { print_key("Esc", attr.is_shift()); }
                serkey::Vk::F1(attr) => { print_key("F1", attr.is_shift()); }
                serkey::Vk::F2(attr) => { print_key("F2", attr.is_shift()); }
                serkey::Vk::F3(attr) => { print_key("F3", attr.is_shift()); }
                serkey::Vk::F4(attr) => { print_key("F4", attr.is_shift()); }
                serkey::Vk::F5(attr) => { print_key("F5", attr.is_shift()); }
                serkey::Vk::F6(attr) => { print_key("F6", attr.is_shift()); }
                serkey::Vk::F7(attr) => { print_key("F7", attr.is_shift()); }
                serkey::Vk::F8(attr) => { print_key("F8", attr.is_shift()); }
                serkey::Vk::F9(attr) => { print_key("F9", attr.is_shift()); }
                serkey::Vk::F10(attr) => { print_key("F10", attr.is_shift()); }
                serkey::Vk::F11(attr) => { print_key("F11", attr.is_shift()); }
                serkey::Vk::F12(attr) => { print_key("F12", attr.is_shift()); }
                _ => {}
            }
        }
    }
}
