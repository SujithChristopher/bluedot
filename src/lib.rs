use godot::prelude::*;

mod bluedot;
mod ble_device;

use bluedot::BlueDot;
use ble_device::BLEDevice;

struct BlueDotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BlueDotExtension {}
