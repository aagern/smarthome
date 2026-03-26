#![allow(unused)]
use anyhow::{Context, Result};
use std::ops::{Index, IndexMut};
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
pub use mock::SmartSocket;
#[cfg(feature = "mock")]
pub use mock::SmartThermometer;

/// Тип непустой вектор:
/// для исключения валидации.
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

/// Реализация методов непустого вектора:
/// функционал вектора.
impl<T> NonEmptyVec<T> {
    /// Создаёт NonEmptyVec из элемента first.
    /// В поле rest будет создан пустой вектор.
    pub fn new(first: T) -> Self {
        NonEmptyVec {
            first,
            rest: Vec::new(),
        }
    }

    /// Создаёт NonEmptyVec из вектора.
    /// Если вектор пустой, функция возвращает None.
    /// Если вектор не пустой, функция возвращает NonEmptyVec,
    /// содержащий первый элемент вектора в поле first,
    /// а остальные элементы вектора в поле rest.
    pub fn from_vec(mut vec: Vec<T>) -> Option<Self> {
        if vec.is_empty() {
            None
        } else {
            let first = vec.remove(0);
            Some(NonEmptyVec { first, rest: vec })
        }
    }

    /// Добавка элемента в вектор
    pub fn push(&mut self, value: T) {
        self.rest.push(value);
    }

    /// Получить длину вектора (с учётом 1 объекта)
    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    /// Clippy требует наличие is_empty в пару к len()
    pub fn is_empty(&self) -> bool {
        false // Всегда false!
    }

    /// Возвращает ссылку на элемент вектора по индексу.
    /// # Паника
    /// Если индекс за пределами вектора, функция возвращает None.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            Some(&self.first)
        } else {
            self.rest.get(index - 1)
        }
    }

    /// Возвращает **изменяемую** ссылку на элемент вектора по индексу.
    /// # Паника
    /// Если индекс за пределами вектора, функция возвращает None.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            Some(&mut self.first)
        } else {
            self.rest.get_mut(index - 1)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }
}

impl<T> Index<usize> for NonEmptyVec<T> {
    type Output = T;
    /// Возвращает **неизменяемую** ссылку на элемент вектора по индексу.
    /// # Паника
    /// Если индекс за пределами вектора, функция возвращает panic.
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Индекс за пределами вектора!")
    }
}

impl<T> IndexMut<usize> for NonEmptyVec<T> {
    /// Возвращает **изменяемую** ссылку на элемент вектора по индексу.
    /// # Паника
    /// Если индекс за пределами вектора, функция возвращает panic.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("Индекс за пределами вектора!")
    }
}

#[derive(PartialEq)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl SmartDevice {
    pub fn print_state(&self) {
        match self {
            SmartDevice::Thermometer(termo) => {
                println!("- Термометр -");
                println!(
                    "Текущая температура: {:.1}°C",
                    termo.get_current_temperature()
                );
                println!("Статус: Активен")
            }
            SmartDevice::Socket(socket) => {
                println!("- Розетка -");
                println!("Текущая мощность: {:.1}Вт.", socket.get_current_power());
                println!(
                    "Статус: {}",
                    if socket.is_on() {
                        "Вкл."
                    } else {
                        "Выкл."
                    }
                )
            }
        }
    }
}

pub struct Room {
    devices: NonEmptyVec<SmartDevice>,
}

impl Room {
    /// Конструктор, принимающий **непустой** массив устройств
    pub fn new(first_device: SmartDevice, other_devices: Vec<SmartDevice>) -> Self {
        let mut devices = NonEmptyVec::new(first_device);
        for device in other_devices {
            devices.push(device);
        }
        Room { devices }
    }

    /// Создаёт Room из вектора устройств.
    /// # Паника
    /// Если входной вектор пустой, функция возвращает ошибку с сообщением
    /// "В комнате должно быть минимум одно устройство!".
    pub fn try_from_vec(devices: Vec<SmartDevice>) -> Result<Self, String> {
        NonEmptyVec::from_vec(devices)
            .map(|devices| Room { devices })
            .ok_or_else(|| "В комнате должно быть минимум одно устройство!".to_string())
    }

    /// Получить ссылку на устройство по индексу
    pub fn get_device(&self, index: usize) -> Option<&SmartDevice> {
        self.devices.get(index)
    }

    /// Получить изменяемую ссылку на устройство по индексу
    pub fn get_device_mut(&mut self, index: usize) -> Option<&mut SmartDevice> {
        self.devices.get_mut(index)
    }

    /// Выводить в стандартный вывод отчёт о всех устройствах в комнате
    pub fn print_room_devices(&self) {
        println!("== Устройства комнаты ==");
        for (index, device) in self.devices.iter().enumerate() {
            println!("Устройство #{}", index + 1);
            device.print_state();
        }
    }
}

pub struct House {
    rooms: NonEmptyVec<Room>,
}

impl House {
    /// Конструктор, принимающий **непустой** массив комнат
    pub fn new(first_room: Room, other_rooms: Vec<Room>) -> Self {
        let mut rooms = NonEmptyVec::new(first_room);
        for room in other_rooms {
            rooms.push(room);
        }
        House { rooms }
    }

    /// Создаёт House из вектора комнат.
    /// # Паника
    /// Если входной вектор пустой, функция возвращает ошибку с сообщением
    /// "В доме должна быть минимум одна комната!".
    pub fn try_from_vec(rooms: Vec<Room>) -> Result<Self, String> {
        NonEmptyVec::from_vec(rooms)
            .map(|rooms| House { rooms })
            .ok_or_else(|| "В доме должна быть минимум одна комната!".to_string())
    }

    /// Получить ссылку на комнату по индексу
    pub fn get_room(&self, index: usize) -> Option<&Room> {
        self.rooms.get(index)
    }

    /// Получить изменяемую ссылку на комнату по индексу
    pub fn get_room_mut(&mut self, index: usize) -> Option<&mut Room> {
        self.rooms.get_mut(index)
    }

    pub fn print_report(&self) {
        println!("== Отчёт о доме ==");
        for (index, room) in self.rooms.iter().enumerate() {
            println!("Комната #{}", index + 1);
            room.print_room_devices();
        }
    }
}

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
        let max_length = devices.len();
        let room = Room::try_from_vec(devices).unwrap();
        let test_some = room.get_device(max_length - 1);
        assert!(test_some.is_some());
        let test_none = room.get_device(max_length + 1);
        assert!(test_none.is_none());
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
        let max_length = devices.len();
        let mut room = Room::try_from_vec(devices).unwrap();
        let test_some = room.get_device_mut(max_length - 1);
        assert!(test_some.is_some());
        let test_none = room.get_device_mut(max_length + 1);
        assert!(test_none.is_none());
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
        let max_length = rooms.len();
        let house = House::try_from_vec(rooms).unwrap();
        let test_some = house.get_room(max_length - 1);
        assert!(test_some.is_some());
        let test_none = house.get_room(max_length + 1);
        assert!(test_none.is_none());
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
        let max_length = rooms.len();
        let mut house = House::try_from_vec(rooms).unwrap();
        let test_some = house.get_room_mut(max_length - 1);
        assert!(test_some.is_some());
        let test_none = house.get_room_mut(max_length + 1);
        assert!(test_none.is_none());
    }
}
