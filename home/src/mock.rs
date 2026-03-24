use rand::prelude::*;

pub struct SmartThermometer {
    current_temperature: f32,
}

impl Default for SmartThermometer {
    fn default() -> Self {
        SmartThermometer {
            current_temperature: rand::rng().random_range(-30.0..50.0),
        }
    }
}

impl SmartThermometer {
    pub fn get_current_temperature(&self) -> f32 {
        self.current_temperature
    }
    pub fn celsius_to_fahrenheit(&self, celsius: f32) -> f32 {
        celsius * 9.0 / 5.0 + 32.0
    }
}

pub struct SmartSocket {
    is_on: bool,
    power: f32, // watts
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket {
            is_on: true,
            power: rand::rng().random_range(1000.0..2000.0),
        }
    }
}

impl SmartSocket {
    pub fn get_current_power(&self) -> f32 {
        self.power
    }
}
