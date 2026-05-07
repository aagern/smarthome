use crate::{InputError, SmartDevice};
use std::collections::HashMap;
use std::fmt;
use tracing::debug;

// Room implemetation with HashMap
pub struct Room {
    devices: HashMap<String, SmartDevice>,
}

// Room constructor with 1 device
impl Room {
    pub fn new(device_name: String, device: SmartDevice) -> Self {
        let mut devices = HashMap::new();
        devices.insert(device_name, device);
        Room { devices }
    }

    // Constructor from vector of devices for fast init. Returns error if vector is empty
    pub fn try_from_vec(devices: Vec<(String, SmartDevice)>) -> Result<Self, InputError> {
        Ok(Room {
            devices: devices.into_iter().collect(),
        })
    }

    // Add device
    pub fn add_device(&mut self, name: String, device: SmartDevice) -> Option<SmartDevice> {
        self.devices.insert(name, device)
    }

    // Remove device. Returns removed device. If there is only 1 device, returns error
    pub fn remove_device(&mut self, name: String) -> Result<SmartDevice, InputError> {
        if self.devices.len() == 1 && self.devices.contains_key(&name) {
            return Err(InputError::DataEmpty);
        }
        self.devices
            .remove(&name)
            .ok_or(InputError::DeviceNotFound(name))
    }

    // Get immutable link to device
    pub fn get_device(&self, name: String) -> Option<&SmartDevice> {
        self.devices.get(&name)
    }

    // Get mutable link to device
    pub fn get_device_mut(&mut self, name: String) -> Option<&mut SmartDevice> {
        self.devices.get_mut(&name)
    }

    // Check if room has device
    pub fn has_device(&self, name: &str) -> bool {
        self.devices.contains_key(name)
    }

    // Get number of devices
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    // Iterate over devices
    pub fn iter_devices(&self) -> impl Iterator<Item = (&String, &SmartDevice)> {
        self.devices.iter()
    }
}

// Room formatting
impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "==== Room Report ====")?;
        for (name, device) in &self.devices {
            writeln!(f, "Name: {} | Device: {}", name, device)?;
        }
        debug!("Creted a report for a room.");
        Ok(())
    }
}

// Room debug data
impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug!("Room {{ devices: {:?} }}", self.devices);
        write!(f, "Room {{ devices: {:?} }}", self.devices)?;
        Ok(())
    }
}
