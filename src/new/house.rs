#![allow(unused)]
use crate::new::room::Room;
use crate::{InputError, SmartDevice};
use std::collections::HashMap;
use std::fmt;
use tracing::debug;

/// Дом, хранящий комнаты в HashMap по строковым ключам.
pub struct House {
    rooms: HashMap<String, Room>,
}

impl House {
    /// Создаёт дом с одной начальной комнатой.
    /// # Параметры
    /// * `room_name` — имя комнаты.
    /// * `room` — комната для добавления в дом.
    pub fn new(room_name: String, room: Room) -> Self {
        let mut rooms = HashMap::new();
        rooms.insert(room_name, room);
        House { rooms }
    }

    /// Создаёт пустой дом без комнат.
    pub fn new_empty() -> Self {
        let rooms = HashMap::new();
        House { rooms }
    }

    /// Добавляет комнату в дом. Возвращает предыдущую комнату с таким именем, если она существовала.
    /// # Параметры
    /// * `room_name` — имя комнаты.
    /// * `room` — комната для добавления.
    /// # Возвращаемое значение
    /// Предыдущую комнату с таким именем, если она существовала, иначе `None`.
    pub fn add_room(&mut self, room_name: String, room: Room) -> Option<Room> {
        self.rooms.insert(room_name, room)
    }

    /// Удаляет комнату из дома. Возвращает удалённую комнату или ошибку, если комната единственная.
    /// # Параметры
    /// * `room_name` — имя комнаты для удаления.
    /// # Errors
    /// Возвращает `InputError::RoomNotFound`, если комната единственная или не найдена.
    pub fn remove_room(&mut self, room_name: &str) -> Result<Room, InputError> {
        if self.rooms.len() == 1 && self.rooms.contains_key(room_name) {
            return Err(InputError::RoomNotFound(room_name.to_string()));
        }
        self.rooms
            .remove(room_name)
            .ok_or(InputError::RoomNotFound(room_name.to_string()))
    }

    /// Возвращает неизменяемую ссылку на комнату по имени.
    /// # Параметры
    /// * `room_name` — имя комнаты.
    /// # Возвращаемое значение
    /// Ссылку на комнату, если комната с указанным именем существует, иначе `None`.
    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    /// Возвращает изменяемую ссылку на комнату по имени.
    /// # Параметры
    /// * `room_name` — имя комнаты.
    /// # Возвращаемое значение
    /// Ссылку на комнату, если комната с указанным именем существует, иначе `None`.
    pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_name)
    }

    /// Возвращает неизменяемую ссылку на устройство в доме по имени комнаты и устройства.
    /// # Параметры
    /// * `room_name` — имя комнаты.
    /// * `device_name` — имя устройства.
    /// # Errors
    /// Возвращает `InputError::RoomNotFound`, если комната не найдена.
    /// Возвращает `InputError::DeviceNotFound`, если устройство не найдено в комнате.
    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDevice, InputError> {
        self.get_room(room_name)
            .ok_or(InputError::RoomNotFound(room_name.to_string()))?
            .get_device(device_name.to_string())
            .ok_or(InputError::DeviceNotFound(device_name.to_string()))
    }

    /// Проверяет, содержит ли дом комнату с указанным именем.
    /// # Параметры
    /// * `room_name` — имя комнаты для проверки.
    /// # Возвращаемое значение
    /// `true`, если дом содержит комнату с указанным именем, `false` иначе.
    pub fn has_room(&self, room_name: &str) -> bool {
        self.rooms.contains_key(room_name)
    }

    /// Возвращает количество комнат в доме.
    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    /// Итератор по всем комнатам дома.
    /// # Возвращаемое значение
    /// Итератор, выдающий пары `(имя_комнаты, ссылка_на_комнату)`.
    pub fn iter_rooms(&self) -> impl Iterator<Item = (&String, &Room)> {
        self.rooms.iter()
    }
}

/// Форматирование House для вывода в виде строки.
impl fmt::Display for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "==== House Report ====")?;
        for (room_name, room) in &self.rooms {
            writeln!(f, "Room name: {} | Room: {}", room_name, room)?;
        }
        debug!("Creted a report for a house.");
        Ok(())
    }
}

/// Форматирование House для отладочного вывода.
impl fmt::Debug for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug!("House {{ rooms: {:?} }}", self.rooms);
        write!(f, "House {{ rooms: {:?} }}", self.rooms)
    }
}
