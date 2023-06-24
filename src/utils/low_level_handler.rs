#[cfg(feature = "ssr")]
use rppal::gpio::Gpio;
#[cfg(feature = "ssr")]
use tokio::time::{sleep, Duration};
#[cfg(feature = "ssr")]
const PUMP_RELAY_PIN: u8 = 7;

#[cfg(feature = "ssr")]
pub async fn pump_water(seconds: u8) -> Result<String, Err<string>> {
    // let mut pin = Gpio::new()?.get(PUMP_RELAY_PIN)?.into_output();
    if let mut pin = Err(e) {
        return Err(());
    }
    pin = pin.get(PUMP_RELAY_PIN)?;
    if let mut pin = Err(e) {
        return Err(());
    }
    pin = pin.into_output();
    pin.set_high();
    sleep(Duration::from_millis(seconds * 1000)).await;
    pin.set_low();
    info!("the rppal turnd off the relay");
    Ok(body)
}
