use godot::prelude::*;

mod bluedot;
mod ble_device;

struct BlueDotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BlueDotExtension {}
