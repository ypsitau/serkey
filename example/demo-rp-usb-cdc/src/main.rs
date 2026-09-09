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
        loop {
            cdc_receiver.wait_connection().await;
            info!("Connected");
            let e = loop {
                let buf_read = match cdc_receiver.read_packet(buf).await {
                    Ok(n) => &buf[..n], Err(e) => break e,
                };
                feed_parser(&mut serkey_parser, buf_read, &mut cdc_sender).await;
            };
            if e != usb::driver::EndpointError::Disabled { break; }
        };
    };
    info!("Starting main loop");
    embassy_futures::join::join(fut_usb, fut_echo).await;
}

async fn feed_parser(parser: &mut serkey::Parser, buf: &[u8], mut writer: impl embedded_io_async::Write) {
    use core::fmt::Write as _;
    use serkey::{Vk, Modifier};
    fn write_key(strbuf: &mut impl core::fmt::Write, text: &str, modifier: Modifier) -> core::fmt::Result {
        write!(strbuf, "{}{}{}{}", text,
            if modifier.is_shift() { " + Shift" } else { "" },
            if modifier.is_control() { " + Control" } else { "" },
            if modifier.is_alt() { " + Alt" } else { "" })
    }
    let mut strbuf: heapless::String<64> = heapless::String::new();
    for &byte in buf {
        parser.push(byte);
        while let Some(vk) = parser.next_vk() {
            strbuf.clear();
            match vk {
                Vk::CookedChar(ch)      => { write!(strbuf, "CookedChar: {}", ch).ok(); }
                Vk::CookedCtrl(n)       => { write!(strbuf, "CookedCtrl: 0x{:02x}", n).ok(); }
                Vk::Back(attr)          => { write_key(&mut strbuf, "Back", attr.modifier()).ok(); }
                Vk::Return(attr)        => { write_key(&mut strbuf, "Return", attr.modifier()).ok(); }
                Vk::Left(attr)          => { write_key(&mut strbuf, "Left", attr.modifier()).ok(); }
                Vk::Right(attr)         => { write_key(&mut strbuf, "Right", attr.modifier()).ok(); }
                Vk::Up(attr)            => { write_key(&mut strbuf, "Up", attr.modifier()).ok(); }
                Vk::Down(attr)          => { write_key(&mut strbuf, "Down", attr.modifier()).ok(); }
                Vk::Home(attr)          => { write_key(&mut strbuf, "Home", attr.modifier()).ok(); }
                Vk::End(attr)           => { write_key(&mut strbuf, "End", attr.modifier()).ok(); }
                Vk::Prior(attr)         => { write_key(&mut strbuf, "Prior", attr.modifier()).ok(); }
                Vk::Next(attr)          => { write_key(&mut strbuf, "Next", attr.modifier()).ok(); }
                Vk::Tab(attr)           => { write_key(&mut strbuf, "Tab", attr.modifier()).ok(); }
                Vk::OemBacktab(attr)    => { write_key(&mut strbuf, "OemBacktab", attr.modifier()).ok(); }
                Vk::Delete(attr)        => { write_key(&mut strbuf, "Delete", attr.modifier()).ok(); }
                Vk::Insert(attr)        => { write_key(&mut strbuf, "Insert", attr.modifier()).ok(); }
                Vk::Escape(attr)        => { write_key(&mut strbuf, "Esc", attr.modifier()).ok(); }
                Vk::F1(attr)            => { write_key(&mut strbuf, "F1", attr.modifier()).ok(); }
                Vk::F2(attr)            => { write_key(&mut strbuf, "F2", attr.modifier()).ok(); }
                Vk::F3(attr)            => { write_key(&mut strbuf, "F3", attr.modifier()).ok(); }
                Vk::F4(attr)            => { write_key(&mut strbuf, "F4", attr.modifier()).ok(); }
                Vk::F5(attr)            => { write_key(&mut strbuf, "F5", attr.modifier()).ok(); }
                Vk::F6(attr)            => { write_key(&mut strbuf, "F6", attr.modifier()).ok(); }
                Vk::F7(attr)            => { write_key(&mut strbuf, "F7", attr.modifier()).ok(); }
                Vk::F8(attr)            => { write_key(&mut strbuf, "F8", attr.modifier()).ok(); }
                Vk::F9(attr)            => { write_key(&mut strbuf, "F9", attr.modifier()).ok(); }
                Vk::F10(attr)           => { write_key(&mut strbuf, "F10", attr.modifier()).ok(); }
                Vk::F11(attr)           => { write_key(&mut strbuf, "F11", attr.modifier()).ok(); }
                Vk::F12(attr)           => { write_key(&mut strbuf, "F12", attr.modifier()).ok(); }
                _ => { continue; }
            }
            writer.write_all(strbuf.as_bytes()).await.ok();
            writer.write_all(b"\r\n").await.ok();
        }
    }
}
