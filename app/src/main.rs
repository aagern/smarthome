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

    let devices = vec![
        SmartDevice::Thermometer(thermo),
        SmartDevice::Socket(socket),
    ];

    let room = home::Room::new(devices);

    info!("Room check.");
    room.print_room_devices();

    debug!("App finished.");
    Ok(())
}
