use tokio_cron_scheduler::{JobScheduler, Job};

use crate::error::Result;
use crate::config::Config;
use crate::pubsub::send_job;

pub async fn run(config: Config) -> Result<()> {
    let mut sched = JobScheduler::new().await?;

    for task in config.tasks.iter() {
        println!("Added job: {} -> {}", task.name, task.schedule);

        let name = task.name.clone();
        let schedule = task.schedule.clone();
        let pubsub_config = config.pubsub.clone();
        let jwt_secret = config.jwt_secret.clone();

        let job = Job::new_async(schedule.as_str(), move |_uuid, mut _lock| {
            let job_name = name.clone();
            let pubsub_config_copy = pubsub_config.clone();
            let jwt_secret_copy = jwt_secret.clone();
            println!("{} at {}", job_name, chrono::Utc::now());
            Box::pin(async move {
                if let Err(err) = send_job(&pubsub_config_copy, &jwt_secret_copy, &job_name).await {
                    eprintln!("Error on {}: {}", job_name, err);
                }
            })
        }).unwrap();

        sched.add(job).await?;
    }

    println!("");

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    sched.set_shutdown_handler(Box::new(|| {
      Box::pin(async move {
        println!("Shut down done");
      })
    }));

    let _ = sched.start().await;

    // Poor man's main loop
    loop {
        tokio::time::sleep(core::time::Duration::from_secs(60)).await;
    }

    Ok(())
}
