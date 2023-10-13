use std::collections::HashMap;
use pub_sub_client::PubSubClient;
use serde::{Serialize, Deserialize};

use crate::error::Result;
use crate::config::PubSubConfig;
use crate::jwt::create_token;

#[derive(Debug, Serialize, Deserialize)]
struct DummyLabels {
    labels: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublishedJobDto {
    id: String,
    job: String,
    data: DummyLabels,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublishedEventDto {
    id: String,
    event: String,
    data: DummyLabels,
}

pub async fn send_job(pubsub_config: &PubSubConfig, jwt_secret: &String, name: &String) -> Result<()> {
    let client = PubSubClient::new(pubsub_config.key_file.as_str(), std::time::Duration::from_secs(30))?;

    if name.ends_with("Job") {
        // Publish a job
        let id = uuid::Uuid::new_v4().to_string();
        let token = create_token(&id, jwt_secret).unwrap();
        let messages = vec![(
            PublishedJobDto {
                id: id,
                job: name.to_string(),
                data: DummyLabels {
                    labels: None
                }
            },
            HashMap::from([("token".to_string(), token)]),
        )];

        let result = client.publish::<PublishedJobDto, _>(&pubsub_config.jobs_topic, messages, None, None).await;
        if let Err(err) = result {
            eprintln!("Publish failed: {}", err);
        }
    } else if name.ends_with("Event") {
        // Publish an event
        let id = uuid::Uuid::new_v4().to_string();
        let token = create_token(&id, jwt_secret).unwrap();
        let messages = vec![(
            PublishedEventDto {
                id: id,
                event: name.to_string(),
                data: DummyLabels {
                    labels: None,
                }
            },
            HashMap::from([("token".to_string(), token)]),
        )];

        let result = client.publish::<PublishedEventDto, _>(&pubsub_config.events_topic, messages, None, None).await;
        if let Err(err) = result {
            eprintln!("Publish failed: {}", err);
        }
    }

    Ok(())
}
