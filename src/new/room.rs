#![allow(unused)]
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
}
