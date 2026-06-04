use smarthouse::new::{House, Room};
use smarthouse::{SmartDevice, SmartSocket, SmartThermometer};

fn main() {
    let thermo = SmartThermometer::default();
    let socket = SmartSocket::default();

    let living_room = Room::try_from_vec(vec![
        ("thermo".to_string(), SmartDevice::Thermometer(thermo)),
        ("socket".to_string(), SmartDevice::Socket(socket)),
    ])
    .unwrap();

    let house = House::new("Test House".to_string(), living_room);
    println!("{}", house);
}
