use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use crate::Result;
use crate::config::{Config, TaskConfig};
use crate::pubsub::{create_client, create_message, send_message};

pub async fn run(config: Config) -> Result<()> {
    let sched_res = JobScheduler::new().await;
    match sched_res {
        Ok(sched) => run_scheduler(&config, sched).await,
        Err(err) => {
            error!("Error creating scheduler: {}", err);
            Ok(())
        }
    }
}

async fn run_scheduler(config: &Config, mut sched: JobScheduler) -> Result<()> {
    for task in config.tasks.iter() {
        add_job(&sched, &config, task).await?;
    }

    info!("Application started");

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            info!("Shut down done");
        })
    }));

    let _ = sched.start().await;

    // Wait for a signal to shut down
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");

    info!("Shutting down scheduler");
    let _ = sched.shutdown().await;

    Ok(())
}

async fn add_job(sched: &JobScheduler, config: &Config, task: &TaskConfig) -> Result<()> {
    info!("Add job: {} -> {}", task.name, task.schedule);

    // Clone values to feed into closures run in separate threads
    let name = task.name.clone();
    let pubsub_config = config.pubsub.clone();
    let jwt_secret = config.jwt_secret.clone();

    let job = Job::new_async(task.schedule.as_str(), move |_uuid, mut _lock| {
        // Clone values to feed into Box pin, whatever that means...
        let job_name = name.clone();
        let pubsub_config_copy = pubsub_config.clone();
        let jwt_secret_copy = jwt_secret.clone();

        info!("Send {}", job_name);

        Box::pin(async move {
            let is_job = job_name.ends_with("Job");
            let topic = match is_job {
                true => pubsub_config_copy.jobs_topic,
                false => pubsub_config_copy.events_topic,
            };

            if let Ok(client) = create_client(&pubsub_config_copy.key_file) {
                let message = create_message(&job_name, is_job, &jwt_secret_copy);
                if let Err(err) = send_message(&client, &topic, message).await {
                    error!("Error on {}: {}", job_name, err);
                }
            }
        })
    })
    .unwrap();

    match sched.add(job).await {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = format!("Error adding job: {}", err);
            Err(msg.into())
        }
    }
}
