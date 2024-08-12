//! USB gadget MIDI function.
//!
//! The Linux kernel configuration option `CONFIG_USB_CONFIGFS_F_HID` must be enabled.

use std::{
    ffi::OsString,
    io::Result,
};

use super::{
    util::{FunctionDir, Status},
    Function, Handle,
};

/// Builder for USB gadget MIDI function.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MidiBuilder {
	/// index value for the USB MIDI adapter?
	pub index: i8,
    /// ID string for the USB MIDI adapter
    pub id: String,
    /// MIDI buffer length
    pub buflen: u16,
    /// HID report length.
    pub qlen: u16,
    /// number of MIDI input ports
    pub in_ports: u8,
    /// number of MIDI output ports
    pub out_ports: u8,
}

impl MidiBuilder {
    /// Build the USB function.
    ///
    /// The returned handle must be added to a USB gadget configuration.
    pub fn build(self) -> (Midi, Handle) {
        let dir = FunctionDir::new();
        (Midi { dir: dir.clone() }, Handle::new(MidiFunction { builder: self, dir }))
    }
}

#[derive(Debug)]
struct MidiFunction {
    builder: MidiBuilder,
    dir: FunctionDir,
}

impl Function for MidiFunction {
    fn driver(&self) -> OsString {
        "midi".into()
    }

    fn dir(&self) -> FunctionDir {
        self.dir.clone()
    }

    fn register(&self) -> Result<()> {
        self.dir.write("id", self.builder.id.to_string())?;
        self.dir.write("in_ports", self.builder.in_ports.to_string())?;
        self.dir.write("out_ports", &self.builder.out_ports.to_string())?;
		self.dir.write("buflen", self.builder.buflen.to_string())?;
		self.dir.write("qlen", self.builder.qlen.to_string())?;

        Ok(())
    }
}

/// USB gadget MIDI function.
#[derive(Debug)]
pub struct Midi {
    dir: FunctionDir,
}

impl Midi {
    /// Creates a new USB gadget MIDI builder.
    pub fn builder() -> MidiBuilder {
        MidiBuilder { id: "Midi USB Gadget".to_string(), in_ports: 1, out_ports: 1, buflen: 512, qlen: 32, index: -1 }
    }

    /// Access to registration status.
    pub fn status(&self) -> Status {
        self.dir.status()
    }

	// Add some way to return the devices + ports created by function -> f_midi creates two separate devices for each /dev/snd/mi* / + /dev/mi*
}
