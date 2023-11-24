/* handles all the low level comunications with the hardware layer
 * this an actor can recive messages and will handle async messages
 */
use crate::utils::config::config_builder::SETTINGS;
use actix::prelude::*;
use anyhow::Result;
use rppal::gpio::Gpio;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{event, instrument};
// use tracing::instrument;

#[derive(Debug, Clone, Copy)]
pub struct LowLevelHandlerCommand {
    pub pin_num: u8,
    pub message: LowLevelHandlerMessage,
}
#[derive(Debug, Clone, Copy)]
pub enum LowLevelHandlerMessage {
    CloseRelayFor(usize),
    OpenRelayImmediately,
}
impl Message for LowLevelHandlerMessage {
    type Result = Result<String, std::io::Error>;
}
impl Message for LowLevelHandlerCommand {
    type Result = Result<String, std::io::Error>;
}
#[derive(Debug)]
pub struct LowLevelHandler {
    pub gpio_pins: Vec<GpioPin>,
}

#[derive(Debug)]
pub struct GpioPin {
    pub pin_num: u8,
    pub close_immediately: bool,
    pub water_pump_handler: Option<tokio::task::JoinHandle<()>>,
    pub pump_cancellation_token: CancellationToken,
}

impl LowLevelHandler {
    pub fn new() -> Self {
        let gpio_table = SETTINGS
            .read()
            .unwrap()
            .get_table("lua.gpio_table")
            .unwrap();
        let gpio_pins: Vec<GpioPin> = gpio_table
            .values()
            .map(|value| {
                let table = value.clone().into_table().unwrap();
                GpioPin {
                    pin_num: table.get("gpio_pin").unwrap().clone().into_int().unwrap() as u8,
                    close_immediately: false,
                    water_pump_handler: None,
                    pump_cancellation_token: CancellationToken::new(),
                }
            })
            .collect();
        dbg!(&gpio_pins);

        LowLevelHandler { gpio_pins }
    }
}
impl Default for LowLevelHandler {
    fn default() -> Self {
        LowLevelHandler::new()
    }
}
impl Actor for LowLevelHandler {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        event!(tracing::Level::TRACE, "the LowLevelHandler has started")
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        event!(tracing::Level::TRACE, "the LowLevelHandler has stopped")
    }
}
impl Handler<LowLevelHandlerCommand> for LowLevelHandler {
    type Result = Result<String, std::io::Error>;

    #[instrument(level = "trace", skip(self, _ctx, msg))]
    fn handle(&mut self, msg: LowLevelHandlerCommand, _ctx: &mut Context<Self>) -> Self::Result {
        let gpio_pin = self
            .gpio_pins
            .iter_mut()
            .find(|pin| pin.pin_num == msg.pin_num)
            .expect("there is no gpio pin with this num");
        match msg.message {
            LowLevelHandlerMessage::CloseRelayFor(seconds) => {
                let cancelation_token = gpio_pin.pump_cancellation_token.clone();
                let pin_num = gpio_pin.pin_num;
                gpio_pin.water_pump_handler = Some(tokio::spawn(async move {
                    let res = Self::pump_water(pin_num, seconds, cancelation_token.clone()).await;
                    match res {
                        Ok(_res) => {}
                        Err(e) => event!(tracing::Level::ERROR, "pump_water has returnd {e}"),
                    }
                }));

                Ok(format!("opening the relay for {seconds:}"))
            }
            LowLevelHandlerMessage::OpenRelayImmediately => {
                match &mut self.gpio_pins[0].water_pump_handler {
                    Some(_handler) => {
                        self.gpio_pins[0].pump_cancellation_token.cancel();
                        self.gpio_pins[0].pump_cancellation_token = CancellationToken::new();
                        Ok("aborted the low level task".to_string())
                    }
                    None => Ok("there is no low level task running".to_string()),
                }
            }
        }
    }
}

impl LowLevelHandler {
    #[instrument(level = "trace", skip(cancelation_token))]
    async fn pump_water(
        pin_num: u8,
        seconds: usize,
        cancelation_token: CancellationToken,
    ) -> Result<&'static str> {
        event!(tracing::Level::TRACE, "opening the relay");
        let mut pin = Gpio::new()?.get(pin_num)?.into_output(); // because this function call
                                                                // had the ? operator it sent the error to the caller that *ignored the error*.
        pin.set_high();
        tokio::select! {
                _ = cancelation_token.cancelled() => {
                    // The token was cancelled

        event!(tracing::Level::TRACE, " closing the relay after recived the cancelation_token");
        let mut pin = Gpio::new()?.get(pin_num)?.into_output();
        pin.set_low();
                }
                _ = tokio::time::sleep(Duration::from_secs(seconds.try_into().unwrap())) => {
                //NOTE:the unwrap cant fail so its ok.
        event!(tracing::Level::INFO, " closing the relay after {seconds:} has passed");
        pin.set_low();
                }
            }

        Ok("finished the pumping")
    }
}

#[test]
fn ingest_lua_table() {
    crate::utils::config::config_builder::config_build().unwrap();
    LowLevelHandler::new();
}
