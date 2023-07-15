use crate::utils::pump_water as pump_water_actually;
use anyhow::Result;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};
pub struct Config<'a> {
    pub cron_string: &'a str,
    pub seconds_to_pump_water: usize,
}

impl Config<'static> {
    pub fn new() -> Self {
        Config {
            cron_string: env!("CRON_STRING"),
            seconds_to_pump_water: env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap(),
        }
    }
}
lazy_static::lazy_static! {
    pub static ref CONFIG: Mutex<Config<'static>> = Mutex::new(Config::new());
}
pub async fn lunch_the_watering_schedualed_program() -> Result<()> {
    info!("enterd lunch_the_watering_schedualed_program");
    // let mut file_config = CONFIG.lock().unwrap();
    // let config = Config {
    //     cron_string: env!("CRON_STRING"),
    //     seconds_to_pump_water: env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap(),
    // };
    // file_config.unlock().unwrap();
    let mut sched = JobScheduler::new().await?;
    info!("the cron string is  - {:?}", env!("CRON_STRING"));
    sched
        .add(Job::new_async(env!("CRON_STRING"), |uuid, mut l| {
            info!("the cron string is - {:?}", CONFIG.lock().cron_string);
            Box::pin(async move {
                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next time for 7s job is {:?}", ts),
                    _ => info!("Could not get next tick for 7s job"),
                }
                match pump_water_actually(env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap())
                    .await
                {
                    Ok(res) => info!("the pump_water returnd without errors"),
                    Err(e) => error!("there was an error with the pump{:?}", e),
                }
            })
        })?)
        .await?;
    sched.start().await?;

    Ok(())
}
