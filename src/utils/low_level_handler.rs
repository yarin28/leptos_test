/* handles all the low level comunications with the hardware layer
 * this an actor can recive messages and will handle async messages
 */
use actix::prelude::*;
use anyhow::Result;
use rppal::gpio::Gpio;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
const PUMP_RELAY_PIN: u8 = 4;
// use tracing::instrument;

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
    pub pump_cancellation_token: CancellationToken,
}
impl LowLevelHandler {
    pub fn new() -> Self {
        //i dont belive this function has a possability to fail so i wont use result.
        LowLevelHandler {
            pump_relay_pin: PUMP_RELAY_PIN,
            close_immediately: false,
            water_pump_handler: None,
            pump_cancellation_token: CancellationToken::new(),
        }
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
        // println!("LowLevelHandler is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        // println!("LowLevelHandler is stopped");
    }
}
impl Handler<LowLevelHandlerCommand> for LowLevelHandler {
    type Result = Result<String, std::io::Error>;

    // #[instrument(fields(msg))]
    fn handle(&mut self, msg: LowLevelHandlerCommand, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            LowLevelHandlerCommand::CloseRelayFor(seconds) => {
                let cancelation_token = self.pump_cancellation_token.clone();
                self.water_pump_handler = Some(tokio::spawn(async move {
                    let _res = Self::pump_water(seconds, cancelation_token.clone()).await;
                }));

                Ok(format!("opening the relay for {seconds:}"))
            }
            LowLevelHandlerCommand::OpenRelayImmediately => match &self.water_pump_handler {
                Some(_handler) => {
                    self.pump_cancellation_token.cancel();
                    self.pump_cancellation_token = CancellationToken::new();
                    Ok("aborted the low level task".to_string())
                }
                None => Ok("there is no low level task running".to_string()),
            },
        }
    }
}

impl LowLevelHandler {
    async fn pump_water(
        seconds: usize,
        cancelation_token: CancellationToken,
    ) -> Result<&'static str> {
        let mut pin = Gpio::new()?.get(PUMP_RELAY_PIN)?.into_output();
        pin.set_high();
        tokio::select! {
                _ = cancelation_token.cancelled() => {
                    // The token was cancelled
        let mut pin = Gpio::new()?.get(PUMP_RELAY_PIN)?.into_output();
        pin.set_low();
                }
                _ = tokio::time::sleep(Duration::from_secs(seconds.try_into().unwrap())) => {
                //NOTE:the unwrap cant fail so its ok.
        pin.set_low();
                }
            }

        Ok("finished the pumping")
    }
}
