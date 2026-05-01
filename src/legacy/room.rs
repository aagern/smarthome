use super::non_empty_vec::NonEmptyVec;
use crate::{DeviceId, InputError, SmartDevice};
use std::fmt;
use tracing::debug;

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

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "== Устройства комнаты ==")?;
        for (index, device) in self.devices.iter().enumerate() {
            writeln!(f, "Устройство #{}", index + 1)?;
            write!(f, "{}", device)?;
            if index < self.devices.len() - 1 {
                writeln!(f)?;
            }
        }
        debug!("Отчёт о комнате создан.");
        Ok(())
    }
}

impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug!("Room {{ devices: {:?} }}", self.devices);
        write!(f, "Room {{ devices: {:?} }}", self.devices)
    }
}
