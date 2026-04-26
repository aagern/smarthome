use rand::prelude::*;

#[derive(PartialEq, Debug)]
pub struct SmartThermometer {
    current_temperature: f32,
}

impl Default for SmartThermometer {
    /// Creates a new SmartThermometer with a random current temperature between -30.0 and 50.0 degrees Celsius.
    fn default() -> Self {
        Self::new(rand::rng().random_range(-30.0..50.0))
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
    pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
        celsius * 9.0 / 5.0 + 32.0
    }
}

#[derive(PartialEq, Debug)]
pub enum SocketState {
    On { power: f32 },
    Off,
}

#[derive(PartialEq, Debug)]
pub struct SmartSocket {
    state: SocketState,
}

impl Default for SmartSocket {
    /// Creates a new SmartSocket with default values: is_on = true, power = a random value between 1000.0 and 2000.0 watts.
    fn default() -> Self {
        Self::new(rand::rng().random_range(1000.0..2000.0))
    }
}

impl SmartSocket {
    /// Creates a new SmartSocket with the specified state.
    pub fn new(watts: f32) -> Self {
        SmartSocket {
            state: SocketState::On { power: watts },
        }
    }
    /// Returns the current power of the smart socket in watts.
    /// If the socket is off, returns 0.0.
    /// If the socket is on, returns the current power.
    pub fn get_current_power(&self) -> f32 {
        match self.state {
            SocketState::Off => 0.0,
            SocketState::On { power } => power,
        }
    }

    /// Turns the smart socket on.
    /// If the socket is off, sets its power to a random value between 1000.0 and 2000.0 watts.
    /// If the socket is already on, does nothing.
    /// # Note
    /// A smart socket can only be turned on if it is currently off.
    pub fn turn_on(&mut self) {
        if let SocketState::Off = self.state {
            self.state = SocketState::On {
                power: rand::rng().random_range(1000.0..2000.0),
            };
        }
    }

    /// Turns the smart socket off.
    /// If the socket is on, sets its state to Off and its power to 0.0.
    /// If the socket is already off, does nothing.
    /// # Note
    /// A smart socket can only be turned off if it is currently on.
    pub fn turn_off(&mut self) {
        self.state = SocketState::Off;
    }

    /// Returns true if the smart socket is on, false otherwise.
    pub fn is_on(&self) -> bool {
        //matches!(self.state, SocketState::On { .. })
        match self.state {
            SocketState::Off => false,
            SocketState::On { .. } => true,
        }
    }
}
