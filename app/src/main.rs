#![allow(unused)]
use anyhow::{Context, Result};
use home::SmartThermometer;
use logger::setup_tracing;
use tracing::{debug, error, info, warn};

fn main() -> Result<()> {
    setup_tracing(); // common logger init
    debug!("Logger initialized. App started.");

    info!("Temperature check.");
    let thermo = SmartThermometer::default();
    let temp = thermo.get_current_temperature();

    println!("Current temperature: {:.1}°C", temp);
    println!(
        "Current temperature: {:.1}°F",
        thermo.celsius_to_fahrenheit(temp)
    );

    debug!("App finished.");
    Ok(())
}
