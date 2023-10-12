use std::collections::HashMap;
use base64::{engine::general_purpose::STANDARD, Engine};
use pub_sub_client::{PubSubClient, PublishedMessage, RawPublishedMessage};
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
    let client = PubSubClient::new(pubsub_config.key_file.as_str(), std::time::Duration::from_secs(30))?;

    if name.ends_with("Job") {
        // Publish a job
        let job = PublishedJobDto {
            id: uuid::Uuid::new_v4().to_string(),
            job: name.to_string(),
            data: {}
        };

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("token".to_string(), String::from("sample"));

        let msg = RawPublishedMessage {
            data: Some(STANDARD.encode(serde_json::to_string(&job).unwrap())),
            attributes: Some(attributes),
            ordering_key: None,
        };
        let mut messages: Vec<RawPublishedMessage<'_>> = Vec::new();
        messages.push(msg);
        let _ = client.publish_raw(&pubsub_config.jobs_topic, messages, None).await?;
    } else if name.ends_with("Event") {
        // Publish an event
         let event = PublishedEventDto {
            id: uuid::Uuid::new_v4().to_string(),
            event: name.to_string(),
            data: {}
        };

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("token".to_string(), String::from("sample"));

        let msg = RawPublishedMessage {
            data: Some(STANDARD.encode(serde_json::to_string(&event).unwrap())),
            attributes: Some(attributes),
            ordering_key: None,
        };
        let mut messages: Vec<RawPublishedMessage<'_>> = Vec::new();
        messages.push(msg);
        let _ = client.publish_raw(&pubsub_config.events_topic, messages, None).await?;

    }

    Ok(())
}
