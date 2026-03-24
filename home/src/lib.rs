#![allow(unused)]
use anyhow::{Context, Result};
use tracing::{debug, error, info, warn};

#[cfg(not(feature = "mock"))]
mod real;
#[cfg(not(feature = "mock"))]
pub use real::SmartThermometer;

#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use mock::SmartThermometer;

struct SmartSocket {
    is_on: bool,
    power: f32, // watts
}

enum SmartDevice {
    SmartThermometer(SmartThermometer),
    SmartSocket(SmartSocket),
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
