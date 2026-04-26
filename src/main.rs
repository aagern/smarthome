use anyhow::{Result, anyhow};
use lib::{DeviceId, RoomId, SmartDevice, SmartSocket, SmartThermometer};
use logger::setup_tracing;
use tracing::{debug, info, warn};

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
    println!("{}", living_room);
    debug!("Room report created.");

    let bed_room = home::Room::try_from_vec(bed_room_devices)
        .map_err(|err| anyhow!("Комната не создана: {}", err))?;

    info!("Bedroom created.");
    println!("{}", bed_room);
    debug!("Room report created.");

    let mut house = home::House::new(living_room, vec![bed_room]);
    info!("House created.");
    for (i, room) in house.rooms.iter().enumerate() {
        println!("Room #{}: {}", i + 1, room);
    }
    //house.print_report();
    debug!("House report created.");

    info!("Выключение розетки в доме...");
    let bedroom_id = RoomId(1);
    let socket_id = DeviceId(2);

    if let Ok(bed_room) = house.try_get_room_mut(bedroom_id) {
        if let Ok(device) = bed_room.try_get_device_mut(socket_id) {
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

    info!("Включение розетки в доме...");
    let bedroom_id = RoomId(1);
    let socket_id = DeviceId(2);

    if let Ok(bed_room) = house.try_get_room_mut(bedroom_id) {
        if let Ok(device) = bed_room.try_get_device_mut(socket_id) {
            match device {
                SmartDevice::Socket(socket) => {
                    socket.turn_on();
                    info!("Socket in room turned ON!");
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
    for (i, room) in house.rooms.iter().enumerate() {
        println!("Room #{}: {}", i + 1, room);
    }
    //house.print_report();
    debug!("House report created.");

    debug!("App finished.");
    Ok(())
}
