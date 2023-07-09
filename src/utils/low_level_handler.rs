use anyhow::Result;
// use embedded_hal::digital::v2::OutputPin;
// use rppal::gpio::Gpio;
use tokio::time::{sleep, Duration};
const PUMP_RELAY_PIN: u8 = 4;

use tracing::info;
// pub async fn pump_water(seconds: usize) -> Result<impl OutputPin> {
//     let mut pin = Gpio::new()?.get(PUMP_RELAY_PIN)?.into_output();
//     pin.set_high();
//     sleep(Duration::from_secs(seconds.try_into().unwrap())).await;
//     pin.set_low();
//     info!("the rppal turnd off the relay");
//     Ok(pin)
// }
pub async fn pump_water(seconds: usize) -> Result<&'static str> {
    sleep(Duration::from_secs(seconds.try_into().unwrap())).await;
    info!("the rppal turnd off the relay");
    Ok("gogo gaga")
}
