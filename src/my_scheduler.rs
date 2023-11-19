use crate::utils::config_builder::SETTINGS;
use actix::Addr;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::error;
use tracing::instrument;
use uuid::Uuid;

use crate::utils::LowLevelHandler;

#[derive(Debug, Clone)]
pub struct Config {
    pub cron_string: String,
    pub seconds_to_pump_water: usize,
    pub low_level_handler_sender: Addr<LowLevelHandler>,
}
impl Config {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(low_level_handler_sender: Addr<LowLevelHandler>) -> ConfigBuilder {
        ConfigBuilder {
            cron_string: SETTINGS
                .read()
                .unwrap()
                .get_string("lua.cron_string")
                .unwrap(),
            seconds_to_pump_water: usize::try_from(
                SETTINGS
                    .read()
                    .unwrap()
                    .get_int("lua.seconds_to_pump_water")
                    .unwrap(),
            )
            .unwrap(),
            low_level_handler_sender,
        }
    }
}
pub struct ConfigBuilder {
    pub cron_string: String,
    pub seconds_to_pump_water: usize,
    pub low_level_handler_sender: Addr<LowLevelHandler>,
}
impl ConfigBuilder {
    pub fn cron_string(&mut self, cron_string: String) -> &mut Self {
        self.cron_string = cron_string;
        self
    }
    pub fn seconds_to_pump_water(&mut self, seconds_to_pump_water: usize) -> &mut Self {
        self.seconds_to_pump_water = seconds_to_pump_water;
        self
    }
    pub fn low_level_handler_sender(
        &mut self,
        low_level_handler_sender: Addr<LowLevelHandler>,
    ) -> &mut Self {
        self.low_level_handler_sender = low_level_handler_sender;
        self
    }
    pub fn build(&self) -> Config {
        Config {
            cron_string: self.cron_string.clone(),
            seconds_to_pump_water: self.seconds_to_pump_water,
            low_level_handler_sender: self.low_level_handler_sender.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SchedulerMutex {
    pub scheduler: Arc<Mutex<MyScheduler>>,
}

impl SchedulerMutex {
    #[instrument]
    pub async fn new(sender: Addr<LowLevelHandler>) -> Result<Self> {
        let scheduler_mutex = SchedulerMutex {
            scheduler: Arc::new(Mutex::new(MyScheduler::start(sender).await?)),
        };
        Ok(scheduler_mutex)
    }

    #[instrument]
    pub async fn change_seconds_to_pump_water(&self, new_seconds: usize) -> Result<()> {
        self.scheduler
            .lock()
            .await
            .change_seconds_to_pump_water_in_config(new_seconds)
            .unwrap()
            .change_job(None, Some(new_seconds))
            .await
    }
    #[instrument(skip(self))]
    pub async fn change_cron_string(&self, new_cron_string: String) -> Result<()> {
        self.scheduler
            .lock()
            .await
            .change_cron_string_in_config(new_cron_string.clone())
            .unwrap()
            .change_job(Some(new_cron_string), None)
            .await
    }
    // #[instrument(skip(self), fields(self.scheduler.water_pump_job_uuid = %self.scheduler.lock().await.water_pump_job_uuid,self.scheduler.water_pump_job_curret_corn_string= %self.scheduler.lock().await.water_pump_job_curret_corn_string))]
}
pub struct MyScheduler {
    sched: JobScheduler,
    water_pump_job_uuid: Uuid,
    pub water_pump_job_curret_corn_string: String,
    pub config: Config,
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
    pub async fn start(sender: Addr<LowLevelHandler>) -> Result<Self> {
        let config = Config::new(sender).build();
        let sched = JobScheduler::new().await?;
        let water_pump_job = Self::create_water_pump_job(config.clone()).await.unwrap();
        let water_pump_job_uuid = sched.add(water_pump_job).await?;
        let water_pump_job_curret_corn_string = SETTINGS
            .read()
            .unwrap()
            .get_string("lua.cron_string")
            .unwrap();
        sched.start().await?;
        Ok(MyScheduler {
            sched,
            water_pump_job_uuid,
            water_pump_job_curret_corn_string,
            config,
        })
    }
    #[instrument(skip(self))]
    fn change_cron_string_in_config(&mut self, new_cron_string: String) -> Result<&mut Self> {
        // the end goal is to validate the cron string here.
        self.config.cron_string = new_cron_string;
        Ok(self)
    }
    #[instrument]
    fn change_seconds_to_pump_water_in_config(&mut self, seconds: usize) -> Result<&mut Self> {
        self.config.seconds_to_pump_water = seconds;
        Ok(self)
    }
    //the function create a new job with paramenter from *config*
    //i want to remove the parametes that the function gets but i need to check if the current
    //string in the config is deffrent from the current string inside the job.
    #[instrument]
    pub async fn change_job(
        &mut self,
        new_cron_string: Option<String>,
        new_seconds: Option<usize>,
    ) -> Result<()> {
        if (new_cron_string
            .clone()
            .unwrap_or(self.water_pump_job_curret_corn_string.clone())
            != self.water_pump_job_curret_corn_string)
            || new_seconds.is_some()
        {
            self.sched
                .remove(&self.water_pump_job_uuid)
                .await
                .expect("the remove from the Scheduler didn`t work");
            let jj = Self::create_water_pump_job(self.config.clone())
                .await
                .expect("couldn't create the new water pump job with the new cron srting ");
            let new_uuid = self
                .sched
                .add(jj)
                .await
                .expect("could not add the new job to Scheduler");
            self.water_pump_job_curret_corn_string = self.config.cron_string.clone();
            self.water_pump_job_uuid = new_uuid;
            // self.sched
            //     .start()
            //     .await
            //     .expect("couldnt start the Scheduler");
        };
        Ok(())
    }
    #[instrument]
    async fn create_water_pump_job(config: Config) -> Result<Job> {
        let jj = Job::new_async(config.cron_string.clone().as_str(), move |_uuid, mut _l| {
            {
                let low_level_sender_address = config.low_level_handler_sender.clone();
                Box::pin(async move {
                    // Query the next execution time for this job
                    match low_level_sender_address
                        .send(crate::utils::LowLevelHandlerMessage::CloseRelayFor(
                            config.seconds_to_pump_water,
                        ))
                        .await
                    {
                        Ok(_res) => {}
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
        let jj = Job::new_async("1/10 * * * * *", move |_uuid, mut _l| {
            let scheduler_mutex_cloned = Arc::clone(&scheduler_mutex);
            Box::pin(async move {
                // let file_config = CONFIG.lock().await;
                let current_seconds_of_minute = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    % 60;
                let new_str = format!("{}{}{}", "1/", current_seconds_of_minute, " * * * * * * ");
                scheduler_mutex_cloned
                    .lock()
                    .await
                    .change_job(Some(new_str), None)
                    .await
                    .expect("there was an error with creating the new water pump job");
            })
        })?;
        Ok(jj)
    }
}
