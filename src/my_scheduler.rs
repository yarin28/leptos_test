use crate::utils::pump_water as pump_water_actually;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::Duration;
use tokio::{sync::Mutex, task::futures};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::instrument;
use tracing::{error, info};
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

pub struct SchedulerMutex {
    scheduler: Mutex<MyScheduler>,
}

impl SchedulerMutex {
    pub async fn new() -> Result<Self> {
        Ok(SchedulerMutex {
            scheduler: Mutex::new(MyScheduler::new().await?),
        })
    }
}
pub struct MyScheduler {
    sched: JobScheduler,
    water_pump_job_uuid: Uuid,
    water_pump_job_curret_corn_string: String,
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
    pub async fn add_all_jobs_to_sched(&mut self) -> Result<Uuid> {
        let job = self.create_cron_changing_job().await?;
        let uuid_of_job = self.sched.add(job).await?;
        Ok(uuid_of_job)
    }
    #[instrument(skip(self), fields(self.water_pump_job_uuid = %self.water_pump_job_uuid,self.water_pump_job_curret_corn_string,%self.water_pump_job_curret_corn_string))]
    pub async fn change_cron_string_in_job(&mut self, new_cron_string: String) {
        info!(
            "if current- {:?} != new-{:?}",
            self.water_pump_job_curret_corn_string, new_cron_string
        );
        if new_cron_string != self.water_pump_job_curret_corn_string {
            self.sched
                .remove(&self.water_pump_job_uuid)
                .await
                .expect("the remove from the Scheduler didn`t work");
            let jj = Self::create_water_pump_job(new_cron_string)
                .await
                .expect("couldn't create the new water pump job with the new cron srting ");
            self.sched
                .add(jj)
                .await
                .expect("could not add the new job to Scheduler");
            self.sched
                .start()
                .await
                .expect("couldnt start the Scheduler");
        };
    }
    #[instrument]
    async fn create_water_pump_job(cron_string: String) -> Result<Job> {
        let mut jj = Job::new_async(cron_string.as_str(), move |uuid, mut l| {
            {
                Box::pin(async move {
                    // l.context.metadata_storage.read().await.get(uuid);
                    info!(
                        "inside the water pump job and the cron string is - {:?}",
                        CONFIG.lock().await
                    );
                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => info!("Next time for 8s job is {:?}", ts),
                        _ => info!("Could not get next tick for 8s job"),
                    }
                    // info!("the cron string is - {:?}", file_config.cron_string);
                    match pump_water_actually(
                        env!("SECONDS_TO_PUMP_WATER").parse::<usize>().unwrap(),
                    )
                    .await
                    {
                        Ok(res) => info!(
                            "the pump_water returnd without errors and returnd this {:}",
                            res
                        ),
                        Err(e) => error!("there was an error with the pump{:?}", e),
                    }
                })
            }
        })?;
        println!("{:?}", jj.job_data().unwrap());
        Ok(jj)
    }
    #[instrument(skip(self), fields(self.water_pump_job_uuid = %self.water_pump_job_uuid,self.water_pump_job_curret_corn_string,%self.water_pump_job_curret_corn_string))]
    async fn create_cron_changing_job(&mut self) -> Result<Job> {
        let jj = Job::new_async("1/10 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                let mut file_config = CONFIG.lock().await;
                let current_seconds_of_minute = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    % 60;
                let new_str = format!("{}{}{}", "1/", current_seconds_of_minute, " * * * * * * ");
                info!("new cron String{:?}", new_str);
                let mut jj = self.change_cron_string_in_job(new_str).await;
                // .expect("there was an error with creating the new water pump job");
                // info!("the job data is{:?}", jj.job_data());
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                    _ => println!("Could not get next tick for 7s job"),
                }
            })
        })?;
        Ok(jj)
    }
}
