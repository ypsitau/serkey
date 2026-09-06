#![no_std]
#![no_main]

use core::fmt::Write as _;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp as rp;
use embassy_usb as usb;
use embassy_time::Timer;
use static_cell::StaticCell;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

rp::bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => rp::usb::InterruptHandler<rp::peripherals::USB>;
});

mod embedded_terminal {
    pub trait Terminal {
        async fn print(&mut self, text: &str);
        async fn flush(&mut self);
        async fn clear(&mut self);
        async fn hide_cursor(&mut self);
        async fn show_cursor(&mut self);
        async fn move_cursor(&mut self, row: usize, col: usize);
        async fn move_up_n(&mut self, rows: usize);
        async fn move_up(&mut self) { self.move_up_n(1).await; }
        async fn move_down_n(&mut self, rows: usize);
        async fn move_down(&mut self) { self.move_down_n(1).await; }
        async fn move_left_n(&mut self, cols: usize);
        async fn move_left(&mut self) { self.move_left_n(1).await; }
        async fn move_right_n(&mut self, cols: usize);
        async fn move_right(&mut self) { self.move_right_n(1).await; }
        async fn move_to_beginning_of_line(&mut self);
        async fn erase_line(&mut self);
        async fn erase_screen(&mut self);
        async fn erase_to_end_of_line(&mut self);
        async fn erase_to_end_of_screen(&mut self);
        async fn erase_to_beginning_of_line(&mut self);
        async fn erase_to_beginning_of_screen(&mut self);
        async fn save_cursor_position(&mut self);
        async fn restore_cursor_position(&mut self);
    }
}

use embedded_terminal::Terminal as _;

struct SerialTerminal<W: embedded_io_async::Write> {
    writer: W,
}

impl<W: embedded_io_async::Write> SerialTerminal<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

struct LineEditor {
    line_buf: String::<128>,
    idx: usize,
}

impl LineEditor {
    pub fn new() -> Self {
        Self {
            line_buf: String::new(),
            idx: 0,
        }
    }
    pub async fn handle_keycode(&mut self, terminal: &mut impl embedded_terminal::Terminal, keycode: serkey::KeyCode) {
        match keycode {
            serkey::KeyCode::Char(ch) => if ch >= 0x20 as char {
                self.line_buf.insert(self.idx, ch).ok();
                terminal.save_cursor_position().await;
                terminal.print(&self.line_buf[self.idx..]).await;
                terminal.restore_cursor_position().await;
                terminal.move_right().await;
                self.idx += 1;
            } else if ch == '\x01' {
            } else if ch == '\x0b' {
            
            }
            serkey::KeyCode::Enter => {
                self.idx = 0;
                self.line_buf.clear();
                terminal.print("\r\n").await;
            }
            serkey::KeyCode::Delete => {
                if self.idx < self.line_buf.len() {
                    self.line_buf.remove(self.idx);
                    terminal.save_cursor_position().await;
                    terminal.print(&self.line_buf[self.idx..]).await;
                    terminal.erase_to_end_of_line().await;
                    terminal.restore_cursor_position().await;
                }
            }
            serkey::KeyCode::Backspace => {
                if self.idx > 0 {
                    self.idx -= 1;
                    self.line_buf.remove(self.idx);
                    terminal.move_left().await;
                    terminal.save_cursor_position().await;
                    terminal.print(&self.line_buf[self.idx..]).await;
                    terminal.erase_to_end_of_line().await;
                    terminal.restore_cursor_position().await;
                }
            }
            serkey::KeyCode::Home => {
                self.idx = 0;
                terminal.move_to_beginning_of_line().await;
            }
            serkey::KeyCode::Left => {
                if self.idx > 0 {
                    self.idx -= 1;
                    terminal.move_left().await;
                }
            }
            serkey::KeyCode::Right => {
                if self.idx < self.line_buf.len() {
                    self.idx += 1;
                    terminal.move_right().await;
                }
            }
            _ => {}
        }
    }
}

impl<W: embedded_io_async::Write> embedded_terminal::Terminal for SerialTerminal<W> {
    async fn print(&mut self, text: &str) {
        self.writer.write_all(text.as_bytes()).await;
    }
    async fn flush(&mut self) {
        self.writer.flush().await;
    }
    async fn clear(&mut self) {
        self.print("\x1b[2J\x1b[H").await;
    }
    async fn hide_cursor(&mut self) {
        self.print("\x1b[?25l").await;
    }
    async fn show_cursor(&mut self) {
        self.print("\x1b[?25h").await;
    }
    async fn move_cursor(&mut self, row: usize, col: usize) {
        let mut text = String::<64>::new();
        write!(text, "\x1b[{};{}H", row, col).unwrap();
        self.print(&text).await;
    }
    async fn move_up_n(&mut self, rows: usize) {
        let mut text = String::<64>::new();
        write!(text, "\x1b[{}A", rows).unwrap();
        self.print(&text).await;
    }
    async fn move_down_n(&mut self, rows: usize) {
        let mut text = String::<64>::new();
        write!(text, "\x1b[{}B", rows).unwrap();
        self.print(&text).await;
    }
    async fn move_left_n(&mut self, cols: usize) {
        let mut text = String::<64>::new();
        write!(text, "\x1b[{}D", cols).unwrap();
        self.print(&text).await;
    }
    async fn move_right_n(&mut self, cols: usize) {
        let mut text = String::<64>::new();
        write!(text, "\x1b[{}C", cols).unwrap();
        self.print(&text).await;
    }
    async fn move_to_beginning_of_line(&mut self) {
        self.print("\x1b[1G").await;
    }
    async fn erase_line(&mut self) {
        self.print("\x1b[2K").await;
    }
    async fn erase_screen(&mut self) {
        self.print("\x1b[2J").await;
    }
    async fn erase_to_end_of_line(&mut self) {
        self.print("\x1b[0K").await;
    }
    async fn erase_to_end_of_screen(&mut self) {
        self.print("\x1b[0J").await;
    }
    async fn erase_to_beginning_of_line(&mut self) {
        self.print("\x1b[1K").await;
    }
    async fn erase_to_beginning_of_screen(&mut self) {
        self.print("\x1b[1J").await;
    }
    async fn save_cursor_position(&mut self) {
        self.print("\x1b7").await;
    }
    async fn restore_cursor_position(&mut self) {
        self.print("\x1b8").await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = rp::init(Default::default());
    let usb_driver = rp::usb::Driver::new(p.USB, Irqs);
    let mut usb_builder = {
        const VID: u16 = 0xc0de;
        const PID: u16 = 0xcafe;
        const CONFIG_DESCRIPTOR_SIZE: usize = 256;
        const BOS_DESCRIPTOR_SIZE: usize = 256;
        const MSOS_DESCRIPTOR_SIZE: usize = 256;
        const CONTROL_BUF_SIZE: usize = 64;
        let mut config = usb::Config::new(VID, PID);
        config.manufacturer = Some("Embassy");
        config.product = Some("rp_usb_cdc");
        config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = CONTROL_BUF_SIZE as u8;
        let config_descriptor_buf = { // should be replaced by make_static macro when it becomes available
            static STATIC_CELL: StaticCell<[u8; CONFIG_DESCRIPTOR_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0; CONFIG_DESCRIPTOR_SIZE])
        };
        let bos_descriptor_buf = { // should be replaced by make_static macro when it becomes available
            static STATIC_CELL: StaticCell<[u8; BOS_DESCRIPTOR_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0; BOS_DESCRIPTOR_SIZE])
        };
        let msos_descriptor_buf = { // should be replaced by make_static macro when it becomes available
            static STATIC_CELL: StaticCell<[u8; MSOS_DESCRIPTOR_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0; MSOS_DESCRIPTOR_SIZE])
        };
        let control_buf = { // should be replaced by make_static macro when it becomes available
            static STATIC_CELL: StaticCell<[u8; CONTROL_BUF_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0; CONTROL_BUF_SIZE])
        };
        //let device_handler = { // should be replaced by make_static macro when it becomes available
        //    static STATIC_CELL: StaticCell<DeviceHandler> = StaticCell::new();
        //    STATIC_CELL.init(DeviceHandler::new())
        //};
        let usb_builder = usb::Builder::new(usb_driver, config,
            config_descriptor_buf, bos_descriptor_buf, msos_descriptor_buf, control_buf);
        //usb_builder.handler(device_handler);
        usb_builder
    };
    let cdc_driver = {
        let state = {
            static STATE: StaticCell<usb::class::cdc_acm::State> = StaticCell::new();
            STATE.init(usb::class::cdc_acm::State::new())
        };
        let max_packet_size = 64;
        usb::class::cdc_acm::CdcAcmClass::new(&mut usb_builder, state, max_packet_size)
    };
    let mut usb_device = usb_builder.build();
    let fut_usb = usb_device.run();
    let fut_echo = async {
        let (mut cdc_sender, mut cdc_receiver) = cdc_driver.split();
        let buf = {
            static STATIC_CELL: StaticCell<[u8; 64]> = StaticCell::new();
            STATIC_CELL.init([0u8; 64])
        };
        let mut serkey_parser = serkey::Parser::new();
        let mut line_editor = LineEditor::new();
        let mut terminal = SerialTerminal::new(&mut cdc_sender);
        loop {
            cdc_receiver.wait_connection().await;
            info!("Connected");
            let e = loop {
                let buf_read = match cdc_receiver.read_packet(buf).await {
                    Ok(n) => &buf[..n], Err(e) => break e,
                };
                info!("Read packet: {:02x}", buf_read);
                for &byte in buf_read {
                    serkey_parser.push(byte);
                    while let Some(keycode) = serkey_parser.next_keycode() {
                        line_editor.handle_keycode(&mut terminal, keycode).await;
                    }
                }
            };
            if e != usb::driver::EndpointError::Disabled { break; }
        };
    };
    let fut_gpio = async {
        let mut gpio_led = rp::gpio::Output::new(p.PIN_25, rp::gpio::Level::Low);
        loop {
            gpio_led.set_high();
            Timer::after_secs(1).await;
            gpio_led.set_low();
            Timer::after_secs(1).await;
        }
    };
    info!("Starting main loop");
    embassy_futures::join::join3(fut_usb, fut_echo, fut_gpio).await;
}
