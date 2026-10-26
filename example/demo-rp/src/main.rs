#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp as rp;
use embassy_usb as usb;
use static_cell::StaticCell;
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
        let mut parser = serkey::Parser::new();
        let mut buf = [0u8; 64];
        loop {
            cdc_receiver.wait_connection().await;
            info!("Connected");
            let e = loop {
                let buf_read = match cdc_receiver.read_packet(&mut buf).await {
                    Ok(n) => &buf[..n], Err(e) => break e,
                };
                feed_parser(&mut parser, buf_read, &mut cdc_sender).await;
            };
            if e != usb::driver::EndpointError::Disabled { break; }
        };
    };
    info!("Starting main loop");
    embassy_futures::join::join(fut_usb, fut_echo).await;
}

async fn feed_parser(parser: &mut serkey::Parser, buf: &[u8], mut writer: impl embedded_io_async::Write) {
    use core::fmt::Write as _;
    use serkey::{Key, Modifier};
    fn write_key(strbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) {
        write!(strbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_ctrl() { " + Ctrl" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" }).ok();
    }
    let mut strbuf: heapless::String<64> = heapless::String::new();
    for &byte in buf {
        parser.push(byte);
        while let Some(key) = parser.next_key() {
            match key {
                Key::CookedChar(ch)      => { write!(strbuf, "CookedChar: {}", ch).ok(); }
                Key::CookedCtrl(n)       => { write!(strbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
                Key::Tab(attr)           => { write_key(&mut strbuf, "Tab", attr.modifier()); }
                Key::Enter(attr)         => { write_key(&mut strbuf, "Enter", attr.modifier()); }
                Key::Left(attr)          => { write_key(&mut strbuf, "Left", attr.modifier()); }
                Key::Right(attr)         => { write_key(&mut strbuf, "Right", attr.modifier()); }
                Key::Up(attr)            => { write_key(&mut strbuf, "Up", attr.modifier()); }
                Key::Down(attr)          => { write_key(&mut strbuf, "Down", attr.modifier()); }
                Key::Home(attr)          => { write_key(&mut strbuf, "Home", attr.modifier()); }
                Key::End(attr)           => { write_key(&mut strbuf, "End", attr.modifier()); }
                Key::PageUp(attr)        => { write_key(&mut strbuf, "PageUp", attr.modifier()); }
                Key::PageDown(attr)      => { write_key(&mut strbuf, "PageDown", attr.modifier()); }
                Key::Backspace(attr)     => { write_key(&mut strbuf, "Backspace", attr.modifier()); }
                Key::Delete(attr)        => { write_key(&mut strbuf, "Delete", attr.modifier()); }
                Key::Insert(attr)        => { write_key(&mut strbuf, "Insert", attr.modifier()); }
                Key::Esc(attr)           => { write_key(&mut strbuf, "Esc", attr.modifier()); }
                Key::F1(attr)            => { write_key(&mut strbuf, "F1", attr.modifier()); }
                Key::F2(attr)            => { write_key(&mut strbuf, "F2", attr.modifier()); }
                Key::F3(attr)            => { write_key(&mut strbuf, "F3", attr.modifier()); }
                Key::F4(attr)            => { write_key(&mut strbuf, "F4", attr.modifier()); }
                Key::F5(attr)            => { write_key(&mut strbuf, "F5", attr.modifier()); }
                Key::F6(attr)            => { write_key(&mut strbuf, "F6", attr.modifier()); }
                Key::F7(attr)            => { write_key(&mut strbuf, "F7", attr.modifier()); }
                Key::F8(attr)            => { write_key(&mut strbuf, "F8", attr.modifier()); }
                Key::F9(attr)            => { write_key(&mut strbuf, "F9", attr.modifier()); }
                Key::F10(attr)           => { write_key(&mut strbuf, "F10", attr.modifier()); }
                Key::F11(attr)           => { write_key(&mut strbuf, "F11", attr.modifier()); }
                Key::F12(attr)           => { write_key(&mut strbuf, "F12", attr.modifier()); }
                _ => { continue; }
            }
            //info!("{}", strbuf.as_str());
            writer.write_all(strbuf.as_bytes()).await.ok();
            writer.write_all(b"\r\n").await.ok();
        }
    }
}
