use pub_sub_client::{PubSubClient, PublishedMessage};
use pub_sub_client_derive::PublishedMessage;
use serde::{Serialize, Deserialize};

use crate::error::Result;
use crate::config::PubSubConfig;

#[derive(Debug, Serialize, Deserialize, PublishedMessage)]
struct PublishedJobDto {
    id: String,
    job: String,
    data: ()
}

#[derive(Debug, Serialize, Deserialize, PublishedMessage)]
struct PublishedEventDto {
    id: String,
    event: String,
    data: ()
}

pub async fn send_job(pubsub_config: &PubSubConfig, name: &String) -> Result<()> {
    println!("Sending job: {} to Google pub/sub", name);
    let client = PubSubClient::new(pubsub_config.key_file.as_str(), std::time::Duration::from_secs(30))?;

    if name.ends_with("Job") {
        // Publish a job
        let job = PublishedJobDto {
            id: uuid::Uuid::new_v4().to_string(),
            job: name.to_string(),
            data: {}
        };

        let mut messages: Vec<PublishedJobDto> = Vec::new();
        messages.push(job);
        let message_ids = client.publish(&pubsub_config.jobs_topic, messages, None, None).await?;
        println!("{:?}", message_ids);
    } else if name.ends_with("Event") {
        // Publish an event
         let event = PublishedEventDto {
            id: uuid::Uuid::new_v4().to_string(),
            event: name.to_string(),
            data: {}
        };
        let mut messages: Vec<PublishedEventDto> = Vec::new();
        messages.push(event);
        let message_ids = client.publish(&pubsub_config.jobs_topic, messages, None, None).await?;
        println!("{:?}", message_ids);
    }

    Ok(())
}
