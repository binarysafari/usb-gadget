//! Device-side example for USB gadget with custom interface.

use std::{
    thread,
    time::Duration,
};

use nix::unistd::sleep;
use usb_gadget::{
    default_udc, function::{hid::Hid, midi::Midi}, Class, Config, Gadget, Id, OsDescriptor, Strings, WebUsb,
};

fn main() {
    env_logger::init();

    usb_gadget::remove_all().expect("cannot remove all gadgets");
    let udc = default_udc().expect("cannot get UDC");
    
	let mut midi_builder = Midi::builder();
	midi_builder.id = "ABC".to_string();
	let (_midi_control, midi_handle ) = midi_builder.build();

	let gadget = Gadget::new(
		Class::new(0xEF, 0x02, 0x01),
        Id::new(6, 0x11),
		Strings::new("manufacturer", "Midi interface", "serial_number"),
	);

	let mut reg = gadget
    .with_config(Config::new("config")
	.with_function(midi_handle))
	// .with_function(device_control_handle))
    .with_os_descriptor(OsDescriptor::microsoft())
    .with_web_usb(WebUsb::new(0xf1, "http://webusb.org"))
    .bind(&udc)
    .expect("cannot bind to UDC");

    thread::sleep(Duration::from_secs(1));
	reg.detach();
    // println!("Unregistering");
    // reg.remove().unwrap();

}
