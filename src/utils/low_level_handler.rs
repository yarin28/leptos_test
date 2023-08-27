use actix::prelude::*;
use anyhow::Result;
// use embedded_hal::digital::v2::OutputPin;
// use rppal::gpio::Gpio;
use futures;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
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
#[derive(Debug)]
pub enum LowLevelHandlerCommand {
    CloseRelayFor(usize),
    OpenRelayImindatly,
}
impl Message for LowLevelHandlerCommand {
    type Result = Result<bool, std::io::Error>;
}
#[derive(Debug)]
pub struct LowLevelHandler {
    pump_relay_pin: u8,
}
impl LowLevelHandler {
    pub async fn new() -> Result<Self> {
        Ok(LowLevelHandler {
            pump_relay_pin: PUMP_RELAY_PIN,
        })
    }
}
impl Actor for LowLevelHandler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}
impl Handler<LowLevelHandlerCommand> for LowLevelHandler {
    type Result = Result<bool, std::io::Error>;

    #[instrument(fields(msg))]
    fn handle(&mut self, msg: LowLevelHandlerCommand, ctx: &mut Context<Self>) -> Self::Result {
        event!(Level::WARN, "the msg is -> {:?}", msg);
        println!("the msg is -> {:?}", msg);
        match msg {
            LowLevelHandlerCommand::CloseRelayFor(seconds) => {
                futures::executor::block_on(async move { pump_water(seconds).await });
            }
            LowLevelHandlerCommand::OpenRelayImindatly => {
                event!(Level::WARN, "the msg is -> {:?}", msg)
            }
        }
        Ok(true)
    }
}

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
