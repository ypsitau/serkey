#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp as rp;
use embassy_usb as usb;
use static_cell::StaticCell;
use embedded_io_async::Write as _;
use {defmt_rtt as _, panic_probe as _};

rp::bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => rp::usb::InterruptHandler<rp::peripherals::USB>;
});


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
        usb::Builder::new(usb_driver, config,
            config_descriptor_buf, bos_descriptor_buf, msos_descriptor_buf, control_buf)
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
        let (mut cdc_sender, cdc_receiver) = cdc_driver.split();
        let buf_receiver = {
            static STATIC_CELL: StaticCell<[u8; 64]> = StaticCell::new();
            STATIC_CELL.init([0u8; 64])
        };
        let mut cdc_receiver = cdc_receiver.into_buffered(buf_receiver);
        let mut sbuf: heapless::String<64> = heapless::String::new();
        let mut serkey_reader = serkey::Reader::new();
        loop {
            cdc_receiver.wait_connection().await;
            info!("Connected");
            loop {
                if let Some(key) = serkey_reader.read_key(&mut cdc_receiver).await {
                    sbuf.clear();
                    format_key(&mut sbuf, &key);
                    cdc_sender.write_all(sbuf.as_bytes()).await.unwrap();
                    cdc_sender.write_all(b"\r\n").await.unwrap();
                    cdc_sender.flush().await.unwrap();
                } else {
                    break;
                }
            }
        }
    };
    info!("Starting main loop");
    embassy_futures::join::join(fut_usb, fut_echo).await;
}

fn format_key(w: &mut impl core::fmt::Write, key: &serkey::Key) {
    use serkey::{Key, Modifier};
    fn sub(w: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(w, "{}", text).ok();
        if modifier.is_shift() { write!(w, " + Shift").ok(); }
        if modifier.is_ctrl() { write!(w, " + Ctrl").ok(); }
        if modifier.is_alt() { write!(w, " + Alt").ok(); }
    }
    match key {
        Key::CookedChar(ch)      => { write!(w, "CookedChar: {}", ch).ok(); }
        Key::CookedCtrl(n)       => { write!(w, "CookedCtrl: 0x{:02x}", n).ok(); }
        Key::Tab(attr)           => { sub(w, "Tab", attr.modifier()); }
        Key::Enter(attr)         => { sub(w, "Enter", attr.modifier()); }
        Key::Left(attr)          => { sub(w, "Left", attr.modifier()); }
        Key::Right(attr)         => { sub(w, "Right", attr.modifier()); }
        Key::Up(attr)            => { sub(w, "Up", attr.modifier()); }
        Key::Down(attr)          => { sub(w, "Down", attr.modifier()); }
        Key::Home(attr)          => { sub(w, "Home", attr.modifier()); }
        Key::End(attr)           => { sub(w, "End", attr.modifier()); }
        Key::PageUp(attr)        => { sub(w, "PageUp", attr.modifier()); }
        Key::PageDown(attr)      => { sub(w, "PageDown", attr.modifier()); }
        Key::Backspace(attr)     => { sub(w, "Backspace", attr.modifier()); }
        Key::Delete(attr)        => { sub(w, "Delete", attr.modifier()); }
        Key::Insert(attr)        => { sub(w, "Insert", attr.modifier()); }
        Key::Esc(attr)           => { sub(w, "Esc", attr.modifier()); }
        Key::F1(attr)            => { sub(w, "F1", attr.modifier()); }
        Key::F2(attr)            => { sub(w, "F2", attr.modifier()); }
        Key::F3(attr)            => { sub(w, "F3", attr.modifier()); }
        Key::F4(attr)            => { sub(w, "F4", attr.modifier()); }
        Key::F5(attr)            => { sub(w, "F5", attr.modifier()); }
        Key::F6(attr)            => { sub(w, "F6", attr.modifier()); }
        Key::F7(attr)            => { sub(w, "F7", attr.modifier()); }
        Key::F8(attr)            => { sub(w, "F8", attr.modifier()); }
        Key::F9(attr)            => { sub(w, "F9", attr.modifier()); }
        Key::F10(attr)           => { sub(w, "F10", attr.modifier()); }
        Key::F11(attr)           => { sub(w, "F11", attr.modifier()); }
        Key::F12(attr)           => { sub(w, "F12", attr.modifier()); }
        _ => ()
    }
}
