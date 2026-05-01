use crate::InputError;
use std::ops::{Index, IndexMut};

/// Тип непустой вектор:
/// для исключения валидации.
#[derive(Debug)]
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
