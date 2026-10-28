#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
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
    print!("Ctrl-C to exit\r\n");
    //let mut stdin = embedded_io_adapters::tokio_1::FromTokio::new(tokio::io::stdin());
    //print_key_async(stdin).await;
    let stdin = embedded_io_adapters::std::FromStd::new(std::io::stdin());
    print_key_blocking(stdin);
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

async fn print_key_async(mut reader: impl embedded_io_async::Read) {
    let mut serkey_reader = serkey::Reader::new();
    loop {
        if let Some(key) = serkey_reader.read_key(&mut reader).await {
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

fn print_key_blocking(mut reader: impl embedded_io::Read) {
    let mut serkey_reader = serkey::Reader::new();
    loop {
        if let Some(key) = serkey_reader.read_key_blocking(&mut reader) {
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