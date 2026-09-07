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
        //writeln!(out, "Read byte: {:02x}", buf[0]).ok();
        feed_parser(&mut parser, &buf);
        if buf[0] == 3 { break; }
    }
    unsafe {
        libc::tcsetattr(fd_stdin, libc::TCSANOW, &termios_org);
    }
    Ok(())
}

fn feed_parser(parser: &mut serkey::Parser, buf: &[u8]) {
    for &byte in buf {
        parser.push(byte);
        while let Some(keycode) = parser.next_keycode() {
            match keycode {
                serkey::KeyCode::Backspace => { info!("Backspace"); }
                serkey::KeyCode::Enter => { info!("Enter"); }
                serkey::KeyCode::Left => { info!("Left"); }
                serkey::KeyCode::Right => { info!("Right"); }
                serkey::KeyCode::Up => { info!("Up"); }
                serkey::KeyCode::Down => { info!("Down"); }
                serkey::KeyCode::Home => { info!("Home"); }
                serkey::KeyCode::End => { info!("End"); }
                serkey::KeyCode::PageUp => { info!("PageUp"); }
                serkey::KeyCode::PageDown => { info!("PageDown"); }
                serkey::KeyCode::Tab => { info!("Tab"); }
                serkey::KeyCode::BackTab => { info!("BackTab"); }
                serkey::KeyCode::Delete => { info!("Delete"); }
                serkey::KeyCode::Insert => { info!("Insert"); }
                serkey::KeyCode::F(n) => { info!("F{}", n); }
                serkey::KeyCode::Char(ch) => { info!("Char: {}", ch); }
                serkey::KeyCode::Ctrl(n) => { info!("Ctrl: 0x{:02x}", n); }
                serkey::KeyCode::Null => { info!("Null"); }
                serkey::KeyCode::Esc => { info!("Esc"); }
            }
        }
    }
}
