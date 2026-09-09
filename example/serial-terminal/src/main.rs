#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic;
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
    icursor: usize,
}

impl LineEditor {
    pub fn new() -> Self {
        Self {
            line_buf: String::new(),
            icursor: 0,
        }
    }
    pub async fn handle_vk(&mut self, terminal: &mut impl embedded_terminal::Terminal, vk: serkey::Vk) {
        match vk {
            serkey::Vk::CookedChar(ch) => {
                self.line_buf.insert(self.icursor, ch).ok();
                let icursor = self.icursor;
                self.icursor += 1;
                terminal.save_cursor_position().await;
                terminal.print(&self.line_buf[icursor..]).await;
                terminal.restore_cursor_position().await;
                terminal.move_right().await;
            }
            serkey::Vk::CookedCtrl(ctrl) =>
            if ctrl == b'A' - b'@' {
            } else if ctrl == b'B' - b'@' {
            } else if ctrl == b'D' - b'@' {
            } else if ctrl == b'E' - b'@' {
            } else if ctrl == b'F' - b'@' {
            } else if ctrl == b'K' - b'@' {
            } else if ctrl == b'N' - b'@' {
            } else if ctrl == b'P' - b'@' {
            }
            serkey::Vk::Return(_) => {
                self.icursor = 0;
                self.line_buf.clear();
                terminal.print("\r\n").await;
            }
            serkey::Vk::Delete(_) => {
                if self.icursor < self.line_buf.len() {
                    let icursor = self.icursor;
                    self.line_buf.remove(self.icursor);
                    terminal.save_cursor_position().await;
                    terminal.print(&self.line_buf[icursor..]).await;
                    terminal.erase_to_end_of_line().await;
                    terminal.restore_cursor_position().await;
                }
            }
            serkey::Vk::Back(_) => {
                if self.icursor > 0 {
                    self.icursor -= 1;
                    let icursor = self.icursor;
                    self.line_buf.remove(self.icursor);
                    terminal.move_left().await;
                    terminal.save_cursor_position().await;
                    terminal.print(&self.line_buf[icursor..]).await;
                    terminal.erase_to_end_of_line().await;
                    terminal.restore_cursor_position().await;
                }
            }
            serkey::Vk::Home(_) => {
                self.icursor = 0;
                terminal.move_to_beginning_of_line().await;
            }
            serkey::Vk::Left(_) => {
                if self.icursor > 0 {
                    self.icursor -= 1;
                    terminal.move_left().await;
                }
            }
            serkey::Vk::Right(_) => {
                if self.icursor < self.line_buf.len() {
                    self.icursor += 1;
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
        let mut usb_config = usb::Config::new(VID, PID);
        usb_config.manufacturer = Some("Embassy");
        usb_config.product = Some("serial-terminal");
        usb_config.serial_number = Some("12345678");
        usb_config.max_power = 100;
        usb_config.max_packet_size_0 = CONTROL_BUF_SIZE as u8;
        let config_descriptor_buf = {
            static STATIC_CELL: StaticCell<[u8; CONFIG_DESCRIPTOR_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0u8; CONFIG_DESCRIPTOR_SIZE])
        };
        let bos_descriptor_buf = {
            static STATIC_CELL: StaticCell<[u8; BOS_DESCRIPTOR_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0u8; BOS_DESCRIPTOR_SIZE])
        };
        let msos_descriptor_buf = {
            static STATIC_CELL: StaticCell<[u8; MSOS_DESCRIPTOR_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0u8; MSOS_DESCRIPTOR_SIZE])
        };
        let control_buf = {
            static STATIC_CELL: StaticCell<[u8; CONTROL_BUF_SIZE]> = StaticCell::new();
            STATIC_CELL.init([0u8; CONTROL_BUF_SIZE])
        };
        let usb_handler = {
            static STATIC_CELL: StaticCell<USBHandler> = StaticCell::new();
            STATIC_CELL.init(USBHandler::new())
        };
        let mut usb_builder = usb::Builder::new(usb_driver, usb_config,
            config_descriptor_buf, bos_descriptor_buf, msos_descriptor_buf, control_buf);
        usb_builder.handler(usb_handler);
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
                terminal.process_input(buf_read);
                //if let Err(e) = cdc_sender.write_packet(buf_read).await { break e; }
            };
            if e != usb::driver::EndpointError::Disabled { break; }
        };
    };
    let fut_blinky = async {
        let mut gpio_led = rp::gpio::Output::new(p.PIN_25, rp::gpio::Level::Low);
        loop {
            gpio_led.set_high();
            Timer::after_secs(1).await;
            gpio_led.set_low();
            Timer::after_secs(1).await;
        }
    };
    info!("Starting main loop");
    embassy_futures::join::join3(fut_usb, fut_echo, fut_blinky).await;
}

//-----------------------------------------------------------------------------
// USBHandler
//-----------------------------------------------------------------------------
struct USBHandler {
    configured: atomic::AtomicBool,
}

impl USBHandler {
    fn new() -> Self {
        USBHandler { configured: atomic::AtomicBool::new(false), }
    }
}

impl usb::Handler for USBHandler {
    /// Called when the USB device has been enabled or disabled.
    fn enabled(&mut self, enabled: bool) {
        info!("usb::Handler.enabled({})", enabled);
        self.configured.store(false, atomic::Ordering::Relaxed);
    }
    /// Called after a USB reset after the bus reset sequence is complete.
    fn reset(&mut self) {
        info!("usb::Handler.reset()");
        self.configured.store(false, atomic::Ordering::Relaxed);
    }
    /// Called when the host has set the address of the device to `addr`.
    fn addressed(&mut self, addr: u8) {
        info!("usb::Handler.addressed(addr: {})", addr);
        self.configured.store(false, atomic::Ordering::Relaxed);
    }
    /// Called when the host has enabled or disabled the configuration of the device.
    fn configured(&mut self, configured: bool) {
        info!("usb::Handler.configured(configured: {})", configured);
        self.configured.store(configured, atomic::Ordering::Relaxed);
    }
}
