#![allow(unused)]
use anyhow::{Context, Result};
use home::{SmartDevice, SmartSocket, SmartThermometer};
use logger::setup_tracing;
use tracing::{debug, error, info, warn};

fn main() -> Result<()> {
    setup_tracing(); // common logger init
    debug!("Logger initialized. App started.");

    debug!("Device init.");
    let thermo = SmartThermometer::default();
    let socket = SmartSocket::default();

    let devices = [
        SmartDevice::Thermometer(thermo),
        SmartDevice::Socket(socket),
    ];

    info!("Device check.");
    for (i, device) in devices.iter().enumerate() {
        println!("Device #{}", i + 1);
        device.print_state();
    }

    debug!("App finished.");
    Ok(())
}
