fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console as wc;
        unsafe {
            let handle = wc::GetStdHandle(wc::STD_INPUT_HANDLE);
            let mut mode = 0;
            if wc::GetConsoleMode(handle, &mut mode) != 0 {
                wc::SetConsoleMode(handle, mode | wc::ENABLE_VIRTUAL_TERMINAL_INPUT);
            }
        }
    }
    print!("Test of read_key_blocking(). Press Ctrl-C to exit\r\n");
    let stdin = embedded_io_adapters::std::FromStd::new(std::io::stdin());
    print_key_blocking(stdin);
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn print_key_blocking(mut reader: impl embedded_io::Read) {
    let mut serkey_reader = serkey::Reader::new();
    loop {
        if let Ok(key) = serkey_reader.read_key_blocking(&mut reader) {
            match key {
                serkey::Key::CookedChar('a') => println!("'a' is pressed"),
                serkey::Key::CookedCtrl(0x01) => println!("Ctrl-A is pressed"),
                serkey::Key::CookedCtrl(0x03) => break,
                serkey::Key::Left(attr) => println!("[Left] is pressed with {:?}", attr),
                serkey::Key::Delete(attr) => println!("[Delete] is pressed with {:?}", attr),
                _ => println!("{}", key),
            }
        }
    }
}