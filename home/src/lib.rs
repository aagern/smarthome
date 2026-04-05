use std::ops::{Index, IndexMut};
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Device not found!")]
    DeviceNotFound,
    #[error("Room not found!")]
    RoomNotFound, // TODO Usage!
    #[error("Data cannot be empty!")]
    DataEmpty,
}

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

    /// Creates a NonEmptyVec from a vector.
    /// # Errors
    /// If the input vector is empty, returns an error with the message "Vector must not be empty".
    pub fn from_vec(mut vec: Vec<T>) -> Result<Self, InputError> {
        if vec.is_empty() {
            Err(InputError::DataEmpty)
        } else {
            let first = vec.remove(0);
            Ok(NonEmptyVec { first, rest: vec })
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

impl SmartDevice {
    pub fn print_state(&self) {
        match self {
            SmartDevice::Thermometer(termo) => {
                println!("- Термометр -");
                println!(
                    "Текущая температура: {:.1}°C",
                    termo.get_current_temperature()
                );
                println!("Статус: Активен");
                debug!(
                    "Запрос температуры -> текущая температура: {:.1}°C",
                    termo.get_current_temperature()
                );
            }
            SmartDevice::Socket(socket) => {
                println!("- Розетка -");
                println!("Текущая мощность: {:.1}Вт.", socket.get_current_power());
                debug!(
                    "Запрос мощности -> текущая мощность: {:.1}Вт.",
                    socket.get_current_power()
                );
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
    pub fn try_from_vec(devices: Vec<SmartDevice>) -> Result<Self, InputError> {
        NonEmptyVec::from_vec(devices)
            .map(|devices| Room { devices })
            .map_err(|_| InputError::DataEmpty)
    }

    /// Тries to get a device by its id.
    pub fn try_get_device(&self, id: DeviceId) -> Result<&SmartDevice, InputError> {
        debug!("try_get_device {:?}", id);
        self.get_device(id).ok_or(InputError::DeviceNotFound)
    }

    /// Тries to get a mutable reference to a device by its id.
    pub fn try_get_device_mut(&mut self, id: DeviceId) -> Result<&mut SmartDevice, InputError> {
        debug!("try_get_device_mut {:?}", id);
        self.get_device_mut(id).ok_or(InputError::DeviceNotFound)
    }

    /// Получить ссылку на устройство по индексу
    pub fn get_device(&self, id: DeviceId) -> Option<&SmartDevice> {
        self.devices.get(id.0)
    }

    /// Получить изменяемую ссылку на устройство по индексу
    pub fn get_device_mut(&mut self, id: DeviceId) -> Option<&mut SmartDevice> {
        self.devices.get_mut(id.0)
    }

    /// Выводить в стандартный вывод отчёт о всех устройствах в комнате
    pub fn print_room_devices(&self) {
        println!("== Устройства комнаты ==");
        for (index, device) in self.devices.iter().enumerate() {
            println!("Устройство #{}", index + 1);
            device.print_state();
            debug!("Устройство #{} отчёт создан.", index + 1);
        }
        debug!("Отчёт о комнате создан.");
    }

    /// Возвращает DeviceId для первого устройства в комнате.
    pub fn first_device_id(&self) -> DeviceId {
        DeviceId(0)
    }

    /// Возвращает DeviceId для последнего устройства в комнате.
    pub fn last_device_id(&self) -> DeviceId {
        DeviceId(self.devices.len() - 1)
    }

    /// Проверяет, содержит ли комната устройство с указанным идентификатором.
    /// # Параметры
    /// * id - идентификатор устройства.
    /// # Возвращаемое значение
    /// true, если комната содержит устройство с указанным идентификатором, false иначе.
    pub fn has_device(&self, id: DeviceId) -> bool {
        id.0 < self.devices.len()
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
    pub fn try_from_vec(rooms: Vec<Room>) -> Result<Self, InputError> {
        NonEmptyVec::from_vec(rooms)
            .map(|rooms| House { rooms })
            .map_err(|_| InputError::RoomNotFound)
    }

    /// Тries to get a room by its id.
    /// # Параметры
    /// * id - идентификатор комнаты.
    /// # Возвращаемое значение
    /// Result с ссылкой на комнату, если комната с указанным идентификатором существует,
    /// иначе результат с ошибкой "Room not found!"
    pub fn try_get_room(&self, id: RoomId) -> Result<&Room, InputError> {
        debug!("try_get_room {:?}", id);
        self.get_room(id).ok_or(InputError::RoomNotFound)
    }

    /// Тries to get a mutable reference to a room by its id.
    /// # Параметры
    /// * id - идентификатор комнаты.
    /// # Возвращаемое значение
    /// Result с изменяемой ссылкой на комнату, если комната с указанным идентификатором существует,
    /// иначе результат с ошибкой "Room not found!"
    pub fn try_get_room_mut(&mut self, id: RoomId) -> Result<&mut Room, InputError> {
        debug!("try_get_room_mut {:?}", id);
        self.get_room_mut(id).ok_or(InputError::RoomNotFound)
    }

    /// Получить ссылку на комнату по индексу
    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(id.0)
    }

    /// Получить изменяемую ссылку на комнату по индексу
    pub fn get_room_mut(&mut self, id: RoomId) -> Option<&mut Room> {
        self.rooms.get_mut(id.0)
    }

    pub fn print_report(&self) {
        println!("== Отчёт о доме ==");
        for (index, room) in self.rooms.iter().enumerate() {
            println!("Комната #{}", index + 1);
            room.print_room_devices();
            debug!("Отчёт о комнате #{} создан.", index + 1);
        }
        debug!("Отчёт о доме создан.");
    }

    /// Возвращает RoomId для первой комнаты в доме.
    /// # Возвращаемое значение
    /// RoomId с индексом 0.
    pub fn first_room_id(&self) -> RoomId {
        RoomId(0)
    }

    /// Возвращает RoomId для последней комнаты в доме.
    /// # Возвращаемое значение
    /// RoomId с индексом self.rooms.len() - 1.
    pub fn last_room_id(&self) -> RoomId {
        RoomId(self.rooms.len() - 1)
    }

    /// Проверяет, содержит ли дом комнату с указанным идентификатором.
    /// # Параметры
    /// * id - идентификатор комнаты.
    /// # Возвращаемое значение
    /// true, если дом содержит комнату с указанным идентификатором, false иначе.
    pub fn has_room(&self, id: RoomId) -> bool {
        id.0 < self.rooms.len()
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
        let max_length = devices.len();
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
        let max_length = rooms.len();
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
        let max_length = rooms.len();
        let mut house = House::try_from_vec(rooms).unwrap();
        let last_room_id = house.last_room_id();
        let invalid_room_id = RoomId(house.rooms.len() + 1);
        let test_some = house.try_get_room_mut(last_room_id);
        assert!(test_some.is_ok());
        let test_none = house.try_get_room_mut(invalid_room_id);
        assert!(test_none.is_err());
    }
}
