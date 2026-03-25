#![allow(unused)]
use anyhow::{Context, Result};
use tracing::{debug, error, info, warn};

// Реальные показания с датчиков
#[cfg(not(feature = "mock"))]
mod real;
#[cfg(not(feature = "mock"))]
use real::SmartSocket;
#[cfg(not(feature = "mock"))]
pub use real::SmartThermometer;

// Случайные числа в качестве показаний
#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
use mock::SmartSocket;
#[cfg(feature = "mock")]
pub use mock::SmartThermometer;

pub enum SmartDevice {
    SmartThermometer(SmartThermometer),
    SmartSocket(SmartSocket),
}

pub struct Room {
    devices: Vec<SmartDevice>,
}

impl Room {
    /// Конструктор, принимающий массив устройств
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        Room { devices }
    }

    /// Получить ссылку на устройство по индексу
    pub fn get_device(&self, index: usize) -> Option<&SmartDevice> {
        self.devices.get(index)
    }

    /// Получить изменяемую ссылку на устройство по индексу
    pub fn get_device_mut(&mut self, index: usize) -> Option<&mut SmartDevice> {
        self.devices.get_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_temperature() {
        let smart_thermometer = SmartThermometer::default();
        assert!(smart_thermometer.get_current_temperature() >= -30.0);
        assert!(smart_thermometer.get_current_temperature() <= 50.0);
    }

    #[test]
    fn test_get_current_power() {
        let smart_socket = SmartSocket::default();
        assert!(smart_socket.get_current_power() >= 0.0);
        assert!(smart_socket.get_current_power() <= 2000.0);
    }
}
