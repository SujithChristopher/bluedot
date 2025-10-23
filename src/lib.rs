use godot::prelude::*;

mod gdble;
mod ble_device;

struct GdBLEExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GdBLEExtension {}
