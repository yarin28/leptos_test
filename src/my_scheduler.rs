use crate::utils::pump_water as pump_water_actually;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::Duration;
use tokio::{sync::Mutex, task::futures};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

#[derive(Debug)]
pub struct Config {
    pub cron_string: String,
    pub seconds_to_pump_water: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
impl Config {
    pub fn new() -> Self {
        Config {
            cron_string: env!("CRON_STRING").to_string(),
            seconds_to_pump_water: env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap(),
        }
    }
}
lazy_static::lazy_static! {
    pub static ref CONFIG: Mutex<Config<>> = Mutex::new(Config::new());
}

fn create_water_pump_job() -> Result<Job> {
    let mut jj = Job::new_repeated(Duration::from_secs(8), |uuid, l| {
        {
            info!(
                "the cron string is - {:?}",
                CONFIG.blocking_lock().cron_string
            );
            Box::pin(async move {
                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next time for 7s job is {:?}", ts),
                    _ => info!("Could not get next tick for 7s job"),
                }
                // info!("the cron string is - {:?}", file_config.cron_string);
                match pump_water_actually(env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap())
                    .await
                {
                    Ok(res) => info!(
                        "the pump_water returnd without errors and returnd this {:}",
                        res
                    ),
                    Err(e) => error!("there was an error with the pump{:?}", e),
                }
            });
        }
    })?;
    return Ok(jj);
}
pub async fn lunch_the_watering_schedualed_program() -> Result<()> {
    info!("enterd lunch_the_watering_schedualed_program");
    {
        let file_config = CONFIG.lock().await;
        info!("the file_config is {:?}", file_config.cron_string);
    }
    let mut sched = JobScheduler::new().await?;
    info!("the cron string is  - {:?}", env!("CRON_STRING"));
    sched
        .add(Job::new_async(env!("CRON_STRING"), move |uuid, mut l| {
            info!("the cron string is - {:?}", CONFIG.lock().await.cron_string);
            Box::pin(async move {
                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next time for 7s job is {:?}", ts),
                    _ => info!("Could not get next tick for 7s job"),
                }
                // info!("the cron string is - {:?}", file_config.cron_string);
                match pump_water_actually(env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap())
                    .await
                {
                    Ok(res) => info!(
                        "the pump_water returnd without errors and returnd this {:}",
                        res
                    ),
                    Err(e) => error!("there was an error with the pump{:?}", e),
                }
            })
        })?)
        .await?;
    sched
        .add(Job::new_async("1/10 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                let mut file_config = CONFIG.lock().await;
                let current_seconds_of_minute = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    % 60;
                let new_str = format!("{}{}{}", "1/", current_seconds_of_minute, " * * * * * * ");
                file_config.cron_string = new_str.clone();
                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                    _ => println!("Could not get next tick for 7s job"),
                }
            })
        })?)
        .await?;
    sched.start().await?;

    Ok(())
}
