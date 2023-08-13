use crate::utils::pump_water as pump_water_actually;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, Level};
use tracing::{event, instrument};
use uuid::Uuid;

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

#[derive(Clone, Debug)]
pub struct SchedulerMutex {
    scheduler: Arc<Mutex<MyScheduler>>,
}

impl SchedulerMutex {
    pub async fn new() -> Result<Self> {
        let scheduler_mutex = SchedulerMutex {
            scheduler: Arc::new(Mutex::new(MyScheduler::new().await?)),
        };
        Ok(scheduler_mutex)
    }

    pub async fn change_cron_string(&mut self, new_cron_string: String) -> Result<()> {
        self.scheduler
            .lock()
            .await
            .change_cron_string_in_job(new_cron_string)
            .await
    }
    // #[instrument(skip(self), fields(self.scheduler.water_pump_job_uuid = %self.scheduler.lock().await.water_pump_job_uuid,self.scheduler.water_pump_job_curret_corn_string= %self.scheduler.lock().await.water_pump_job_curret_corn_string))]
}
pub struct MyScheduler {
    sched: JobScheduler,
    water_pump_job_uuid: Uuid,
    water_pump_job_curret_corn_string: String,
}
impl std::fmt::Debug for MyScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyScheduler")
            .field("water_pump_job_uuid", &self.water_pump_job_uuid)
            .field(
                "self.water_pump_job_curret_corn_string",
                &self.water_pump_job_curret_corn_string,
            )
            .finish()
    }
}
impl MyScheduler {
    #[instrument]
    pub async fn new() -> Result<Self> {
        let sched = JobScheduler::new().await?;
        let water_pump_job = Self::create_water_pump_job(CONFIG.lock().await.cron_string.clone())
            .await
            .unwrap();
        let water_pump_job_uuid = sched.add(water_pump_job).await?;
        let water_pump_job_curret_corn_string = env!("CRON_STRING").to_string();
        sched.start().await?;
        Ok(MyScheduler {
            sched,
            water_pump_job_uuid,
            water_pump_job_curret_corn_string,
        })
    }

    #[instrument(skip(self))]
    pub async fn change_cron_string_in_job(&mut self, new_cron_string: String) -> Result<()> {
        info!(
            "if current- {:?} != new-{:?}",
            self.water_pump_job_curret_corn_string, new_cron_string
        );
        if new_cron_string != self.water_pump_job_curret_corn_string {
            self.sched
                .remove(&self.water_pump_job_uuid)
                .await
                .expect("the remove from the Scheduler didn`t work");
            let jj = Self::create_water_pump_job(new_cron_string.clone())
                .await
                .expect("couldn't create the new water pump job with the new cron srting ");
            let new_uuid = self
                .sched
                .add(jj)
                .await
                .expect("could not add the new job to Scheduler");
            self.water_pump_job_curret_corn_string = new_cron_string;
            self.water_pump_job_uuid = new_uuid;
            // self.sched
            //     .start()
            //     .await
            //     .expect("couldnt start the Scheduler");
        };
        Ok(())
    }
    #[instrument]
    async fn create_water_pump_job(cron_string: String) -> Result<Job> {
        let jj = Job::new_async(cron_string.clone().as_str(), move |uuid, mut l| {
            {
                let cron_string_2 = cron_string.clone();
                Box::pin(async move {
                    event!(
                        Level::INFO,
                        "inside the water pump job and the cron string is - {:?}",
                        cron_string_2
                    );
                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => {
                            event!(Level::TRACE, "Next time for PUMP_WATER job is {:?}", ts)
                        }
                        _ => event!(Level::TRACE, "Could not get next tick for 8s job"),
                    }
                    // info!("the cron string is - {:?}", file_config.cron_string);
                    match pump_water_actually(
                        env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap(),
                    )
                    .await
                    {
                        Ok(res) => {
                            event!(
                                Level::TRACE,
                                "the pump_water returnd without errors and returnd this {:}",
                                res
                            )
                        }
                        Err(e) => error!("there was an error with the pump{:?}", e),
                    }
                })
            }
        })?;
        Ok(jj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[instrument(skip(scheduler_mutex))]
    async fn create_cron_changing_job(scheduler_mutex: Arc<Mutex<MyScheduler>>) -> Result<Job> {
        event!(Level::INFO, "enterd the create_cron_changing_job");
        let jj = Job::new_async("1/10 * * * * *", move |uuid, mut l| {
            let scheduler_mutex_cloned = Arc::clone(&scheduler_mutex);
            Box::pin(async move {
                // let file_config = CONFIG.lock().await;
                let current_seconds_of_minute = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    % 60;
                let new_str = format!("{}{}{}", "1/", current_seconds_of_minute, " * * * * * * ");
                info!("new cron String{:?}", new_str);
                scheduler_mutex_cloned
                    .lock()
                    .await
                    .change_cron_string_in_job(new_str)
                    .await
                    .expect("there was an error with creating the new water pump job");
                // info!("the job data is{:?}", jj.job_data());
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next time for 7s job is {:?}", ts),
                    _ => println!("Could not get next tick for 7s job"),
                }
            })
        })?;
        Ok(jj)
    }
}
