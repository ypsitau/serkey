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
        let (mut cdc_sender, mut cdc_receiver) = cdc_driver.split();
        let buf_receiver = {
            static STATIC_CELL: StaticCell<[u8; 64]> = StaticCell::new();
            STATIC_CELL.init([0u8; 64])
        };
        let mut sbuf: heapless::String<64> = heapless::String::new();
        cdc_receiver.wait_connection().await;
        info!("Connected");
        let mut cdc_receiver = cdc_receiver.into_buffered(buf_receiver);
        let mut reader = serkey::Reader::new(&mut cdc_receiver);
        loop {
            if let Some(key) = reader.next_key().await {
                sbuf.clear();
                key_to_string(&mut sbuf, &key);
                cdc_sender.write_all(sbuf.as_bytes()).await.unwrap();
                cdc_sender.write_all(b"\r\n").await.unwrap();
                cdc_sender.flush().await.unwrap();
            }
        };
    };
    info!("Starting main loop");
    embassy_futures::join::join(fut_usb, fut_echo).await;
}

fn key_to_string(sbuf: &mut impl core::fmt::Write, key: &serkey::Key) {
    use serkey::{Key, Modifier};
    fn write_key(sbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(sbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_ctrl() { " + Ctrl" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" }).ok();
    }
    match key {
        Key::CookedChar(ch)      => { write!(sbuf, "CookedChar: {}", ch).ok(); }
        Key::CookedCtrl(n)       => { write!(sbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
        Key::Tab(attr)           => { write_key(sbuf, "Tab", attr.modifier()); }
        Key::Enter(attr)         => { write_key(sbuf, "Enter", attr.modifier()); }
        Key::Left(attr)          => { write_key(sbuf, "Left", attr.modifier()); }
        Key::Right(attr)         => { write_key(sbuf, "Right", attr.modifier()); }
        Key::Up(attr)            => { write_key(sbuf, "Up", attr.modifier()); }
        Key::Down(attr)          => { write_key(sbuf, "Down", attr.modifier()); }
        Key::Home(attr)          => { write_key(sbuf, "Home", attr.modifier()); }
        Key::End(attr)           => { write_key(sbuf, "End", attr.modifier()); }
        Key::PageUp(attr)        => { write_key(sbuf, "PageUp", attr.modifier()); }
        Key::PageDown(attr)      => { write_key(sbuf, "PageDown", attr.modifier()); }
        Key::Backspace(attr)     => { write_key(sbuf, "Backspace", attr.modifier()); }
        Key::Delete(attr)        => { write_key(sbuf, "Delete", attr.modifier()); }
        Key::Insert(attr)        => { write_key(sbuf, "Insert", attr.modifier()); }
        Key::Esc(attr)           => { write_key(sbuf, "Esc", attr.modifier()); }
        Key::F1(attr)            => { write_key(sbuf, "F1", attr.modifier()); }
        Key::F2(attr)            => { write_key(sbuf, "F2", attr.modifier()); }
        Key::F3(attr)            => { write_key(sbuf, "F3", attr.modifier()); }
        Key::F4(attr)            => { write_key(sbuf, "F4", attr.modifier()); }
        Key::F5(attr)            => { write_key(sbuf, "F5", attr.modifier()); }
        Key::F6(attr)            => { write_key(sbuf, "F6", attr.modifier()); }
        Key::F7(attr)            => { write_key(sbuf, "F7", attr.modifier()); }
        Key::F8(attr)            => { write_key(sbuf, "F8", attr.modifier()); }
        Key::F9(attr)            => { write_key(sbuf, "F9", attr.modifier()); }
        Key::F10(attr)           => { write_key(sbuf, "F10", attr.modifier()); }
        Key::F11(attr)           => { write_key(sbuf, "F11", attr.modifier()); }
        Key::F12(attr)           => { write_key(sbuf, "F12", attr.modifier()); }
        _ => ()
    }
}
