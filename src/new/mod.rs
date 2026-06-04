//! Модуль с новой HashMap-реализацией для `House` и `Room`.
//!
//! Использует строковые ключи вместо числовых индексов для доступа к комнатам и устройствам.
//! Поддерживает динамическое добавление/удаление комнат и устройств.

mod house;
mod room;

// Data types based on HashMap
pub use house::House;
pub use room::Room;
