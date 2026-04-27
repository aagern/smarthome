const TEMPERATURE: f32 = 36.6;
const POWER: f32 = 2000.0;

#[derive(PartialEq)]
pub struct SmartThermometer {
    current_temperature: f32,
}

impl Default for SmartThermometer {
    /// Creates a new SmartThermometer with a default current temperature of 36.6.
    fn default() -> Self {
        Self::new(TEMPERATURE)
    }
}

impl SmartThermometer {
    /// Creates a new SmartThermometer with the specified initial temperature in Celsius.
    pub fn new(initial_temp: f32) -> Self {
        SmartThermometer {
            current_temperature: initial_temp,
        }
    }
    /// Returns the current temperature of the thermometer in Celsius.
    pub fn get_current_temperature(&self) -> f32 {
        self.current_temperature
    }
    /// Converts a temperature in Celsius to Fahrenheit.
    /// # Parameters
    /// * celsius - the temperature in Celsius to convert.
    /// # Returns
    /// The temperature in Fahrenheit.
    pub fn celsius_to_fahrenheit(&self, celsius: f32) -> f32 {
        celsius * 9.0 / 5.0 + 32.0
    }
}

#[derive(PartialEq)]
pub enum SocketState {
    On { power: f32 },
    Off,
}

#[derive(PartialEq)]
pub struct SmartSocket {
    state: SocketState,
}

impl Default for SmartSocket {
    /// Creates a new SmartSocket with default values: is_on = true, power = 2000.0.
    fn default() -> Self {
        Self::new(POWER)
    }
}

impl SmartSocket {
    pub fn new(watts: f32) -> Self {
        SmartSocket {
            state: SocketState::On { power: watts },
        }
    }
    /// Returns the current power of the smart socket in watts.
    pub fn get_current_power(&self) -> f32 {
        self.power
    }

    /// Turns the smart socket on.
    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    /// Turns the smart socket off and resets its power to 0.0.
    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.power = 0.0;
    }

    /// Returns true if the smart socket is on, false otherwise.
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}
