const TEMPERATURE: f32 = 36.6;
const POWER: f32 = 2000.0;

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

pub struct SmartSocket {
    is_on: bool,
    power: f32, // watts
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket {
            is_on: true,
            power: POWER,
        }
    }
}

impl SmartSocket {
    pub fn get_current_power(&self) -> f32 {
        self.power
    }
}
