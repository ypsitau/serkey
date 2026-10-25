use std::io::{Read as _, Write as _};

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
    let mut buffer = [0u8; 1024];
    println!("Ctrl-C to exit");
    loop {
        let bytes_read = stdin.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let raw_bytes = &buffer[..bytes_read];

        if raw_bytes.contains(&0x03) {
            break;
        }

        let hex_str: Vec<String> = raw_bytes.iter().map(|b| format!("{:02X}", b)).collect();
        let escaped_str = String::from_utf8_lossy(raw_bytes).replace('\x1b', "\\x1b");

        println!(
            "受信 ({} bytes): [{}]  文字列: {}\r",
            bytes_read,
            hex_str.join(" "),
            escaped_str
        );

        std::io::stdout().flush()?;
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
