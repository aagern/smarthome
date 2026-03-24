#![allow(unused)]
use anyhow::{Context, Result};
use tracing::{debug, error, info, warn};

#[cfg(not(feature = "mock"))]
mod real;
#[cfg(not(feature = "mock"))]
use real::SmartSocket;
#[cfg(not(feature = "mock"))]
pub use real::SmartThermometer;

#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
use mock::SmartSocket;
#[cfg(feature = "mock")]
pub use mock::SmartThermometer;

enum SmartDevice {
    SmartThermometer(SmartThermometer),
    SmartSocket(SmartSocket),
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
