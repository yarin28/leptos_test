use actix::prelude::*;
use actix::AsyncContext;
use anyhow::Result;
use core::future::Future;
// use embedded_hal::digital::v2::OutputPin;
// use rppal::gpio::Gpio;
use futures;
use tokio::task::JoinHandle;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
const PUMP_RELAY_PIN: u8 = 4;
use tracing::{event, info, instrument, Level};

use super::relay::RelayActor;
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
    type Result = Result<String, std::io::Error>;
}
#[derive(Debug)]
pub struct LowLevelHandler {
    pub pump_relay_pin: u8,
    pub close_immediately: bool,
    pub water_pump_handler: Option<tokio::task::JoinHandle<()>>,
}
impl LowLevelHandler {
    pub fn new() -> Self {
        //i dont belive this function has a possability to fail so i wont use result.
        LowLevelHandler {
            pump_relay_pin: PUMP_RELAY_PIN,
            close_immediately: false,
            water_pump_handler: None,
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

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("LowLevelHandler is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("LowLevelHandler is stopped");
    }
}
impl Handler<LowLevelHandlerCommand> for LowLevelHandler {
    type Result = Result<String, std::io::Error>;

    // #[instrument(fields(msg))]
    fn handle(&mut self, msg: LowLevelHandlerCommand, ctx: &mut Context<Self>) -> Self::Result {
        event!(Level::WARN, "the msg is -> {:?}", msg);
        match msg {
            LowLevelHandlerCommand::CloseRelayFor(seconds) => {
                event!(Level::INFO, "initiating the stupid_pump_water thread");
                self.water_pump_handler = Some(actix_web::rt::spawn(async move {
                    let _res = Self::stupid_pump_water(seconds).await;
                }));
                event!(
                    Level::INFO,
                    "finished initiating the stupid_pump_water thread"
                );
                Ok(format!("opening the relay for {seconds:}"))
            }
            LowLevelHandlerCommand::OpenRelayImmediately => match &self.water_pump_handler {
                Some(handler) => {
                    handler.abort();
                    Ok("aborted the low level task".to_string())
                }
                None => Ok("there is no low level task running".to_string()),
            },
        }
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
        std::thread::sleep(Duration::from_secs(seconds.try_into().unwrap()));

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
    // fn call_pump_water(&mut self, seconds: usize) -> Result<()> {
    //     actix_web::rt::spawn(async move {
    //         let client = self.close_immediately = true;
    //         let res = client.get(&url).send().await;
    //         if res.is_ok() {
    //             info!("It works!");
    //         }
    //     })
    // }
    async fn stupid_pump_water(seconds: usize) -> Result<&'static str> {
        event!(
            Level::INFO,
            "ENTERD the stupid_pump_water and will be here for {:?}",
            seconds.to_string()
        );
        sleep(Duration::from_secs(seconds.try_into().unwrap())).await;

        event!(
            Level::INFO,
            "EXITING the stupid_pump_water and was  here for {:?}",
            seconds.to_string()
        );
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
