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
                //info!("Read packet: {:02x}", buf_read);
                feed_parser(&mut serkey_parser, buf_read);
                //for &byte in buf_read {
                //    serkey_parser.push(byte);
                //    while let Some(vk) = serkey_parser.next_vk() {
                //        line_editor.handle_vk(&mut terminal, vk).await;
                //    }
                //}
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
