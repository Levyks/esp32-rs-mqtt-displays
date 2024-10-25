use std::fmt::Debug;
use anyhow::Context;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;

async fn async_main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take()?;

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        led.toggle()?;
        interval.tick().await;
    }
}

// We cannot use tokio's macro and make main directly async because we need to do some setup before
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::io::vfs::initialize_eventfd(1) // Increase this if you're going to use the eventfd for something other than tokio
        .context("Failed to initialize eventfd")
        .map_err(print_error_and_delay)?;

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Failed to build Tokio runtime")
        .map_err(print_error_and_delay)?
        .block_on(async_main())
        .context("Failed to run async main")
        .map_err(print_error_and_delay)?;

    log::info!("Finished executing main.");

    Ok(())
}

// We delay after printing the error to make sure we can read it in the serial monitor before it restarts
fn print_error_and_delay<T: Debug>(err: T) -> T {
    log::error!("{err:?}");
    std::thread::sleep(std::time::Duration::from_secs(3));
    err
}