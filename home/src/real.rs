const TEMPERATURE: f32 = 36.6;

pub struct SmartThermometer {
    current_temperature: f32,
}

impl Default for SmartThermometer {
    fn default() -> Self {
        SmartThermometer {
            current_temperature: TEMPERATURE,
        }
    }
}

impl SmartThermometer {
    pub fn get_current_temperature(&self) -> f32 {
        self.current_temperature
    }
}
