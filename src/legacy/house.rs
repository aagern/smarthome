use super::non_empty_vec::NonEmptyVec;
use super::room::Room;
use crate::{InputError, RoomId};
use std::fmt;
use tracing::debug;

pub struct House {
    pub rooms: NonEmptyVec<Room>,
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

impl fmt::Display for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "== Отчёт о доме ==")?;
        for (index, room) in self.rooms.iter().enumerate() {
            writeln!(f, "Комната #{}", index + 1)?;
            write!(f, "{}", room)?;
            if index < self.rooms.len() - 1 {
                writeln!(f)?;
            }
        }
        debug!("Отчёт о доме создан.");
        Ok(())
    }
}

impl fmt::Debug for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug!("House {{ rooms: {:?} }}", self.rooms);
        write!(f, "House {{ rooms: {:?} }}", self.rooms)
    }
}
