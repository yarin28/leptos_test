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
#[derive(Debug, Clone, Copy)]
pub enum LowLevelHandlerCommand {
    CloseRelayFor(usize),
    OpenRelayImmediately,
}
impl Message for LowLevelHandlerCommand {
    type Result = Result<bool, std::io::Error>;
}
#[derive(Debug, Clone, Copy)]
pub struct LowLevelHandler {
    pub pump_relay_pin: u8,
    pub close_immediately: bool,
}
impl LowLevelHandler {
    pub fn new() -> Self {
        //i dont belive this function has a possability to fail so i wont use result.
        LowLevelHandler {
            pump_relay_pin: PUMP_RELAY_PIN,
            close_immediately: false,
        }
    }
    pub fn say_hello(&self) {
        println!("hello!")
    }
}
impl Default for LowLevelHandler {
    fn default() -> Self {
        LowLevelHandler::new()
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
                futures::executor::block_on(async move { self.pump_water(seconds).await });
            }
            LowLevelHandlerCommand::OpenRelayImmediately => {
                self.close_immediately = true;
            }
        }
        Ok(true)
    }
}

impl LowLevelHandler {
    #[instrument(fields(seconds))]
    pub async fn pump_water(&mut self, seconds: usize) -> Result<&'static str> {
        event!(
            Level::INFO,
            "ENTERD the pump_water_function and will be here for {:?}",
            seconds.to_string()
        );
        let mut i = 0;
        let mut close_relay_immediately = false;
        while i < seconds && close_relay_immediately == false {
            sleep(Duration::from_secs(1)).await;
            if self.close_immediately == true {
                event!(Level::INFO, "turned close_immediately to true");
                close_relay_immediately = true;
            }
            i += 1;
        }

        event!(Level::INFO, "exited the pump water function");
        // event!(
        //     Level::INFO,
        //     "ENTERD the pump_water_function and will be here for {:?}",
        //     seconds.to_string()
        // );
        // sleep(Duration::from_secs(seconds.try_into().unwrap())).await;
        // event!(
        //     Level::INFO,
        //     "EXITING the pump_water_function and after being here for {:?}",
        //     seconds.to_string()
        // );
        Ok("finished the pumping")
    }
}
