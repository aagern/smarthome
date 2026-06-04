use crate::{InputError, SmartDevice};
use std::collections::HashMap;
use std::fmt;
use tracing::debug;

/// Комната, хранящая устройства в HashMap по строковым ключам.
pub struct Room {
    devices: HashMap<String, SmartDevice>,
}

/// Методы конструирования и управления устройствами в комнате.
impl Room {
    /// Создаёт комнату с одним начальным устройством.
    /// # Параметры
    /// * `device_name` — имя устройства.
    /// * `device` — устройство для добавления в комнату.
    pub fn new(device_name: String, device: SmartDevice) -> Self {
        let mut devices = HashMap::new();
        devices.insert(device_name, device);
        Room { devices }
    }

    /// Создаёт комнату из вектора устройств. Возвращает ошибку, если вектор пустой.
    /// # Параметры
    /// * `devices` — вектор пар (имя, устройство).
    /// # Errors
    /// Возвращает `InputError::DataEmpty`, если входной вектор пуст.
    pub fn try_from_vec(devices: Vec<(String, SmartDevice)>) -> Result<Self, InputError> {
        Ok(Room {
            devices: devices.into_iter().collect(),
        })
    }

    /// Добавляет устройство в комнату. Возвращает предыдущее устройство с таким именем, если оно существовало.
    /// # Параметры
    /// * `name` — имя устройства.
    /// * `device` — устройство для добавления.
    /// # Возвращаемое значение
    /// Предыдущее устройство с таким именем, если оно существовало, иначе `None`.
    pub fn add_device(&mut self, name: String, device: SmartDevice) -> Option<SmartDevice> {
        self.devices.insert(name, device)
    }

    /// Удаляет устройство из комнаты. Возвращает удалённое устройство или ошибку, если устройство единственное.
    /// # Параметры
    /// * `name` — имя устройства для удаления.
    /// # Errors
    /// Возвращает `InputError::DataEmpty`, если устройство единственное.
    /// Возвращает `InputError::DeviceNotFound`, если устройство не найдено.
    pub fn remove_device(&mut self, name: String) -> Result<SmartDevice, InputError> {
        if self.devices.len() == 1 && self.devices.contains_key(&name) {
            return Err(InputError::DataEmpty);
        }
        self.devices
            .remove(&name)
            .ok_or(InputError::DeviceNotFound(name))
    }

    /// Возвращает неизменяемую ссылку на устройство по имени.
    /// # Параметры
    /// * `name` — имя устройства.
    /// # Возвращаемое значение
    /// Ссылку на устройство, если устройство с указанным именем существует, иначе `None`.
    pub fn get_device(&self, name: String) -> Option<&SmartDevice> {
        self.devices.get(&name)
    }

    /// Возвращает изменяемую ссылку на устройство по имени.
    /// # Параметры
    /// * `name` — имя устройства.
    /// # Возвращаемое значение
    /// Ссылку на устройство, если устройство с указанным именем существует, иначе `None`.
    pub fn get_device_mut(&mut self, name: String) -> Option<&mut SmartDevice> {
        self.devices.get_mut(&name)
    }

    /// Проверяет, содержит ли комната устройство с указанным именем.
    /// # Параметры
    /// * `name` — имя устройства для проверки.
    /// # Возвращаемое значение
    /// `true`, если комната содержит устройство с указанным именем, `false` иначе.
    pub fn has_device(&self, name: &str) -> bool {
        self.devices.contains_key(name)
    }

    /// Возвращает количество устройств в комнате.
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Итератор по всем устройствам комнаты.
    /// # Возвращаемое значение
    /// Итератор, выдающий пары `(имя_устройства, ссылка_на_устройство)`.
    pub fn iter_devices(&self) -> impl Iterator<Item = (&String, &SmartDevice)> {
        self.devices.iter()
    }
}

/// Форматирование Room для вывода в виде строки.
impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "==== Room Report ====")?;
        for (name, device) in &self.devices {
            writeln!(f, "Name: {} | Device: {}", name, device)?;
        }
        debug!("Creted a report for a room.");
        Ok(())
    }
}

/// Форматирование Room для отладочного вывода.
impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug!("Room {{ devices: {:?} }}", self.devices);
        write!(f, "Room {{ devices: {:?} }}", self.devices)?;
        Ok(())
    }
}
