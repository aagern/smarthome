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
}
