use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};

use tracing::info;
pub async fn lunch_the_watering_schedualed_program() -> Result<String> {
    info!("enterd lunch_the_watering_schedualed_program");
    let mut sched = JobScheduler::new().await?;
    sched
        .add(Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        })?)
        .await?;
    sched
        .add(Job::new_async("1/7 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                info!("I run async every 7 seconds");
                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next time for 7s job is {:?}", ts),
                    _ => info!("Could not get next tick for 7s job"),
                }
            })
        })?)
        .await?;
    Ok("gogo gaga".to_string())
}
