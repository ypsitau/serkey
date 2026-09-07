use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

fn main() -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();

    // 端末の現在の設定を取得・バックアップ
    let orig_termios = unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(stdin_fd, &mut termios);
        termios
    };

    // RAW モード用の設定に変更
    let mut raw_termios = orig_termios;
    unsafe {
        libc::cfmakeraw(&mut raw_termios);
        libc::tcsetattr(stdin_fd, libc::TCSANOW, &raw_termios);
    }

    println!("キーを押してください ('q' で終了):\r");

    let mut buffer = [0u8; 1];
    loop {
        // 1バイト読み込み
        io::stdin().read_exact(&mut buffer)?;
        let c = buffer[0] as char;

        if c == 'q' {
            println!("\r\n終了します。");
            break;
        }

        // Rawモード下での改行は \r\n が必要
        println!("\r\n押されたキー: '{}' (ASCII: {})", c, buffer[0]);
    }

    // 終了前に必ず元の端末設定へ復元
    unsafe {
        libc::tcsetattr(stdin_fd, libc::TCSANOW, &orig_termios);
    }

    Ok(())
}
