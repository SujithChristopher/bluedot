use godot::prelude::*;
use godot::classes::{RefCounted, IRefCounted};
use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Manager;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

use crate::ble_device::BLEDevice;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GdBLE {
    base: Base<RefCounted>,
    runtime: Arc<Mutex<Option<Runtime>>>,
    manager: Arc<Mutex<Option<Manager>>>,
}

#[godot_api]
impl IRefCounted for GdBLE {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            runtime: Arc::new(Mutex::new(None)),
            manager: Arc::new(Mutex::new(None)),
        }
    }
}

#[godot_api]
impl GdBLE {
    /// Initialize the Bluetooth LE manager
    /// Returns true if successful
    #[func]
    fn initialize(&mut self) -> bool {
        let rt = Runtime::new();
        match rt {
            Ok(runtime) => {
                let manager_result = runtime.block_on(async {
                    Manager::new().await
                });

                match manager_result {
                    Ok(manager) => {
                        *self.runtime.lock().unwrap() = Some(runtime);
                        *self.manager.lock().unwrap() = Some(manager);
                        godot_print!("GdBLE: Bluetooth LE initialized successfully");
                        true
                    }
                    Err(e) => {
                        godot_error!("GdBLE: Failed to create BLE manager: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                godot_error!("GdBLE: Failed to create runtime: {}", e);
                false
            }
        }
    }

    /// Scan for nearby Bluetooth LE devices
    /// timeout_seconds: How long to scan (default: 5 seconds)
    /// Returns an array of BLEDevice objects
    #[func]
    fn scan(&self, timeout_seconds: f32) -> Array<Gd<BLEDevice>> {
        let timeout = if timeout_seconds <= 0.0 { 5.0 } else { timeout_seconds };
        let mut devices = Array::new();

        let runtime_guard = self.runtime.lock().unwrap();
        let manager_guard = self.manager.lock().unwrap();

        if let (Some(runtime), Some(manager)) = (runtime_guard.as_ref(), manager_guard.as_ref()) {
            let scan_result = runtime.block_on(async {
                let adapters = manager.adapters().await?;
                if adapters.is_empty() {
                    godot_error!("GdBLE: No Bluetooth adapters found");
                    return Ok::<Vec<btleplug::platform::Peripheral>, btleplug::Error>(Vec::new());
                }

                let central = &adapters[0];
                godot_print!("GdBLE: Starting scan for {} seconds...", timeout);

                central.start_scan(ScanFilter::default()).await?;
                tokio::time::sleep(tokio::time::Duration::from_secs_f32(timeout)).await;
                central.stop_scan().await?;

                let peripherals = central.peripherals().await?;
                Ok(peripherals)
            });

            match scan_result {
                Ok(peripherals) => {
                    godot_print!("GdBLE: Found {} devices", peripherals.len());
                    for peripheral in peripherals {
                        let device = Gd::from_init_fn(|base| {
                            BLEDevice::from_peripheral(base, peripheral, self.runtime.clone())
                        });
                        devices.push(&device);
                    }
                }
                Err(e) => {
                    godot_error!("GdBLE: Scan failed: {}", e);
                }
            }
        } else {
            godot_error!("GdBLE: Not initialized. Call initialize() first.");
        }

        devices
    }

    /// Check if GdBLE is initialized and ready to use
    #[func]
    fn is_initialized(&self) -> bool {
        let runtime_guard = self.runtime.lock().unwrap();
        let manager_guard = self.manager.lock().unwrap();
        runtime_guard.is_some() && manager_guard.is_some()
    }
}
