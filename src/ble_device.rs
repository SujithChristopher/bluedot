use godot::prelude::*;
use godot::classes::{RefCounted, IRefCounted};
use btleplug::api::{Peripheral as _, WriteType, Characteristic};
use btleplug::platform::Peripheral;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct BLEDevice {
    base: Base<RefCounted>,
    peripheral: Arc<Mutex<Peripheral>>,
    runtime: Arc<Mutex<Option<Runtime>>>,
    name: GString,
    address: GString,
    is_connected: bool,
}

#[godot_api]
impl IRefCounted for BLEDevice {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            peripheral: Arc::new(Mutex::new(unsafe { std::mem::zeroed() })),
            runtime: Arc::new(Mutex::new(None)),
            name: GString::from("Unknown"),
            address: GString::from(""),
            is_connected: false,
        }
    }
}

impl BLEDevice {
    pub fn from_peripheral(base: Base<RefCounted>, peripheral: Peripheral, runtime: Arc<Mutex<Option<Runtime>>>) -> Self {
        let runtime_guard = runtime.lock().unwrap();
        let mut name = GString::from("Unknown");
        let mut address = GString::from("");

        if let Some(rt) = runtime_guard.as_ref() {
            if let Ok(props) = rt.block_on(peripheral.properties()) {
                if let Some(props) = props {
                    if let Some(local_name) = props.local_name {
                        name = GString::from(local_name.as_str());
                    }
                    address = GString::from(props.address.to_string());
                }
            }
        }

        drop(runtime_guard);

        Self {
            base,
            peripheral: Arc::new(Mutex::new(peripheral)),
            runtime,
            name,
            address,
            is_connected: false,
        }
    }
}

#[godot_api]
impl BLEDevice {
    /// Get the device name
    #[func]
    fn get_name(&self) -> GString {
        self.name.clone()
    }

    /// Get the device address (MAC address)
    #[func]
    fn get_address(&self) -> GString {
        self.address.clone()
    }

    /// Connect to this BLE device
    /// Returns true if successful
    #[func]
    fn connect(&mut self) -> bool {
        let runtime_guard = self.runtime.lock().unwrap();
        let peripheral = self.peripheral.lock().unwrap();

        if let Some(rt) = runtime_guard.as_ref() {
            let result = rt.block_on(async {
                peripheral.connect().await
            });

            match result {
                Ok(_) => {
                    godot_print!("BlueDot: Connected to {}", self.name);
                    self.is_connected = true;

                    // Discover services automatically
                    if let Err(e) = rt.block_on(peripheral.discover_services()) {
                        godot_error!("BlueDot: Failed to discover services: {}", e);
                    }

                    true
                }
                Err(e) => {
                    godot_error!("BlueDot: Connection failed: {}", e);
                    false
                }
            }
        } else {
            godot_error!("BlueDot: Runtime not available");
            false
        }
    }

    /// Disconnect from this BLE device
    #[func]
    fn disconnect(&mut self) -> bool {
        let runtime_guard = self.runtime.lock().unwrap();
        let peripheral = self.peripheral.lock().unwrap();

        if let Some(rt) = runtime_guard.as_ref() {
            let result = rt.block_on(async {
                peripheral.disconnect().await
            });

            match result {
                Ok(_) => {
                    godot_print!("BlueDot: Disconnected from {}", self.name);
                    self.is_connected = false;
                    true
                }
                Err(e) => {
                    godot_error!("BlueDot: Disconnection failed: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Check if connected to this device
    #[func]
    fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// Write data to a characteristic
    /// service_uuid: The service UUID (e.g., "0000180f-0000-1000-8000-00805f9b34fb")
    /// characteristic_uuid: The characteristic UUID
    /// data: PackedByteArray of data to write
    /// Returns true if successful
    #[func]
    fn write(&self, service_uuid: GString, characteristic_uuid: GString, data: PackedByteArray) -> bool {
        if !self.is_connected {
            godot_error!("BlueDot: Device not connected");
            return false;
        }

        let service_uuid_str = service_uuid.to_string();
        let char_uuid_str = characteristic_uuid.to_string();

        let service_uuid = match Uuid::parse_str(&service_uuid_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                godot_error!("BlueDot: Invalid service UUID: {}", e);
                return false;
            }
        };

        let char_uuid = match Uuid::parse_str(&char_uuid_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                godot_error!("BlueDot: Invalid characteristic UUID: {}", e);
                return false;
            }
        };

        let runtime_guard = self.runtime.lock().unwrap();
        let peripheral = self.peripheral.lock().unwrap();

        if let Some(rt) = runtime_guard.as_ref() {
            let result = rt.block_on(async {
                let characteristics = peripheral.characteristics();
                let characteristic = characteristics.iter()
                    .find(|c| c.service_uuid == service_uuid && c.uuid == char_uuid);

                if let Some(char) = characteristic {
                    let bytes: Vec<u8> = data.to_vec();
                    peripheral.write(char, &bytes, WriteType::WithoutResponse).await
                } else {
                    Err(btleplug::Error::NotSupported("Characteristic not found".to_string()))
                }
            });

            match result {
                Ok(_) => {
                    godot_print!("BlueDot: Write successful");
                    true
                }
                Err(e) => {
                    godot_error!("BlueDot: Write failed: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Read data from a characteristic
    /// service_uuid: The service UUID
    /// characteristic_uuid: The characteristic UUID
    /// Returns PackedByteArray with the data
    #[func]
    fn read(&self, service_uuid: GString, characteristic_uuid: GString) -> PackedByteArray {
        let mut result = PackedByteArray::new();

        if !self.is_connected {
            godot_error!("BlueDot: Device not connected");
            return result;
        }

        let service_uuid_str = service_uuid.to_string();
        let char_uuid_str = characteristic_uuid.to_string();

        let service_uuid = match Uuid::parse_str(&service_uuid_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                godot_error!("BlueDot: Invalid service UUID: {}", e);
                return result;
            }
        };

        let char_uuid = match Uuid::parse_str(&char_uuid_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                godot_error!("BlueDot: Invalid characteristic UUID: {}", e);
                return result;
            }
        };

        let runtime_guard = self.runtime.lock().unwrap();
        let peripheral = self.peripheral.lock().unwrap();

        if let Some(rt) = runtime_guard.as_ref() {
            let read_result = rt.block_on(async {
                let characteristics = peripheral.characteristics();
                let characteristic = characteristics.iter()
                    .find(|c| c.service_uuid == service_uuid && c.uuid == char_uuid);

                if let Some(char) = characteristic {
                    peripheral.read(char).await
                } else {
                    Err(btleplug::Error::NotSupported("Characteristic not found".to_string()))
                }
            });

            match read_result {
                Ok(data) => {
                    godot_print!("BlueDot: Read {} bytes", data.len());
                    for byte in data {
                        result.push(byte);
                    }
                }
                Err(e) => {
                    godot_error!("BlueDot: Read failed: {}", e);
                }
            }
        }

        result
    }

    /// Get list of available services
    /// Returns an array of service UUIDs as strings
    #[func]
    fn get_services(&self) -> PackedStringArray {
        let mut services = PackedStringArray::new();

        if !self.is_connected {
            godot_error!("BlueDot: Device not connected");
            return services;
        }

        let peripheral = self.peripheral.lock().unwrap();
        let characteristics = peripheral.characteristics();

        let mut seen_services = std::collections::HashSet::new();
        for char in characteristics {
            if seen_services.insert(char.service_uuid) {
                services.push(GString::from(char.service_uuid.to_string()));
            }
        }

        services
    }

    /// Get list of characteristics for a service
    /// service_uuid: The service UUID
    /// Returns an array of characteristic UUIDs as strings
    #[func]
    fn get_characteristics(&self, service_uuid: GString) -> PackedStringArray {
        let mut chars = PackedStringArray::new();

        if !self.is_connected {
            godot_error!("BlueDot: Device not connected");
            return chars;
        }

        let service_uuid_str = service_uuid.to_string();
        let service_uuid = match Uuid::parse_str(&service_uuid_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                godot_error!("BlueDot: Invalid service UUID: {}", e);
                return chars;
            }
        };

        let peripheral = self.peripheral.lock().unwrap();
        let characteristics = peripheral.characteristics();

        for char in characteristics {
            if char.service_uuid == service_uuid {
                chars.push(GString::from(char.uuid.to_string()));
            }
        }

        chars
    }
}
