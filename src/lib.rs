use std::fmt;
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Room not found {0}")]
    RoomNotFound(String), // TODO Usage!
    #[error("Data cannot be empty!")]
    DataEmpty,
}

// Реальные показания с датчиков
#[cfg(not(feature = "mock"))]
mod real;
#[cfg(not(feature = "mock"))]
pub use real::SmartSocket;
#[cfg(not(feature = "mock"))]
pub use real::SmartThermometer;

// Случайные числа в качестве показаний
#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use mock::{SmartSocket, SmartThermometer};

/// Идентификатор комнаты
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub usize);

/// Идентификатор устройства
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub usize);

impl From<usize> for DeviceId {
    /// Создаёт DeviceId из целого числа.
    /// # Параметры
    /// * idx - целое число, которое будет использоваться как идентификатор устройства.
    fn from(idx: usize) -> Self {
        DeviceId(idx)
    }
}

impl From<DeviceId> for usize {
    /// Создаёт целое число из DeviceId.
    /// # Параметры
    /// * id - идентификатор устройства.
    /// # Возвращаемое значение
    /// Целое число = идентификатор устройства.
    fn from(id: DeviceId) -> Self {
        id.0
    }
}

impl From<usize> for RoomId {
    /// Создаёт RoomId из целого числа.
    /// # Параметры
    /// * idx - целое число = идентификатор комнаты.
    fn from(idx: usize) -> Self {
        RoomId(idx)
    }
}

#[derive(PartialEq)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl fmt::Display for SmartDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmartDevice::Thermometer(t) => {
                write!(
                    f,
                    "- Термометр -\nТекущая температура: {:.1}°C\nСтатус: Активен",
                    t.get_current_temperature()
                )
            }
            SmartDevice::Socket(s) => {
                write!(
                    f,
                    "- Розетка -\nТекущая мощность: {:.1}Вт.\nСтатус: {}",
                    s.get_current_power(),
                    if s.is_on() { "Вкл." } else { "Выкл." }
                )
            }
        }
    }
}

impl fmt::Debug for SmartDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmartDevice::Thermometer(t) => {
                debug!("SmartDevice::Thermometer = {:?}", t);
                write!(f, "T: {:?}", t)
            }
            SmartDevice::Socket(s) => {
                debug!("SmartDevice::Socket = {:?}", s);
                write!(f, "S: {:?}", s)
            }
        }
    }
}

pub mod legacy;
pub mod new;

pub use legacy::{House, Room};
// pub use new::{Room, House}; // переключить на новую реализацию, когда готово

#[cfg(test)]
mod tests {
    use super::*;

    /// Проверка, что температура в комнате лежит в допустимом диапазоне.
    /// Тест слабый, так как зависит от рандома.
    #[test]
    fn test_get_current_temperature() {
        let smart_thermometer = SmartThermometer::default();
        assert!(smart_thermometer.get_current_temperature() >= -30.0);
        assert!(smart_thermometer.get_current_temperature() <= 50.0);
    }

    /// Проверка, что мощность в комнате лежит в допустимом диапазоне.
    /// Тест слабый, так как зависит от рандома.
    #[test]
    fn test_get_current_power() {
        let smart_socket = SmartSocket::default();
        assert!(smart_socket.get_current_power() >= 0.0);
        assert!(smart_socket.get_current_power() <= 2000.0);
    }

    /// Проверка, что get_device возвращает Option<SmartDevice> с указанным устройством, если индекс в пределах длины вектора устройств комнаты, иначе возвращает None.
    #[test]
    fn test_get_device() {
        let thermo = SmartThermometer::default();
        let socket = SmartSocket::default();

        let devices = vec![
            SmartDevice::Thermometer(thermo),
            SmartDevice::Socket(socket),
        ];
        let room = Room::try_from_vec(devices).unwrap();

        let last_id = room.last_device_id();
        let out_of_range_id = DeviceId(room.devices.len() + 1);
        assert!(room.has_device(last_id));
        assert!(!room.has_device(out_of_range_id));
        let test_none = room.get_device(out_of_range_id);
        assert!(test_none.is_none());
        let test_some = room.get_device(last_id);
        assert!(test_some.is_some());
    }

    /// Проверка, что get_device_mut возвращает Option<&mut SmartDevice> с указанным устройством, если индекс в пределах длины вектора устройств комнаты, иначе возвращает None.
    #[test]
    fn test_get_device_mut() {
        let thermo = SmartThermometer::default();
        let socket = SmartSocket::default();

        let devices = vec![
            SmartDevice::Thermometer(thermo),
            SmartDevice::Socket(socket),
        ];
        let mut room = Room::try_from_vec(devices).unwrap();
        let last_id = room.last_device_id();
        let out_of_range_id = DeviceId(room.devices.len() + 1);
        let test_none = room.try_get_device_mut(out_of_range_id);
        assert!(test_none.is_err());
        let test_some = room.try_get_device_mut(last_id);
        assert!(test_some.is_ok());
    }

    /// Проверка, что try_from_vec для Room возвращает Result<Room, String> с комнатой,
    /// если входной вектор не пустой, иначе возвращает ошибку с сообщением
    /// "В комнате должно быть минимум одно устройство!".
    #[test]
    fn test_try_from_vec_devices() {
        let thermo = SmartThermometer::default();
        let socket = SmartSocket::default();
        let devices = vec![
            SmartDevice::Thermometer(thermo),
            SmartDevice::Socket(socket),
        ];
        let check = devices.len();
        let room = Room::try_from_vec(devices).unwrap();
        assert_eq!(room.devices.len(), check);
    }

    /// Проверка, что try_from_vec для House возвращает Result<House, String> с домом,
    /// если входной вектор не пустой, иначе возвращает ошибку с сообщением
    /// "В доме должна быть минимум одна комната!".
    #[test]
    fn test_try_from_vec_rooms() {
        let thermo = SmartThermometer::default();
        let socket = SmartSocket::default();
        let devices = vec![
            SmartDevice::Thermometer(thermo),
            SmartDevice::Socket(socket),
        ];
        let rooms = vec![Room::try_from_vec(devices).unwrap()];
        let check = rooms.len();
        let house = House::try_from_vec(rooms).unwrap();
        assert_eq!(house.rooms.len(), check);
    }

    /// Проверка, что get_room возвращает Option<&Room> с комнатой,
    /// если индекс в пределах длины вектора комнат дома, иначе возвращает None.
    #[test]
    fn test_get_room() {
        let thermo = SmartThermometer::default();
        let socket = SmartSocket::default();
        let devices = vec![
            SmartDevice::Thermometer(thermo),
            SmartDevice::Socket(socket),
        ];
        let rooms = vec![Room::try_from_vec(devices).unwrap()];
        let house = House::try_from_vec(rooms).unwrap();
        let last_room_id = house.last_room_id();
        let invalid_room_id = RoomId(house.rooms.len() + 1);
        assert!(house.has_room(last_room_id));
        assert!(!house.has_room(invalid_room_id));
        let test_some = house.try_get_room(last_room_id);
        assert!(test_some.is_ok());
        let test_none = house.try_get_room(invalid_room_id);
        assert!(test_none.is_err());
    }

    /// Проверка, что get_room_mut возвращает Option<&mut Room> с комнатой,
    /// если индекс в пределах длины вектора комнат дома, иначе возвращает None.
    #[test]
    fn test_get_room_mut() {
        let thermo = SmartThermometer::default();
        let socket = SmartSocket::default();
        let devices = vec![
            SmartDevice::Thermometer(thermo),
            SmartDevice::Socket(socket),
        ];
        let rooms = vec![Room::try_from_vec(devices).unwrap()];
        let mut house = House::try_from_vec(rooms).unwrap();
        let last_room_id = house.last_room_id();
        let invalid_room_id = RoomId(house.rooms.len() + 1);
        let test_some = house.try_get_room_mut(last_room_id);
        assert!(test_some.is_ok());
        let test_none = house.try_get_room_mut(invalid_room_id);
        assert!(test_none.is_err());
    }
}
