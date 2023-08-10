use anyhow::Result;
// use embedded_hal::digital::v2::OutputPin;
// use rppal::gpio::Gpio;
use tokio::time::{sleep, Duration};
const PUMP_RELAY_PIN: u8 = 4;

use tracing::{event, info, instrument, Level};
// pub async fn pump_water(seconds: usize) -> Result<impl OutputPin> {
//     let mut pin = Gpio::new()?.get(PUMP_RELAY_PIN)?.into_output();
//     pin.set_high();
//     sleep(Duration::from_secs(seconds.try_into().unwrap())).await;
//     pin.set_low();
//     info!("the rppal turnd off the relay");
//     Ok(pin)
// }
#[instrument(fields(seconds))]
pub async fn pump_water(seconds: usize) -> Result<&'static str> {
    // event!(
    //     Level::INFO,
    //     "ENTERD the pump_water_function and will be here for {:?}",
    //     seconds.to_string()
    // );
    sleep(Duration::from_secs(seconds.try_into().unwrap())).await;
    // event!(
    //     Level::INFO,
    //     "EXITING the pump_water_function and after being here for {:?}",
    //     seconds.to_string()
    // );
    Ok("gogo gaga")
}
