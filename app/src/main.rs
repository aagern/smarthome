#![allow(unused)]
use anyhow::{Context, Result};
use logger::setup_tracing;
use tracing::{debug, error, info, warn};

fn main() -> Result<()> {
    setup_tracing(); // common logger init
    info!("Logger initialized. App started.");

    println!("Hello, template app!");

    info!("App finished.");
    Ok(())
}
