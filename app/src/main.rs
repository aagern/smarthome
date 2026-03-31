#![allow(unused)]
use anyhow::{Context, Result, anyhow};
use home::{DeviceId, RoomId, SmartDevice, SmartSocket, SmartThermometer};
use logger::setup_tracing;
use tracing::{debug, error, info, warn};

fn main() -> Result<()> {
    setup_tracing(); // common logger init
    debug!("Logger initialized. App started.");

    debug!("Device init.");
    let thermo1 = SmartThermometer::default();
    let thermo2 = SmartThermometer::default();
    let thermo3 = SmartThermometer::default();
    let socket1 = SmartSocket::default();
    let socket2 = SmartSocket::default();

    let living_room_devices = vec![
        SmartDevice::Thermometer(thermo1),
        SmartDevice::Socket(socket1),
    ];

    let bed_room_devices = vec![
        SmartDevice::Thermometer(thermo2),
        SmartDevice::Thermometer(thermo3),
        SmartDevice::Socket(socket2),
    ];

    let living_room = home::Room::try_from_vec(living_room_devices)
        .map_err(|err| anyhow!("Комната не создана: {}", err))?;

    info!("Living Room created.");
    living_room.print_room_devices();
    debug!("Room report created.");

    let bed_room = home::Room::try_from_vec(bed_room_devices)
        .map_err(|err| anyhow!("Комната не создана: {}", err))?;

    info!("Bedroom created.");
    bed_room.print_room_devices();
    debug!("Room report created.");

    let mut house = home::House::new(living_room, vec![bed_room]);
    info!("House created.");
    house.print_report();
    debug!("House report created.");

    info!("Выключение розетки в доме...");
    let bedroom_id = RoomId(1);
    let socket_id = DeviceId(2);

    if let Some(bed_room) = house.get_room_mut(bedroom_id) {
        if let Some(device) = bed_room.get_device_mut(socket_id) {
            match device {
                SmartDevice::Socket(socket) => {
                    socket.turn_off();
                    info!("Socket in room turned OFF!");
                    debug!(
                        "Текущая мощность розетки: {}Вт.",
                        socket.get_current_power()
                    );
                }
                SmartDevice::Thermometer(_) => {
                    warn!("Thermometer found, but socket expected at index");
                }
            }
        } else {
            warn!("Device not found by id {:?} in bedroom", socket_id);
        }
    } else {
        warn!("Bedroom not found at id {:?}", bedroom_id);
    }

    println!(" ==== Обновлённый отчёт по дому ====");
    house.print_report();
    debug!("House report created.");

    debug!("App finished.");
    Ok(())
}
